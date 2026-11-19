//! /dev/ashmem: named shared memory regions (libcutils ashmem_create_region).

use crate::errno::*;
use crate::mm::SharedMem;
use crate::task::Ctx;
use crate::vfs::*;
use parking_lot::Mutex;
use std::sync::Arc;

pub struct Ashmem {
    name: Mutex<String>,
    size: Mutex<u64>,
    mem: Mutex<Option<Arc<SharedMem>>>,
    prot: Mutex<u32>,
    id: u64,
}

impl Ashmem {
    pub fn new() -> Arc<Ashmem> {
        Arc::new(Ashmem {
            name: Mutex::new("dev/ashmem".into()),
            size: Mutex::new(0),
            mem: Mutex::new(None),
            prot: Mutex::new(7),
            id: new_id(),
        })
    }
}

const ASHMEM_SET_NAME: u32 = 0x41007701;
const ASHMEM_GET_NAME: u32 = 0x81007702;
const ASHMEM_SET_SIZE: u32 = 0x40087703;
const ASHMEM_GET_SIZE: u32 = 0x7704;
const ASHMEM_SET_PROT_MASK: u32 = 0x40087705;
const ASHMEM_GET_PROT_MASK: u32 = 0x7706;
const ASHMEM_PIN: u32 = 0x40087707;
const ASHMEM_UNPIN: u32 = 0x40087708;
const ASHMEM_GET_PIN_STATUS: u32 = 0x7709;
const ASHMEM_PURGE_ALL_CACHES: u32 = 0x770a;
const ASHMEM_GET_FILE_ID: u32 = 0x8008770b;

impl Node for Ashmem {
    fn id(&self) -> u64 {
        self.id
    }
    fn stat(&self) -> Stat {
        Stat {
            mode: S_IFCHR | 0o666,
            ino: self.id,
            rdev: 0x0a3d,
            size: *self.size.lock() as i64,
            nlink: 1,
            ..Default::default()
        }
    }
    fn ioctl(&self, ctx: &Ctx, cmd: u32, arg: u64) -> Result<i64> {
        match cmd {
            ASHMEM_SET_NAME => {
                let n = ctx.mm.read_cstr(arg, 256).ok_or(EFAULT)?;
                *self.name.lock() = format!("dev/ashmem/{}", String::from_utf8_lossy(&n));
                Ok(0)
            }
            ASHMEM_GET_NAME => {
                let n = self.name.lock().clone();
                let short = n.strip_prefix("dev/ashmem/").unwrap_or(&n);
                ctx.mm
                    .write(arg, &[short.as_bytes(), b"\0"].concat())
                    .then_some(0)
                    .ok_or(EFAULT)
            }
            ASHMEM_SET_SIZE => {
                if self.mem.lock().is_some() {
                    return Err(EINVAL);
                }
                *self.size.lock() = arg;
                Ok(0)
            }
            ASHMEM_GET_SIZE => Ok(*self.size.lock() as i64),
            ASHMEM_SET_PROT_MASK => {
                *self.prot.lock() &= arg as u32;
                Ok(0)
            }
            ASHMEM_GET_PROT_MASK => Ok(*self.prot.lock() as i64),
            ASHMEM_PIN | ASHMEM_UNPIN => Ok(0), // ASHMEM_NOT_PURGED / ASHMEM_IS_UNPINNED (0)
            ASHMEM_GET_PIN_STATUS => Ok(1),     // ASHMEM_IS_PINNED
            ASHMEM_PURGE_ALL_CACHES => Ok(0),
            ASHMEM_GET_FILE_ID => ctx
                .mm
                .write(arg, &self.id.to_le_bytes())
                .then_some(0)
                .ok_or(EFAULT),
            _ => Err(ENOTTY),
        }
    }
    fn shared_mem(&self) -> Option<Arc<SharedMem>> {
        let size = *self.size.lock();
        if size == 0 {
            return None;
        }
        Some(
            self.mem
                .lock()
                .get_or_insert_with(|| SharedMem::new(size))
                .clone(),
        )
    }
    fn read_at(&self, off: u64, buf: &mut [u8], _nb: bool) -> Result<usize> {
        let size = *self.size.lock();
        let Some(mem) = self.shared_mem() else {
            return Err(EBADF);
        };
        if off >= size {
            return Ok(0);
        }
        let n = buf.len().min((size - off) as usize);
        let mut done = 0;
        while done < n {
            let a = off + done as u64;
            let p = mem.page(a / 4096).ok_or(EIO)?;
            let o = (a % 4096) as usize;
            let k = (4096 - o).min(n - done);
            buf[done..done + k].copy_from_slice(&p.bytes()[o..o + k]);
            done += k;
        }
        Ok(n)
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
