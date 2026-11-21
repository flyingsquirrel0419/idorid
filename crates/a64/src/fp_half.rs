//! IEEE binary16 scalar format conversion helpers.
use crate::cpu::Cpu;

fn round(v: f64, mode: u64, negative: bool) -> f64 {
    match mode {
        0 => v.round_ties_even(),
        1 if !negative => v.ceil(),
        2 if negative => v.ceil(),
        _ => v.floor(),
    }
}

pub(crate) fn exec(cpu: &mut Cpu, insn: u32) -> bool {
    convert(cpu, insn, false)
}

fn convert(cpu: &mut Cpu, insn: u32, arithmetic: bool) -> bool {
    let op = insn & 0xffff_fc00;
    if !matches!(op, 0x1e23_c000 | 0x1e63_c000 | 0x1ee2_4000 | 0x1ee2_c000) {
        return false;
    }
    let alternate = cpu.fpcr & (1 << 26) != 0 && !arithmetic;
    let n = ((insn >> 5) & 31) as usize;
    let d = (insn & 31) as usize;
    if op == 0x1ee2_4000 || op == 0x1ee2_c000 {
        let raw = cpu.v[n] as u16;
        let sign = (raw as u64 & 0x8000) << 48;
        let exp = (raw >> 10) & 31;
        let frac = raw & 1023;
        let is64 = op == 0x1ee2_c000;
        let bits = if !alternate && exp == 31 && frac != 0 {
            if frac & 512 == 0 {
                cpu.fpsr |= 1;
            }
            if cpu.fpcr & (1 << 25) != 0 {
                0x7ff8_0000_0000_0000
            } else {
                sign | 0x7ff0_0000_0000_0000 | (((frac | 512) as u64) << 42)
            }
        } else if !alternate && exp == 31 {
            sign | 0x7ff0_0000_0000_0000
        } else if exp == 0 && (frac == 0 || (arithmetic && cpu.fpcr & (1 << 19) != 0)) {
            // Zero, or a denormal flushed by FZ16.
            sign
        } else {
            let val = if exp == 0 {
                (frac as f64) * 2f64.powi(-24)
            } else {
                (1024 + frac as u32) as f64 * 2f64.powi(exp as i32 - 25)
            };
            val.to_bits() | sign
        };
        cpu.v[d] = if is64 {
            bits as u128
        } else if bits & 0x7ff0_0000_0000_0000 == 0x7ff0_0000_0000_0000
            && bits & 0x000f_ffff_ffff_ffff != 0
        {
            (((bits >> 32) as u32 & 0x8000_0000)
                | 0x7f80_0000
                | ((bits >> 29) as u32 & 0x007f_ffff)) as u128
        } else {
            (f64::from_bits(bits) as f32).to_bits() as u128
        };
        return true;
    }
    let is64 = op == 0x1e63_c000;
    let raw = if is64 {
        cpu.v[n] as u64
    } else {
        cpu.v[n] as u32 as u64
    };
    let (sign, exp, frac, quiet, denorm) = if is64 {
        (
            raw >> 63,
            raw & 0x7ff0_0000_0000_0000,
            raw & 0x000f_ffff_ffff_ffff,
            1u64 << 51,
            0x7ff0_0000_0000_0000,
        )
    } else {
        (
            raw >> 31,
            raw & 0x7f80_0000,
            raw & 0x007f_ffff,
            1u64 << 22,
            0x7f80_0000,
        )
    };
    let sign16 = (sign as u16) << 15;
    let out = if exp == denorm && frac != 0 {
        if frac & quiet == 0 || alternate {
            cpu.fpsr |= 1;
        }
        if alternate {
            sign16
        } else if cpu.fpcr & (1 << 25) != 0 {
            0x7e00
        } else {
            sign16 | 0x7c00 | (((frac >> if is64 { 42 } else { 13 }) as u16) | 0x200)
        }
    } else if exp == denorm {
        if alternate {
            cpu.fpsr |= 1;
            sign16 | 0x7fff
        } else {
            sign16 | 0x7c00
        }
    } else {
        let mut value = if is64 {
            f64::from_bits(raw)
        } else {
            f32::from_bits(raw as u32) as f64
        };
        if exp == 0 && frac != 0 && cpu.fpcr & (1 << 24) != 0 {
            cpu.fpsr |= 1 << 7;
            value = 0.0;
        }
        let abs = value.abs();
        let mode = (cpu.fpcr >> 22) & 3;
        if abs == 0.0 {
            sign16
        } else if arithmetic && cpu.fpcr & (1 << 19) != 0 && abs < 2f64.powi(-14) {
            cpu.fpsr |= 1 << 3;
            sign16
        } else if alternate && abs >= 131072.0 {
            cpu.fpsr |= 1;
            sign16 | 0x7fff
        } else if !alternate && abs >= 65536.0 {
            cpu.fpsr |= (1 << 2) | (1 << 4);
            let infinity = mode == 0 || (mode == 1 && sign == 0) || (mode == 2 && sign != 0);
            sign16 | if infinity { 0x7c00 } else { 0x7bff }
        } else {
            let e = abs.log2().floor() as i32;
            let step = if e < -14 { -24 } else { e - 10 };
            let scaled = abs * 2f64.powi(-step);
            let rounded = round(scaled, mode, sign != 0);
            let inexact = scaled != rounded;
            let result = if e < -14 {
                rounded as u16
            } else {
                (((e + 15) as u16) << 10) + (rounded as u16 - 1024)
            };
            if alternate && result >= 0x8000 {
                cpu.fpsr |= 1;
                cpu.v[d] = (sign16 | 0x7fff) as u128;
                return true;
            }
            if inexact {
                cpu.fpsr |= 1 << 4;
            }
            if !alternate && result >= 0x7c00 {
                cpu.fpsr |= (1 << 2) | (1 << 4);
            }
            if e < -14 && inexact {
                cpu.fpsr |= 1 << 3;
            }
            if result != 0 && result < 0x400 && arithmetic && cpu.fpcr & (1 << 19) != 0 {
                cpu.fpsr |= 1 << 3;
                sign16
            } else {
                sign16 | result
            }
        }
    };
    cpu.v[d] = out as u128;
    true
}

