//! SIMD and floating-point dispatch and register memory transfers.
use crate::access;
use crate::cpu::{Cpu, Exit};
use crate::mem::AddressSpace;

pub(crate) fn exec_dp<A: AddressSpace + ?Sized>(
    cpu: &mut Cpu,
    _mem: &A,
    insn: u32,
) -> Result<(), Exit> {
    if crate::fp_estimate::step(cpu, insn)
        || crate::fp_estimate::exec(cpu, insn)
        || crate::fp_half::exec(cpu, insn)
        || crate::fp_half::arithmetic(cpu, insn)
        || crate::fp::exec(cpu, insn)
        || crate::fp_vector_half::exec(cpu, insn)
        || crate::fp_vector::exec(cpu, insn)
        || crate::simd_crypto::exec(cpu, insn)
        || vector(cpu, insn)
    {
        Ok(())
    } else {
        Err(Exit::Undefined(insn))
    }
}

fn vector(cpu: &mut Cpu, insn: u32) -> bool {
    let d = (insn & 31) as usize;
    let n = ((insn >> 5) & 31) as usize;
    let m = ((insn >> 16) & 31) as usize;
    let q = insn & (1 << 30) != 0;
    let mask = if q { u128::MAX } else { u64::MAX as u128 };
    let a = cpu.v[n];
    let b = cpu.v[m];
    let old = cpu.v[d];
    // Variable per-lane shifts, optionally rounding and saturating.
    let var_shift = insn & 0x8f20_e400;
    if var_shift == 0x0e20_4400 {
        let scalar = insn & (1 << 28) != 0;
        if scalar && !q {
            return false;
        }
        let width = 8 << ((insn >> 22) & 3);
        if !scalar && !q && width == 64 {
            return false;
        }
        let lanes = if scalar {
            1
        } else {
            (if q { 128 } else { 64 }) / width
        };
        let lm = (1u128 << width) - 1;
        let unsigned = insn & (1 << 29) != 0;
        let sat = insn & (1 << 11) != 0;
        let rounding = insn & (1 << 12) != 0;
        let mut out = 0u128;
        for i in 0..lanes {
            let raw = (a >> (i * width)) & lm;
            let amount = ((b >> (i * width)) as u8 as i8) as i32;
            let value = if unsigned {
                raw as i128
            } else {
                ((raw << (128 - width)) as i128) >> (128 - width)
            };
            let result = if amount >= 0 {
                if sat {
                    let lo = if unsigned { 0 } else { -(1i128 << (width - 1)) };
                    let hi = if unsigned {
                        lm as i128
                    } else {
                        (1i128 << (width - 1)) - 1
                    };
                    if amount >= width as i32 {
                        if value != 0 {
                            cpu.fpsr |= 1 << 27;
                            if value < 0 {
                                lo
                            } else {
                                hi
                            }
                        } else {
                            0
                        }
                    } else {
                        let shifted = value << amount;
                        let clamped = shifted.clamp(lo, hi);
                        if clamped != shifted {
                            cpu.fpsr |= 1 << 27;
                        }
                        clamped
                    }
                } else if amount >= width as i32 {
                    0
                } else {
                    value << amount
                }
            } else {
                let shift = (-amount) as u32;
                if shift > width {
                    if !rounding && value < 0 {
                        -1
                    } else {
                        0
                    }
                } else {
                    let offset = if rounding { 1i128 << (shift - 1) } else { 0 };
                    (value + offset) >> shift
                }
            };
            out |= ((result as u128) & lm) << (i * width);
        }
        cpu.v[d] = out;
        return true;
    }
    // Unsigned reciprocal and reciprocal-square-root estimate tables.
    let estimate = insn & 0xbfff_fc00;
    if matches!(estimate, 0x0ea1_c800 | 0x2ea1_c800) {
        let sqrt = estimate == 0x2ea1_c800;
        let mut out = 0u128;
        for i in 0..if q { 4 } else { 2 } {
            let x = (a >> (32 * i)) as u32;
            let result = if (!sqrt && x < 0x8000_0000) || (sqrt && x < 0x4000_0000) {
                u32::MAX
            } else if !sqrt {
                let q = x >> 23;
                let s = ((262144u32 * 2) / (2 * q + 1)).div_ceil(2);
                s << 23
            } else {
                let (q, scale) = if x & 0x8000_0000 == 0 {
                    (x >> 23, 512.0)
                } else {
                    (x >> 24, 256.0)
                };
                let s = (256.0 / (((q as f64) + 0.5) / scale).sqrt() + 0.5) as u32;
                s << 23
            };
            out |= (result as u128) << (32 * i);
        }
        cpu.v[d] = out;
        return true;
    }
    // Saturating add/subtract and unary absolute/negate (scalar or vector).
    let sat_binary = insn & 0x8f20_fc00;
    let sat_unary = insn & 0x8f3f_fc00;
    if matches!(sat_binary, 0x0e20_0c00 | 0x0e20_2c00) || sat_unary == 0x0e20_7800 {
        let scalar = insn & (1 << 28) != 0;
        if scalar && !q {
            return false;
        }
        let width = 8 << ((insn >> 22) & 3);
        if !scalar && !q && width == 64 {
            return false;
        }
        let lanes = if scalar {
            1
        } else {
            (if q { 128 } else { 64 }) / width
        };
        let lm = (1u128 << width) - 1;
        let unsigned = insn & (1 << 29) != 0;
        let unary = sat_unary == 0x0e20_7800;
        let mut out = 0;
        for i in 0..lanes {
            let x = (a >> (i * width)) & lm;
            let y = (b >> (i * width)) & lm;
            let signed_x = ((x << (128 - width)) as i128) >> (128 - width);
            let signed_y = ((y << (128 - width)) as i128) >> (128 - width);
            let (v, lo, hi) = if unary {
                (
                    if unsigned { -signed_x } else { signed_x.abs() },
                    -(1i128 << (width - 1)),
                    (1i128 << (width - 1)) - 1,
                )
            } else if unsigned {
                (
                    if sat_binary == 0x0e20_0c00 {
                        x as i128 + y as i128
                    } else {
                        x as i128 - y as i128
                    },
                    0,
                    lm as i128,
                )
            } else {
                (
                    if sat_binary == 0x0e20_0c00 {
                        signed_x + signed_y
                    } else {
                        signed_x - signed_y
                    },
                    -(1i128 << (width - 1)),
                    (1i128 << (width - 1)) - 1,
                )
            };
            let result = v.clamp(lo, hi);
            if result != v {
                cpu.fpsr |= 1 << 27;
            }
            out |= ((result as u128) & lm) << (i * width);
        }
        cpu.v[d] = out;
        return true;
    }
    // Four byte products accumulate into each 32-bit lane (dot product).
    let dot_vector = insn & 0x9fe0_fc00 == 0x0e80_9400;
    let dot_indexed = insn & 0x9fc0_f400 == 0x0f80_e000;
    if dot_vector || dot_indexed {
        let unsigned = insn & (1 << 29) != 0;
        let index = (((insn >> 11) & 1) << 1) | ((insn >> 21) & 1);
        let mut out = 0u128;
        for i in 0..if q { 4 } else { 2 } {
            let mut acc = ((old >> (i * 32)) & 0xffff_ffff) as u32;
            for j in 0..4 {
                let x = ((a >> (i * 32 + j * 8)) & 255) as u8;
                let pos = if dot_indexed {
                    index * 32 + j * 8
                } else {
                    i * 32 + j * 8
                };
                let y = ((b >> pos) & 255) as u8;
                let product = if unsigned {
                    (x as u32).wrapping_mul(y as u32)
                } else {
                    (x as i8 as i32).wrapping_mul(y as i8 as i32) as u32
                };
                acc = acc.wrapping_add(product);
            }
            out |= (acc as u128) << (i * 32);
        }
        cpu.v[d] = out;
        return true;
    }
    // Saturating doubling multiply by an indexed element.
    if insn & 0x8f00_0400 == 0x0f00_0000 {
        let opcode = (insn >> 12) & 15;
        let u = insn & (1 << 29) != 0;
        let valid =
            (!u && matches!(opcode, 3 | 7 | 11 | 12 | 13)) || (u && matches!(opcode, 13 | 15));
        if valid {
            let scalar = insn & (1 << 28) != 0;
            let size = (insn >> 22) & 3;
            if !matches!(size, 1 | 2) || (scalar && !q) {
                return false;
            }
            let width = 8 << size;
            let long = matches!(opcode, 3 | 7 | 11);
            let out_width = if long { 2 * width } else { width };
            let h = (insn >> 11) & 1;
            let l = (insn >> 21) & 1;
            let mr = (insn >> 20) & 1;
            let index = if width == 16 {
                (h << 2) | (l << 1) | mr
            } else {
                (h << 1) | l
            };
            let reg = if width == 16 {
                ((insn >> 16) & 15) as usize
            } else {
                ((insn >> 16) & 31) as usize
            };
            let lm = (1u128 << width) - 1;
            let om = (1u128 << out_width) - 1;
            let signed = |v: u128, w: u32| -> i128 { ((v << (128 - w)) as i128) >> (128 - w) };
            let elem = signed((cpu.v[reg] >> (index * width)) & lm, width);
            let lo = -(1i128 << (out_width - 1));
            let hi = (1i128 << (out_width - 1)) - 1;
            let lanes = if scalar {
                1
            } else if long {
                64 / width
            } else {
                (if q { 128 } else { 64 }) / width
            };
            let mut out = 0;
            for i in 0..lanes {
                let shift = i * width + if long && q && !scalar { 64 } else { 0 };
                let prod = 2 * signed((a >> shift) & lm, width) * elem;
                let prev = signed((old >> (i * out_width)) & om, out_width);
                let value = if long {
                    let p = prod.clamp(lo, hi);
                    if p != prod {
                        cpu.fpsr |= 1 << 27;
                    }
                    if opcode == 3 {
                        prev + p
                    } else if opcode == 7 {
                        prev - p
                    } else {
                        p
                    }
                } else {
                    let acc = if u { prev << width } else { 0 };
                    let p = if opcode == 15 { -prod } else { prod };
                    (acc + p
                        + if opcode == 13 || opcode == 15 {
                            1i128 << (width - 1)
                        } else {
                            0
                        })
                        >> width
                };
                let clipped = value.clamp(lo, hi);
                if clipped != value {
                    cpu.fpsr |= 1 << 27;
                }
                out |= ((clipped as u128) & om) << (i * out_width);
            }
            cpu.v[d] = out;
            return true;
        }
    }
    // Integer multiply by an indexed vector element, including long forms.
    if insn & 0x9f00_0400 == 0x0f00_0000 && insn & (1 << 28) == 0 {
        let opcode = (insn >> 12) & 15;
        let unsigned = insn & (1 << 29) != 0;
        let size = (insn >> 22) & 3;
        let valid = matches!(size, 1 | 2)
            && ((opcode == 8 && !unsigned)
                || (matches!(opcode, 0 | 4) && unsigned)
                || matches!(opcode, 2 | 6 | 10));
        if valid {
            let width = 8 << size;
            let h = (insn >> 11) & 1;
            let l = (insn >> 21) & 1;
            let mr = (insn >> 20) & 1;
            let index = if width == 16 {
                (h << 2) | (l << 1) | mr
            } else {
                (h << 1) | l
            };
            let reg = if width == 16 {
                ((insn >> 16) & 15) as usize
            } else {
                ((insn >> 16) & 31) as usize
            };
            let lm = (1u128 << width) - 1;
            let elem = (cpu.v[reg] >> (index * width)) & lm;
            let long = matches!(opcode, 2 | 6 | 10);
            let out_width = if long { width * 2 } else { width };
            let out_mask = (1u128 << out_width) - 1;
            let lanes = if long {
                64 / width
            } else {
                (if q { 128 } else { 64 }) / width
            };
            let mut out = 0;
            for i in 0..lanes {
                let x = (a >> (i * width + if long && q { 64 } else { 0 })) & lm;
                let extend = |v: u128| {
                    if !unsigned && long {
                        (((v << (128 - width)) as i128) >> (128 - width)) as u128
                    } else {
                        v
                    }
                };
                let product = extend(x).wrapping_mul(extend(elem));
                let prev = (old >> (i * out_width)) & out_mask;
                let result = match opcode {
                    0 | 2 => prev.wrapping_add(product),
                    4 | 6 => prev.wrapping_sub(product),
                    _ => product,
                };
                out |= (result & out_mask) << (i * out_width);
            }
            cpu.v[d] = out;
            return true;
        }
    }
    // Scalar integer operations on D registers.
    let scalar_op = insn & 0xffe0_fc00;
    if matches!(
        scalar_op,
        0x5ee0_8400
            | 0x7ee0_8400
            | 0x7ee0_8c00
            | 0x5ee0_8c00
            | 0x5ee0_3400
            | 0x5ee0_3c00
            | 0x7ee0_3400
            | 0x7ee0_3c00
    ) {
        let x = a as u64;
        let y = b as u64;
        let v = match scalar_op {
            0x5ee0_8400 => x.wrapping_add(y),
            0x7ee0_8400 => x.wrapping_sub(y),
            0x7ee0_8c00 => {
                if x == y {
                    u64::MAX
                } else {
                    0
                }
            }
            0x5ee0_8c00 => {
                if x & y != 0 {
                    u64::MAX
                } else {
                    0
                }
            }
            0x5ee0_3400 => {
                if (x as i64) > (y as i64) {
                    u64::MAX
                } else {
                    0
                }
            }
            0x5ee0_3c00 => {
                if (x as i64) >= (y as i64) {
                    u64::MAX
                } else {
                    0
                }
            }
            0x7ee0_3400 => {
                if x > y {
                    u64::MAX
                } else {
                    0
                }
            }
            _ => {
                if x >= y {
                    u64::MAX
                } else {
                    0
                }
            }
        };
        cpu.v[d] = v as u128;
        return true;
    }
    let scalar_misc = insn & 0xffff_fc00;
    if matches!(
        scalar_misc,
        0x5ee0_b800
            | 0x7ee0_b800
            | 0x5ee0_9800
            | 0x5ee0_8800
            | 0x7ee0_8800
            | 0x5ee0_a800
            | 0x7ee0_9800
    ) {
        let x = a as u64;
        let signed = x as i64;
        let v = match scalar_misc {
            0x5ee0_b800 => signed.wrapping_abs() as u64,
            0x7ee0_b800 => x.wrapping_neg(),
            0x5ee0_9800 => {
                if x == 0 {
                    u64::MAX
                } else {
                    0
                }
            }
            0x5ee0_8800 => {
                if signed > 0 {
                    u64::MAX
                } else {
                    0
                }
            }
            0x7ee0_8800 => {
                if signed >= 0 {
                    u64::MAX
                } else {
                    0
                }
            }
            0x5ee0_a800 => {
                if signed < 0 {
                    u64::MAX
                } else {
                    0
                }
            }
            _ => {
                if signed <= 0 {
                    u64::MAX
                } else {
                    0
                }
            }
        };
        cpu.v[d] = v as u128;
        return true;
    }
    // Signed and unsigned halving add/subtract, with optional rounding.
    let half_op = insn & 0x9f20_fc00;
    if matches!(half_op, 0x0e20_0400 | 0x0e20_1400 | 0x0e20_2400) {
        let size = (insn >> 22) & 3;
        if size == 3 {
            return false;
        }
        let width = 8 << size;
        let lm = (1u128 << width) - 1;
        let unsigned = insn & (1 << 29) != 0;
        let mut out = 0;
        for i in 0..(if q { 128 } else { 64 }) / width {
            let x = (a >> (i * width)) & lm;
            let y = (b >> (i * width)) & lm;
            let extend = |v: u128| -> i128 {
                if unsigned {
                    v as i128
                } else {
                    ((v << (128 - width)) as i128) >> (128 - width)
                }
            };
            let t = if half_op == 0x0e20_2400 {
                extend(x) - extend(y)
            } else {
                extend(x) + extend(y) + if half_op == 0x0e20_1400 { 1 } else { 0 }
            };
            out |= (((t >> 1) as u128) & lm) << (i * width);
        }
        cpu.v[d] = out;
        return true;
    }
    // Absolute differences and accumulating absolute differences.
    let abd = insn & 0x9f20_fc00;
    let long = matches!(abd, 0x0e20_7000 | 0x0e20_5000);
    let short = matches!(abd, 0x0e20_7400 | 0x0e20_7c00);
    if long || short {
        let size = (insn >> 22) & 3;
        if size == 3 {
            return false;
        }
        let width = 8 << size;
        let dst_width = if long { width * 2 } else { width };
        let lm = (1u128 << width) - 1;
        let dm = (1u128 << dst_width) - 1;
        let unsigned = insn & (1 << 29) != 0;
        let accumulate = matches!(abd, 0x0e20_5000 | 0x0e20_7c00);
        let lanes = if long {
            64 / width
        } else {
            (if q { 128 } else { 64 }) / width
        };
        let mut out = 0;
        for i in 0..lanes {
            let sh = i * width + if long && q { 64 } else { 0 };
            let x = (a >> sh) & lm;
            let y = (b >> sh) & lm;
            let extend = |v: u128| -> i128 {
                if unsigned {
                    v as i128
                } else {
                    ((v << (128 - width)) as i128) >> (128 - width)
                }
            };
            let diff = (extend(x) - extend(y)).unsigned_abs();
            let acc = if accumulate {
                (old >> (i * dst_width)) & dm
            } else {
                0
            };
            out |= ((diff + acc) & dm) << (i * dst_width);
        }
        cpu.v[d] = out;
        return true;
    }
    // Saturating doubling multiply, multiply-high, and accumulating forms.
    let qdm = insn & 0x8f20_fc00;
    let high = qdm == 0x0e20_b400;
    let rdm = matches!(qdm, 0x0e00_8400 | 0x0e00_8c00) && insn & (1 << 29) != 0;
    let long = matches!(qdm, 0x0e20_9000 | 0x0e20_b000 | 0x0e20_d000) && insn & (1 << 29) == 0;
    if high || rdm || long {
        let size = (insn >> 22) & 3;
        if !matches!(size, 1 | 2) {
            return false;
        }
        let scalar = insn & (1 << 28) != 0;
        if scalar && !q {
            return false;
        }
        let width = 8 << size;
        let out_width = if long { 2 * width } else { width };
        let lm = (1u128 << width) - 1;
        let om = (1u128 << out_width) - 1;
        let lo = -(1i128 << (out_width - 1));
        let hi = (1i128 << (out_width - 1)) - 1;
        let lanes = if scalar {
            1
        } else if long {
            64 / width
        } else {
            (if q { 128 } else { 64 }) / width
        };
        let round = rdm || (high && insn & (1 << 29) != 0);
        let mut out = 0;
        for i in 0..lanes {
            let shift = i * width + if long && q && !scalar { 64 } else { 0 };
            let signed = |v: u128, w: u32| -> i128 { ((v << (128 - w)) as i128) >> (128 - w) };
            let x = signed((a >> shift) & lm, width);
            let y = signed((b >> shift) & lm, width);
            let prod = x * y * 2;
            let result = if long {
                let p = prod.clamp(lo, hi);
                if p != prod {
                    cpu.fpsr |= 1 << 27;
                }
                let acc = signed((old >> (i * out_width)) & om, out_width);
                if qdm == 0x0e20_9000 {
                    acc + p
                } else if qdm == 0x0e20_b000 {
                    acc - p
                } else {
                    p
                }
            } else {
                let acc = if rdm {
                    signed((old >> (i * width)) & lm, width) << width
                } else {
                    0
                };
                let product = if qdm == 0x0e00_8c00 { -prod } else { prod };
                (acc + product + if round { 1i128 << (width - 1) } else { 0 }) >> width
            };
            let clipped = result.clamp(lo, hi);
            if clipped != result {
                cpu.fpsr |= 1 << 27;
            }
            out |= ((clipped as u128) & om) << (i * out_width);
        }
        cpu.v[d] = out;
        return true;
    }
    // Narrow the high half of widened add/subtract results.
    let addhn = insn & 0x9f20_fc00;
    if matches!(addhn, 0x0e20_4000 | 0x0e20_6000) {
        let size = (insn >> 22) & 3;
        if size == 3 {
            return false;
        }
        let width = 8 << size;
        let source_mask = (1u128 << (2 * width)) - 1;
        let dest_mask = (1u128 << width) - 1;
        let rounding = insn & (1 << 29) != 0;
        let mut out = if q { old & u64::MAX as u128 } else { 0 };
        for i in 0..64 / width {
            let x = (a >> (i * 2 * width)) & source_mask;
            let y = (b >> (i * 2 * width)) & source_mask;
            let value = if addhn == 0x0e20_4000 {
                x.wrapping_add(y)
            } else {
                x.wrapping_sub(y)
            };
            let value = value.wrapping_add(if rounding { 1u128 << (width - 1) } else { 0 });
            out |= ((value >> width) & dest_mask) << (i * width + if q { 64 } else { 0 });
        }
        cpu.v[d] = out;
        return true;
    }
    // Widening integer arithmetic and multiply-accumulate.
    if insn & 0x9f20_0c00 == 0x0e20_0000 {
        let opcode = (insn >> 12) & 15;
        if matches!(opcode, 0 | 1 | 2 | 3 | 8 | 10 | 12) {
            let size = (insn >> 22) & 3;
            if size == 3 {
                return false;
            }
            let width = 8 << size;
            let lm = (1u128 << width) - 1;
            let wide_mask = (1u128 << (2 * width)) - 1;
            let unsigned = insn & (1 << 29) != 0;
            let mut out = 0;
            for i in 0..64 / width {
                let shift = i * width + if q { 64 } else { 0 };
                let extend = |v: u128| -> u128 {
                    if unsigned {
                        v
                    } else {
                        (((v << (128 - width)) as i128) >> (128 - width)) as u128
                    }
                };
                let x = if opcode == 1 || opcode == 3 {
                    (a >> (i * 2 * width)) & wide_mask
                } else {
                    extend((a >> shift) & lm)
                };
                let y = extend((b >> shift) & lm);
                let acc = (old >> (i * 2 * width)) & wide_mask;
                let value = match opcode {
                    0 | 1 => x.wrapping_add(y),
                    2 | 3 => x.wrapping_sub(y),
                    8 => acc.wrapping_add(x.wrapping_mul(y)),
                    10 => acc.wrapping_sub(x.wrapping_mul(y)),
                    _ => x.wrapping_mul(y),
                };
                out |= (value & wide_mask) << (i * 2 * width);
            }
            cpu.v[d] = out;
            return true;
        }
    }
    // Reverse elements within 16-, 32-, or 64-bit containers.
    let rev = insn & 0xbf3f_fc00;
    if matches!(rev, 0x0e20_0800 | 0x2e20_0800 | 0x0e20_1800) {
        let width = 8 << ((insn >> 22) & 3);
        let block = match rev {
            0x0e20_0800 => 64,
            0x2e20_0800 => 32,
            _ => 16,
        };
        if width >= block {
            return false;
        }
        let lm = (1u128 << width) - 1;
        let lanes = (if q { 128 } else { 64 }) / width;
        let per = block / width;
        let mut out = 0;
        for i in 0..lanes {
            let j = (i / per) * per + per - 1 - (i % per);
            out |= ((a >> (j * width)) & lm) << (i * width);
        }
        cpu.v[d] = out;
        return true;
    }
    if insn & 0xbfff_fc00 == 0x2e60_5800 {
        let mut out = 0;
        for i in 0..if q { 16 } else { 8 } {
            out |= (((a >> (i * 8)) as u8).reverse_bits() as u128) << (i * 8);
        }
        cpu.v[d] = out;
        return true;
    }
    // Pairwise integer minimum and maximum.
    let pair = insn & 0xbf20_fc00;
    if matches!(pair, 0x0e20_a400 | 0x0e20_ac00 | 0x2e20_a400 | 0x2e20_ac00) {
        let size = (insn >> 22) & 3;
        if size == 3 {
            return false;
        }
        let width = 8 << size;
        let count = (if q { 128 } else { 64 }) / width;
        let lm = (1u128 << width) - 1;
        let signed = pair & (1 << 29) == 0;
        let minimum = pair & (1 << 11) != 0;
        let mut out = 0;
        for i in 0..count {
            let source = if i < count / 2 { a } else { b };
            let j = (i % (count / 2)) * 2;
            let x = (source >> (j * width)) & lm;
            let y = (source >> ((j + 1) * width)) & lm;
            let sx = ((x << (128 - width)) as i128) >> (128 - width);
            let sy = ((y << (128 - width)) as i128) >> (128 - width);
            let less = if signed { sx < sy } else { x < y };
            out |= (if less == minimum { x } else { y }) << (i * width);
        }
        cpu.v[d] = out;
        return true;
    }
    // Pairwise widening sum, optionally accumulated.
    let pl = insn & 0x9f3f_fc00;
    if matches!(pl, 0x0e20_2800 | 0x0e20_6800) {
        let size = (insn >> 22) & 3;
        if size == 3 {
            return false;
        }
        let width = 8 << size;
        let lm = (1u128 << width) - 1;
        let wm = (1u128 << (2 * width)) - 1;
        let unsigned = insn & (1 << 29) != 0;
        let mut out = 0;
        for i in 0..(if q { 128 } else { 64 }) / (2 * width) {
            let extend = |x: u128| {
                if unsigned {
                    x
                } else {
                    (((x << (128 - width)) as i128) >> (128 - width)) as u128
                }
            };
            let x = extend((a >> (i * 2 * width)) & lm);
            let y = extend((a >> ((i * 2 + 1) * width)) & lm);
            let acc = if pl == 0x0e20_6800 {
                (old >> (i * 2 * width)) & wm
            } else {
                0
            };
            out |= (x.wrapping_add(y).wrapping_add(acc) & wm) << (i * 2 * width);
        }
        cpu.v[d] = out;
        return true;
    }
    if insn & 0xffff_fc00 == 0x5ef1_b800 {
        cpu.v[d] = (a as u64).wrapping_add((a >> 64) as u64) as u128;
        return true;
    }
    // Integer reductions over all active vector lanes.
    let reduce = insn & 0xbf3f_fc00;
    if matches!(
        reduce,
        0x0e30_3800
            | 0x2e30_3800
            | 0x0e31_b800
            | 0x0e30_a800
            | 0x2e30_a800
            | 0x0e31_a800
            | 0x2e31_a800
    ) {
        let size = (insn >> 22) & 3;
        if size == 3 || (size == 2 && !q) {
            return false;
        }
        let width = 8 << size;
        let lanes = (if q { 128 } else { 64 }) / width;
        let lane_mask = (1u128 << width) - 1;
        let signed = reduce & (1 << 29) == 0;
        let long = reduce & 0x0000_fc00 == 0x3800;
        let sum = long || reduce == 0x0e31_b800;
        let min = reduce & 0x0001_0000 != 0;
        let mut result = 0i128;
        for i in 0..lanes {
            let v = (a >> (i * width)) & lane_mask;
            let val = if signed && reduce != 0x0e31_b800 {
                ((v << (128 - width)) as i128) >> (128 - width)
            } else {
                v as i128
            };
            if sum {
                result += val;
            } else if i == 0 || (min && val < result) || (!min && val > result) {
                result = val;
            }
        }
        let dest_width = if long { width * 2 } else { width };
        cpu.v[d] = (result as u128) & ((1u128 << dest_width) - 1);
        return true;
    }
    if insn & 0xbfe0_8400 == 0x2e00_0000 {
        let bytes = if q { 16 } else { 8 };
        let index = (insn >> 11) & 15;
        if index >= bytes {
            return false;
        }
        let mut out = 0u128;
        for i in 0..bytes {
            let j = i + index;
            let value = if j < bytes {
                (a >> (8 * j)) & 255
            } else {
                (b >> (8 * (j - bytes))) & 255
            };
            out |= value << (8 * i);
        }
        cpu.v[d] = out;
        return true;
    }
    // Table lookup and table extension, with consecutive registers wrapping at V31.
    if insn & 0xbfe0_8c00 == 0x0e00_0000 {
        let count = ((insn >> 13) & 3) + 1;
        let extend = insn & (1 << 12) != 0;
        let bytes = if q { 16 } else { 8 };
        let mut out = 0u128;
        for i in 0..bytes {
            let index = ((b >> (i * 8)) & 255) as usize;
            let value = if index < count as usize * 16 {
                (cpu.v[(n + index / 16) % 32] >> ((index % 16) * 8)) & 255
            } else if extend {
                (old >> (i * 8)) & 255
            } else {
                0
            };
            out |= value << (i * 8);
        }
        cpu.v[d] = out;
        return true;
    }
    // UZP/TRN/ZIP lane permutations.
    if insn & 0xbf20_8c00 == 0x0e00_0800 {
        let opcode = (insn >> 12) & 7;
        if !matches!(opcode, 1 | 2 | 3 | 5 | 6 | 7) {
            return false;
        }
        let width = 8 << ((insn >> 22) & 3);
        let lanes = (if q { 128 } else { 64 }) / width;
        if lanes < 2 {
            return false;
        }
        let half = (opcode >> 2) as usize;
        let lm = (1u128 << width) - 1;
        let mut out = 0;
        for i in 0..lanes {
            let (src, j) = match opcode & 3 {
                1 => (
                    if i < lanes / 2 { a } else { b },
                    (i % (lanes / 2)) * 2 + half,
                ),
                2 => (if i % 2 == 0 { a } else { b }, (i / 2) * 2 + half),
                _ => (if i % 2 == 0 { a } else { b }, i / 2 + half * (lanes / 2)),
            };
            out |= ((src >> (j * width)) & lm) << (i * width);
        }
        cpu.v[d] = out;
        return true;
    }
    // Saturating left shifts by an immediate, including signed-to-unsigned.
    let sat_shift = insn & 0x8f80_fc00;
    if matches!(sat_shift, 0x0f00_7400 | 0x0f00_6400) && (insn >> 19) & 15 != 0 {
        let imm = (insn >> 16) & 127;
        if imm < 8 {
            return false;
        }
        let width = 1u32 << (31 - imm.leading_zeros());
        let scalar = insn & (1 << 28) != 0;
        if width > 64 || (scalar && !q) || (!scalar && !q && width == 64) {
            return false;
        }
        let u = insn & (1 << 29) != 0;
        if sat_shift == 0x0f00_6400 && !u {
            return false;
        }
        let sign_to_unsigned = sat_shift == 0x0f00_6400;
        let input_unsigned = u && !sign_to_unsigned;
        let shift = imm - width;
        let lm = (1u128 << width) - 1;
        let lanes = if scalar {
            1
        } else {
            (if q { 128 } else { 64 }) / width
        };
        let mut out = 0;
        for i in 0..lanes {
            let raw = (a >> (i * width)) & lm;
            let value = if input_unsigned {
                raw as i128
            } else {
                ((raw << (128 - width)) as i128) >> (128 - width)
            };
            let v = value << shift;
            let signed_output = !u;
            let lo = if signed_output {
                -(1i128 << (width - 1))
            } else {
                0
            };
            let hi = if signed_output {
                (1i128 << (width - 1)) - 1
            } else {
                lm as i128
            };
            let r = v.clamp(lo, hi);
            if r != v {
                cpu.fpsr |= 1 << 27;
            }
            out |= ((r as u128) & lm) << (i * width);
        }
        cpu.v[d] = out;
        return true;
    }
    // Per-lane shifts by an immediate, with rounding and accumulation.
    if insn & 0x8f80_0400 == 0x0f00_0400 {
        let scalar = insn & (1 << 28) != 0;
        let imm = (insn >> 16) & 127;
        let opcode = (insn >> 11) & 31;
        let unsigned = insn & (1 << 29) != 0;
        if imm >= 8 && matches!(opcode, 0 | 2 | 4 | 6 | 8 | 10) {
            let width = 1u32 << (31 - imm.leading_zeros());
            if width > 64
                || (width == 64 && !q)
                || (scalar && (!q || width != 64))
                || (opcode == 8 && !unsigned)
            {
                return false;
            }
            let bits = if scalar {
                64
            } else if q {
                128
            } else {
                64
            };
            let lm = (1u128 << width) - 1;
            let shift = if opcode == 10 {
                imm - width
            } else {
                2 * width - imm
            };
            let mut out = 0;
            for i in 0..bits / width {
                let v = (a >> (i * width)) & lm;
                let prev = (old >> (i * width)) & lm;
                let value = if opcode == 10 {
                    let shifted = (v << shift) & lm;
                    if unsigned {
                        shifted | (prev & ((1u128 << shift) - 1))
                    } else {
                        shifted
                    }
                } else if opcode == 8 {
                    let keep = lm ^ ((1u128 << (width - shift)) - 1);
                    (prev & keep) | (v >> shift)
                } else {
                    let round = if opcode & 4 != 0 {
                        1i128 << (shift - 1)
                    } else {
                        0
                    };
                    let val = if unsigned {
                        v as i128
                    } else {
                        ((v << (128 - width)) as i128) >> (128 - width)
                    };
                    let shifted = ((val + round) >> shift) as u128;
                    if opcode & 2 != 0 {
                        shifted.wrapping_add(prev)
                    } else {
                        shifted
                    }
                };
                out |= (value & lm) << (i * width);
            }
            cpu.v[d] = out;
            return true;
        }
    }
    // SSHLL/USHLL, including the high-half forms.
    // (immh == 0 is the modified-immediate group, handled below.)
    if insn & 0x9f80_fc00 == 0x0f00_a400 && (insn >> 19) & 15 != 0 {
        let imm = (insn >> 16) & 127;
        if !(8..64).contains(&imm) {
            return false;
        }
        let width = 1u32 << (31 - imm.leading_zeros());
        let shift = imm - width;
        let lm = (1u128 << width) - 1;
        let output_mask = (1u128 << (2 * width)) - 1;
        let unsigned = insn & (1 << 29) != 0;
        let mut out = 0;
        for i in 0..64 / width {
            let value = (a >> (i * width + if q { 64 } else { 0 })) & lm;
            let wide = if unsigned {
                value
            } else {
                (((value << (128 - width)) as i128) >> (128 - width)) as u128
            };
            out |= ((wide << shift) & output_mask) << (i * 2 * width);
        }
        cpu.v[d] = out;
        return true;
    }
    // Narrow lanes and saturating narrow, preserving the low half for *2.
    let narrow_lane = insn & 0xbf3f_fc00;
    if matches!(
        narrow_lane,
        0x0e21_2800 | 0x0e21_4800 | 0x2e21_4800 | 0x2e21_2800
    ) {
        let size = (insn >> 22) & 3;
        if size == 3 {
            return false;
        }
        let width = 8 << size;
        let lm = (1u128 << width) - 1;
        let wide_mask = (1u128 << (2 * width)) - 1;
        let mut out = if q { old & u64::MAX as u128 } else { 0 };
        for i in 0..64 / width {
            let raw = (a >> (i * 2 * width)) & wide_mask;
            let value = if narrow_lane == 0x0e21_2800 {
                raw & lm
            } else {
                let signed = narrow_lane != 0x2e21_4800;
                let v = if signed {
                    ((raw << (128 - 2 * width)) as i128) >> (128 - 2 * width)
                } else {
                    raw as i128
                };
                let signed_out = narrow_lane == 0x0e21_4800;
                let lo = if signed_out {
                    -(1i128 << (width - 1))
                } else {
                    0
                };
                let hi = if signed_out {
                    (1i128 << (width - 1)) - 1
                } else {
                    lm as i128
                };
                let clamped = v.clamp(lo, hi);
                if clamped != v {
                    cpu.fpsr |= 1 << 27;
                }
                (clamped as u128) & lm
            };
            out |= value << (i * width + if q { 64 } else { 0 });
        }
        cpu.v[d] = out;
        return true;
    }
    // Saturating narrowing right shifts, optionally rounded and unsigned.
    let sat_narrow = insn & 0x8f80_f400;
    if matches!(sat_narrow, 0x0f00_8400 | 0x0f00_9400) && insn & (1 << 19) != 0 {
        let unsigned = insn & (1 << 29) != 0;
        let to_unsigned = sat_narrow == 0x0f00_8400;
        if !to_unsigned || unsigned {
            let scalar = insn & (1 << 28) != 0;
            if scalar && !q {
                return false;
            }
            let imm = (insn >> 16) & 127;
            let width = 1u32 << (31 - imm.leading_zeros());
            if width > 32 {
                return false;
            }
            let shift = 2 * width - imm;
            let rounding = insn & (1 << 11) != 0;
            let lm = (1u128 << width) - 1;
            let sm = (1u128 << (2 * width)) - 1;
            let mut out = if q && !scalar {
                old & u64::MAX as u128
            } else {
                0
            };
            let lanes = if scalar { 1 } else { 64 / width };
            for i in 0..lanes {
                let raw = (a >> (i * 2 * width)) & sm;
                let signed = to_unsigned || !unsigned;
                let v = if signed {
                    ((raw << (128 - 2 * width)) as i128) >> (128 - 2 * width)
                } else {
                    raw as i128
                };
                let rounded = (v + if rounding { 1i128 << (shift - 1) } else { 0 }) >> shift;
                let lo = if signed && !to_unsigned {
                    -(1i128 << (width - 1))
                } else {
                    0
                };
                let hi = if signed && !to_unsigned {
                    (1i128 << (width - 1)) - 1
                } else {
                    lm as i128
                };
                let result = rounded.clamp(lo, hi);
                if result != rounded {
                    cpu.fpsr |= 1 << 27;
                }
                out |= ((result as u128) & lm) << (i * width + if q && !scalar { 64 } else { 0 });
            }
            cpu.v[d] = out;
            return true;
        }
    }
    // Narrowing shifts (SHRN/RSHRN and their upper-half forms).
    let narrow = insn & 0xbf80_fc00;
    if (narrow == 0x0f00_8400 || narrow == 0x0f00_8c00) && (insn >> 19) & 15 != 0 {
        let imm = (insn >> 16) & 127;
        if imm < 8 {
            return false;
        }
        let width = 1u32 << (31 - imm.leading_zeros());
        if width > 32 {
            return false;
        }
        let shift = 2 * width - imm;
        let lm = (1u128 << width) - 1;
        let mut result = if q { old & u64::MAX as u128 } else { 0 };
        for i in 0..64 / width {
            let v = (a >> (i * 2 * width)) & ((1u128 << (2 * width)) - 1);
            let round = if narrow == 0x0f00_8c00 {
                1u128 << (shift - 1)
            } else {
                0
            };
            result |= (((v + round) >> shift) & lm) << (i * width + if q { 64 } else { 0 });
        }
        cpu.v[d] = result;
        return true;
    }
    // Advanced SIMD modified immediate.
    if insn & 0x9ff8_0400 == 0x0f00_0400 {
        let cmode = (insn >> 12) & 15;
        let op = (insn >> 29) & 1;
        let imm = (((insn >> 16) & 7) << 5) | ((insn >> 5) & 31);
        let mut value = 0u64;
        if cmode < 8 {
            let w = (imm as u64) << (8 * (cmode >> 1));
            value = w | (w << 32);
        } else if cmode < 12 {
            let h = (imm as u64) << (8 * ((cmode >> 1) & 1));
            value = h * 0x0001_0001_0001_0001;
        } else if cmode < 14 {
            let shift = if cmode == 12 { 8 } else { 16 };
            let w = ((imm as u64) << shift) | ((1 << shift) - 1);
            value = w | (w << 32);
        } else if cmode == 14 {
            if op == 0 {
                value = imm as u64 * 0x0101_0101_0101_0101;
            } else {
                for i in 0..8 {
                    if imm & (1 << i) != 0 {
                        value |= 0xffu64 << (i * 8);
                    }
                }
            }
        } else {
            let b = ((imm >> 6) & 1) as u64;
            let sign = (imm >> 7) as u64;
            let exp_low = ((imm >> 4) & 3) as u64;
            let frac = (imm & 15) as u64;
            if op != 0 {
                if !q {
                    return false;
                }
                value = (sign << 63)
                    | ((!b & 1) << 62)
                    | (if b != 0 { 0xff << 54 } else { 0 })
                    | (exp_low << 52)
                    | (frac << 48);
            } else {
                let w = (sign << 31)
                    | ((!b & 1) << 30)
                    | (if b != 0 { 0x1f << 25 } else { 0 })
                    | (exp_low << 23)
                    | (frac << 19);
                value = w | (w << 32);
            }
        }
        let mut v = (value as u128) | ((value as u128) << 64);
        if cmode < 14 && op != 0 {
            v = !v;
        }
        if cmode < 12 && cmode & 1 != 0 {
            v = if op == 0 { old | v } else { old & v };
        }
        cpu.v[d] = v & mask;
        return true;
    }
    if insn & 0xffe0_fc00 == 0x5e00_0400 {
        let imm = (insn >> 16) & 31;
        if imm == 0 {
            return false;
        }
        let size = imm.trailing_zeros();
        if size > 3 {
            return false;
        }
        let width = 8 << size;
        let index = imm >> (size + 1);
        cpu.v[d] = (a >> (index * width)) & ((1u128 << width) - 1);
        return true;
    }
    // SIMD copy group: DUP, INS, UMOV and SMOV.
    if insn & 0x9fe0_8400 == 0x0e00_0400 {
        let imm5 = (insn >> 16) & 31;
        if imm5 == 0 {
            return false;
        }
        let size = imm5.trailing_zeros();
        if size > 3 {
            return false;
        }
        let width = 8u32 << size;
        let index = imm5 >> (size + 1);
        let lane_mask = (1u128 << width) - 1;
        let op = (insn >> 11) & 15;
        let is_insert = insn & (1 << 29) != 0;
        if is_insert {
            if !q || op & ((1 << size) - 1) != 0 {
                return false;
            }
            let source = op >> size;
            cpu.v[d] = (old & !(lane_mask << (index * width)))
                | (((a >> (source * width)) & lane_mask) << (index * width));
        } else {
            match op {
                0 | 1 => {
                    let value = if op == 0 {
                        (a >> (index * width)) & lane_mask
                    } else {
                        if index != 0 {
                            return false;
                        }
                        if n == 31 {
                            0
                        } else {
                            cpu.x[n] as u128 & lane_mask
                        }
                    };
                    if !q && width == 64 {
                        return false;
                    }
                    let mut result = 0;
                    for i in 0..(if q { 128 } else { 64 }) / width {
                        result |= value << (i * width);
                    }
                    cpu.v[d] = result;
                }
                3 => {
                    if !q {
                        return false;
                    }
                    let value = if n == 31 {
                        0
                    } else {
                        cpu.x[n] as u128 & lane_mask
                    };
                    cpu.v[d] = (old & !(lane_mask << (index * width))) | (value << (index * width));
                }
                5 | 7 => {
                    if op == 7 && ((q && width != 64) || (!q && width == 64)) {
                        return false;
                    }
                    if op == 5 && (width == 64 || (!q && width == 32)) {
                        return false;
                    }
                    let value = ((a >> (index * width)) & lane_mask) as u64;
                    let value = if op == 5 {
                        ((value << (64 - width)) as i64 >> (64 - width)) as u64
                    } else {
                        value
                    };
                    if d != 31 {
                        cpu.x[d] = if q { value } else { value as u32 as u64 };
                    }
                }
                _ => return false,
            }
        }
        return true;
    }
    let misc = insn & 0xbf3f_fc00;
    if matches!(
        misc,
        0x0e20_9800
            | 0x0e20_8800
            | 0x2e20_8800
            | 0x0e20_a800
            | 0x2e20_9800
            | 0x0e20_5800
            | 0x2e20_5800
            | 0x2e20_4800
            | 0x0e20_4800
            | 0x0e20_b800
            | 0x2e20_b800
    ) {
        let bits = 8 << ((insn >> 22) & 3);
        if bits == 64 && !q {
            return false;
        }
        if matches!(misc, 0x0e20_4800 | 0x2e20_4800) && bits == 64 {
            return false;
        }
        if matches!(misc, 0x0e20_5800 | 0x2e20_5800) && bits != 8 {
            return false;
        }
        let lm = (1u128 << bits) - 1;
        let mut out = 0;
        for i in 0..(if q { 128 } else { 64 }) / bits {
            let v = (a >> (i * bits)) & lm;
            let signed = ((v << (128 - bits)) as i128) >> (128 - bits);
            let r = match misc {
                0x0e20_9800 => {
                    if v == 0 {
                        lm
                    } else {
                        0
                    }
                }
                0x0e20_8800 => {
                    if signed > 0 {
                        lm
                    } else {
                        0
                    }
                }
                0x2e20_8800 => {
                    if signed >= 0 {
                        lm
                    } else {
                        0
                    }
                }
                0x0e20_a800 => {
                    if signed < 0 {
                        lm
                    } else {
                        0
                    }
                }
                0x2e20_9800 => {
                    if signed <= 0 {
                        lm
                    } else {
                        0
                    }
                }
                0x0e20_5800 => v.count_ones() as u128,
                0x2e20_5800 => !v,
                0x2e20_4800 => (v.leading_zeros() - (128 - bits)) as u128,
                0x0e20_4800 => {
                    let x = if signed < 0 { !v & lm } else { v };
                    (x.leading_zeros() - (128 - bits) - 1) as u128
                }
                0x0e20_b800 => signed.wrapping_abs() as u128,
                _ => (0u128).wrapping_sub(v),
            } & lm;
            out |= r << (i * bits);
        }
        cpu.v[d] = out;
        return true;
    }
    let key = insn & 0xbf20_fc00;
    if key == 0x0e20_1c00 || key == 0x2e20_1c00 {
        let u = (insn >> 29) & 1;
        let op = (insn >> 22) & 3;
        let result = match (u, op) {
            (0, 0) => a & b,
            (0, 1) => a & !b,
            (0, 2) => a | b,
            (0, 3) => a | !b,
            (1, 0) => a ^ b,
            (1, 1) => (old & a) | (!old & b),
            (1, 2) => (old & !b) | (a & b),
            (1, 3) => (old & b) | (a & !b),
            _ => unreachable!(),
        };
        cpu.v[d] = result & mask;
        return true;
    }
    if insn & 0xbf20_fc00 == 0x0e20_bc00 {
        let bits = 8 << ((insn >> 22) & 3);
        if bits == 64 && !q {
            return false;
        }
        let count = (if q { 128 } else { 64 }) / bits;
        let lm = (1u128 << bits) - 1;
        let mut out = 0;
        for i in 0..count {
            let source = if i < count / 2 { a } else { b };
            let j = (i % (count / 2)) * 2;
            let sum =
                ((source >> (j * bits)) & lm).wrapping_add((source >> ((j + 1) * bits)) & lm) & lm;
            out |= sum << (i * bits);
        }
        cpu.v[d] = out;
        return true;
    }
    let op = insn & 0xbf20_fc00;
    if !matches!(
        op,
        0x0e20_8400
            | 0x2e20_8400
            | 0x2e20_8c00
            | 0x0e20_8c00
            | 0x0e20_9c00
            | 0x0e20_9400
            | 0x2e20_9400
            | 0x2e20_3c00
            | 0x2e20_3400
            | 0x0e20_3c00
            | 0x0e20_3400
            | 0x2e20_6c00
            | 0x2e20_6400
            | 0x0e20_6c00
            | 0x0e20_6400
    ) {
        return false;
    }
    let bits = 8 << ((insn >> 22) & 3);
    if bits == 64 && (!q || matches!(op, 0x0e20_9c00 | 0x0e20_9400 | 0x2e20_9400)) {
        return false;
    }
    let lane_mask = (1u128 << bits) - 1;
    let mut out = 0;
    for i in 0..(if q { 128 } else { 64 }) / bits {
        let x = (a >> (i * bits)) & lane_mask;
        let y = (b >> (i * bits)) & lane_mask;
        let sx = ((x << (128 - bits)) as i128) >> (128 - bits);
        let sy = ((y << (128 - bits)) as i128) >> (128 - bits);
        let z = match op {
            0x2e20_3c00 => {
                if x >= y {
                    lane_mask
                } else {
                    0
                }
            }
            0x2e20_3400 => {
                if x > y {
                    lane_mask
                } else {
                    0
                }
            }
            0x0e20_3c00 => {
                if sx >= sy {
                    lane_mask
                } else {
                    0
                }
            }
            0x0e20_3400 => {
                if sx > sy {
                    lane_mask
                } else {
                    0
                }
            }
            0x2e20_6c00 => x.min(y),
            0x2e20_6400 => x.max(y),
            0x0e20_6c00 => {
                if sx < sy {
                    x
                } else {
                    y
                }
            }
            0x0e20_6400 => {
                if sx > sy {
                    x
                } else {
                    y
                }
            }
            0x0e20_9400 => (old >> (i * bits)).wrapping_add(x.wrapping_mul(y)),
            0x2e20_9400 => (old >> (i * bits)).wrapping_sub(x.wrapping_mul(y)),
            0x0e20_8400 => x.wrapping_add(y),
            0x2e20_8400 => x.wrapping_sub(y),
            0x0e20_8c00 => {
                if x & y != 0 {
                    lane_mask
                } else {
                    0
                }
            }
            0x2e20_8c00 => {
                if x == y {
                    lane_mask
                } else {
                    0
                }
            }
            _ => x.wrapping_mul(y),
        } & lane_mask;
        out |= z << (i * bits);
    }
    cpu.v[d] = out;
    true
}

