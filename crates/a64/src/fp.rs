//! Scalar FP operations. Unimplemented encodings are left for the caller to reject.
//!
//! Arithmetic adjusts host round-to-nearest results for FPCR-directed rounding.
//! Fused operations use an exact integer residual for directed rounding.
//! Arithmetic exception reporting is incomplete, especially for fused operations.

use crate::cpu::Cpu;

pub(crate) fn input(cpu: &mut Cpu, bits: u64, double: bool) -> u64 {
    let (exp, frac, sign) = if double {
        (0x7ff0_0000_0000_0000, 0x000f_ffff_ffff_ffff, 1u64 << 63)
    } else {
        (0x7f80_0000, 0x007f_ffff, 1u64 << 31)
    };
    if cpu.fpcr & (1 << 24) != 0 && bits & exp == 0 && bits & frac != 0 {
        cpu.fpsr |= 1 << 7;
        bits & sign
    } else {
        bits
    }
}
pub(crate) fn nan(cpu: &mut Cpu, a: u64, b: Option<u64>, double: bool) -> Option<u64> {
    let (exp, frac, quiet, default) = if double {
        (
            0x7ff0_0000_0000_0000,
            0x000f_ffff_ffff_ffff,
            1u64 << 51,
            0x7ff8_0000_0000_0000,
        )
    } else {
        (0x7f80_0000, 0x007f_ffff, 1u64 << 22, 0x7fc0_0000)
    };
    let is_nan = |x: u64| x & exp == exp && x & frac != 0;
    let snan = |x: u64| is_nan(x) && x & quiet == 0;
    let b = b.unwrap_or(0);
    if !is_nan(a) && !is_nan(b) {
        return None;
    }
    if snan(a) || snan(b) {
        cpu.fpsr |= 1;
    }
    if cpu.fpcr & (1 << 25) != 0 {
        return Some(default);
    }
    Some(
        (if snan(a) {
            a
        } else if snan(b) {
            b
        } else if is_nan(a) {
            a
        } else {
            b
        }) | quiet,
    )
}
pub(crate) fn finish(cpu: &mut Cpu, bits: u64, double: bool) -> u64 {
    let (exp, frac, sign, default) = if double {
        (
            0x7ff0_0000_0000_0000,
            0x000f_ffff_ffff_ffff,
            1u64 << 63,
            0x7ff8_0000_0000_0000,
        )
    } else {
        (0x7f80_0000, 0x007f_ffff, 1u64 << 31, 0x7fc0_0000)
    };
    if bits & exp == exp && bits & frac != 0 {
        cpu.fpsr |= 1;
        return default;
    }
    if cpu.fpcr & (1 << 24) != 0 && bits & exp == 0 && bits & frac != 0 {
        cpu.fpsr |= 1 << 3;
        return bits & sign;
    }
    bits
}

fn exact_product_equal(a: f64, b: f64, c: f64) -> bool {
    fn parts(x: f64) -> (u64, i32) {
        let bits = x.to_bits() & 0x7fff_ffff_ffff_ffff;
        let e = (bits >> 52) as i32;
        let frac = bits & ((1u64 << 52) - 1);
        if e == 0 {
            (frac, -1074)
        } else {
            (frac | (1u64 << 52), e - 1023 - 52)
        }
    }
    let (a, ea) = parts(a);
    let (b, eb) = parts(b);
    let (c, ec) = parts(c);
    let mut product = (a as u128) * (b as u128);
    if product == 0 {
        return c == 0;
    }
    if c == 0 {
        return false;
    }
    let pz = product.trailing_zeros();
    let cz = c.trailing_zeros();
    product >>= pz;
    product == (c >> cz) as u128 && ea + eb + pz as i32 == ec + cz as i32
}

fn product_magnitude_cmp(a: f64, b: f64, c: f64) -> core::cmp::Ordering {
    fn parts(x: f64) -> (u64, i32) {
        let bits = x.to_bits() & 0x7fff_ffff_ffff_ffff;
        let e = (bits >> 52) as i32;
        let f = bits & ((1u64 << 52) - 1);
        if e == 0 {
            (f, -1074)
        } else {
            (f | (1u64 << 52), e - 1075)
        }
    }
    let (a, ea) = parts(a);
    let (b, eb) = parts(b);
    let (c, ec) = parts(c);
    let p = (a as u128) * (b as u128);
    let c = c as u128;
    if p == 0 || c == 0 {
        return p.cmp(&c);
    }
    let pb = 128 - p.leading_zeros();
    let cb = 128 - c.leading_zeros();
    (ea + eb + pb as i32)
        .cmp(&(ec + cb as i32))
        .then_with(|| (p << (128 - pb)).cmp(&(c << (128 - cb))))
}