/// Scalar half arithmetic via exact widening to binary64 and narrowing once.
pub(crate) fn arithmetic(cpu: &mut Cpu, insn: u32) -> bool {
    if (insn >> 22) & 3 != 3 {
        return false;
    }
    let d = (insn & 31) as usize;
    let n = ((insn >> 5) & 31) as usize;
    let m = ((insn >> 16) & 31) as usize;
    let unary = insn & 0xff3f_fc00;
    if matches!(unary, 0x1e20_4000 | 0x1e20_c000 | 0x1e21_4000) {
        let v = cpu.v[n] as u16;
        cpu.v[d] = match unary {
            0x1e20_c000 => v & 0x7fff,
            0x1e21_4000 => v ^ 0x8000,
            _ => v,
        } as u128;
        return true;
    }
    let binary = insn & 0xff20_0c00 == 0x1e20_0800 && (insn >> 12) & 15 <= 8;
    let sqrt = unary == 0x1e21_c000;
    let fused = insn & 0xff00_0000 == 0x1f00_0000;
    let compare = insn & 0xff20_fc07 == 0x1e20_2000;
    if !binary && !sqrt && !fused && !compare {
        return false;
    }
    let original_fpcr = cpu.fpcr;
    let original_fpsr = cpu.fpsr;
    cpu.fpcr &= !(1 << 26);
    let a = cpu.v[n] as u16;
    let b = cpu.v[m] as u16;
    let addend = cpu.v[((insn >> 10) & 31) as usize] as u16;
    let saved = [cpu.v[28], cpu.v[29], cpu.v[30], cpu.v[31]];
    for (reg, value, used) in [
        (28, addend, fused),
        (29, a, true),
        (30, b, binary || fused || (compare && insn & 8 == 0)),
    ] {
        if !used {
            continue;
        }
        cpu.v[reg] = value as u128;
        let _ = convert(cpu, 0x1ee2_c000 | ((reg as u32) << 5) | reg as u32, true);
        if value & 0x7c00 == 0x7c00
            && value & 0x3ff != 0
            && value & 0x200 == 0
            && cpu.fpcr & (1 << 25) == 0
        {
            cpu.v[reg] &= !(1u128 << 51);
        }
    }
    let synthetic = if fused {
        (insn & 0xff20_8000) | (1 << 22) | (30 << 16) | (28 << 10) | (29 << 5) | 31
    } else if binary {
        (insn & 0xff20_fc00) | (1 << 22) | (30 << 16) | (29 << 5) | 31
    } else if sqrt {
        0x1e61_c000 | (29 << 5) | 31
    } else {
        0x1e60_2000 | (30 << 16) | (29 << 5) | (insn & 24)
    };
    let ok = crate::fp::exec(cpu, synthetic);
    let result = if ok && !compare {
        let value = f64::from_bits(cpu.v[31] as u64);
        if cpu.fpcr & (1 << 19) != 0
            && value != 0.0
            && value.is_finite()
            && value.abs() < 2f64.powi(-14)
        {
            cpu.fpsr = (cpu.fpsr & !(1 << 4)) | (original_fpsr & (1 << 4));
        }
        let _ = convert(cpu, 0x1e63_c000 | (31 << 5) | 31, true);
        cpu.v[31]
    } else {
        0
    };
    cpu.v[28..32].copy_from_slice(&saved);
    cpu.fpcr = original_fpcr;
    if ok && !compare {
        cpu.v[d] = result;
    }
    ok
}
