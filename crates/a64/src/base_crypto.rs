//! Host AES acceleration for the architectural AES round primitives.
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "aes")]
unsafe fn x86_aes(key: u32, d: u128, n: u128) -> u128 {
    use core::arch::x86_64::*;
    let zero = _mm_setzero_si128();
    let x: __m128i = unsafe { core::mem::transmute(if key & 0x2000 == 0 { d ^ n } else { n }) };
    let y = match key {
        0x4e28_4800 => _mm_aesenclast_si128(x, zero),
        0x4e28_5800 => _mm_aesdeclast_si128(x, zero),
        0x4e28_6800 => _mm_aesenc_si128(_mm_aesdeclast_si128(x, zero), zero),
        0x4e28_7800 => _mm_aesimc_si128(x),
        _ => unreachable!(),
    };
    unsafe { core::mem::transmute(y) }
}
#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "aes")]
unsafe fn arm_aes(key: u32, d: u128, n: u128) -> u128 {
    use core::arch::aarch64::*;
    let a: uint8x16_t = unsafe { core::mem::transmute(d) };
    let b: uint8x16_t = unsafe { core::mem::transmute(n) };
    let y = match key {
        0x4e28_4800 => vaeseq_u8(a, b),
        0x4e28_5800 => vaesdq_u8(a, b),
        0x4e28_6800 => vaesmcq_u8(b),
        0x4e28_7800 => vaesimcq_u8(b),
        _ => unreachable!(),
    };
    unsafe { core::mem::transmute(y) }
}
#[inline]
pub(crate) fn aes(key: u32, d: u128, n: u128) -> Option<u128> {
    #[cfg(target_arch = "x86_64")]
    if std::is_x86_feature_detected!("aes") {
        return Some(unsafe { x86_aes(key, d, n) });
    }
    #[cfg(target_arch = "aarch64")]
    if std::arch::is_aarch64_feature_detected!("aes") {
        return Some(unsafe { arm_aes(key, d, n) });
    }
    let _ = (key, d, n);
    None
}
