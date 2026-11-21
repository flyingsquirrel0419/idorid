//! In-process BPF objects. Programs are recorded but never executed.
use crate::{
    errno::*,
    fd::OpenFile,
    task::Ctx,
    vfs::{self, Node, Stat, S_IFREG},
};
use parking_lot::Mutex;
use std::{
    collections::{BTreeMap, HashMap},
    os::unix::fs::MetadataExt,
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc,
    },
};

#[derive(Default)]
pub struct BpfState {
    // Keep the host inode open so an unlinked pin cannot be confused with a
    // subsequently created file that reuses its inode number.
    pins: Mutex<HashMap<u64, (vfs::FileRef, std::fs::File)>>,
    next_id: AtomicU32,
}
struct Object {
    id: u32,
    kind: Kind,
}
enum Kind {
    Map(Map),
    Prog {
        ty: u32,
        name: [u8; 16],
        _license: Vec<u8>,
    },
    Btf,
}
struct Map {
    ty: u32,
    key: u32,
    value: u32,
    max: u32,
    flags: u32,
    name: [u8; 16],
    state: Mutex<MapState>,
}
#[derive(Default)]
struct MapState {
    entries: BTreeMap<Vec<u8>, Vec<u8>>,
    frozen: bool,
}
impl Node for Object {
    fn id(&self) -> u64 {
        self.id as u64
    }
    fn stat(&self) -> Stat {
        Stat {
            ino: self.id as u64,
            mode: S_IFREG | 0o600,
            nlink: 1,
            ..Default::default()
        }
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
fn u32at(b: &[u8], i: usize) -> u32 {
    u32::from_le_bytes(b[i..i + 4].try_into().unwrap())
}
fn u64at(b: &[u8], i: usize) -> u64 {
    u64::from_le_bytes(b[i..i + 8].try_into().unwrap())
}
fn put32(b: &mut [u8], i: usize, v: u32) {
    b[i..i + 4].copy_from_slice(&v.to_le_bytes());
}
fn read(ctx: &Ctx, p: u64, n: usize) -> vfs::Result<Vec<u8>> {
    if n > 16 * 1024 * 1024 {
        return Err(E2BIG);
    }
    let mut b = vec![0; n];
    if !ctx.mm.read(p, &mut b) {
        return Err(EFAULT);
    }
    Ok(b)
}
fn write(ctx: &Ctx, p: u64, b: &[u8]) -> vfs::Result<()> {
    if ctx.mm.write(p, b) {
        Ok(())
    } else {
        Err(EFAULT)
    }
}
fn object(ctx: &Ctx, fd: u32) -> vfs::Result<Arc<OpenFile>> {
    let f = ctx.fds().lock().get(fd as i32)?;
    if f.node.as_any().downcast_ref::<Object>().is_none() {
        return Err(EINVAL);
    }
    Ok(f)
}
fn install(ctx: &Ctx, obj: vfs::FileRef, flags: u32) -> vfs::Result<i64> {
    Ok(ctx
        .fds()
        .lock()
        .insert(OpenFile::new(obj, flags, "anon_inode:bpf"), true)? as i64)
}
fn array(m: &Map) -> bool {
    matches!(m.ty, 2 | 6)
}
fn value_len(m: &Map) -> usize {
    if matches!(m.ty, 5 | 6 | 10 | 21) {
        (m.value as usize + 7) & !7
    } else {
        m.value as usize
    }
}
fn key(ctx: &Ctx, m: &Map, p: u64) -> vfs::Result<Vec<u8>> {
    read(ctx, p, m.key as usize)
}
fn valid_index(m: &Map, k: &[u8]) -> bool {
    k.len() == 4 && u32at(k, 0) < m.max
}
fn path(ctx: &Ctx, p: u64) -> vfs::Result<(String, vfs::Loc)> {
    let s = ctx.mm.read_cstr(p, 4096).ok_or(EFAULT)?;
    let s = String::from_utf8_lossy(&s);
    ctx.kernel
        .vfs
        .resolve(ctx, &ctx.process.cwd.lock(), &s, true)
}
fn new_obj(ctx: &Ctx, kind: Kind) -> Arc<Object> {
    Arc::new(Object {
        id: ctx.kernel.bpf.next_id.fetch_add(1, Ordering::Relaxed) + 1,
        kind,
    })
}

pub fn syscall(ctx: &Ctx, cmd: u64, ptr: u64, size: u64) -> i64 {
    let result = run(ctx, cmd, ptr, size).unwrap_or_else(|e| e);
    if matches!(cmd, 6 | 7) && ctx.kernel.trace_syscalls.load(Ordering::Relaxed) {
        let path = ctx
            .mm
            .read_u64(ptr)
            .and_then(|p| ctx.mm.read_cstr(p, 4096))
            .unwrap_or_default();
        ctx.kernel.log(&format!(
            "[{}] bpf {} {:?} = {}",
            ctx.thread.tid,
            cmd,
            String::from_utf8_lossy(&path),
            result
        ));
    }
    result
}
fn run(ctx: &Ctx, cmd: u64, ptr: u64, size: u64) -> vfs::Result<i64> {
    if size > 4096 {
        return Err(E2BIG);
    }
    let mut a = vec![0u8; 4096];
    let n = size as usize;
    if !ctx.mm.read(ptr, &mut a[..n]) {
        return Err(EFAULT);
    }
    match cmd {
        0 => {
            let ty = u32at(&a, 0);
            let k = u32at(&a, 4);
            let v = u32at(&a, 8);
            let max = u32at(&a, 12);
            let flags = u32at(&a, 16) | if matches!(ty, 14 | 25) { 128 } else { 0 };
            if !matches!(ty, 1 | 2 | 5 | 6 | 9 | 10 | 14 | 25 | 27)
                || max == 0
                || (ty != 27 && (k == 0 || v == 0))
                || k > 65536
                || v > 1048576
                || (matches!(ty, 2 | 6) && k != 4)
            {
                return Err(EINVAL);
            }
            let mut name = [0; 16];
            name.copy_from_slice(&a[28..44]);
            install(
                ctx,
                new_obj(
                    ctx,
                    Kind::Map(Map {
                        ty,
                        key: k,
                        value: v,
                        max,
                        flags,
                        name,
                        state: Mutex::new(MapState::default()),
                    }),
                ),
                2,
            )
        }
        1 | 2 | 3 | 4 | 22 => {
            let f = object(ctx, u32at(&a, 0))?;
            let obj = f.node.as_any().downcast_ref::<Object>().unwrap();
            let Kind::Map(m) = &obj.kind else {
                return Err(EINVAL);
            };
            let access = f.flags.load(Ordering::Relaxed) & 3;
            if (cmd == 1 || cmd == 4) && (access == 1 || m.flags & 16 != 0)
                || (cmd == 2 || cmd == 3 || cmd == 22) && (access == 0 || m.flags & 8 != 0)
            {
                return Err(EPERM);
            }
            let mut st = m.state.lock();
            if cmd == 22 {
                if st.frozen {
                    return Err(EBUSY);
                }
                st.frozen = true;
                return Ok(0);
            }
            if m.ty == 27 {
                return Err(EINVAL);
            }
            let kp = u64at(&a, 8);
            let vp = u64at(&a, 16);
            let k = if cmd == 4 && kp == 0 {
                None
            } else {
                Some(key(ctx, m, kp)?)
            };
            match cmd {
                1 => {
                    let k = k.unwrap();
                    let val = if array(m) {
                        if !valid_index(m, &k) {
                            return Err(ENOENT);
                        }
                        st.entries
                            .get(&k)
                            .cloned()
                            .unwrap_or_else(|| vec![0; value_len(m)])
                    } else {
                        st.entries.get(&k).cloned().ok_or(ENOENT)?
                    };
                    write(ctx, vp, &val)?;
                }
                2 => {
                    if st.frozen {
                        return Err(EPERM);
                    }
                    let k = k.unwrap();
                    let fl = u64at(&a, 24);
                    if fl > 2 {
                        return Err(EINVAL);
                    }
                    let exists = if array(m) {
                        if !valid_index(m, &k) {
                            return Err(E2BIG);
                        }
                        true
                    } else {
                        st.entries.contains_key(&k)
                    };
                    if fl == 1 && exists {
                        return Err(EEXIST);
                    }
                    if fl == 2 && !exists {
                        return Err(ENOENT);
                    }
                    if !exists && st.entries.len() >= m.max as usize {
                        return Err(E2BIG);
                    }
                    let v = read(ctx, vp, value_len(m))?;
                    st.entries.insert(k, v);
                }
                3 => {
                    if st.frozen {
                        return Err(EPERM);
                    }
                    if array(m) {
                        return Err(EINVAL);
                    }
                    if st.entries.remove(&k.unwrap()).is_none() {
                        return Err(ENOENT);
                    }
                }
                4 => {
                    let next = if array(m) {
                        let i = match k {
                            Some(ref k) if valid_index(m, k) => u32at(k, 0) + 1,
                            _ => 0,
                        };
                        if i >= m.max {
                            return Err(ENOENT);
                        }
                        i.to_le_bytes().to_vec()
                    } else {
                        let first = || st.entries.keys().next().cloned();
                        match k {
                            Some(ref k) if st.entries.contains_key(k) => st
                                .entries
                                .range::<Vec<u8>, _>((
                                    std::ops::Bound::Excluded(k),
                                    std::ops::Bound::Unbounded,
                                ))
                                .next()
                                .map(|(k, _)| k.clone()),
                            _ => first(),
                        }
                        .ok_or(ENOENT)?
                    };
                    write(ctx, vp, &next)?;
                }
                _ => unreachable!(),
            }
            Ok(0)
        }
        5 => {
            let ty = u32at(&a, 0);
            let license = ctx.mm.read_cstr(u64at(&a, 16), 4096).ok_or(EFAULT)?;
            let mut name = [0; 16];
            name.copy_from_slice(&a[48..64]);
            install(
                ctx,
                new_obj(
                    ctx,
                    Kind::Prog {
                        ty,
                        name,
                        _license: license,
                    },
                ),
                2,
            )
        }
        6 => {
            let (guest_path, loc) = path(ctx, u64at(&a, 0))?;
            if !guest_path.starts_with("/sys/fs/bpf/") {
                return Err(EINVAL);
            }
            let f = object(ctx, u32at(&a, 8))?;
            let vfs::Loc::Missing { host, rw: true } = loc else {
                return Err(EEXIST);
            };
            let file = std::fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&host)
                .map_err(|e| vfs::host_err(&e))?;
            let ino = file.metadata().map_err(|e| vfs::host_err(&e))?.ino();
            let obj = f.node.clone();
            ctx.kernel.bpf.pins.lock().insert(ino, (obj, file));
            Ok(0)
        }
        7 => {
            let (guest_path, loc) = path(ctx, u64at(&a, 0))?;
            if !guest_path.starts_with("/sys/fs/bpf/") {
                return Err(EINVAL);
            }
            let vfs::Loc::Host { host, .. } = loc else {
                return Err(ENOENT);
            };
            let ino = std::fs::metadata(&host)
                .map_err(|e| vfs::host_err(&e))?
                .ino();
            let pins = ctx.kernel.bpf.pins.lock();
            let (obj, _) = pins.get(&ino).ok_or(ENOENT)?;
            let fl = u32at(&a, 12);
            let access = match fl {
                0 => 2,
                8 => 0,
                16 => 1,
                _ => return Err(EINVAL),
            };
            install(ctx, obj.clone(), access)
        }
        8 | 9 => Ok(0),
        15 => {
            let f = object(ctx, u32at(&a, 0))?;
            let o = f.node.as_any().downcast_ref::<Object>().unwrap();
            let len = u32at(&a, 4) as usize;
            let p = u64at(&a, 8);
            let mut info = vec![0u8; 256];
            match &o.kind {
                Kind::Map(m) => {
                    put32(&mut info, 0, m.ty);
                    put32(&mut info, 4, o.id);
                    put32(&mut info, 8, m.key);
                    put32(&mut info, 12, m.value);
                    put32(&mut info, 16, m.max);
                    put32(&mut info, 20, m.flags);
                    info[24..40].copy_from_slice(&m.name)
                }
                Kind::Prog { ty, name, .. } => {
                    put32(&mut info, 0, *ty);
                    put32(&mut info, 4, o.id);
                    info[64..80].copy_from_slice(name)
                }
                Kind::Btf => put32(&mut info, 12, o.id),
            }
            let actual_len = match &o.kind {
                Kind::Map(_) => 88,
                Kind::Prog { .. } => 232,
                Kind::Btf => 32,
            };
            let n = len.min(actual_len);
            write(ctx, p, &info[..n])?;
            write(ctx, ptr + 4, &(n as u32).to_le_bytes())?;
            Ok(0)
        }
        16 => {
            write(ctx, ptr + 12, &0u32.to_le_bytes())?;
            write(ctx, ptr + 24, &0u32.to_le_bytes())?;
            Ok(0)
        }
        18 => install(ctx, new_obj(ctx, Kind::Btf), 2),
        _ => Err(EINVAL),
    }
}
