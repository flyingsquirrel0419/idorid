//! Pseudo terminals: /dev/ptmx and /dev/pts/N (raw mode; no line discipline
//! beyond what callers need: logwrapper and the console use them as pipes).

use crate::errno::*;
use crate::sync::WaitQueue;
use crate::task::Ctx;
use crate::vfs::*;
use parking_lot::Mutex;
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Weak};

pub struct PtyPair {
    n: u32,
    to_slave: Mutex<VecDeque<u8>>,
    to_master: Mutex<VecDeque<u8>>,
    wq: Arc<WaitQueue>,
    master_open: Mutex<bool>,
    slaves: Mutex<u32>,
    winsize: Mutex<[u16; 4]>,
}

static PTYS: std::sync::LazyLock<Mutex<(u32, HashMap<u32, Weak<PtyPair>>)>> =
    std::sync::LazyLock::new(|| Mutex::new((0, HashMap::new())));

pub struct PtyEnd {
    pair: Arc<PtyPair>,
    master: bool,
    id: u64,
}

pub fn open_master() -> FileRef {
    let mut t = PTYS.lock();
    let n = t.0;
    t.0 += 1;
    let pair = Arc::new(PtyPair {
        n,
        to_slave: Mutex::new(VecDeque::new()),
        to_master: Mutex::new(VecDeque::new()),
        wq: WaitQueue::new(),
        master_open: Mutex::new(true),
        slaves: Mutex::new(0),
        winsize: Mutex::new([24, 80, 0, 0]),
    });
    t.1.insert(n, Arc::downgrade(&pair));
    Arc::new(PtyEnd {
        pair,
        master: true,
        id: new_id(),
    })
}

pub fn open_slave(n: u32) -> Option<FileRef> {
    let pair = PTYS.lock().1.get(&n)?.upgrade()?;
    *pair.slaves.lock() += 1;
    Some(Arc::new(PtyEnd {
        pair,
        master: false,
        id: new_id(),
    }))
}

pub fn exists(n: u32) -> bool {
    PTYS.lock().1.get(&n).is_some_and(|w| w.strong_count() > 0)
}

pub fn list() -> Vec<u32> {
    PTYS.lock()
        .1
        .iter()
        .filter(|(_, w)| w.strong_count() > 0)
        .map(|(k, _)| *k)
        .collect()
}

