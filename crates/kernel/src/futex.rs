//! futex(2).

use crate::errno::*;
use crate::task::Ctx;
use parking_lot::{Condvar, Mutex};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

const FUTEX_WAIT: u32 = 0;
const FUTEX_WAKE: u32 = 1;
const FUTEX_REQUEUE: u32 = 3;
const FUTEX_CMP_REQUEUE: u32 = 4;
const FUTEX_WAKE_OP: u32 = 5;
const FUTEX_LOCK_PI: u32 = 6;
const FUTEX_UNLOCK_PI: u32 = 7;
const FUTEX_TRYLOCK_PI: u32 = 8;
const FUTEX_WAIT_BITSET: u32 = 9;
const FUTEX_WAKE_BITSET: u32 = 10;
const FUTEX_PRIVATE: u32 = 128;
const FUTEX_CLOCK_REALTIME: u32 = 256;

/// Futex identity: private futexes are keyed by address space + address,
/// shared ones by the host address of the backing memory.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Key {
    Private(usize, u64),
    Shared(usize),
}

struct Waiter {
    key: Mutex<Key>,
    bitset: u32,
    woken: Mutex<bool>,
    cv: Condvar,
}

#[derive(Default)]
pub struct Futexes {
    queues: Mutex<HashMap<Key, Vec<Arc<Waiter>>>>,
}

impl Futexes {
    fn key(ctx: &Ctx, addr: u64, private: bool) -> Key {
        let addr = crate::mm::untag(addr);
        if !private {
            if let Some(k) = ctx.mm.shared_key(addr) {
                return Key::Shared(k);
            }
        }
        Key::Private(Arc::as_ptr(&ctx.mm) as usize, addr)
    }

    pub fn wake_key(&self, key: Key, n: u32, bitset: u32) -> u32 {
        let mut q = self.queues.lock();
        let Some(list) = q.get_mut(&key) else {
            return 0;
        };
        let mut woken = 0;
        list.retain(|w| {
            if woken < n && w.bitset & bitset != 0 {
                *w.woken.lock() = true;
                w.cv.notify_one();
                woken += 1;
                false
            } else {
                true
            }
        });
        if list.is_empty() {
            q.remove(&key);
        }
        woken
    }

    /// Wakes one waiter on a private futex at `addr` in `mm` (thread exit
    /// CLONE_CHILD_CLEARTID).
    pub fn wake_addr(&self, mm_ptr: usize, addr: u64) {
        let addr = crate::mm::untag(addr);
        self.wake_key(Key::Private(mm_ptr, addr), 1, u32::MAX);
    }

    fn wait(
        &self,
        ctx: &Ctx,
        key: Key,
        ptr: *mut u32,
        val: u32,
        bitset: u32,
        deadline: Option<Instant>,
    ) -> i64 {
        let w = Arc::new(Waiter {
            key: Mutex::new(key),
            bitset,
            woken: Mutex::new(false),
            cv: Condvar::new(),
        });
        {
            let mut q = self.queues.lock();
            // Check the value under the queue lock (wakers take it too).
            let cur = unsafe { (*(ptr as *const AtomicU32)).load(Ordering::SeqCst) };
            if cur != val {
                return EAGAIN;
            }
            q.entry(key).or_default().push(w.clone());
        }
        let mut woken = w.woken.lock();
        loop {
            if *woken {
                return 0;
            }
            if ctx.signal_pending() {
                break;
            }
            let step = Duration::from_millis(20);
            match deadline {
                Some(d) => {
                    let now = Instant::now();
                    if now >= d {
                        drop(woken);
                        self.remove(&w);
                        return if *w.woken.lock() { 0 } else { ETIMEDOUT };
                    }
                    w.cv.wait_for(&mut woken, step.min(d - now));
                }
                None => {
                    w.cv.wait_for(&mut woken, step);
                }
            }
        }
        drop(woken);
        self.remove(&w);
        if *w.woken.lock() {
            0
        } else {
            EINTR
        }
    }

    fn remove(&self, w: &Arc<Waiter>) {
        let key = *w.key.lock();
        let mut q = self.queues.lock();
        if let Some(list) = q.get_mut(&key) {
            list.retain(|x| !Arc::ptr_eq(x, w));
            if list.is_empty() {
                q.remove(&key);
            }
        }
    }

    fn requeue(&self, from: Key, to: Key, nwake: u32, nreq: u32) -> u32 {
        let woken = self.wake_key(from, nwake, u32::MAX);
        let mut q = self.queues.lock();
        let Some(mut list) = q.remove(&from) else {
            return woken;
        };
        let n = (nreq as usize).min(list.len());
        let moved: Vec<_> = list.drain(..n).collect();
        if !list.is_empty() {
            q.insert(from, list);
        }
        for w in &moved {
            *w.key.lock() = to;
        }
        q.entry(to).or_default().extend(moved.iter().cloned());
        woken + n as u32
    }
}

