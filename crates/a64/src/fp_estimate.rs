//! Architectural reciprocal estimate operations on binary32 and binary64.
use crate::cpu::Cpu;

const fn sqrt_table() -> [u16; 512] {
    let mut result = [0; 512];
    let mut i = 128;
    while i < 512 {
        let a = if i < 256 { i * 2 + 1 } else { (i | 1) * 2 };
        let mut b = 512usize;
        while a * (b + 1) * (b + 1) < (1 << 28) {
            b += 1;
        }
        result[i] = b.div_ceil(2) as u16;
        i += 1;
    }
    result
}
const SQRT_TABLE: [u16; 512] = sqrt_table();

fn estimate(cpu: &mut Cpu, mut bits: u64, double: bool, sqrt: bool, recipx: bool) -> u64 {
    let frac_width = if double { 52 } else { 23 };
    let exp_bits = if double { 11 } else { 8 };
    let exp_max = (1u64 << exp_bits) - 1;
    let fm = (1u64 << frac_width) - 1;
    let sign = bits & (1u64 << (frac_width + exp_bits));
    let mut exp = ((bits >> frac_width) & exp_max) as i32;
    let mut frac = bits & fm;
    let inf = exp_max << frac_width;
    let quiet = 1u64 << (frac_width - 1);
    if exp as u64 == exp_max && frac != 0 {
        if frac & quiet == 0 {
            cpu.fpsr |= 1;
        }
        return if cpu.fpcr & (1 << 25) != 0 {
            inf | quiet
        } else {
            bits | quiet
        };
    }
    if exp == 0 && frac != 0 && cpu.fpcr & (1 << 24) != 0 {
        cpu.fpsr |= 1 << 7;
        bits = sign;
        frac = 0;
    }
    if recipx {
        return sign
            | (if exp == 0 {
                exp_max - 1
            } else {
                !exp as u64 & exp_max
            }) << frac_width;
    }
    if frac == 0 && exp == 0 {
        cpu.fpsr |= 1 << 1;
        return sign | inf;
    }
    if sqrt && sign != 0 {
        cpu.fpsr |= 1;
        return inf | quiet;
    }
    if exp as u64 == exp_max {
        return sign;
    }
    if !sqrt {
        if exp == 0 && frac < (1u64 << (frac_width - 2)) {
            cpu.fpsr |= (1 << 2) | (1 << 4);
            let mode = (cpu.fpcr >> 22) & 3;
            let to_inf = mode == 0 || (mode == 1 && sign == 0) || (mode == 2 && sign != 0);
            return sign | if to_inf { inf } else { inf - 1 };
        }
        if cpu.fpcr & (1 << 24) != 0 && exp >= (exp_max as i32) - 2 {
            cpu.fpsr |= 1 << 3;
            return sign;
        }
        if exp == 0 {
            if frac & (1u64 << (frac_width - 1)) == 0 {
                exp = -1;
                frac = (frac << 2) & fm;
            } else {
                frac = (frac << 1) & fm;
            }
        }
        let scaled = 256 | (frac >> (frac_width - 8)) as u32;
        let estimate = ((1u32 << 19) / (scaled * 2 + 1)).div_ceil(2);
        let mut rexp = (exp_max as i32) - 2 - exp;
        let mut rfrac = ((estimate as u64) & 255) << (frac_width - 8);
        if rexp == 0 {
            rfrac = (1u64 << (frac_width - 1)) | (rfrac >> 1);
        } else if rexp == -1 {
            rfrac = (1u64 << (frac_width - 2)) | (rfrac >> 2);
            rexp = 0;
        }
        return sign | ((rexp as u64) << frac_width) | rfrac;
    }
    if exp == 0 {
        while frac & (1u64 << (frac_width - 1)) == 0 {
            frac = (frac << 1) & fm;
            exp -= 1;
        }
        frac = (frac << 1) & fm;
    }
    let scaled = if exp & 1 == 0 {
        256 | (frac >> (frac_width - 8)) as usize
    } else {
        128 | (frac >> (frac_width - 7)) as usize
    };
    let rexp = if double {
        (3068 - exp) / 2
    } else {
        (380 - exp) / 2
    };
    let rfrac = ((SQRT_TABLE[scaled] as u64) & 255) << (frac_width - 8);
    let _ = bits;
    ((rexp as u64) << frac_width) | rfrac
}

