//! Elementwise vector floating-point operations using the scalar FP machinery.
use crate::cpu::Cpu;

pub(crate) fn exec(cpu: &mut Cpu, insn: u32) -> bool {
    // SCVTF/UCVTF (integer to FP), scalar and vector forms: reuse the
    // general-register conversion with a temporarily borrowed x16.
    if insn & 0xdfbf_fc00 == 0x5e21_d800 || insn & 0x9fbf_fc00 == 0x0e21_d800 {
        let scalar = insn & (1 << 28) != 0;
        let q = insn & (1 << 30) != 0;
        let u = (insn >> 29) & 1;
        let double = insn & (1 << 22) != 0;
        if !scalar && double && !q {
            return false;
        }
        let d = (insn & 31) as usize;
        let n = ((insn >> 5) & 31) as usize;
        let width = if double { 64 } else { 32 };
        let lanes = if scalar {
            1
        } else {
            (if q { 128 } else { 64 }) / width
        };
        let lm = (1u128 << width) - 1;
        let src = cpu.v[n];
        let saved_x = cpu.x[16];
        let saved_v = cpu.v[31];
        let base = if double { 0x9e62_0000 } else { 0x1e22_0000 };
        let mut out = 0u128;
        for i in 0..lanes {
            cpu.x[16] = ((src >> (i * width)) & lm) as u64;
            if !crate::fp::exec(cpu, base | (u << 16) | (16 << 5) | 31) {
                cpu.x[16] = saved_x;
                cpu.v[31] = saved_v;
                return false;
            }
            out |= (cpu.v[31] & lm) << (i * width);
        }
        cpu.x[16] = saved_x;
        cpu.v[31] = saved_v;
        cpu.v[d] = out;
        return true;
    }
    // Floating multiply and fused multiply-add/subtract by an indexed element.
    if insn & 0x8f80_0400 == 0x0f80_0000 {
        let op = (insn >> 12) & 15;
        let u = insn & (1 << 29) != 0;
        if (!u && matches!(op, 1 | 5 | 9)) || (u && op == 9) {
            let scalar = insn & (1 << 28) != 0;
            let q = insn & (1 << 30) != 0;
            let double = insn & (1 << 22) != 0;
            let h = (insn >> 11) & 1;
            let l = (insn >> 21) & 1;
            if (scalar && !q) || (double && (l != 0 || (!scalar && !q))) {
                return false;
            }
            let index = if double { h } else { (h << 1) | l };
            let width = if double { 64 } else { 32 };
            let lanes = if scalar {
                1
            } else {
                (if q { 128 } else { 64 }) / width
            };
            let d = (insn & 31) as usize;
            let n = ((insn >> 5) & 31) as usize;
            let m = ((insn >> 16) & 31) as usize;
            let src = cpu.v[n];
            let acc = cpu.v[d];
            let lm = (1u128 << width) - 1;
            let elem = (cpu.v[m] >> (index * width)) & lm;
            let saved = [cpu.v[28], cpu.v[29], cpu.v[30], cpu.v[31]];
            let mut out = 0u128;
            for i in 0..lanes {
                cpu.v[28] = (acc >> (i * width)) & lm;
                cpu.v[29] = (src >> (i * width)) & lm;
                cpu.v[30] = elem;
                let word = if op == 9 {
                    0x1e20_0800 | (if double { 1 << 22 } else { 0 }) | (30 << 16) | (29 << 5) | 31
                } else {
                    0x1f00_0000
                        | (if double { 1 << 22 } else { 0 })
                        | (30 << 16)
                        | (if op == 5 { 1 << 15 } else { 0 })
                        | (28 << 10)
                        | (29 << 5)
                        | 31
                };
                let x = crate::fp::input(cpu, cpu.v[29] as u64, double);
                let y = crate::fp::input(cpu, elem as u64, double);
                let exp = if double {
                    0x7ff0_0000_0000_0000
                } else {
                    0x7f80_0000
                };
                let sign = if double { 1u64 << 63 } else { 1u64 << 31 };
                let mulx_special = u
                    && (((x & !sign) == exp && (y & !sign) == 0)
                        || ((y & !sign) == exp && (x & !sign) == 0));
                let value = if mulx_special {
                    ((x ^ y) & sign) as u128
                        | if double {
                            0x4000_0000_0000_0000
                        } else {
                            0x4000_0000
                        }
                } else {
                    let _ = crate::fp::exec(cpu, word);
                    cpu.v[31]
                };
                out |= (value & lm) << (i * width);
            }
            cpu.v[28..32].copy_from_slice(&saved);
            cpu.v[d] = out;
            return true;
        }
    }
    // Per-lane integer/FP conversions, with optional fixed-point scaling.
    // Scalar forms (01U11110...) fold onto the vector keys by clearing bit 28.
    let cvt_key = if insn & 0x5000_0000 == 0x5000_0000 {
        insn & 0x8fbf_fc00
    } else {
        insn & 0x9fbf_fc00
    };
    let immediate = insn & 0x8f80_0400 == 0x0f00_0400 && matches!((insn >> 11) & 31, 28 | 31);
    let conversion = match cvt_key {
        0x0e21_d800 => Some(2u32),
        0x0ea1_b800 => Some(24),
        0x0e21_a800 => Some(0),
        0x0ea1_a800 => Some(8),
        0x0e21_b800 => Some(16),
        0x0e21_c800 => Some(4),
        _ => None,
    };
    if immediate || conversion.is_some() {
        let scalar = insn & (1 << 28) != 0;
        let q = insn & (1 << 30) != 0;
        let unsigned = insn & (1 << 29) != 0;
        let imm = (insn >> 16) & 127;
        let width = if immediate {
            if imm < 32 {
                return false;
            }
            1u32 << (31 - imm.leading_zeros())
        } else if insn & (1 << 22) != 0 {
            64
        } else {
            32
        };
        if width > 64 || (scalar && !q) || (!scalar && width == 64 && !q) {
            return false;
        }
        let fbits = if immediate { 2 * width - imm } else { 0 };
        let opcode = if immediate {
            if (insn >> 11) & 31 == 28 {
                2
            } else {
                24
            }
        } else {
            conversion.unwrap()
        };
        let opcode = opcode + unsigned as u32;
        let to_float = opcode == 2 || opcode == 3;
        let lanes = if scalar {
            1
        } else {
            (if q { 128 } else { 64 }) / width
        };
        let d = (insn & 31) as usize;
        let n = ((insn >> 5) & 31) as usize;
        let source = cpu.v[n];
        let lm = (1u128 << width) - 1;
        let saved_x = cpu.x[30];
        let saved_v = cpu.v[30];
        let mut out = 0u128;
        for i in 0..lanes {
            let value = ((source >> (i * width)) & lm) as u64;
            cpu.x[30] = value;
            cpu.v[30] = value as u128;
            let mut word = if immediate {
                0x1e00_0000 | ((64 - fbits) << 10)
            } else {
                0x1e20_0000
            };
            word |= (opcode << 16) | (30 << 5) | 30;
            if width == 64 {
                word |= (1 << 31) | (1 << 22);
            }
            if !crate::fp::exec(cpu, word) {
                cpu.x[30] = saved_x;
                cpu.v[30] = saved_v;
                return false;
            }
            let result = if to_float {
                cpu.v[30]
            } else {
                cpu.x[30] as u128
            };
            out |= (result & lm) << (i * width);
        }
        cpu.x[30] = saved_x;
        cpu.v[30] = saved_v;
        cpu.v[d] = out;
        return true;
    }
    // Floating comparisons against zero, absolute comparisons, and FABD.
    let zero_key = insn & 0xafbf_fc00;
    let zero_op = match zero_key {
        0x0ea0_d800 => Some(0),
        0x0ea0_c800 => Some(1),
        0x2ea0_c800 => Some(2),
        0x0ea0_e800 => Some(3),
        0x2ea0_d800 => Some(4),
        _ => None,
    };
    let other = insn & 0xafa0_fc00;
    let other_op = match other {
        0x2ea0_ec00 => Some(5),
        0x2e20_ec00 => Some(6),
        0x2ea0_d400 => Some(7),
        _ => None,
    };
    if let Some(op) = zero_op.or(other_op) {
        let scalar = insn & (1 << 28) != 0;
        let q = insn & (1 << 30) != 0;
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
        let m = ((insn >> 16) & 31) as usize;
        let a = cpu.v[n];
        let b = cpu.v[m];
        let lm = (1u128 << width) - 1;
        let sign = 1u128 << (width - 1);
        let saved = [cpu.v[29], cpu.v[30], cpu.v[31]];
        let flags = cpu.nzcv;
        let mut out = 0u128;
        for i in 0..lanes {
            let mut x = (a >> (i * width)) & lm;
            let mut y = if op <= 4 { 0 } else { (b >> (i * width)) & lm };
            if op == 5 || op == 6 {
                x &= !sign;
                y &= !sign;
            }
            cpu.v[29] = x;
            cpu.v[30] = y;
            let word = if op == 7 {
                0x1e20_3800 | (if double { 1 << 22 } else { 0 }) | (30 << 16) | (29 << 5) | 31
            } else {
                0x1e20_2000
                    | (if double { 1 << 22 } else { 0 })
                    | (30 << 16)
                    | (29 << 5)
                    | if op == 0 { 0 } else { 16 }
            };
            let _ = crate::fp::exec(cpu, word);
            let nz = cpu.nzcv;
            let val = match op {
                0 => {
                    if nz == 0x6000_0000 {
                        lm
                    } else {
                        0
                    }
                }
                1 | 5 => {
                    if nz == 0x2000_0000 {
                        lm
                    } else {
                        0
                    }
                }
                2 | 6 => {
                    if nz == 0x2000_0000 || nz == 0x6000_0000 {
                        lm
                    } else {
                        0
                    }
                }
                3 => {
                    if nz == 0x8000_0000 {
                        lm
                    } else {
                        0
                    }
                }
                4 => {
                    if nz == 0x8000_0000 || nz == 0x6000_0000 {
                        lm
                    } else {
                        0
                    }
                }
                _ => cpu.v[31] & !sign,
            };
            out |= (val & lm) << (i * width);
        }
        cpu.v[29..32].copy_from_slice(&saved);
        cpu.nzcv = flags;
        cpu.v[d] = out;
        return true;
    }
    // Scalar result from a pair of FP lanes.
    let scalar_pair = match insn & 0xffbf_fc00 {
        0x7e30_d800 => Some(2),
        0x7e30_f800 => Some(4),
        0x7eb0_f800 => Some(5),
        0x7e30_c800 => Some(6),
        0x7eb0_c800 => Some(7),
        _ => None,
    };
    if let Some(op) = scalar_pair {
        let double = insn & (1 << 22) != 0;
        let width = if double { 64 } else { 32 };
        let d = (insn & 31) as usize;
        let n = ((insn >> 5) & 31) as usize;
        let src = cpu.v[n];
        let lm = (1u128 << width) - 1;
        let saved = [cpu.v[29], cpu.v[30], cpu.v[31]];
        cpu.v[29] = src & lm;
        cpu.v[30] = (src >> width) & lm;
        let word = 0x1e20_0800
            | (if double { 1 << 22 } else { 0 })
            | (30 << 16)
            | (op << 12)
            | (29 << 5)
            | 31;
        let ok = crate::fp::exec(cpu, word);
        let result = cpu.v[31];
        cpu.v[29..32].copy_from_slice(&saved);
        if ok {
            cpu.v[d] = result;
        }
        return ok;
    }
    // FP widening and narrowing conversions, including round-to-odd FCVTXN.
    let cvt = insn & 0x9fbf_fc00;
    if matches!(cvt, 0x0e21_6800 | 0x0e21_7800) {
        let scalar = insn & (1 << 28) != 0;
        let q = insn & (1 << 30) != 0;
        let odd = insn & (1 << 29) != 0;
        let size = (insn >> 22) & 1;
        let widen = cvt == 0x0e21_7800;
        if (scalar && (!odd || !q || widen)) || (odd && (widen || size == 0)) {
            return false;
        }
        let src_width = if widen {
            if size == 0 {
                16
            } else {
                32
            }
        } else if size == 0 {
            32
        } else {
            64
        };
        let dst_width = if widen { src_width * 2 } else { src_width / 2 };
        let lanes = if scalar {
            1
        } else {
            64 / (if widen { src_width } else { dst_width })
        };
        let d = (insn & 31) as usize;
        let n = ((insn >> 5) & 31) as usize;
        let source = cpu.v[n];
        let original = cpu.v[d];
        let saved = [cpu.v[30], cpu.v[31]];
        let mut output = if !widen && q && !scalar {
            original & u64::MAX as u128
        } else {
            0
        };
        let prev_fpcr = cpu.fpcr;
        for i in 0..lanes {
            let shift = i * src_width + if widen && q { 64 } else { 0 };
            let value = (source >> shift) & ((1u128 << src_width) - 1);
            cpu.v[30] = value;
            let op = match (widen, size) {
                (true, 0) => 0x1ee2_4000,
                (true, 1) => 0x1e22_c000,
                (false, 0) => 0x1e23_c000,
                _ => 0x1e62_4000,
            };
            if odd {
                cpu.fpcr = (cpu.fpcr & !(3 << 22)) | (3 << 22);
            }
            let converted = if size == 0 {
                crate::fp_half::exec(cpu, op | (30 << 5) | 31)
            } else {
                crate::fp::exec(cpu, op | (30 << 5) | 31)
            };
            cpu.fpcr = prev_fpcr;
            if !converted {
                cpu.v[30..32].copy_from_slice(&saved);
                return false;
            }
            let mut val = cpu.v[31];
            if odd {
                let x = f64::from_bits(value as u64);
                let y = f32::from_bits(val as u32) as f64;
                if x.is_finite() && x != y {
                    val |= 1;
                }
            }
            output |= val << (i * dst_width + if !widen && q && !scalar { 64 } else { 0 });
        }
        cpu.v[30..32].copy_from_slice(&saved);
        cpu.v[d] = output;
        return true;
    }
    let reduce = match insn & 0xffff_fc00 {
        0x6e30_f800 => Some(4u32),
        0x6eb0_f800 => Some(5),
        0x6e30_c800 => Some(6),
        0x6eb0_c800 => Some(7),
        _ => None,
    };
    let pair = match insn & 0xbfa0_fc00 {
        0x2e20_d400 => Some(2u32),
        0x2e20_f400 => Some(4),
        0x2ea0_f400 => Some(5),
        0x2e20_c400 => Some(6),
        0x2ea0_c400 => Some(7),
        _ => None,
    };
    if let Some(op) = reduce.or(pair) {
        let q = insn & (1 << 30) != 0;
        let double = if reduce.is_some() {
            false
        } else {
            insn & (1 << 22) != 0
        };
        if double && !q {
            return false;
        }
        let width = if double { 64 } else { 32 };
        let lanes = (if q { 128 } else { 64 }) / width;
        let lm = (1u128 << width) - 1;
        let d = (insn & 31) as usize;
        let n = ((insn >> 5) & 31) as usize;
        let m = ((insn >> 16) & 31) as usize;
        let a = cpu.v[n];
        let b = cpu.v[m];
        let saved = [cpu.v[29], cpu.v[30], cpu.v[31]];
        let mut result = 0u128;
        let combine = |cpu: &mut Cpu, x: u128, y: u128| -> u128 {
            cpu.v[29] = x;
            cpu.v[30] = y;
            let scalar = 0x1e20_0800
                | (if double { 1 << 22 } else { 0 })
                | (30 << 16)
                | (op << 12)
                | (29 << 5)
                | 31;
            let _ = crate::fp::exec(cpu, scalar);
            cpu.v[31] & lm
        };
        if reduce.is_some() {
            let x = combine(cpu, a & lm, (a >> width) & lm);
            let y = combine(cpu, (a >> (2 * width)) & lm, (a >> (3 * width)) & lm);
            result = combine(cpu, x, y);
        } else {
            for i in 0..lanes {
                let source = if i < lanes / 2 { a } else { b };
                let j = (i % (lanes / 2)) * 2;
                let x = (source >> (j * width)) & lm;
                let y = (source >> ((j + 1) * width)) & lm;
                result |= combine(cpu, x, y) << (i * width);
            }
        }
        cpu.v[29..32].copy_from_slice(&saved);
        cpu.v[d] = result;
        return true;
    }
    let round_n = match insn & 0xbfbf_fc00 {
        0x0e21_e800 => Some(0x1e28_4000),
        0x2e21_e800 => Some(0x1e28_c000),
        0x0e21_f800 => Some(0x1e29_4000),
        0x2e21_f800 => Some(0x1e29_c000),
        _ => None,
    };
    let unary = round_n.or(match insn & 0xbfbf_fc00 {
        0x0ea0_f800 => Some(0x1e20_c000),
        0x2ea0_f800 => Some(0x1e21_4000),
        0x2ea1_f800 => Some(0x1e21_c000),
        0x0e21_8800 => Some(0x1e24_4000),
        0x0ea1_8800 => Some(0x1e24_c000),
        0x0e21_9800 => Some(0x1e25_4000),
        0x0ea1_9800 => Some(0x1e25_c000),
        0x2e21_8800 => Some(0x1e26_4000),
        0x2e21_9800 => Some(0x1e27_4000),
        0x2ea1_9800 => Some(0x1e27_c000),
        _ => None,
    });
    if let Some(op) = unary {
        let q = insn & (1 << 30) != 0;
        let double = insn & (1 << 22) != 0;
        if double && !q {
            return false;
        }
        let width = if double { 64 } else { 32 };
        let lanes = (if q { 128 } else { 64 }) / width;
        let mask = (1u128 << width) - 1;
        let d = (insn & 31) as usize;
        let n = ((insn >> 5) & 31) as usize;
        let a = cpu.v[n];
        let save = [cpu.v[30], cpu.v[31]];
        let mut result = 0;
        for i in 0..lanes {
            cpu.v[30] = (a >> (i * width)) & mask;
            if !crate::fp::exec(
                cpu,
                op | (if double { 1 << 22 } else { 0 }) | (30 << 5) | 31,
            ) {
                cpu.v[30..32].copy_from_slice(&save);
                return false;
            }
            result |= (cpu.v[31] & mask) << (i * width);
        }
        cpu.v[30..32].copy_from_slice(&save);
        cpu.v[d] = result;
        return true;
    }
    // FMULX differs from FMUL for zero times infinity.
    if insn & 0xbfa0_fc00 == 0x0e20_dc00 {
        let q = insn & (1 << 30) != 0;
        let double = insn & (1 << 22) != 0;
        if double && !q {
            return false;
        }
        let width = if double { 64 } else { 32 };
        let lanes = (if q { 128 } else { 64 }) / width;
        let d = (insn & 31) as usize;
        let n = ((insn >> 5) & 31) as usize;
        let m = ((insn >> 16) & 31) as usize;
        let a = cpu.v[n];
        let b = cpu.v[m];
        let lm = (1u128 << width) - 1;
        let saved = [cpu.v[29], cpu.v[30], cpu.v[31]];
        let mut out = 0u128;
        for i in 0..lanes {
            let x = crate::fp::input(cpu, ((a >> (i * width)) & lm) as u64, double);
            let y = crate::fp::input(cpu, ((b >> (i * width)) & lm) as u64, double);
            let sign = if double { 1u64 << 63 } else { 1u64 << 31 };
            let inf = if double {
                0x7ff0_0000_0000_0000
            } else {
                0x7f80_0000
            };
            let special = ((x & !sign) == inf && (y & !sign) == 0)
                || ((y & !sign) == inf && (x & !sign) == 0);
            let value = if special {
                ((x ^ y) & sign) as u128
                    | if double {
                        0x4000_0000_0000_0000
                    } else {
                        0x4000_0000
                    }
            } else {
                cpu.v[29] = x as u128;
                cpu.v[30] = y as u128;
                let word =
                    0x1e20_0800 | (if double { 1 << 22 } else { 0 }) | (30 << 16) | (29 << 5) | 31;
                let _ = crate::fp::exec(cpu, word);
                cpu.v[31]
            };
            out |= (value & lm) << (i * width);
        }
        cpu.v[29..32].copy_from_slice(&saved);
        cpu.v[d] = out;
        return true;
    }
    let key = insn & 0xbfa0_fc00;
    let operation = match key {
        0x0e20_d400 => 0,
        0x0ea0_d400 => 1,
        0x2e20_dc00 => 2,
        0x2e20_fc00 => 3,
        0x0e20_f400 => 4,
        0x0ea0_f400 => 5,
        0x0e20_c400 => 6,
        0x0ea0_c400 => 7,
        0x0e20_cc00 => 8,
        0x0ea0_cc00 => 9,
        0x0e20_e400 => 10,
        0x2ea0_e400 => 11,
        0x2e20_e400 => 12,
        _ => return false,
    };
    let q = insn & (1 << 30) != 0;
    let double = insn & (1 << 22) != 0;
    if double && !q {
        return false;
    }
    let width = if double { 64 } else { 32 };
    let lanes = (if q { 128 } else { 64 }) / width;
    let lm = (1u128 << width) - 1;
    let d = (insn & 31) as usize;
    let n = ((insn >> 5) & 31) as usize;
    let m = ((insn >> 16) & 31) as usize;
    let a = cpu.v[n];
    let b = cpu.v[m];
    let acc = cpu.v[d];
    let saved = [cpu.v[28], cpu.v[29], cpu.v[30], cpu.v[31]];
    let flags = cpu.nzcv;
    let mut out = 0u128;
    for i in 0..lanes {
        cpu.v[28] = (acc >> (i * width)) & lm;
        cpu.v[29] = (a >> (i * width)) & lm;
        cpu.v[30] = (b >> (i * width)) & lm;
        let ty = if double { 1 << 22 } else { 0 };
        let scalar = match operation {
            0..=7 => {
                let op = [2u32, 3, 0, 1, 4, 5, 6, 7][operation];
                0x1e20_0800 | ty | (30 << 16) | (op << 12) | (29 << 5) | 31
            }
            8 | 9 => {
                0x1f00_0000
                    | ty
                    | (30 << 16)
                    | (if operation == 9 { 1 << 15 } else { 0 })
                    | (28 << 10)
                    | (29 << 5)
                    | 31
            }
            _ => 0x1e20_2000 | ty | (30 << 16) | (29 << 5) | (if operation == 10 { 0 } else { 16 }),
        };
        if !crate::fp::exec(cpu, scalar) {
            cpu.v[28..32].copy_from_slice(&saved);
            cpu.nzcv = flags;
            return false;
        }
        let result = if operation < 10 {
            cpu.v[31] & lm
        } else {
            let nzcv = cpu.nzcv;
            let pass = match operation {
                10 => nzcv == 0x6000_0000,
                11 => nzcv == 0x2000_0000,
                _ => nzcv == 0x2000_0000 || nzcv == 0x6000_0000,
            };
            if pass {
                lm
            } else {
                0
            }
        };
        out |= result << (i * width);
    }
    cpu.v[28..32].copy_from_slice(&saved);
    cpu.nzcv = flags;
    cpu.v[d] = out;
    true
}
