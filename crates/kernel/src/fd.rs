//! File descriptors, open file descriptions, pipes, eventfd, epoll.

use crate::errno::*;
use crate::sync::WaitQueue;
use crate::vfs::*;
use parking_lot::Mutex;
use std::collections::{BTreeMap, VecDeque};
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::sync::Arc;

pub const O_ACCMODE: u32 = 3;
pub const O_WRONLY: u32 = 1;
pub const O_RDWR: u32 = 2;
pub const O_CREAT: u32 = 0o100;
pub const O_EXCL: u32 = 0o200;
pub const O_NOCTTY: u32 = 0o400;
pub const O_TRUNC: u32 = 0o1000;
pub const O_APPEND: u32 = 0o2000;
pub const O_NONBLOCK: u32 = 0o4000;
// arm64 values:
pub const O_DIRECTORY: u32 = 0o40000;
pub const O_NOFOLLOW: u32 = 0o100000;
pub const O_CLOEXEC: u32 = 0o2000000;
pub const O_PATH: u32 = 0o10000000;
pub const O_TMPFILE: u32 = 0o20000000 | O_DIRECTORY;

/// An open file description (shared by dup'd fds and across fork).
pub struct OpenFile {
    pub node: FileRef,
    pub pos: Mutex<u64>,
    pub flags: AtomicU32,
    /// Guest path at open time (for /proc/self/fd and diagnostics).
    pub path: String,
    /// Directory listing cursor (index into readdir()).
    pub dir_cursor: Mutex<Option<(Vec<DirEntry>, usize)>>,
}

impl OpenFile {
    pub fn new(node: FileRef, flags: u32, path: impl Into<String>) -> Arc<OpenFile> {
        Arc::new(OpenFile {
            node,
            pos: Mutex::new(0),
            flags: AtomicU32::new(flags),
            path: path.into(),
            dir_cursor: Mutex::new(None),
        })
    }
    pub fn nonblock(&self) -> bool {
        self.flags.load(Ordering::Relaxed) & O_NONBLOCK != 0
    }
    pub fn readable(&self) -> bool {
        self.flags.load(Ordering::Relaxed) & O_ACCMODE != O_WRONLY
    }
    pub fn writable(&self) -> bool {
        self.flags.load(Ordering::Relaxed) & O_ACCMODE != 0
    }
}

impl Drop for OpenFile {
    fn drop(&mut self) {
        self.node.release();
    }
}

#[derive(Clone)]
pub struct Fd {
    pub file: Arc<OpenFile>,
    pub cloexec: bool,
}

#[derive(Clone, Default)]
pub struct FdTable {
    pub fds: BTreeMap<i32, Fd>,
    pub limit: i32,
}

impl FdTable {
    pub fn new() -> FdTable {
        FdTable {
            fds: BTreeMap::new(),
            limit: 32768,
        }
    }
    pub fn get(&self, fd: i32) -> Result<Arc<OpenFile>> {
        self.fds.get(&fd).map(|f| f.file.clone()).ok_or(EBADF)
    }
    pub fn lowest_free(&self, from: i32) -> Result<i32> {
        let mut n = from;
        for (&k, _) in self.fds.range(from..) {
            if k != n {
                break;
            }
            n += 1;
        }
        if n >= self.limit {
            Err(EMFILE)
        } else {
            Ok(n)
        }
    }
    pub fn insert(&mut self, file: Arc<OpenFile>, cloexec: bool) -> Result<i32> {
        let n = self.lowest_free(0)?;
        self.fds.insert(n, Fd { file, cloexec });
        Ok(n)
    }
    pub fn insert_at(&mut self, n: i32, file: Arc<OpenFile>, cloexec: bool) {
        self.fds.insert(n, Fd { file, cloexec });
    }
    pub fn close(&mut self, fd: i32) -> Result<()> {
        self.fds.remove(&fd).map(|_| ()).ok_or(EBADF)
    }
    pub fn close_on_exec(&mut self) {
        self.fds.retain(|_, f| !f.cloexec);
    }
}

// ---------------------------------------------------------------------------
// Pipes.

pub struct PipeBuf {
    pub data: Mutex<PipeState>,
    pub wq: Arc<WaitQueue>,
    id: u64,
}

pub struct PipeState {
    pub buf: VecDeque<u8>,
    pub readers: u32,
    pub writers: u32,
    pub cap: usize,
}

pub struct PipeEnd {
    pub pipe: Arc<PipeBuf>,
    pub write_end: bool,
}