pub fn sys_futex(
    ctx: &Ctx,
    uaddr: u64,
    op: u32,
    val: u32,
    timeout: u64,
    uaddr2: u64,
    val3: u32,
) -> i64 {
    let private = op & FUTEX_PRIVATE != 0;
    let cmd = op & !(FUTEX_PRIVATE | FUTEX_CLOCK_REALTIME);
    let fx = &ctx.kernel.futexes;
    let Some(ptr) = ctx.mm.futex_ptr(uaddr) else {
        return EFAULT;
    };
    let key = Futexes::key(ctx, uaddr, private);
    match cmd {
        FUTEX_WAIT | FUTEX_WAIT_BITSET => {
            let bitset = if cmd == FUTEX_WAIT { u32::MAX } else { val3 };
            if bitset == 0 {
                return EINVAL;
            }
            let deadline = if timeout == 0 {
                None
            } else {
                let Some(sec) = ctx.mm.read_u64(timeout) else {
                    return EFAULT;
                };
                let Some(nsec) = ctx.mm.read_u64(timeout + 8) else {
                    return EFAULT;
                };
                let d = Duration::new(sec, nsec as u32);
                if cmd == FUTEX_WAIT {
                    Some(Instant::now() + d) // relative
                } else {
                    // Absolute on CLOCK_MONOTONIC or CLOCK_REALTIME.
                    let now = if op & FUTEX_CLOCK_REALTIME != 0 {
                        crate::time::realtime()
                    } else {
                        crate::time::monotonic()
                    };
                    Some(Instant::now() + d.saturating_sub(now))
                }
            };
            fx.wait(ctx, key, ptr, val, bitset, deadline)
        }
        FUTEX_WAKE => fx.wake_key(key, val, u32::MAX) as i64,
        FUTEX_WAKE_BITSET => {
            if val3 == 0 {
                return EINVAL;
            }
            fx.wake_key(key, val, val3) as i64
        }
        FUTEX_REQUEUE | FUTEX_CMP_REQUEUE => {
            if cmd == FUTEX_CMP_REQUEUE {
                let cur = unsafe { (*(ptr as *const AtomicU32)).load(Ordering::SeqCst) };
                if cur != val3 {
                    return EAGAIN;
                }
            }
            if ctx.mm.futex_ptr(uaddr2).is_none() {
                return EFAULT;
            }
            let key2 = Futexes::key(ctx, uaddr2, private);
            fx.requeue(key, key2, val, timeout as u32) as i64
        }
        FUTEX_WAKE_OP => {
            let Some(p2) = ctx.mm.futex_ptr(uaddr2) else {
                return EFAULT;
            };
            let a2 = unsafe { &*(p2 as *const AtomicU32) };
            let opn = (val3 >> 28) & 7;
            let cmp = (val3 >> 24) & 15;
            let mut oparg = (val3 >> 12) & 0xfff;
            let cmparg = val3 & 0xfff;
            if (val3 >> 28) & 8 != 0 {
                oparg = 1u32.wrapping_shl(oparg);
            }
            let old = match opn {
                0 => a2.swap(oparg, Ordering::SeqCst),
                1 => a2.fetch_add(oparg, Ordering::SeqCst),
                2 => a2.fetch_or(oparg, Ordering::SeqCst),
                3 => a2.fetch_and(!oparg, Ordering::SeqCst),
                4 => a2.fetch_xor(oparg, Ordering::SeqCst),
                _ => return ENOSYS,
            };
            let mut n = fx.wake_key(key, val, u32::MAX);
            let hit = match cmp {
                0 => old == cmparg,
                1 => old != cmparg,
                2 => (old as i32) < cmparg as i32,
                3 => (old as i32) <= cmparg as i32,
                4 => (old as i32) > cmparg as i32,
                5 => (old as i32) >= cmparg as i32,
                _ => false,
            };
            if hit {
                let key2 = Futexes::key(ctx, uaddr2, private);
                n += fx.wake_key(key2, timeout as u32, u32::MAX);
            }
            n as i64
        }
        FUTEX_LOCK_PI | FUTEX_TRYLOCK_PI => {
            // Minimal PI support: spin/sleep until we can take the lock word.
            let a = unsafe { &*(ptr as *const AtomicU32) };
            let tid = ctx.thread.tid as u32;
            loop {
                let cur = a.load(Ordering::SeqCst);
                if cur & 0x3fff_ffff == 0 {
                    if a.compare_exchange(
                        cur,
                        tid | (cur & 0x8000_0000),
                        Ordering::SeqCst,
                        Ordering::SeqCst,
                    )
                    .is_ok()
                    {
                        return 0;
                    }
                    continue;
                }
                if cmd == FUTEX_TRYLOCK_PI {
                    return EAGAIN;
                }
                if cur & 0x3fff_ffff == tid {
                    return EDEADLK;
                }
                // Mark waiters and sleep.
                let _ =
                    a.compare_exchange(cur, cur | 0x8000_0000, Ordering::SeqCst, Ordering::SeqCst);
                let r = fx.wait(
                    ctx,
                    key,
                    ptr,
                    cur | 0x8000_0000,
                    u32::MAX,
                    Some(Instant::now() + Duration::from_millis(10)),
                );
                if r == EINTR {
                    return EINTR;
                }
            }
        }
        FUTEX_UNLOCK_PI => {
            let a = unsafe { &*(ptr as *const AtomicU32) };
            a.store(0, Ordering::SeqCst);
            fx.wake_key(key, 1, u32::MAX);
            0
        }
        _ => ENOSYS,
    }
}
