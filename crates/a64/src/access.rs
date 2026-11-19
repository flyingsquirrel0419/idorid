//! Guest memory accessors shared by the integer and SIMD/FP executors.
//! All return `Err(Exit::Fault{..})` on unmapped/forbidden pages; accesses that
//! straddle a page boundary are split.

use crate::cpu::{Cpu, Exit};
use crate::mem::{Access, AddressSpace, PAGE_SIZE};

/// Linux EL0 data accesses ignore the top byte of tagged pointers.
#[inline(always)]
pub(crate) fn untag(addr: u64) -> u64 {
    addr & 0x00ff_ffff_ffff_ffff
}

#[inline(always)]
fn fault(addr: u64, access: Access) -> Exit {
    Exit::Fault { addr, access }
}

/// Reads `N` bytes (N <= 16) little-endian.
#[inline(always)]
pub(crate) fn read_bytes<A: AddressSpace + ?Sized, const N: usize>(
    cpu: &mut Cpu,
    mem: &A,
    addr: u64,
) -> Result<[u8; N], Exit> {
    let addr = untag(addr);
    let mut out = [0u8; N];
    let off = (addr & (PAGE_SIZE - 1)) as usize;
    if off + N <= PAGE_SIZE as usize {
        let p = cpu
            .tlb
            .lookup(mem, addr, Access::Read)
            .ok_or(fault(addr, Access::Read))?;
        unsafe { core::ptr::copy_nonoverlapping(p, out.as_mut_ptr(), N) };
    } else {
        let first = PAGE_SIZE as usize - off;
        let p = cpu
            .tlb
            .lookup(mem, addr, Access::Read)
            .ok_or(fault(addr, Access::Read))?;
        let a2 = addr.wrapping_add(first as u64);
        let q = cpu
            .tlb
            .lookup(mem, a2, Access::Read)
            .ok_or(fault(a2, Access::Read))?;
        unsafe {
            core::ptr::copy_nonoverlapping(p, out.as_mut_ptr(), first);
            core::ptr::copy_nonoverlapping(q, out.as_mut_ptr().add(first), N - first);
        }
    }
    Ok(out)
}

#[inline(always)]
pub(crate) fn write_bytes<A: AddressSpace + ?Sized, const N: usize>(
    cpu: &mut Cpu,
    mem: &A,
    addr: u64,
    data: [u8; N],
) -> Result<(), Exit> {
    let addr = untag(addr);
    let off = (addr & (PAGE_SIZE - 1)) as usize;
    if off + N <= PAGE_SIZE as usize {
        let p = cpu
            .tlb
            .lookup(mem, addr, Access::Write)
            .ok_or(fault(addr, Access::Write))?;
        unsafe { core::ptr::copy_nonoverlapping(data.as_ptr(), p, N) };
    } else {
        let first = PAGE_SIZE as usize - off;
        // Check both pages before writing anything (no partial stores).
        let p = cpu
            .tlb
            .lookup(mem, addr, Access::Write)
            .ok_or(fault(addr, Access::Write))?;
        let a2 = addr.wrapping_add(first as u64);
        let q = cpu
            .tlb
            .lookup(mem, a2, Access::Write)
            .ok_or(fault(a2, Access::Write))?;
        unsafe {
            core::ptr::copy_nonoverlapping(data.as_ptr(), p, first);
            core::ptr::copy_nonoverlapping(data.as_ptr().add(first), q, N - first);
        }
    }
    // A store by this CPU clears its own exclusive monitor only if it hits the
    // monitored granule; we keep it simple and leave it (STXR re-checks).
    Ok(())
}

macro_rules! rw {
    ($r:ident, $w:ident, $t:ty, $n:expr) => {
        #[inline(always)]
        #[allow(dead_code)]
        pub(crate) fn $r<A: AddressSpace + ?Sized>(
            cpu: &mut Cpu,
            mem: &A,
            addr: u64,
        ) -> Result<$t, Exit> {
            Ok(<$t>::from_le_bytes(read_bytes::<A, $n>(cpu, mem, addr)?))
        }
        #[inline(always)]
        #[allow(dead_code)]
        pub(crate) fn $w<A: AddressSpace + ?Sized>(
            cpu: &mut Cpu,
            mem: &A,
            addr: u64,
            v: $t,
        ) -> Result<(), Exit> {
            write_bytes::<A, $n>(cpu, mem, addr, v.to_le_bytes())
        }
    };
}
rw!(read_u8, write_u8, u8, 1);
rw!(read_u16, write_u16, u16, 2);
rw!(read_u32, write_u32, u32, 4);
rw!(read_u64, write_u64, u64, 8);
rw!(read_u128, write_u128, u128, 16);

/// Host pointer for an aligned atomic/exclusive access of `size` bytes, for
/// use with host atomic instructions. Requires natural alignment (else
/// `Exit::Alignment`) and write permission.
#[inline(always)]
#[allow(dead_code)]
pub(crate) fn atomic_ptr<A: AddressSpace + ?Sized>(
    cpu: &mut Cpu,
    mem: &A,
    addr: u64,
    size: u64,
) -> Result<*mut u8, Exit> {
    let addr = untag(addr);
    if addr & (size - 1) != 0 {
        return Err(Exit::Alignment { addr });
    }
    // Need both read and write permission; look up read first so a read-only
    // page reports a write fault on the write lookup.
    cpu.tlb
        .lookup(mem, addr, Access::Read)
        .ok_or(fault(addr, Access::Read))?;
    cpu.tlb
        .lookup(mem, addr, Access::Write)
        .ok_or(fault(addr, Access::Write))
}