pub fn pipe() -> (FileRef, FileRef) {
    let p = Arc::new(PipeBuf {
        data: Mutex::new(PipeState {
            buf: VecDeque::new(),
            readers: 1,
            writers: 1,
            cap: 65536,
        }),
        wq: WaitQueue::new(),
        id: new_id(),
    });
    (
        Arc::new(PipeEnd {
            pipe: p.clone(),
            write_end: false,
        }),
        Arc::new(PipeEnd {
            pipe: p,
            write_end: true,
        }),
    )
}

impl Node for PipeEnd {
    fn id(&self) -> u64 {
        self.pipe.id
    }
    fn stat(&self) -> Stat {
        Stat {
            mode: S_IFIFO | 0o600,
            ino: self.pipe.id,
            nlink: 1,
            blksize: 4096,
            ..Default::default()
        }
    }
    fn is_stream(&self) -> bool {
        true
    }
    fn read_at(&self, _off: u64, buf: &mut [u8], nonblock: bool) -> Result<usize> {
        loop {
            let seen = self.pipe.wq.epoch();
            {
                let mut s = self.pipe.data.lock();
                if !s.buf.is_empty() {
                    let n = buf.len().min(s.buf.len());
                    for (i, b) in s.buf.drain(..n).enumerate() {
                        buf[i] = b;
                    }
                    drop(s);
                    self.pipe.wq.notify();
                    return Ok(n);
                }
                if s.writers == 0 {
                    return Ok(0);
                }
                if nonblock {
                    return Err(EAGAIN);
                }
            }
            if crate::task::current_signal_pending() {
                return Err(EINTR);
            }
            self.pipe.wq.wait_change(seen, None);
        }
    }
    fn write_at(&self, _off: u64, buf: &[u8], nonblock: bool) -> Result<usize> {
        let mut done = 0;
        loop {
            let seen = self.pipe.wq.epoch();
            {
                let mut s = self.pipe.data.lock();
                if s.readers == 0 {
                    crate::task::raise_current(13); // SIGPIPE
                    return Err(EPIPE);
                }
                let room = s.cap.saturating_sub(s.buf.len());
                if room > 0 {
                    let n = room.min(buf.len() - done);
                    s.buf.extend(&buf[done..done + n]);
                    done += n;
                    drop(s);
                    self.pipe.wq.notify();
                    if done == buf.len() {
                        return Ok(done);
                    }
                    continue;
                }
                if nonblock {
                    return if done > 0 { Ok(done) } else { Err(EAGAIN) };
                }
            }
            if crate::task::current_signal_pending() {
                return if done > 0 { Ok(done) } else { Err(EINTR) };
            }
            self.pipe.wq.wait_change(seen, None);
        }
    }
    fn poll(&self) -> u32 {
        let s = self.pipe.data.lock();
        if self.write_end {
            if s.readers == 0 {
                POLLERR | POLLOUT
            } else if s.buf.len() < s.cap {
                POLLOUT
            } else {
                0
            }
        } else {
            let mut r = 0;
            if !s.buf.is_empty() {
                r |= POLLIN;
            }
            if s.writers == 0 {
                r |= POLLHUP;
            }
            r
        }
    }
    fn wait_queue(&self) -> Option<Arc<WaitQueue>> {
        Some(self.pipe.wq.clone())
    }
    fn ioctl(&self, ctx: &crate::task::Ctx, cmd: u32, arg: u64) -> Result<i64> {
        if cmd == 0x541b {
            // FIONREAD
            let n = self.pipe.data.lock().buf.len() as u32;
            ctx.mm.write_u32(arg, n).then_some(0).ok_or(EFAULT)
        } else {
            Err(ENOTTY)
        }
    }
    fn release(&self) {
        let mut s = self.pipe.data.lock();
        if self.write_end {
            s.writers -= 1;
        } else {
            s.readers -= 1;
        }
        drop(s);
        self.pipe.wq.notify();
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

// ---------------------------------------------------------------------------
// eventfd

pub struct EventFd {
    count: Mutex<u64>,
    semaphore: bool,
    wq: Arc<WaitQueue>,
    id: u64,
}

impl EventFd {
    pub fn new(init: u64, semaphore: bool) -> Arc<EventFd> {
        Arc::new(EventFd {
            count: Mutex::new(init),
            semaphore,
            wq: WaitQueue::new(),
            id: new_id(),
        })
    }
}

impl Node for EventFd {
    fn id(&self) -> u64 {
        self.id
    }
    fn stat(&self) -> Stat {
        Stat {
            mode: 0o600,
            ino: self.id,
            nlink: 1,
            ..Default::default()
        }
    }
    fn is_stream(&self) -> bool {
        true
    }
    fn read_at(&self, _o: u64, buf: &mut [u8], nonblock: bool) -> Result<usize> {
        if buf.len() < 8 {
            return Err(EINVAL);
        }
        loop {
            let seen = self.wq.epoch();
            {
                let mut c = self.count.lock();
                if *c > 0 {
                    let v = if self.semaphore { 1 } else { *c };
                    *c -= v;
                    buf[..8].copy_from_slice(&v.to_le_bytes());
                    drop(c);
                    self.wq.notify();
                    return Ok(8);
                }
            }
            if nonblock {
                return Err(EAGAIN);
            }
            if crate::task::current_signal_pending() {
                return Err(EINTR);
            }
            self.wq.wait_change(seen, None);
        }
    }
    fn write_at(&self, _o: u64, buf: &[u8], _nb: bool) -> Result<usize> {
        if buf.len() < 8 {
            return Err(EINVAL);
        }
        let v = u64::from_le_bytes(buf[..8].try_into().unwrap());
        if v == u64::MAX {
            return Err(EINVAL);
        }
        *self.count.lock() += v;
        self.wq.notify();
        Ok(8)
    }
    fn poll(&self) -> u32 {
        if *self.count.lock() > 0 {
            POLLIN | POLLOUT
        } else {
            POLLOUT
        }
    }
    fn wait_queue(&self) -> Option<Arc<WaitQueue>> {
        Some(self.wq.clone())
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

// ---------------------------------------------------------------------------
// epoll

pub const EPOLLET: u32 = 1 << 31;
pub const EPOLLONESHOT: u32 = 1 << 30;

pub struct EpollItem {
    pub file: std::sync::Weak<OpenFile>,
    pub events: u32,
    pub data: u64,
    /// For edge-triggered: last readiness reported.
    pub last: u32,
    pub disabled: bool,
}

pub struct Epoll {
    pub items: Mutex<BTreeMap<i32, EpollItem>>,
    pub wq: Arc<WaitQueue>,
    id: u64,
    pub gen: AtomicU64,
}

impl Epoll {
    pub fn new() -> Arc<Epoll> {
        Arc::new(Epoll {
            items: Mutex::new(BTreeMap::new()),
            wq: WaitQueue::new(),
            id: new_id(),
            gen: AtomicU64::new(0),
        })
    }

    /// Collects ready events (at most `max`).
    pub fn collect(&self, max: usize) -> Vec<(u32, u64)> {
        let mut out = Vec::new();
        let mut items = self.items.lock();
        let mut dead = Vec::new();
        for (fd, it) in items.iter_mut() {
            if out.len() >= max {
                break;
            }
            let Some(f) = it.file.upgrade() else {
                dead.push(*fd);
                continue;
            };
            if it.disabled {
                continue;
            }
            let ready = f.node.poll() & (it.events | POLLERR | POLLHUP);
            if ready == 0 {
                it.last = 0;
                continue;
            }
            if it.events & EPOLLET != 0 {
                if ready == it.last {
                    continue;
                }
            }
            it.last = ready;
            if it.events & EPOLLONESHOT != 0 {
                it.disabled = true;
            }
            out.push((ready, it.data));
        }
        for d in dead {
            items.remove(&d);
        }
        out
    }

    /// Wait queues of all watched files (to sleep on).
    pub fn queues(&self) -> Vec<Arc<WaitQueue>> {
        self.items
            .lock()
            .values()
            .filter_map(|it| it.file.upgrade())
            .filter_map(|f| f.node.wait_queue())
            .collect()
    }
}

impl Node for Epoll {
    fn id(&self) -> u64 {
        self.id
    }
    fn stat(&self) -> Stat {
        Stat {
            mode: 0o600,
            ino: self.id,
            nlink: 1,
            ..Default::default()
        }
    }
    fn is_stream(&self) -> bool {
        true
    }
    fn poll(&self) -> u32 {
        // Readable if any watched item is ready (nested epoll).
        let items = self.items.lock();
        for it in items.values() {
            if let Some(f) = it.file.upgrade() {
                if !it.disabled && f.node.poll() & (it.events | POLLERR | POLLHUP) != 0 {
                    return POLLIN;
                }
            }
        }
        0
    }
    fn wait_queue(&self) -> Option<Arc<WaitQueue>> {
        Some(self.wq.clone())
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

// ---------------------------------------------------------------------------
// Simple character devices.

pub struct NullDev {
    pub kind: u8, // 0 null, 1 zero, 2 random, 3 full, 4 empty block device
    id: u64,
}

impl NullDev {
    pub fn new(kind: u8) -> Arc<NullDev> {
        Arc::new(NullDev { kind, id: new_id() })
    }
}

impl Node for NullDev {
    fn id(&self) -> u64 {
        self.id
    }
    fn stat(&self) -> Stat {
        Stat {
            mode: if self.kind == 4 {
                S_IFBLK | 0o600
            } else {
                S_IFCHR | 0o666
            },
            ino: self.id,
            rdev: [0x103, 0x105, 0x109, 0x107, 0xfd20][self.kind as usize],
            nlink: 1,
            ..Default::default()
        }
    }
    fn is_stream(&self) -> bool {
        true
    }
    fn read_at(&self, _o: u64, buf: &mut [u8], _nb: bool) -> Result<usize> {
        match self.kind {
            0 => Ok(0),
            2 => {
                fill_random(buf);
                Ok(buf.len())
            }
            _ => {
                buf.fill(0);
                Ok(buf.len())
            }
        }
    }
    fn write_at(&self, _o: u64, buf: &[u8], _nb: bool) -> Result<usize> {
        if self.kind == 3 {
            Err(ENOSPC)
        } else {
            Ok(buf.len())
        }
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub fn fill_random(buf: &mut [u8]) {
    use std::io::Read;
    if let Ok(mut f) = std::fs::File::open("/dev/urandom") {
        if f.read_exact(buf).is_ok() {
            return;
        }
    }
    let mut x = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(1)
        | 1;
    for b in buf {
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        *b = x as u8;
    }
}

/// Console: guest stdout/stderr/kmsg go to the embedder's log sink.
pub struct Console {
    pub sink: Arc<dyn Fn(&[u8]) + Send + Sync>,
    pub input: Mutex<VecDeque<u8>>,
    pub wq: Arc<WaitQueue>,
    id: u64,
}

impl Console {
    pub fn new(sink: Arc<dyn Fn(&[u8]) + Send + Sync>) -> Arc<Console> {
        Arc::new(Console {
            sink,
            input: Mutex::new(VecDeque::new()),
            wq: WaitQueue::new(),
            id: new_id(),
        })
    }
}

impl Node for Console {
    fn id(&self) -> u64 {
        self.id
    }
    fn stat(&self) -> Stat {
        Stat {
            mode: S_IFCHR | 0o620,
            ino: self.id,
            rdev: 0x501,
            nlink: 1,
            blksize: 1024,
            ..Default::default()
        }
    }
    fn is_stream(&self) -> bool {
        true
    }
    fn read_at(&self, _o: u64, buf: &mut [u8], nonblock: bool) -> Result<usize> {
        loop {
            let seen = self.wq.epoch();
            {
                let mut q = self.input.lock();
                if !q.is_empty() {
                    let n = buf.len().min(q.len());
                    for (i, b) in q.drain(..n).enumerate() {
                        buf[i] = b;
                    }
                    return Ok(n);
                }
            }
            if nonblock {
                return Err(EAGAIN);
            }
            if crate::task::current_signal_pending() {
                return Err(EINTR);
            }
            self.wq.wait_change(seen, None);
        }
    }
    fn write_at(&self, _o: u64, buf: &[u8], _nb: bool) -> Result<usize> {
        (self.sink)(buf);
        Ok(buf.len())
    }
    fn poll(&self) -> u32 {
        if self.input.lock().is_empty() {
            POLLOUT
        } else {
            POLLIN | POLLOUT
        }
    }
    fn wait_queue(&self) -> Option<Arc<WaitQueue>> {
        Some(self.wq.clone())
    }
    fn ioctl(&self, ctx: &crate::task::Ctx, cmd: u32, arg: u64) -> Result<i64> {
        match cmd {
            // TCGETS: report a sane termios so isatty() is true.
            0x5401 => {
                let mut t = [0u8; 36];
                t[0..4].copy_from_slice(&0x500u32.to_le_bytes()); // iflag ICRNL|IXON
                t[4..8].copy_from_slice(&0x5u32.to_le_bytes()); // oflag OPOST|ONLCR
                t[8..12].copy_from_slice(&0xbfu32.to_le_bytes()); // cflag
                t[12..16].copy_from_slice(&0x8a3bu32.to_le_bytes()); // lflag
                ctx.mm.write(arg, &t).then_some(0).ok_or(EFAULT)
            }
            0x5402..=0x5404 => Ok(0), // TCSETS*
            0x5413 => {
                // TIOCGWINSZ
                let ws: [u16; 4] = [50, 120, 0, 0];
                let b: Vec<u8> = ws.iter().flat_map(|v| v.to_le_bytes()).collect();
                ctx.mm.write(arg, &b).then_some(0).ok_or(EFAULT)
            }
            0x540f => ctx
                .mm
                .write_u32(arg, ctx.process.pgid())
                .then_some(0)
                .ok_or(EFAULT), // TIOCGPGRP
            0x5410 | 0x540e | 0x5414 => Ok(0), // TIOCSPGRP, TIOCSCTTY, TIOCSWINSZ
            _ => Err(ENOTTY),
        }
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