fn sx(v: u32, bits: u32) -> i64 {
    ((v << (32 - bits)) as i32 >> (32 - bits)) as i64
}
fn base(cpu: &Cpu, r: usize) -> u64 {
    if r == 31 {
        cpu.sp
    } else {
        cpu.x[r]
    }
}
fn set_base(cpu: &mut Cpu, r: usize, v: u64) {
    if r == 31 {
        cpu.sp = v
    } else {
        cpu.x[r] = v
    }
}
fn read<A: AddressSpace + ?Sized>(
    cpu: &mut Cpu,
    mem: &A,
    addr: u64,
    size: u32,
) -> Result<u128, Exit> {
    Ok(match size {
        0 => access::read_u8(cpu, mem, addr)? as u128,
        1 => access::read_u16(cpu, mem, addr)? as u128,
        2 => access::read_u32(cpu, mem, addr)? as u128,
        3 => access::read_u64(cpu, mem, addr)? as u128,
        4 => access::read_u128(cpu, mem, addr)?,
        _ => unreachable!(),
    })
}
fn write<A: AddressSpace + ?Sized>(
    cpu: &mut Cpu,
    mem: &A,
    addr: u64,
    size: u32,
    v: u128,
) -> Result<(), Exit> {
    match size {
        0 => access::write_u8(cpu, mem, addr, v as u8),
        1 => access::write_u16(cpu, mem, addr, v as u16),
        2 => access::write_u32(cpu, mem, addr, v as u32),
        3 => access::write_u64(cpu, mem, addr, v as u64),
        4 => access::write_u128(cpu, mem, addr, v),
        _ => unreachable!(),
    }
}