// Adjust a nearest-even host result using the sign of its exact rounding error.
fn directed_arithmetic(cpu: &mut Cpu, x: f64, y: f64, z: f64, op: u32, double: bool) -> f64 {
    use core::cmp::Ordering;
    let mode = (cpu.fpcr >> 22) & 3;
    if mode == 0 || !x.is_finite() || !y.is_finite() || (op == 1 && y == 0.0) {
        return z;
    }
    let negative = if op <= 1 {
        x.is_sign_negative() ^ y.is_sign_negative()
    } else {
        z.is_sign_negative()
    };
    let ordering = if z.is_infinite() {
        if z.is_sign_negative() {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    } else if op <= 1 {
        let mag = if op == 0 {
            product_magnitude_cmp(x, y, z)
        } else {
            product_magnitude_cmp(z, y, x).reverse()
        };
        if negative {
            mag.reverse()
        } else {
            mag
        }
    } else {
        let b = if op == 3 { -y } else { y };
        let v = z - x;
        let error = (x - (z - v)) + (b - v);
        if error > 0.0 {
            Ordering::Greater
        } else if error < 0.0 {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    };
    if ordering == Ordering::Equal {
        if z == 0.0 && mode == 2 && op >= 2 {
            let b = if op == 3 { -y } else { y };
            if x.is_sign_negative() != b.is_sign_negative() {
                return -0.0;
            }
        }
        return z;
    }
    let up = mode == 1 && ordering == Ordering::Greater
        || mode == 3 && z.is_sign_negative() && ordering == Ordering::Greater;
    let down = mode == 2 && ordering == Ordering::Less
        || mode == 3 && !z.is_sign_negative() && ordering == Ordering::Less;
    if !up && !down {
        return z;
    }
    let result = if double {
        let bits = z.to_bits();
        let next = if up {
            if bits == 1u64 << 63 {
                1
            } else if bits >> 63 != 0 {
                bits - 1
            } else {
                bits + 1
            }
        } else if bits == 0 {
            (1u64 << 63) | 1
        } else if bits >> 63 != 0 {
            bits + 1
        } else {
            bits - 1
        };
        f64::from_bits(next)
    } else {
        let bits = (z as f32).to_bits();
        let next = if up {
            if bits == 1u32 << 31 {
                1
            } else if bits >> 31 != 0 {
                bits - 1
            } else {
                bits + 1
            }
        } else if bits == 0 {
            (1u32 << 31) | 1
        } else if bits >> 31 != 0 {
            bits + 1
        } else {
            bits - 1
        };
        f32::from_bits(next) as f64
    };
    if result.is_infinite() {
        cpu.fpsr |= (1 << 2) | (1 << 4);
    }
    result
}

// Compare the exact fused expression with its rounded result. The fixed-size
// accumulators cover every product exponent of two finite binary64 values.
fn fused_error(x: f64, y: f64, add: f64, result: f64) -> core::cmp::Ordering {
    fn parts(v: f64) -> (u64, i32, bool) {
        let bits = v.to_bits();
        let e = ((bits >> 52) & 0x7ff) as i32;
        let f = bits & ((1u64 << 52) - 1);
        (
            if e == 0 { f } else { f | (1u64 << 52) },
            if e == 0 { -1074 } else { e - 1075 },
            bits >> 63 != 0,
        )
    }
    fn add_word(words: &mut [u64; 70], mut index: usize, mut value: u64) {
        while value != 0 {
            let (v, carry) = words[index].overflowing_add(value);
            words[index] = v;
            value = carry as u64;
            index += 1;
        }
    }
    fn add_term(words: &mut [u64; 70], value: u128, exp: i32) {
        if value == 0 {
            return;
        }
        let bit = (exp + 2200) as usize;
        let idx = bit / 64;
        let shift = bit % 64;
        for (i, limb) in [value as u64, (value >> 64) as u64].into_iter().enumerate() {
            add_word(words, idx + i, limb << shift);
            if shift != 0 {
                add_word(words, idx + i + 1, limb >> (64 - shift));
            }
        }
    }
    let (a, ea, sa) = parts(x);
    let (b, eb, sb) = parts(y);
    let (c, ec, sc) = parts(add);
    let (r, er, sr) = parts(result);
    let mut positive = [0u64; 70];
    let mut negative = [0u64; 70];
    add_term(
        if sa ^ sb {
            &mut negative
        } else {
            &mut positive
        },
        (a as u128) * (b as u128),
        ea + eb,
    );
    add_term(
        if sc { &mut negative } else { &mut positive },
        c as u128,
        ec,
    );
    add_term(
        if sr { &mut positive } else { &mut negative },
        r as u128,
        er,
    );
    for i in (0..70).rev() {
        let cmp = positive[i].cmp(&negative[i]);
        if cmp != core::cmp::Ordering::Equal {
            return cmp;
        }
    }
    core::cmp::Ordering::Equal
}

fn fused_round(cpu: &mut Cpu, x: f64, y: f64, add: f64, mut result: f64, double: bool) -> f64 {
    use core::cmp::Ordering;
    if !x.is_finite() || !y.is_finite() || !add.is_finite() {
        return result;
    }
    let mode = (cpu.fpcr >> 22) & 3;
    let cmp = if result.is_infinite() {
        cpu.fpsr |= (1 << 2) | (1 << 4);
        if result.is_sign_negative() {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    } else {
        fused_error(x, y, add, result)
    };
    if cmp == Ordering::Equal {
        if result == 0.0
            && mode == 2
            && (x.is_sign_negative() ^ y.is_sign_negative()) != add.is_sign_negative()
        {
            return -0.0;
        }
        return result;
    }
    cpu.fpsr |= 1 << 4;
    if result.abs()
        < if double {
            f64::MIN_POSITIVE
        } else {
            f32::MIN_POSITIVE as f64
        }
    {
        cpu.fpsr |= 1 << 3;
    }
    let up = mode == 1 && cmp == Ordering::Greater
        || mode == 3 && result.is_sign_negative() && cmp == Ordering::Greater;
    let down = mode == 2 && cmp == Ordering::Less
        || mode == 3 && !result.is_sign_negative() && cmp == Ordering::Less;
    if up || down {
        result = if double {
            let bits = result.to_bits();
            let next = if up {
                if bits == 1u64 << 63 {
                    1
                } else if bits >> 63 != 0 {
                    bits - 1
                } else {
                    bits + 1
                }
            } else if bits == 0 {
                (1u64 << 63) | 1
            } else if bits >> 63 != 0 {
                bits + 1
            } else {
                bits - 1
            };
            f64::from_bits(next)
        } else {
            let bits = (result as f32).to_bits();
            let next = if up {
                if bits == 1u32 << 31 {
                    1
                } else if bits >> 31 != 0 {
                    bits - 1
                } else {
                    bits + 1
                }
            } else if bits == 0 {
                (1u32 << 31) | 1
            } else if bits >> 31 != 0 {
                bits + 1
            } else {
                bits - 1
            };
            f32::from_bits(next) as f64
        };
        if result.is_infinite() {
            cpu.fpsr |= (1 << 2) | (1 << 4);
        }
    }
    result
}

fn arithmetic_flags(cpu: &mut Cpu, x: f64, y: f64, z: f64, op: u32, double: bool) {
    if !x.is_finite() || !y.is_finite() || (op == 1 && y == 0.0) {
        return;
    }
    if z.is_infinite() {
        cpu.fpsr |= (1 << 2) | (1 << 4);
        return;
    }
    let inexact = if double {
        match op {
            0 => !exact_product_equal(x, y, z),
            1 => !exact_product_equal(z, y, x),
            _ => {
                let b = if op == 3 { -y } else { y };
                let v = z - x;
                let error = (x - (z - v)) + (b - v);
                error != 0.0
            }
        }
    } else {
        match op {
            0 => x * y != z,
            1 => z * y != x,
            2 => x + y != z,
            _ => x - y != z,
        }
    };
    if inexact {
        cpu.fpsr |= 1 << 4;
        let min = if double {
            f64::MIN_POSITIVE
        } else {
            f32::MIN_POSITIVE as f64
        };
        if z.abs() < min {
            cpu.fpsr |= 1 << 3;
        }
    }
}

pub(crate) fn exec(cpu: &mut Cpu, insn: u32) -> bool {
    let d = (insn & 31) as usize;
    let n = ((insn >> 5) & 31) as usize;
    let m = ((insn >> 16) & 31) as usize;
    let move_op = insn & 0xffff_fc00;
    if matches!(
        move_op,
        0x1ee6_0000
            | 0x1ee7_0000
            | 0x1e26_0000
            | 0x1e27_0000
            | 0x9e66_0000
            | 0x9e67_0000
            | 0x9eae_0000
            | 0x9eaf_0000
    ) {
        let x = if n == 31 { 0 } else { cpu.x[n] };
        match move_op {
            0x1ee6_0000 => {
                if d != 31 {
                    cpu.x[d] = cpu.v[n] as u16 as u64;
                }
            }
            0x1ee7_0000 => cpu.v[d] = x as u16 as u128,
            0x1e26_0000 => {
                if d != 31 {
                    cpu.x[d] = cpu.v[n] as u32 as u64
                }
            }
            0x1e27_0000 => cpu.v[d] = x as u32 as u128,
            0x9e66_0000 => {
                if d != 31 {
                    cpu.x[d] = cpu.v[n] as u64
                }
            }
            0x9e67_0000 => cpu.v[d] = x as u128,
            0x9eae_0000 => {
                if d != 31 {
                    cpu.x[d] = (cpu.v[n] >> 64) as u64
                }
            }
            _ => cpu.v[d] = (cpu.v[d] & u64::MAX as u128) | ((x as u128) << 64),
        }
        return true;
    }
    let typ = (insn >> 22) & 3;
    if typ > 1 {
        return false;
    }
    let double = typ == 1;
    let raw = |v: u128| if double { v as u64 } else { v as u32 as u64 };
    // Convert between binary32 and binary64, preserving NaN payloads.
    let convert = insn & 0xffff_fc00;
    if matches!(convert, 0x1e62_4000 | 0x1e22_c000) {
        if convert == 0x1e22_c000 {
            let a = input(cpu, cpu.v[n] as u32 as u64, false);
            let out = if let Some(v) = nan(cpu, a, None, false) {
                if cpu.fpcr & (1 << 25) != 0 {
                    0x7ff8_0000_0000_0000
                } else {
                    ((v & 0x8000_0000) << 32) | 0x7ff0_0000_0000_0000 | ((v & 0x007f_ffff) << 29)
                }
            } else {
                (f32::from_bits(a as u32) as f64).to_bits()
            };
            cpu.v[d] = out as u128;
        } else {
            let a = input(cpu, cpu.v[n] as u64, true);
            let out = if let Some(v) = nan(cpu, a, None, true) {
                if cpu.fpcr & (1 << 25) != 0 {
                    0x7fc0_0000
                } else {
                    ((v >> 32) as u32 & 0x8000_0000)
                        | 0x7f80_0000
                        | ((v >> 29) as u32 & 0x007f_ffff)
                }
            } else {
                let v = f64::from_bits(a);
                let mut result = v as f32;
                if v.is_finite() && (result as f64) != v {
                    cpu.fpsr |= 1 << 4;
                    let mode = (cpu.fpcr >> 22) & 3;
                    let rounded = result as f64;
                    let up = mode == 1 || (mode == 3 && v < 0.0);
                    let down = mode == 2 || (mode == 3 && v > 0.0);
                    if (up && rounded < v) || (down && rounded > v) {
                        let bits = result.to_bits();
                        let next = if up {
                            if bits == 0x8000_0000 {
                                1
                            } else if bits & 0x8000_0000 != 0 {
                                bits - 1
                            } else {
                                bits + 1
                            }
                        } else if bits == 0 {
                            0x8000_0001
                        } else if bits & 0x8000_0000 != 0 {
                            bits + 1
                        } else {
                            bits - 1
                        };
                        result = f32::from_bits(next);
                    }
                    if result.is_infinite() || v.abs() > f32::MAX as f64 {
                        cpu.fpsr |= 1 << 2;
                    }
                    if result.abs() < f32::MIN_POSITIVE {
                        cpu.fpsr |= 1 << 3;
                    }
                }
                finish(cpu, result.to_bits() as u64, false) as u32
            };
            cpu.v[d] = out as u128;
        }
        return true;
    }
    // General-register integer conversions, including fixed-point forms.
    let group = insn & 0x7f20_0000;
    let fixed = group == 0x1e00_0000;
    if (fixed || group == 0x1e20_0000) && (fixed || insn & 0xfc00 == 0) {
        let opcode = (insn >> 16) & 31;
        let sf = insn >> 31 != 0;
        let fbits = if fixed { 64 - ((insn >> 10) & 63) } else { 0 };
        if fixed && !sf && fbits > 32 {
            return false;
        }
        let x = if n == 31 { 0 } else { cpu.x[n] };
        if opcode == 2 || opcode == 3 {
            let signed = if sf { x as i64 } else { x as u32 as i32 as i64 };
            let negative = opcode == 2 && signed < 0;
            let magnitude = if opcode == 2 {
                signed.unsigned_abs()
            } else if sf {
                x
            } else {
                x as u32 as u64
            };
            if magnitude == 0 {
                cpu.v[d] = 0;
                return true;
            }
            let precision = if double { 53u32 } else { 24u32 };
            let mut exponent = (63 - magnitude.leading_zeros()) as i32 - fbits as i32;
            let length = 64 - magnitude.leading_zeros();
            let mut significand = if length <= precision {
                magnitude << (precision - length)
            } else {
                let shift = length - precision;
                let mut truncated = magnitude >> shift;
                let remainder = magnitude & ((1u64 << shift) - 1);
                if remainder != 0 {
                    cpu.fpsr |= 1 << 4;
                    let mode = (cpu.fpcr >> 22) & 3;
                    let increment = match mode {
                        0 => {
                            remainder > (1u64 << (shift - 1))
                                || (remainder == (1u64 << (shift - 1)) && truncated & 1 != 0)
                        }
                        1 => !negative,
                        2 => negative,
                        _ => false,
                    };
                    if increment {
                        truncated += 1;
                    }
                }
                truncated
            };
            if significand == 1u64 << precision {
                significand >>= 1;
                exponent += 1;
            }
            let fraction = significand & ((1u64 << (precision - 1)) - 1);
            let bits = if double {
                ((negative as u64) << 63) | (((exponent + 1023) as u64) << 52) | fraction
            } else {
                ((negative as u64) << 31) | (((exponent + 127) as u64) << 23) | fraction
            };
            cpu.v[d] = bits as u128;
            return true;
        }
        if matches!(opcode, 0 | 1 | 4 | 5 | 8 | 9 | 16 | 17 | 24 | 25) && (!fixed || opcode >= 24) {
            let bits = input(cpu, raw(cpu.v[n]), double);
            let v = if double {
                f64::from_bits(bits)
            } else {
                f32::from_bits(bits as u32) as f64
            } * 2f64.powi(fbits as i32);
            let r = match opcode >> 1 {
                0 => v.round_ties_even(),
                2 => v.round(),
                4 => v.ceil(),
                8 => v.floor(),
                _ => v.trunc(),
            };
            let unsigned = opcode & 1 != 0;
            let width = if sf { 64 } else { 32 };
            let limit = 2f64.powi(if unsigned { width } else { width - 1 });
            let invalid = v.is_nan()
                || if unsigned {
                    r < 0.0 || r >= limit
                } else {
                    r < -limit || r >= limit
                };
            let out = if v.is_nan() {
                0
            } else if unsigned {
                if r < 0.0 {
                    0
                } else if r >= limit {
                    if sf {
                        u64::MAX
                    } else {
                        u32::MAX as u64
                    }
                } else {
                    r as u64
                }
            } else if r < -limit {
                if sf {
                    i64::MIN as u64
                } else {
                    i32::MIN as u32 as u64
                }
            } else if r >= limit {
                if sf {
                    i64::MAX as u64
                } else {
                    i32::MAX as u64
                }
            } else {
                let z = r as i64 as u64;
                if sf {
                    z
                } else {
                    z as u32 as u64
                }
            };
            if invalid {
                cpu.fpsr |= 1;
            } else if v != r {
                cpu.fpsr |= 1 << 4;
            }
            if d != 31 {
                cpu.x[d] = out;
            }
            return true;
        }
    }
    // Fused three-operand multiply-add/subtract. The accumulator is the
    // first operand for NaN propagation in the architectural operation.
    if insn & 0xff00_0000 == 0x1f00_0000 {
        let addend = ((insn >> 10) & 31) as usize;
        let sign = if double { 1u64 << 63 } else { 1u64 << 31 };
        let negate_addend = insn & (1 << 21) != 0;
        let negate_product = negate_addend ^ (insn & (1 << 15) != 0);
        let mut x = input(cpu, raw(cpu.v[n]), double);
        let y = input(cpu, raw(cpu.v[m]), double);
        let mut z = input(cpu, raw(cpu.v[addend]), double);
        if negate_product {
            x ^= sign;
        }
        if negate_addend {
            z ^= sign;
        }
        // Signaling NaNs have priority over quiet NaNs across all operands.
        let (exp, frac, quiet) = if double {
            (0x7ff0_0000_0000_0000, 0x000f_ffff_ffff_ffff, 1u64 << 51)
        } else {
            (0x7f80_0000, 0x007f_ffff, 1u64 << 22)
        };
        let is_nan = |v: u64| v & exp == exp && v & frac != 0;
        let snan = |v: u64| is_nan(v) && v & quiet == 0;
        let values = [z, x, y];
        let chosen = values
            .iter()
            .copied()
            .find(|v| snan(*v))
            .or_else(|| values.iter().copied().find(|v| is_nan(*v)));
        let infinity = |v: u64| v & !sign == exp;
        let zero = |v: u64| v & !sign == 0;
        let invalid_product = (infinity(x) && zero(y)) || (zero(x) && infinity(y));
        let result = if invalid_product && is_nan(z) && !snan(z) {
            cpu.fpsr |= 1;
            if double {
                0x7ff8_0000_0000_0000
            } else {
                0x7fc0_0000
            }
        } else if let Some(v) = chosen {
            if values.iter().any(|v| snan(*v)) {
                cpu.fpsr |= 1;
            }
            if cpu.fpcr & (1 << 25) != 0 {
                if double {
                    0x7ff8_0000_0000_0000
                } else {
                    0x7fc0_0000
                }
            } else {
                v | quiet
            }
        } else if double {
            let (a, b, c) = (f64::from_bits(x), f64::from_bits(y), f64::from_bits(z));
            let r = fused_round(cpu, a, b, c, a.mul_add(b, c), true);
            finish(cpu, r.to_bits(), true)
        } else {
            let (a, b, c) = (
                f32::from_bits(x as u32),
                f32::from_bits(y as u32),
                f32::from_bits(z as u32),
            );
            let r = fused_round(
                cpu,
                a as f64,
                b as f64,
                c as f64,
                a.mul_add(b, c) as f64,
                false,
            ) as f32;
            finish(cpu, r.to_bits() as u64, false)
        };
        cpu.v[d] = result as u128;
        return true;
    }
    // Round to an FP integer that fits in a signed 32- or 64-bit range.
    let round_n = insn & 0xff3f_fc00;
    if matches!(
        round_n,
        0x1e28_4000 | 0x1e28_c000 | 0x1e29_4000 | 0x1e29_c000
    ) {
        let bits = input(cpu, raw(cpu.v[n]), double);
        let value = if double {
            f64::from_bits(bits)
        } else {
            f32::from_bits(bits as u32) as f64
        };
        let mode = if round_n & (1 << 15) != 0 {
            (cpu.fpcr >> 22) & 3
        } else {
            3
        };
        let rounded = match mode {
            0 => value.round_ties_even(),
            1 => value.ceil(),
            2 => value.floor(),
            _ => value.trunc(),
        };
        let bound = if round_n & (1 << 16) != 0 {
            9223372036854775808.0
        } else {
            2147483648.0
        };
        let result = if !value.is_finite() || rounded < -bound || rounded >= bound {
            cpu.fpsr |= 1;
            -bound
        } else {
            if rounded != value {
                cpu.fpsr |= 1 << 4;
            }
            rounded
        };
        cpu.v[d] = if double {
            result.to_bits() as u128
        } else {
            (result as f32).to_bits() as u128
        };
        return true;
    }
    // Round floating-point values to integral values in the encoded mode.
    let round_op = insn & 0xff3f_fc00;
    if matches!(
        round_op,
        0x1e24_4000
            | 0x1e24_c000
            | 0x1e25_4000
            | 0x1e25_c000
            | 0x1e26_4000
            | 0x1e27_4000
            | 0x1e27_c000
    ) {
        let a = input(cpu, raw(cpu.v[n]), double);
        let out = if let Some(x) = nan(cpu, a, None, double) {
            x
        } else {
            let v = if double {
                f64::from_bits(a)
            } else {
                f32::from_bits(a as u32) as f64
            };
            let mode = match round_op {
                0x1e24_4000 => 0,
                0x1e24_c000 => 1,
                0x1e25_4000 => 2,
                0x1e25_c000 => 3,
                0x1e26_4000 => 4,
                _ => ((cpu.fpcr >> 22) & 3) as u32,
            };
            let r = match mode {
                0 => v.round_ties_even(),
                1 => v.ceil(),
                2 => v.floor(),
                3 => v.trunc(),
                _ => v.round(),
            };
            if round_op == 0x1e27_4000 && r != v {
                cpu.fpsr |= 1 << 4;
            }
            if double {
                r.to_bits()
            } else {
                (r as f32).to_bits() as u64
            }
        };
        cpu.v[d] = out as u128;
        return true;
    }
    // One-source operations, including bitwise FMOV/FABS/FNEG.
    if insn & 0xff3f_fc00 == 0x1e20_4000
        || insn & 0xff3f_fc00 == 0x1e20_c000
        || insn & 0xff3f_fc00 == 0x1e21_4000
        || insn & 0xff3f_fc00 == 0x1e21_c000
    {
        let op = (insn >> 15) & 3;
        let a = raw(cpu.v[n]);
        let sign = if double { 1u64 << 63 } else { 1u64 << 31 };
        let out = match op {
            0 => a,
            1 => a & !sign,
            2 => a ^ sign,
            _ => {
                let a = input(cpu, a, double);
                if let Some(x) = nan(cpu, a, None, double) {
                    x
                } else {
                    let x = if double {
                        f64::from_bits(a)
                    } else {
                        f32::from_bits(a as u32) as f64
                    };
                    let mut result = if double {
                        x.sqrt()
                    } else {
                        (x as f32).sqrt() as f64
                    };
                    if x > 0.0 && x.is_finite() {
                        let order = product_magnitude_cmp(result, result, x);
                        if order != core::cmp::Ordering::Equal {
                            cpu.fpsr |= 1 << 4;
                            let mode = (cpu.fpcr >> 22) & 3;
                            let up = mode == 1 && order == core::cmp::Ordering::Less;
                            let down = mode >= 2 && order == core::cmp::Ordering::Greater;
                            if up || down {
                                result = if double {
                                    let bits = result.to_bits();
                                    f64::from_bits(if up { bits + 1 } else { bits - 1 })
                                } else {
                                    let bits = (result as f32).to_bits();
                                    f32::from_bits(if up { bits + 1 } else { bits - 1 }) as f64
                                };
                            }
                        }
                    }
                    let bits = if double {
                        result.to_bits()
                    } else {
                        (result as f32).to_bits() as u64
                    };
                    finish(cpu, bits, double)
                }
            }
        };
        cpu.v[d] = out as u128;
        return true;
    }
    // Conditional comparison either performs FCMP/FCMPE or installs NZCV.
    if insn & 0xff20_0c00 == 0x1e20_0400 {
        let cond = (insn >> 12) & 15;
        let flags = cpu.nzcv;
        let nf = flags & (1 << 31) != 0;
        let z = flags & (1 << 30) != 0;
        let c = flags & (1 << 29) != 0;
        let v = flags & (1 << 28) != 0;
        let test = match cond >> 1 {
            0 => z,
            1 => c,
            2 => nf,
            3 => v,
            4 => c && !z,
            5 => nf == v,
            6 => !z && nf == v,
            _ => true,
        };
        let pass = if cond & 1 != 0 && cond != 15 {
            !test
        } else {
            test
        };
        if pass {
            let compare = 0x1e20_2000
                | (if double { 1 << 22 } else { 0 })
                | ((m as u32) << 16)
                | ((n as u32) << 5)
                | (insn & 16);
            return exec(cpu, compare);
        }
        cpu.nzcv = ((insn & 15) as u64) << 28;
        return true;
    }
    // Floating-point immediate and conditional select.
    if insn & 0xff20_1fe0 == 0x1e20_1000 {
        let imm = ((insn >> 13) & 255) as u64;
        let b = (imm >> 6) & 1;
        let frac = imm & 15;
        let exp_low = (imm >> 4) & 3;
        let sign = imm >> 7;
        let bits = if double {
            (sign << 63)
                | ((!b & 1) << 62)
                | (if b != 0 { 0xff << 54 } else { 0 })
                | (exp_low << 52)
                | (frac << 48)
        } else {
            (sign << 31)
                | ((!b & 1) << 30)
                | (if b != 0 { 0x1f << 25 } else { 0 })
                | (exp_low << 23)
                | (frac << 19)
        };
        cpu.v[d] = bits as u128;
        return true;
    }
    if insn & 0xff20_0c00 == 0x1e20_0c00 {
        let cond = (insn >> 12) & 15;
        let flags = cpu.nzcv;
        let nflag = flags & (1 << 31) != 0;
        let z = flags & (1 << 30) != 0;
        let c = flags & (1 << 29) != 0;
        let v = flags & (1 << 28) != 0;
        let base = match cond >> 1 {
            0 => z,
            1 => c,
            2 => nflag,
            3 => v,
            4 => c && !z,
            5 => nflag == v,
            6 => !z && nflag == v,
            _ => true,
        };
        let pass = if cond & 1 != 0 && cond != 15 {
            !base
        } else {
            base
        };
        cpu.v[d] = raw(cpu.v[if pass { n } else { m }]) as u128;
        return true;
    }
    if insn & 0xff20_0c00 == 0x1e20_0800 && matches!((insn >> 12) & 15, 4..=7) {
        let op = (insn >> 12) & 15;
        let a = input(cpu, raw(cpu.v[n]), double);
        let b = input(cpu, raw(cpu.v[m]), double);
        let (exp, frac, quiet) = if double {
            (0x7ff0_0000_0000_0000, 0x000f_ffff_ffff_ffff, 1u64 << 51)
        } else {
            (0x7f80_0000, 0x007f_ffff, 1u64 << 22)
        };
        let isnan = |x: u64| x & exp == exp && x & frac != 0;
        let asnan = isnan(a) && a & quiet == 0;
        let bsnan = isnan(b) && b & quiet == 0;
        let result = if op >= 6 && !asnan && !bsnan && (isnan(a) ^ isnan(b)) {
            if isnan(a) {
                b
            } else {
                a
            }
        } else if let Some(v) = nan(cpu, a, Some(b), double) {
            v
        } else {
            let (x, y) = if double {
                (f64::from_bits(a), f64::from_bits(b))
            } else {
                (
                    f32::from_bits(a as u32) as f64,
                    f32::from_bits(b as u32) as f64,
                )
            };
            let min = op & 1 != 0;
            if x == y {
                if x == 0.0 {
                    if min {
                        a | b
                    } else {
                        a & b
                    }
                } else {
                    a
                }
            } else if (x < y) == min {
                a
            } else {
                b
            }
        };
        cpu.v[d] = result as u128;
        return true;
    }
    if insn & 0xff20_fc00 == 0x1e20_8800 {
        let multiply = insn & !(15 << 12);
        if !exec(cpu, multiply) {
            return false;
        }
        cpu.v[d] ^= if double { 1u128 << 63 } else { 1u128 << 31 };
        return true;
    }
    // Two-source arithmetic.
    if insn & 0xff20_0c00 == 0x1e20_0800 {
        let op = (insn >> 12) & 15;
        if op > 3 {
            return false;
        }
        let a = input(cpu, raw(cpu.v[n]), double);
        let b = input(cpu, raw(cpu.v[m]), double);
        let result = if let Some(x) = nan(cpu, a, Some(b), double) {
            x
        } else if double {
            let x = f64::from_bits(a);
            let y = f64::from_bits(b);
            if op == 1 && y == 0.0 && x != 0.0 && x.is_finite() {
                cpu.fpsr |= 1 << 1;
            }
            let z = match op {
                0 => x * y,
                1 => x / y,
                2 => x + y,
                _ => x - y,
            };
            arithmetic_flags(cpu, x, y, z, op, true);
            let z = directed_arithmetic(cpu, x, y, z, op, true);
            finish(cpu, z.to_bits(), true)
        } else {
            let x = f32::from_bits(a as u32);
            let y = f32::from_bits(b as u32);
            if op == 1 && y == 0.0 && x != 0.0 && x.is_finite() {
                cpu.fpsr |= 1 << 1;
            }
            let z = match op {
                0 => x * y,
                1 => x / y,
                2 => x + y,
                _ => x - y,
            };
            arithmetic_flags(cpu, x as f64, y as f64, z as f64, op, false);
            let z = directed_arithmetic(cpu, x as f64, y as f64, z as f64, op, false) as f32;
            finish(cpu, z.to_bits() as u64, false)
        };
        cpu.v[d] = result as u128;
        return true;
    }
    // FCMP/FCMPE (register or zero).
    if insn & 0xff20_fc07 == 0x1e20_2000 {
        let a = input(cpu, raw(cpu.v[n]), double);
        let b = if insn & 8 != 0 {
            0
        } else {
            input(cpu, raw(cpu.v[m]), double)
        };
        let is_nan = if double {
            f64::from_bits(a).is_nan() || f64::from_bits(b).is_nan()
        } else {
            f32::from_bits(a as u32).is_nan() || f32::from_bits(b as u32).is_nan()
        };
        if is_nan {
            let _ = nan(cpu, a, Some(b), double);
            if insn & 16 != 0 {
                cpu.fpsr |= 1;
            }
            cpu.nzcv = 0x3000_0000;
        } else {
            let (x, y) = if double {
                (f64::from_bits(a), f64::from_bits(b))
            } else {
                (
                    f32::from_bits(a as u32) as f64,
                    f32::from_bits(b as u32) as f64,
                )
            };
            cpu.nzcv = if x == y {
                0x6000_0000
            } else if x < y {
                0x8000_0000
            } else {
                0x2000_0000
            };
        }
        return true;
    }
    false
}