pub(crate) fn step(cpu: &mut Cpu, insn: u32) -> bool {
    let key = insn & 0x8fa0_fc00;
    if !matches!(key, 0x0e20_fc00 | 0x0ea0_fc00) {
        return false;
    }
    // The unsigned bit is fixed clear for these encodings.
    if insn & (1 << 29) != 0 {
        return false;
    }
    let scalar = insn & (1 << 28) != 0;
    let q = insn & (1 << 30) != 0;
    let double = insn & (1 << 22) != 0;
    if (scalar && !q) || (!scalar && double && !q) {
        return false;
    }
    let sqrt = key == 0x0ea0_fc00;
    let width = if double { 64 } else { 32 };
    let lanes = if scalar {
        1
    } else {
        (if q { 128 } else { 64 }) / width
    };
    let d = (insn & 31) as usize;
    let n = ((insn >> 5) & 31) as usize;
    let m = ((insn >> 16) & 31) as usize;
    let a = cpu.v[n];
    let b = cpu.v[m];
    let lm = (1u128 << width) - 1;
    let mut out = 0;
    for i in 0..lanes {
        let x = crate::fp::input(cpu, ((a >> (i * width)) & lm) as u64, double);
        let y = crate::fp::input(cpu, ((b >> (i * width)) & lm) as u64, double);
        let value = if let Some(v) = crate::fp::nan(
            cpu,
            x ^ (if double { 1u64 << 63 } else { 1u64 << 31 }),
            Some(y),
            double,
        ) {
            v
        } else if double {
            let aa = f64::from_bits(x);
            let bb = f64::from_bits(y);
            let v = if (aa.is_infinite() && bb == 0.0) || (bb.is_infinite() && aa == 0.0) {
                if sqrt {
                    1.5
                } else {
                    2.0
                }
            } else if sqrt {
                if aa.abs() >= f64::MIN_POSITIVE * 2.0 {
                    (-aa * 0.5).mul_add(bb, 1.5)
                } else if bb.abs() >= f64::MIN_POSITIVE * 2.0 {
                    (-bb * 0.5).mul_add(aa, 1.5)
                } else {
                    1.5
                }
            } else {
                (-aa).mul_add(bb, 2.0)
            };
            crate::fp::finish(cpu, v.to_bits(), true)
        } else {
            let aa = f32::from_bits(x as u32);
            let bb = f32::from_bits(y as u32);
            let v = if (aa.is_infinite() && bb == 0.0) || (bb.is_infinite() && aa == 0.0) {
                if sqrt {
                    1.5
                } else {
                    2.0
                }
            } else if sqrt {
                if aa.abs() >= f32::MIN_POSITIVE * 2.0 {
                    (-aa * 0.5).mul_add(bb, 1.5)
                } else if bb.abs() >= f32::MIN_POSITIVE * 2.0 {
                    (-bb * 0.5).mul_add(aa, 1.5)
                } else {
                    1.5
                }
            } else {
                (-aa).mul_add(bb, 2.0)
            };
            crate::fp::finish(cpu, v.to_bits() as u64, false)
        };
        out |= (value as u128) << (i * width);
    }
    cpu.v[d] = out;
    true
}

pub(crate) fn exec(cpu: &mut Cpu, insn: u32) -> bool {
    let key = insn & 0x8fbf_fc00;
    let recipx = key == 0x0ea1_f800;
    if key != 0x0ea1_d800 && !recipx {
        return false;
    }
    let scalar = insn & (1 << 28) != 0;
    let q = insn & (1 << 30) != 0;
    let sqrt = insn & (1 << 29) != 0;
    if recipx && (!scalar || sqrt) {
        return false;
    }
    let double = insn & (1 << 22) != 0;
    if (scalar && !q) || (!scalar && double && !q) {
        return false;
    }
    let width = if double { 64 } else { 32 };
    let lanes = if scalar {
        1
    } else {
        (if q { 128 } else { 64 }) / width
    };
    let d = (insn & 31) as usize;
    let n = ((insn >> 5) & 31) as usize;
    let source = cpu.v[n];
    let mask = (1u128 << width) - 1;
    let mut out = 0;
    for i in 0..lanes {
        let x = ((source >> (i * width)) & mask) as u64;
        out |= (estimate(cpu, x, double, sqrt, recipx) as u128) << (i * width);
    }
    cpu.v[d] = out;
    true
}
