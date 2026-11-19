//! Initial virtio-gpu DRM device boundary.
//!
//! Only discovery ioctls are implemented; resource and submission operations
//! deliberately return ENOTTY until their memory and fence lifetimes exist.
use crate::errno::*;
use crate::mm::SharedMem;
use crate::task::Ctx;
use crate::vfs::{self, Node, Stat};
use parking_lot::Mutex;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;

/// Renderer discovery, independent of the host renderer implementation.
pub struct GpuFence {
    signaled: AtomicBool,
    queue: Arc<crate::sync::WaitQueue>,
}
impl Default for GpuFence {
    fn default() -> Self {
        Self {
            signaled: AtomicBool::new(false),
            queue: crate::sync::WaitQueue::new(),
        }
    }
}
impl GpuFence {
    pub fn signal(&self) {
        self.signaled.store(true, Ordering::Release);
        self.queue.notify();
    }
    fn wait(&self, nowait: bool) -> vfs::Result<()> {
        loop {
            let epoch = self.queue.epoch();
            if self.signaled.load(Ordering::Acquire) {
                return Ok(());
            }
            if nowait {
                return Err(EBUSY);
            }
            if crate::task::current_signal_pending() {
                return Err(EINTR);
            }
            self.queue.wait_change(epoch, None);
        }
    }
}
struct DmaBuf {
    id: u64,
    resource: Arc<dyn GpuResource>,
    size: u64,
    mem: u32,
}
impl Node for DmaBuf {
    fn id(&self) -> u64 {
        self.id
    }
    fn stat(&self) -> Stat {
        Stat {
            ino: self.id,
            mode: vfs::S_IFREG | 0o600,
            nlink: 1,
            size: self.size as i64,
            ..Default::default()
        }
    }
    fn shared_mem(&self) -> Option<Arc<SharedMem>> {
        self.resource.clone().map().ok()
    }
    fn mmap_requires_shared_mem(&self) -> bool {
        true
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
struct SyncFile {
    id: u64,
    fence: Arc<GpuFence>,
}
impl Node for SyncFile {
    fn id(&self) -> u64 {
        self.id
    }
    fn stat(&self) -> Stat {
        Stat {
            ino: self.id,
            mode: vfs::S_IFREG | 0o600,
            nlink: 1,
            ..Default::default()
        }
    }
    fn poll(&self) -> u32 {
        if self.fence.signaled.load(Ordering::Acquire) {
            vfs::POLLIN
        } else {
            0
        }
    }
    fn wait_queue(&self) -> Option<Arc<crate::sync::WaitQueue>> {
        Some(self.fence.queue.clone())
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[derive(Clone, Copy)]
pub struct ResourceArgs {
    pub values: [u32; 10],
    pub size: u32,
}
#[derive(Clone, Copy)]
pub struct TransferArgs {
    pub region: [u32; 6],
    pub level: u32,
    pub offset: u32,
    pub stride: u32,
    pub layer_stride: u32,
}
pub trait GpuResource: Send + Sync {
    fn id(&self) -> u32;
    fn transfer(&self, _ctx: u32, _to_host: bool, _args: TransferArgs) -> vfs::Result<()> {
        Err(ENOSYS)
    }

    fn map(self: Arc<Self>) -> vfs::Result<Arc<SharedMem>>;
}

pub trait GpuBackend: Send + Sync {
    /// Return the supported version and bytes for a capability set.
    fn capset(&self, id: u32, version: u32) -> vfs::Result<Vec<u8>>;
    fn supports_context(&self) -> bool {
        false
    }
    fn context_create(&self, _id: u32, _capset: u32) -> vfs::Result<()> {
        Err(ENOSYS)
    }
    fn context_destroy(&self, _id: u32) {}
    fn create_3d(
        &self,
        _ctx: u32,
        _id: u32,
        _args: ResourceArgs,
    ) -> vfs::Result<Arc<dyn GpuResource>> {
        Err(ENOSYS)
    }
    fn create_blob(
        &self,
        _ctx: u32,
        _id: u32,
        _mem: u32,
        _flags: u32,
        _blob_id: u64,
        _size: u64,
    ) -> vfs::Result<Arc<dyn GpuResource>> {
        Err(ENOSYS)
    }
    fn submit(&self, _ctx: u32, _cmd: &mut [u8]) -> vfs::Result<()> {
        Err(ENOSYS)
    }
    fn fence(&self, _ctx: u32, _ring: Option<u8>, _signal: Arc<GpuFence>) -> vfs::Result<()> {
        Err(ENOSYS)
    }
}

pub type BackendSlot = Arc<RwLock<Option<Arc<dyn GpuBackend>>>>;

pub struct VirtGpu {
    id: u64,
    minor: u32,
    backend: BackendSlot,
    state: Mutex<State>,
}
struct State {
    context: Option<(Arc<dyn GpuBackend>, u32)>,
    resources: HashMap<u32, (Arc<dyn GpuResource>, u64, u32)>,
    next_handle: u32,
    last_fence: Option<Arc<GpuFence>>,
}
impl State {
    fn ensure_context(
        &mut self,
        backend: &Arc<dyn GpuBackend>,
    ) -> vfs::Result<(Arc<dyn GpuBackend>, u32)> {
        if let Some(context) = &self.context {
            return Ok(context.clone());
        }
        let id = NEXT_CONTEXT.fetch_add(1, Ordering::Relaxed);
        backend.context_create(id, 0)?;
        self.context = Some((backend.clone(), id));
        Ok((backend.clone(), id))
    }
}
static NEXT_CONTEXT: AtomicU32 = AtomicU32::new(1);
static NEXT_RESOURCE: AtomicU32 = AtomicU32::new(1);

impl VirtGpu {
    pub fn open(backend: BackendSlot, minor: u32) -> vfs::FileRef {
        Arc::new(Self {
            id: vfs::new_id(),
            minor,
            backend,
            state: Mutex::new(State {
                context: None,
                resources: HashMap::new(),
                next_handle: 1,
                last_fence: None,
            }),
        })
    }
}
impl Node for VirtGpu {
    fn id(&self) -> u64 {
        self.id
    }
    fn stat(&self) -> Stat {
        Stat {
            ino: self.id,
            mode: vfs::S_IFCHR | 0o666,
            nlink: 1,
            rdev: (226 << 8) | self.minor as u64,
            ..Default::default()
        }
    }
    fn ioctl(&self, ctx: &Ctx, cmd: u32, arg: u64) -> vfs::Result<i64> {
        if std::env::var_os("IDORID_GPU_TRACE").is_some() {
            eprintln!("[virtgpu] ioctl {cmd:#010x} arg={arg:#x}");
        }
        let backend = self.backend.read().clone().ok_or(ENODEV)?;
        match cmd {
            // No PRIME, syncobj, modesetting or dumb buffer support yet.
            0xc010_640c => {
                let cap = ctx.mm.read_u64(arg).ok_or(EFAULT)?;
                let value = match cap {
                    5 if backend.supports_context() => 3,
                    1..=5 | 7..=9 | 0x10..=0x15 => 0,
                    6 => 1, // DRM_CAP_TIMESTAMP_MONOTONIC
                    _ => return Err(EINVAL),
                };
                if !ctx.mm.write_u64(arg + 8, value) {
                    return Err(EFAULT);
                }
                Ok(0)
            }
            // drm_version: the three size_t/pointer pairs are 8-byte aligned.
            0xc040_6400 => {
                for (off, value) in [(0, 0), (4, 1), (8, 0)] {
                    if !ctx.mm.write_u32(arg + off, value) {
                        return Err(EFAULT);
                    }
                }
                for (off, value) in [
                    (16, &b"virtio_gpu"[..]),
                    (32, &b"0"[..]),
                    (48, &b"virtio GPU"[..]),
                ] {
                    let size = ctx.mm.read_u64(arg + off).ok_or(EFAULT)?;
                    let ptr = ctx.mm.read_u64(arg + off + 8).ok_or(EFAULT)?;
                    let n = (size as usize).min(value.len());
                    if n != 0 && !ctx.mm.write(ptr, &value[..n]) {
                        return Err(EFAULT);
                    }
                    if !ctx.mm.write_u64(arg + off, value.len() as u64) {
                        return Err(EFAULT);
                    }
                }
                Ok(0)
            }
            // GETPARAM: no resource/context features are advertised until implemented.
            0xc010_6443 => {
                let param = ctx.mm.read_u64(arg).ok_or(EFAULT)?;
                let ptr = ctx.mm.read_u64(arg + 8).ok_or(EFAULT)?;
                let value = match param {
                    1 | 2 | 3 | 4 | 6 if backend.supports_context() => 1,
                    7 if backend.supports_context() => 1 << 3,
                    1..=7 => 0,
                    _ => return Err(EINVAL),
                };
                if !ctx.mm.write_u64(ptr, value) {
                    return Err(EFAULT);
                }
                Ok(0)
            }
            0xc00c_642d => {
                let handle = ctx.mm.read_u32(arg).ok_or(EFAULT)?;
                let flags = ctx.mm.read_u32(arg + 4).ok_or(EFAULT)?;
                if flags & !3 != 0 {
                    return Err(EINVAL);
                }
                let state = self.state.lock();
                let (resource, size, mem) = state.resources.get(&handle).ok_or(ENOENT)?;
                let node = Arc::new(DmaBuf {
                    id: vfs::new_id(),
                    resource: resource.clone(),
                    size: *size,
                    mem: *mem,
                });
                drop(state);
                let file = crate::fd::OpenFile::new(node, crate::fd::O_RDWR, "anon_inode:dmabuf");
                let table = ctx.fds();
                let fd = table.lock().insert(file, flags & 1 != 0)?;
                if !ctx.mm.write_u32(arg + 8, fd as u32) {
                    let _ = table.lock().close(fd);
                    return Err(EFAULT);
                }
                Ok(0)
            }
            0xc00c_642e => {
                let fd = ctx.mm.read_u32(arg + 8).ok_or(EFAULT)? as i32;
                let table = ctx.fds();
                let file = table.lock().get(fd)?;
                let buffer = file.node.as_any().downcast_ref::<DmaBuf>().ok_or(EINVAL)?;
                let mut state = self.state.lock();
                let handle = state.next_handle;
                state.next_handle = handle.checked_add(1).ok_or(ENOMEM)?;
                state
                    .resources
                    .insert(handle, (buffer.resource.clone(), buffer.size, buffer.mem));
                if !ctx.mm.write_u32(arg, handle) {
                    state.resources.remove(&handle);
                    return Err(EFAULT);
                }
                Ok(0)
            }
            0xc038_6444 => {
                let mut values = [0u32; 10];
                for (i, value) in values.iter_mut().enumerate() {
                    *value = ctx.mm.read_u32(arg + i as u64 * 4).ok_or(EFAULT)?;
                }
                let size = ctx.mm.read_u32(arg + 48).ok_or(EFAULT)?;
                if size == 0 || size > (1 << 30) {
                    return Err(EINVAL);
                }
                let mut state = self.state.lock();
                let (backend, context) = state.ensure_context(&backend)?;
                let id = NEXT_RESOURCE.fetch_add(1, Ordering::Relaxed);
                let resource = backend.create_3d(context, id, ResourceArgs { values, size })?;
                let handle = state.next_handle;
                state.next_handle = handle.checked_add(1).ok_or(ENOMEM)?;
                state.resources.insert(handle, (resource, size as u64, 0));
                if !ctx.mm.write_u32(arg + 40, handle) || !ctx.mm.write_u32(arg + 44, id) {
                    state.resources.remove(&handle);
                    return Err(EFAULT);
                }
                Ok(0)
            }
            0xc02c_6446 | 0xc02c_6447 => {
                let handle = ctx.mm.read_u32(arg).ok_or(EFAULT)?;
                let mut region = [0u32; 6];
                for (i, value) in region.iter_mut().enumerate() {
                    *value = ctx.mm.read_u32(arg + 4 + i as u64 * 4).ok_or(EFAULT)?;
                }
                let args = TransferArgs {
                    region,
                    level: ctx.mm.read_u32(arg + 28).ok_or(EFAULT)?,
                    offset: ctx.mm.read_u32(arg + 32).ok_or(EFAULT)?,
                    stride: ctx.mm.read_u32(arg + 36).ok_or(EFAULT)?,
                    layer_stride: ctx.mm.read_u32(arg + 40).ok_or(EFAULT)?,
                };
                let state = self.state.lock();
                let (_, context) = state.context.as_ref().ok_or(EINVAL)?;
                let (resource, _, _) = state.resources.get(&handle).ok_or(ENOENT)?;
                resource.transfer(*context, cmd == 0xc02c_6447, args)?;
                Ok(0)
            }
            // CONTEXT_INIT (the Linux ABI permits setting parameters only once).
            0xc010_644b => {
                let count = ctx.mm.read_u32(arg).ok_or(EFAULT)?;
                let ptr = ctx.mm.read_u64(arg + 8).ok_or(EFAULT)?;
                if count > 64 {
                    return Err(EINVAL);
                }
                let mut capset = 0u32;
                for i in 0..count as u64 {
                    let key = ctx.mm.read_u64(ptr + i * 16).ok_or(EFAULT)?;
                    let value = ctx.mm.read_u64(ptr + i * 16 + 8).ok_or(EFAULT)?;
                    match key {
                        1 if value <= u32::MAX as u64 => capset = value as u32,
                        2 if value > 0 && value <= 64 => (),
                        3 => (),
                        _ => return Err(EINVAL),
                    }
                }
                let mut state = self.state.lock();
                if state.context.is_some() {
                    return Err(EBUSY);
                }
                let id = NEXT_CONTEXT.fetch_add(1, Ordering::Relaxed);
                backend.context_create(id, capset)?;
                state.context = Some((backend, id));
                Ok(0)
            }
            // CREATE_BLOB: command payloads are submitted before allocation.
            0xc030_644a => {
                let mem = ctx.mm.read_u32(arg).ok_or(EFAULT)?;
                let flags = ctx.mm.read_u32(arg + 4).ok_or(EFAULT)?;
                let size = ctx.mm.read_u64(arg + 16).ok_or(EFAULT)?;
                let cmd_size = ctx.mm.read_u32(arg + 28).ok_or(EFAULT)? as usize;
                let cmd_ptr = ctx.mm.read_u64(arg + 32).ok_or(EFAULT)?;
                let blob_id = ctx.mm.read_u64(arg + 40).ok_or(EFAULT)?;
                if !(1..=3).contains(&mem)
                    || flags & !7 != 0
                    || size == 0
                    || size > (1 << 32)
                    || cmd_size > (1 << 20)
                {
                    return Err(EINVAL);
                }
                let mut state = self.state.lock();
                let (backend, context) = state.ensure_context(&backend)?;
                if cmd_size != 0 {
                    let mut cmd = vec![0; cmd_size];
                    if !ctx.mm.read(cmd_ptr, &mut cmd) {
                        return Err(EFAULT);
                    }
                    backend.submit(context, &mut cmd)?;
                }
                let id = NEXT_RESOURCE.fetch_add(1, Ordering::Relaxed);
                let resource = backend.create_blob(context, id, mem, flags, blob_id, size)?;
                let handle = state.next_handle;
                state.next_handle = handle.checked_add(1).ok_or(ENOMEM)?;
                state.resources.insert(handle, (resource, size, mem));
                if !ctx.mm.write_u32(arg + 8, handle) || !ctx.mm.write_u32(arg + 12, id) {
                    state.resources.remove(&handle);
                    return Err(EFAULT);
                }
                Ok(0)
            }
            0xc010_6441 => {
                let handle = ctx.mm.read_u32(arg + 8).ok_or(EFAULT)?;
                let state = self.state.lock();
                if !state.resources.contains_key(&handle) {
                    return Err(ENOENT);
                }
                if !ctx.mm.write_u64(arg, (handle as u64) << 32) {
                    return Err(EFAULT);
                }
                Ok(0)
            }
            0xc010_6445 => {
                let handle = ctx.mm.read_u32(arg).ok_or(EFAULT)?;
                let state = self.state.lock();
                let (resource, size, mem) = state.resources.get(&handle).ok_or(ENOENT)?;
                if !ctx.mm.write_u32(arg + 4, resource.id())
                    || !ctx
                        .mm
                        .write_u32(arg + 8, (*size).min(u32::MAX as u64) as u32)
                    || !ctx.mm.write_u32(arg + 12, *mem)
                {
                    return Err(EFAULT);
                }
                Ok(0)
            }
            0x4008_6409 => {
                let handle = ctx.mm.read_u32(arg).ok_or(EFAULT)?;
                self.state.lock().resources.remove(&handle).ok_or(ENOENT)?;
                Ok(0)
            }
            0xc040_6442 | 0xc028_6442 | 0xc020_6442 => {
                let flags = ctx.mm.read_u32(arg).ok_or(EFAULT)?;
                let len = ctx.mm.read_u32(arg + 4).ok_or(EFAULT)? as usize;
                let ptr = ctx.mm.read_u64(arg + 8).ok_or(EFAULT)?;
                if flags & !7 != 0 || len > (1 << 20) {
                    eprintln!("virtgpu: execbuffer flags {flags:#x} len {len}");
                    return Err(EINVAL);
                }
                if flags & 1 != 0 {
                    let fd = ctx.mm.read_u32(arg + 28).ok_or(EFAULT)? as i32;
                    let table = ctx.fds();
                    let file = table.lock().get(fd)?;
                    let sync = file
                        .node
                        .as_any()
                        .downcast_ref::<SyncFile>()
                        .ok_or(EINVAL)?;
                    sync.fence.wait(false)?;
                }
                let ring = if flags & 4 != 0 {
                    if cmd == 0xc020_6442 {
                        return Err(EINVAL);
                    }
                    let idx = ctx.mm.read_u32(arg + 32).ok_or(EFAULT)?;
                    if idx > 255 {
                        return Err(EINVAL);
                    }
                    Some(idx as u8)
                } else {
                    None
                };
                let mut bytes = vec![0; len];
                if !ctx.mm.read(ptr, &mut bytes) {
                    return Err(EFAULT);
                }
                // Like Linux, submitting without CONTEXT_INIT uses an implicit
                // default context (capset 0; goldfish "virtio-gpu-pipe" users).
                let (backend, context) = {
                    let mut state = self.state.lock();
                    if state.context.is_none() {
                        let id = NEXT_CONTEXT.fetch_add(1, Ordering::Relaxed);
                        backend.context_create(id, 0).inspect_err(|e| eprintln!("virtgpu: implicit context_create failed {e}"))?;
                        state.context = Some((backend.clone(), id));
                    }
                    state.context.clone().unwrap()
                };
                backend.submit(context, &mut bytes).inspect_err(|e| eprintln!("virtgpu: submit({} bytes, flags {flags:#x}) failed {e}", bytes.len()))?;
                let fence = Arc::new(GpuFence::default());
                backend.fence(context, ring, fence.clone())?;
                self.state.lock().last_fence = Some(fence.clone());
                if flags & 2 != 0 {
                    let node = Arc::new(SyncFile {
                        id: vfs::new_id(),
                        fence,
                    });
                    let file =
                        crate::fd::OpenFile::new(node, crate::fd::O_RDWR, "anon_inode:[sync_file]");
                    let table = ctx.fds();
                    let fd = table.lock().insert(file, true)?;
                    if !ctx.mm.write_u32(arg + 28, fd as u32) {
                        let _ = table.lock().close(fd);
                        return Err(EFAULT);
                    }
                }
                Ok(0)
            }
            0xc008_6448 => {
                let handle = ctx.mm.read_u32(arg).ok_or(EFAULT)?;
                let flags = ctx.mm.read_u32(arg + 4).ok_or(EFAULT)?;
                if flags & !1 != 0 {
                    return Err(EINVAL);
                }
                let state = self.state.lock();
                if !state.resources.contains_key(&handle) {
                    return Err(ENOENT);
                }
                let fence = state.last_fence.clone();
                drop(state);
                if let Some(fence) = fence {
                    fence.wait(flags & 1 != 0)?;
                }
                Ok(0)
            }
            0xc018_6449 => {
                let id = ctx.mm.read_u32(arg).ok_or(EFAULT)?;
                let version = ctx.mm.read_u32(arg + 4).ok_or(EFAULT)?;
                let ptr = ctx.mm.read_u64(arg + 8).ok_or(EFAULT)?;
                let size = ctx.mm.read_u32(arg + 16).ok_or(EFAULT)? as usize;
                let caps = backend.capset(id, version)?;
                if !ctx.mm.write(ptr, &caps[..size.min(caps.len())]) {
                    return Err(EFAULT);
                }
                Ok(0)
            }
            _ => Err(ENOTTY),
        }
    }
    fn shared_mem_at(&self, offset: u64, len: u64) -> Option<(Arc<SharedMem>, u64)> {
        let handle = (offset >> 32) as u32;
        let inner = offset & 0xffff_ffff;
        let state = self.state.lock();
        let (resource, size, _) = state.resources.get(&handle)?;
        if inner.checked_add(len)? > *size {
            return None;
        }
        Some((resource.clone().map().ok()?, inner))
    }
    fn mmap_requires_shared_mem(&self) -> bool {
        true
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Drop for VirtGpu {
    fn drop(&mut self) {
        let state = self.state.get_mut();
        state.resources.clear();
        if let Some((backend, id)) = state.context.take() {
            backend.context_destroy(id);
        }
    }
}
