//! Blocking primitives for guest threads.

use parking_lot::{Condvar, Mutex};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// A broadcast wait queue: waiters sleep until `notify` bumps the epoch (or a
/// timeout / interruption). Used for pipes, sockets, eventfds, epoll, futexes.
#[derive(Default)]
pub struct WaitQueue {
    epoch: Mutex<u64>,
    cv: Condvar,
}

impl WaitQueue {
    pub fn new() -> Arc<WaitQueue> {
        Arc::new(WaitQueue::default())
    }
    pub fn epoch(&self) -> u64 {
        *self.epoch.lock()
    }
    pub fn notify(&self) {
        *self.epoch.lock() += 1;
        self.cv.notify_all();
    }
    /// Sleeps until the epoch differs from `seen`, or the deadline passes.
    /// Returns false on timeout.
    pub fn wait_change(&self, seen: u64, deadline: Option<Instant>) -> bool {
        let mut e = self.epoch.lock();
        while *e == seen {
            match deadline {
                Some(d) => {
                    let now = Instant::now();
                    if now >= d {
                        return false;
                    }
                    // Wake periodically so signal delivery is noticed.
                    let step = (d - now).min(Duration::from_millis(50));
                    self.cv.wait_for(&mut e, step);
                }
                None => {
                    self.cv.wait_for(&mut e, Duration::from_millis(50));
                    if *e == seen {
                        return true; // spurious; caller re-checks + signals
                    }
                }
            }
        }
        true
    }
}
