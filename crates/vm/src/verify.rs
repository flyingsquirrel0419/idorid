//! Structural bytecode check run once per method before first execution.
//!
//! It guarantees that every register operand is `< registers_size` and that
//! every instruction fits inside the code array. The interpreter relies on this
//! to skip bounds checks on register access (the register window always has
//! one slot of slack so a wide access at the last register stays in bounds).

/// Instruction length (in code units) and register operands for an opcode.
/// Returns Err on malformed code.
pub fn verify(insns: &[u16], regs: u16) -> Result<(), String> {
    let n = insns.len();
    let regs = regs as usize;
    let mut pc = 0usize;
    while pc < n {
        let ins = insns[pc];
        let op = ins & 0xff;
        let aa = (ins >> 8) as usize;
        let a = ((ins >> 8) & 0xf) as usize;
        let b = (ins >> 12) as usize;
        let unit = |k: usize| -> Result<usize, String> {
            insns.get(pc + k).map(|&u| u as usize).ok_or_else(|| format!("truncated instruction at {pc}"))
        };
        let check = |r: usize| -> Result<(), String> {
            if r < regs {
                Ok(())
            } else {
                Err(format!("register v{r} out of range ({regs}) at {pc}"))
            }
        };
        let len = match op {
            0x00 => match aa {
                // payload pseudo-instructions
                0x01 => 4 + unit(1)? * 2,
                0x02 => 2 + unit(1)? * 4,
                0x03 => {
                    let width = unit(1)?;
                    let size = unit(2)? | (unit(3)? << 16);
                    4 + (size * width).div_ceil(2)
                }
                _ => 1,
            },
            // 12x: vA, vB
            0x01 | 0x04 | 0x07 | 0x21 | 0x7b..=0x8f | 0xb0..=0xcf => {
                check(a)?;
                check(b)?;
                1
            }
            // 11n
            0x12 => {
                check(a)?;
                1
            }
            // 11x
            0x0a..=0x11 | 0x1d | 0x1e | 0x27 => {
                if op != 0x0e {
                    check(aa)?;
                }
                1
            }
            0x28 => 1,
            0x29 => 2,
            0x2a => 3,
            // 22x
            0x02 | 0x05 | 0x08 => {
                check(aa)?;
                check(unit(1)?)?;
                2
            }
            // 32x
            0x03 | 0x06 | 0x09 => {
                check(unit(1)?)?;
                check(unit(2)?)?;
                3
            }
            // 21t, 21s, 21h, 21c
            0x13 | 0x15 | 0x16 | 0x19 | 0x1a | 0x1c | 0x1f | 0x22 | 0x38..=0x3d | 0x60..=0x6d | 0xfe | 0xff => {
                check(aa)?;
                2
            }
            // 31i, 31t, 31c
            0x14 | 0x17 | 0x1b | 0x26 | 0x2b | 0x2c => {
                check(aa)?;
                3
            }
            0x18 => {
                check(aa)?;
                5
            }
            // 22c, 22t, 22s
            0x20 | 0x23 | 0x32..=0x37 | 0x52..=0x5f | 0xd0..=0xd7 => {
                check(a)?;
                check(b)?;
                2
            }
            // 23x
            0x2d..=0x31 | 0x44..=0x51 | 0x90..=0xaf => {
                check(aa)?;
                let w = unit(1)?;
                check(w & 0xff)?;
                check(w >> 8)?;
                2
            }
            // 22b
            0xd8..=0xe2 => {
                check(aa)?;
                check(unit(1)? & 0xff)?;
                2
            }
            // 35c
            0x24 | 0x6e..=0x72 | 0xfc => {
                if b > 5 {
                    return Err(format!("bad argument count at {pc}"));
                }
                let w = unit(2)?;
                let list = [w & 0xf, (w >> 4) & 0xf, (w >> 8) & 0xf, w >> 12, a];
                for &r in &list[..b] {
                    check(r)?;
                }
                3
            }
            // 3rc
            0x25 | 0x74..=0x78 | 0xfd => {
                let first = unit(2)?;
                if aa > 0 {
                    check(first + aa - 1)?;
                }
                3
            }
            0xfa => 4,
            0xfb => 4,
            _ => 1, // unused opcodes fault at execution time
        };
        if pc + len > n {
            return Err(format!("instruction at {pc} runs past the end"));
        }
        pc += len;
    }
    Ok(())
}