pub(crate) fn exec_ldst<A: AddressSpace + ?Sized>(
    cpu: &mut Cpu,
    mem: &A,
    insn: u32,
) -> Result<(), Exit> {
    let bad = || Exit::Undefined(insn);
    let rt = (insn & 31) as usize;
    let rn = ((insn >> 5) & 31) as usize;
    // SIMD structure transfers, interleaved or a single lane.
    if insn & 0xbf00_0000 == 0x0c00_0000 || insn & 0xbf00_0000 == 0x0d00_0000 {
        let single = insn & (1 << 24) != 0;
        let load = insn & (1 << 22) != 0;
        let post = insn & (1 << 23) != 0;
        let rm = ((insn >> 16) & 31) as usize;
        if !post && rm != 0 {
            return Err(bad());
        }
        let q = (insn >> 30) & 1;
        let mut addr = base(cpu, rn);
        let start = addr;
        if !single {
            let opcode = (insn >> 12) & 15;
            let (regs, interleave) = match opcode {
                7 => (1, 1),
                10 => (2, 1),
                6 => (3, 1),
                2 => (4, 1),
                8 => (2, 2),
                4 => (3, 3),
                0 => (4, 4),
                _ => return Err(bad()),
            };
            let size = (insn >> 10) & 3;
            if size == 3 && q == 0 && interleave > 1 {
                return Err(bad());
            }
            let lanes = (8 << q) >> size;
            if load && q == 0 {
                for r in 0..regs {
                    cpu.v[(rt + r) % 32] &= u64::MAX as u128;
                }
            }
            for block in 0..regs / interleave {
                for lane in 0..lanes {
                    for r in 0..interleave {
                        let reg = (rt + block * interleave + r) % 32;
                        let shift = lane * (8 << size);
                        if load {
                            let value = read(cpu, mem, addr, size)?;
                            let mask = ((1u128 << (8 << size)) - 1) << shift;
                            cpu.v[reg] = (cpu.v[reg] & !mask) | (value << shift);
                        } else {
                            write(cpu, mem, addr, size, cpu.v[reg] >> shift)?;
                        }
                        addr = addr.wrapping_add(1 << size);
                    }
                }
            }
        } else {
            let opcode = (insn >> 13) & 7;
            let count = (1 + ((opcode & 1) << 1) + ((insn >> 21) & 1)) as usize;
            let scale = opcode >> 1;
            let s = (insn >> 12) & 1;
            let sz = (insn >> 10) & 3;
            let (size, lane, replicate) = match scale {
                0 => (0, (q << 3) | (s << 2) | sz, false),
                1 if sz & 1 == 0 => (1, (q << 2) | (s << 1) | (sz >> 1), false),
                2 if sz == 0 => (2, (q << 1) | s, false),
                2 if sz == 1 && s == 0 => (3, q, false),
                3 if load && s == 0 => (sz, 0, true),
                _ => return Err(bad()),
            };
            for r in 0..count {
                let reg = (rt + r) % 32;
                let shift = lane * (8 << size);
                if load {
                    let value = read(cpu, mem, addr, size)?;
                    if replicate {
                        let mut result = 0;
                        for i in 0..((8 << q) >> size) {
                            result |= value << (i * (8 << size));
                        }
                        cpu.v[reg] = result;
                    } else {
                        let mask = ((1u128 << (8 << size)) - 1) << shift;
                        cpu.v[reg] = (cpu.v[reg] & !mask) | (value << shift);
                    }
                } else {
                    write(cpu, mem, addr, size, cpu.v[reg] >> shift)?;
                }
                addr = addr.wrapping_add(1 << size);
            }
        }
        if post {
            let off = if rm == 31 {
                addr.wrapping_sub(start)
            } else {
                cpu.x[rm]
            };
            set_base(cpu, rn, start.wrapping_add(off));
        }
        return Ok(());
    }
    // Literal SIMD load.
    if insn & 0x3b00_0000 == 0x1800_0000 {
        let size = 2 + ((insn >> 30) & 3);
        if size > 4 {
            return Err(bad());
        }
        let addr = cpu
            .pc
            .wrapping_add((sx((insn >> 5) & 0x7ffff, 19) << 2) as u64);
        cpu.v[rt] = read(cpu, mem, addr, size)?;
        return Ok(());
    }
    // Load/store pair, including non-temporal and pre/post-indexed forms.
    if insn & 0x3a00_0000 == 0x2800_0000 {
        let size = 2 + ((insn >> 30) & 3);
        if size > 4 {
            return Err(bad());
        }
        let mode = (insn >> 23) & 3;
        let offset = sx((insn >> 15) & 127, 7) << size;
        let b = base(cpu, rn);
        let addr = if mode == 1 {
            b
        } else {
            b.wrapping_add(offset as u64)
        };
        let rt2 = ((insn >> 10) & 31) as usize;
        if insn & (1 << 22) != 0 {
            let a = read(cpu, mem, addr, size)?;
            let b = read(cpu, mem, addr.wrapping_add(1 << size), size)?;
            cpu.v[rt] = a;
            cpu.v[rt2] = b;
        } else {
            write(cpu, mem, addr, size, cpu.v[rt])?;
            write(cpu, mem, addr.wrapping_add(1 << size), size, cpu.v[rt2])?;
        }
        if mode == 1 || mode == 3 {
            set_base(cpu, rn, b.wrapping_add(offset as u64));
        }
        return Ok(());
    }
    // Single register, unsigned immediate, unscaled, indexed and register offset.
    if insn & 0x3a00_0000 == 0x3800_0000 {
        let opc = (insn >> 22) & 3;
        let sz = (insn >> 30) & 3;
        if opc >= 2 && sz != 0 {
            return Err(bad());
        }
        let size = if opc >= 2 { 4 } else { sz };
        let load = opc & 1 != 0;
        let b = base(cpu, rn);
        let (addr, wb) = if insn & (1 << 24) != 0 {
            (b.wrapping_add((((insn >> 10) & 4095) as u64) << size), None)
        } else if insn & (1 << 21) != 0 {
            if (insn >> 10) & 3 != 2 {
                return Err(bad());
            }
            let rm = ((insn >> 16) & 31) as usize;
            let x = if rm == 31 { 0 } else { cpu.x[rm] };
            let off = match (insn >> 13) & 7 {
                2 => x as u32 as u64,
                3 => x,
                6 => x as u32 as i32 as i64 as u64,
                7 => x,
                _ => return Err(bad()),
            };
            (
                b.wrapping_add(off.wrapping_shl(if insn & (1 << 12) != 0 { size } else { 0 })),
                None,
            )
        } else {
            let off = sx((insn >> 12) & 511, 9) as u64;
            match (insn >> 10) & 3 {
                0 | 2 => (b.wrapping_add(off), None),
                1 => (b, Some(b.wrapping_add(off))),
                3 => (b.wrapping_add(off), Some(b.wrapping_add(off))),
                _ => unreachable!(),
            }
        };
        if load {
            cpu.v[rt] = read(cpu, mem, addr, size)?;
        } else {
            write(cpu, mem, addr, size, cpu.v[rt])?;
        }
        if let Some(v) = wb {
            set_base(cpu, rn, v);
        }
        return Ok(());
    }
    Err(bad())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mem::Access;
    use std::cell::UnsafeCell;

    struct Memory(UnsafeCell<[u8; 4096]>);
    impl AddressSpace for Memory {
        fn translate(&self, page: u64, _access: Access) -> Option<*mut u8> {
            if page == 0 {
                Some(self.0.get().cast::<u8>())
            } else {
                None
            }
        }
        fn generation(&self) -> u64 {
            0
        }
    }
    fn memory() -> Memory {
        Memory(UnsafeCell::new([0; 4096]))
    }
    #[test]
    fn transfers_and_structure_lanes() {
        let mem = memory();
        let mut c = Cpu::new();
        c.tlb.sync(&mem);
        c.x[1] = 0x100;
        c.v[0] = 0x112233445566778899aabbccddeeff00;
        exec_ldst(&mut c, &mem, 0x3c9f0c20).unwrap(); // str q0,[x1,#-16]!
        assert_eq!(c.x[1], 0xf0);
        c.v[0] = !0;
        exec_ldst(&mut c, &mem, 0x3cc10420).unwrap(); // ldr q0,[x1],#16
        assert_eq!(c.v[0], 0x112233445566778899aabbccddeeff00);
        assert_eq!(c.x[1], 0x100);
        for i in 0..32 {
            unsafe {
                (*mem.0.get())[0x100 + i] = i as u8;
            }
        }
        exec_ldst(&mut c, &mem, 0x4c408020).unwrap(); // ld2 {v0.16b,v1.16b},[x1]
        assert_eq!(c.v[0], 0x1e1c1a18161412100e0c0a0806040200);
        assert_eq!(c.v[1], 0x1f1d1b19171513110f0d0b0907050301);
        exec_ldst(&mut c, &mem, 0x4d40c020).unwrap(); // ld1r {v0.16b},[x1]
        assert_eq!(c.v[0], 0);
    }
    #[test]
    fn basic_vector_arithmetic_and_upper_clear() {
        let mem = memory();
        let mut c = Cpu::new();
        c.v[1] = u128::MAX;
        c.v[2] = 0x01010101010101010101010101010101;
        exec_dp(&mut c, &mem, 0x4e228420).unwrap(); // add v0.16b,v1.16b,v2.16b
        assert_eq!(c.v[0], 0);
        exec_dp(&mut c, &mem, 0x0e221c20).unwrap(); // and v0.8b,v1.8b,v2.8b
        assert_eq!(c.v[0], 0x0101010101010101);
    }
    #[test]
    fn table_and_permutations() {
        let mem = memory();
        let mut c = Cpu::new();
        c.v[1] = 0x0f0e0d0c0b0a09080706050403020100;
        c.v[2] = 0x1f1e1d1c1b1a19181716151413121110;
        exec_dp(&mut c, &mem, 0x4e023820).unwrap(); // zip1 v0.16b,v1.16b,v2.16b
        assert_eq!(c.v[0], 0x17071606150514041303120211011000);
        c.v[3] = 0x201f1e1d1c1b1a191817161514131211;
        exec_dp(&mut c, &mem, 0x4e032020).unwrap(); // tbl v0.16b,{v1,v2},v3
        assert_eq!(c.v[0], 0x001f1e1d1c1b1a191817161514131211);
    }
    #[test]
    fn vector_float_immediates() {
        let mem = memory();
        let mut c = Cpu::new();
        exec_dp(&mut c, &mem, 0x4f03f600).unwrap();
        assert_eq!(c.v[0], 0x3f8000003f8000003f8000003f800000);
        exec_dp(&mut c, &mem, 0x6f04f400).unwrap();
        assert_eq!(c.v[0], 0xc000000000000000c000000000000000);
    }
    #[test]
    fn scalar_nan_and_comparison() {
        let mem = memory();
        let mut c = Cpu::new();
        c.v[1] = 0x7f800123;
        c.v[2] = 0x7fc00456;
        exec_dp(&mut c, &mem, 0x1e222820).unwrap(); // fadd s0,s1,s2
        assert_eq!(c.v[0], 0x7fc00123);
        assert_eq!(c.fpsr & 1, 1);
        c.v[0] = (-0.0f64).to_bits() as u128;
        c.v[1] = 0;
        exec_dp(&mut c, &mem, 0x1e612000).unwrap(); // fcmp d0,d1
        assert_eq!(c.nzcv, 0x60000000);
    }
}