impl Node for PtyEnd {
    fn id(&self) -> u64 {
        self.id
    }
    fn stat(&self) -> Stat {
        Stat {
            mode: S_IFCHR | 0o620,
            ino: self.id,
            rdev: if self.master {
                0x502
            } else {
                0x8800 + self.pair.n as u64
            },
            nlink: 1,
            blksize: 1024,
            ..Default::default()
        }
    }
    fn is_stream(&self) -> bool {
        true
    }
    fn read_at(&self, _o: u64, buf: &mut [u8], nonblock: bool) -> Result<usize> {
        let q = if self.master {
            &self.pair.to_master
        } else {
            &self.pair.to_slave
        };
        loop {
            let seen = self.pair.wq.epoch();
            {
                let mut d = q.lock();
                if !d.is_empty() {
                    let n = buf.len().min(d.len());
                    for (i, b) in d.drain(..n).enumerate() {
                        buf[i] = b;
                    }
                    drop(d);
                    self.pair.wq.notify();
                    return Ok(n);
                }
            }
            // Other side gone: master sees EIO once all slaves closed.
            if self.master && *self.pair.slaves.lock() == 0 && Arc::strong_count(&self.pair) <= 2 {
                return Err(EIO);
            }
            if !self.master && !*self.pair.master_open.lock() {
                return Ok(0);
            }
            if nonblock {
                return Err(EAGAIN);
            }
            if crate::task::current_signal_pending() {
                return Err(EINTR);
            }
            self.pair.wq.wait_change(seen, None);
        }
    }
    fn write_at(&self, _o: u64, buf: &[u8], _nb: bool) -> Result<usize> {
        let q = if self.master {
            &self.pair.to_slave
        } else {
            &self.pair.to_master
        };
        let mut d = q.lock();
        if self.master {
            d.extend(buf.iter().map(|&b| if b == b'\r' { b'\n' } else { b }));
        } else {
            // ONLCR
            for &b in buf {
                if b == b'\n' {
                    d.push_back(b'\r');
                }
                d.push_back(b);
            }
        }
        drop(d);
        self.pair.wq.notify();
        Ok(buf.len())
    }
    fn poll(&self) -> u32 {
        let q = if self.master {
            &self.pair.to_master
        } else {
            &self.pair.to_slave
        };
        let mut r = POLLOUT;
        if !q.lock().is_empty() {
            r |= POLLIN;
        }
        if self.master && *self.pair.slaves.lock() == 0 {
            r |= POLLHUP;
        }
        r
    }
    fn wait_queue(&self) -> Option<Arc<WaitQueue>> {
        Some(self.pair.wq.clone())
    }
    fn ioctl(&self, ctx: &Ctx, cmd: u32, arg: u64) -> Result<i64> {
        match cmd {
            0x80045430 => ctx
                .mm
                .write_u32(arg, self.pair.n)
                .then_some(0)
                .ok_or(EFAULT), // TIOCGPTN
            0x40045431 => Ok(0), // TIOCSPTLCK
            0x5401 => {
                let mut t = [0u8; 36];
                t[0..4].copy_from_slice(&0x500u32.to_le_bytes());
                t[4..8].copy_from_slice(&0x5u32.to_le_bytes());
                t[8..12].copy_from_slice(&0xbfu32.to_le_bytes());
                t[12..16].copy_from_slice(&0x8a3bu32.to_le_bytes());
                ctx.mm.write(arg, &t).then_some(0).ok_or(EFAULT)
            }
            0x5402..=0x5404 | 0x540e | 0x5410 | 0x5422 => Ok(0),
            0x5413 => {
                let w = *self.pair.winsize.lock();
                let b: Vec<u8> = w.iter().flat_map(|v| v.to_le_bytes()).collect();
                ctx.mm.write(arg, &b).then_some(0).ok_or(EFAULT)
            }
            0x5414 => {
                let mut b = [0u8; 8];
                if !ctx.mm.read(arg, &mut b) {
                    return Err(EFAULT);
                }
                *self.pair.winsize.lock() = [
                    u16::from_le_bytes([b[0], b[1]]),
                    u16::from_le_bytes([b[2], b[3]]),
                    u16::from_le_bytes([b[4], b[5]]),
                    u16::from_le_bytes([b[6], b[7]]),
                ];
                Ok(0)
            }
            0x540f => ctx
                .mm
                .write_u32(arg, ctx.process.pgid())
                .then_some(0)
                .ok_or(EFAULT),
            0x541b => {
                let q = if self.master {
                    &self.pair.to_master
                } else {
                    &self.pair.to_slave
                };
                let n = q.lock().len() as u32;
                ctx.mm.write_u32(arg, n).then_some(0).ok_or(EFAULT)
            }
            _ => Err(ENOTTY),
        }
    }
    fn release(&self) {
        if self.master {
            *self.pair.master_open.lock() = false;
        } else {
            let mut s = self.pair.slaves.lock();
            *s = s.saturating_sub(1);
        }
        self.pair.wq.notify();
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// /dev/pts as a synthetic file system.
pub struct PtsFs;

impl SyntheticFs for PtsFs {
    fn lookup(&self, _ctx: &Ctx, rel: &str) -> Result<SynthEntry> {
        if rel.is_empty() {
            let names = list().into_iter().map(|n| (n.to_string(), 2u8)).collect();
            return Ok(SynthEntry::Node(crate::procfs::SynthDir::new(names)));
        }
        if rel == "ptmx" {
            return Ok(SynthEntry::Node(open_master()));
        }
        let n: u32 = rel.parse().map_err(|_| ENOENT)?;
        open_slave(n).map(SynthEntry::Node).ok_or(ENOENT)
    }
}
