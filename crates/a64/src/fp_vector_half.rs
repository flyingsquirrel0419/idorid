//! Elementwise binary16 arithmetic.
use crate::cpu::Cpu;

pub(crate) fn exec(cpu: &mut Cpu, insn: u32) -> bool {
    if insn & 0x8fc0_0400 == 0x0f00_0000 {
        let op = (insn >> 12) & 15;
        let u = insn & (1 << 29) != 0;
        if (!u && matches!(op, 1 | 5 | 9)) || (u && op == 9) {
            let scalar = insn & (1 << 28) != 0;
            let q = insn & (1 << 30) != 0;
            if scalar && !q {
                return false;
            }
            let index = (((insn >> 11) & 1) << 2) | (((insn >> 21) & 1) << 1) | ((insn >> 20) & 1);
            let d = (insn & 31) as usize;
            let n = ((insn >> 5) & 31) as usize;
            let m = ((insn >> 16) & 15) as usize;
            let a = cpu.v[n];
            let old = cpu.v[d];
            let elem = ((cpu.v[m] >> (16 * index)) & 0xffff) as u16;
            let saved = [cpu.v[24], cpu.v[25], cpu.v[26], cpu.v[27]];
            let mut out = 0u128;
            for i in 0..if scalar {
                1
            } else if q {
                8
            } else {
                4
            } {
                let x = ((a >> (16 * i)) & 0xffff) as u16;
                cpu.v[25] = (old >> (16 * i)) & 0xffff;
                cpu.v[26] = x as u128;
                cpu.v[27] = elem as u128;
                let word = if op == 9 {
                    0x1ee0_0800 | (27 << 16) | (26 << 5) | 24
                } else {
                    0x1fc0_0000
                        | (27 << 16)
                        | (if op == 5 { 1 << 15 } else { 0 })
                        | (25 << 10)
                        | (26 << 5)
                        | 24
                };
                let flush = cpu.fpcr & (1 << 19) != 0;
                let zero = |v: u16| v & 0x7fff == 0 || (flush && v & 0x7c00 == 0);
                let inf = |v: u16| v & 0x7fff == 0x7c00;
                let special = u && ((zero(x) && inf(elem)) || (zero(elem) && inf(x)));
                let value = if special {
                    if flush && ((x & 0x7fff != 0 && zero(x)) || (elem & 0x7fff != 0 && zero(elem)))
                    {
                        cpu.fpsr |= 1 << 7;
                    }
                    ((x ^ elem) & 0x8000) | 0x4000
                } else {
                    if !crate::fp_half::arithmetic(cpu, word) {
                        cpu.v[24..28].copy_from_slice(&saved);
                        return false;
                    }
                    cpu.v[24] as u16
                };
                out |= (value as u128) << (i * 16);
            }
            cpu.v[24..28].copy_from_slice(&saved);
            cpu.v[d] = out;
            return true;
        }
    }
    let key = insn & 0xbfe0_fc00;
    let binary = match key {
        0x0e40_1400 => Some(2u32),
        0x0ec0_1400 => Some(3),
        0x2e40_1c00 => Some(0),
        0x0e40_1c00 => Some(0),
        0x2e40_3c00 => Some(1),
        0x0e40_3400 => Some(4),
        0x0ec0_3400 => Some(5),
        0x0e40_0400 => Some(6),
        0x0ec0_0400 => Some(7),
        _ => None,
    };
    let fused = match key {
        0x0e40_0c00 => Some(false),
        0x0ec0_0c00 => Some(true),
        _ => None,
    };
    let unary = match insn & 0xbfff_fc00 {
        0x0ef8_f800 => Some(0x1ee0_c000),
        0x2ef8_f800 => Some(0x1ee1_4000),
        0x2ef9_f800 => Some(0x1ee1_c000),
        _ => None,
    };
    if binary.is_none() && fused.is_none() && unary.is_none() {
        return false;
    }
    let q = insn & (1 << 30) != 0;
    let lanes = if q { 8 } else { 4 };
    let d = (insn & 31) as usize;
    let n = ((insn >> 5) & 31) as usize;
    let m = ((insn >> 16) & 31) as usize;
    let a = cpu.v[n];
    let b = cpu.v[m];
    let old = cpu.v[d];
    let saved = [cpu.v[24], cpu.v[25], cpu.v[26], cpu.v[27]];
    let mut out = 0u128;
    for i in 0..lanes {
        cpu.v[25] = (old >> (16 * i)) & 0xffff;
        cpu.v[26] = (a >> (16 * i)) & 0xffff;
        cpu.v[27] = (b >> (16 * i)) & 0xffff;
        let word = if let Some(op) = binary {
            0x1ee0_0800 | (27 << 16) | (op << 12) | (26 << 5) | 24
        } else if let Some(sub) = fused {
            0x1fc0_0000 | (27 << 16) | (if sub { 1 << 15 } else { 0 }) | (25 << 10) | (26 << 5) | 24
        } else {
            unary.unwrap() | (26 << 5) | 24
        };
        let x = cpu.v[26] as u16;
        let y = cpu.v[27] as u16;
        let flush = cpu.fpcr & (1 << 19) != 0;
        let zero = |v: u16| v & 0x7fff == 0 || (flush && v & 0x7c00 == 0);
        let inf = |v: u16| v & 0x7fff == 0x7c00;
        let special = key == 0x0e40_1c00 && ((zero(x) && inf(y)) || (zero(y) && inf(x)));
        let value = if special {
            if flush && ((zero(x) && x & 0x7fff != 0) || (zero(y) && y & 0x7fff != 0)) {
                cpu.fpsr |= 1 << 7;
            }
            ((x ^ y) & 0x8000) | 0x4000
        } else {
            if !crate::fp_half::arithmetic(cpu, word) {
                cpu.v[24..28].copy_from_slice(&saved);
                return false;
            }
            cpu.v[24] as u16
        };
        out |= (value as u128) << (i * 16);
    }
    cpu.v[24..28].copy_from_slice(&saved);
    cpu.v[d] = out;
    true
}
