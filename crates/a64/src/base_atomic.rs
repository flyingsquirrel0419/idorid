//! Host 16-byte compare-exchange used for guest pair atomics.
#[cfg(target_arch = "x86_64")]
#[inline]
pub(crate) unsafe fn cas128(p: *mut u8, expected: u128, new: u128) -> u128 {
    assert!(std::is_x86_feature_detected!("cmpxchg16b"));
    let mut lo = expected as u64;
    let mut hi = (expected >> 64) as u64;
    let newlo = new as u64;
    unsafe {
        core::arch::asm!(
            "xchg {newlo}, rbx",
            "lock cmpxchg16b [{ptr}]",
            "xchg {newlo}, rbx",
            ptr=in(reg) p, newlo=inout(reg) newlo => _,
            inout("rax") lo, inout("rdx") hi, in("rcx") (new>>64) as u64,
            options(nostack)
        );
    }
    (lo as u128) | ((hi as u128) << 64)
}
#[cfg(target_arch = "aarch64")]
#[inline]
pub(crate) unsafe fn cas128(p: *mut u8, expected: u128, new: u128) -> u128 {
    let lo: u64;
    let hi: u64;
    let status: u32;
    unsafe {
        core::arch::asm!(
            "2: ldaxp {lo}, {hi}, [{ptr}]",
            "cmp {lo}, {exlo}", "ccmp {hi}, {exhi}, #0, eq", "b.ne 3f",
            "stlxp {status:w}, {newlo}, {newhi}, [{ptr}]", "cbnz {status:w}, 2b", "b 4f",
            "3: clrex", "4:",
            ptr=in(reg) p, exlo=in(reg) expected as u64, exhi=in(reg) (expected>>64) as u64,
            newlo=in(reg) new as u64, newhi=in(reg) (new>>64) as u64,
            lo=out(reg) lo, hi=out(reg) hi, status=out(reg) status,
            options(nostack)
        );
    }
    let _ = status;
    (lo as u128) | ((hi as u128) << 64)
}

#[cfg(test)]
mod tests {
    use super::cas128;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::Arc;
    #[repr(align(16))]
    struct Pair([AtomicU64; 2]);
    #[test]
    fn pair_cas_does_not_lose_overlapping_single_updates() {
        let pair = Arc::new(Pair([AtomicU64::new(0), AtomicU64::new(0)]));
        let other = pair.clone();
        let thread = std::thread::spawn(move || {
            for _ in 0..100_000 {
                other.0[0].fetch_add(1, Ordering::SeqCst);
            }
        });
        let ptr = pair.0.as_ptr().cast_mut().cast::<u8>();
        let mut old = unsafe { cas128(ptr, 0, 0) };
        for _ in 0..100_000 {
            loop {
                let new = old.wrapping_add(1u128 << 64);
                let observed = unsafe { cas128(ptr, old, new) };
                if observed == old {
                    old = new;
                    break;
                }
                old = observed;
            }
        }
        thread.join().unwrap();
        assert_eq!(pair.0[0].load(Ordering::SeqCst), 100_000);
        assert_eq!(pair.0[1].load(Ordering::SeqCst), 100_000);
    }
}
