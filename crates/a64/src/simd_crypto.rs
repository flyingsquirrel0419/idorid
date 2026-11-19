//! Cryptographic SIMD instructions.
use crate::cpu::Cpu;

const fn multiply(mut a: u8, mut b: u8) -> u8 {
    let mut r = 0;
    let mut i = 0;
    while i < 8 {
        if b & 1 != 0 {
            r ^= a;
        }
        let high = a & 0x80;
        a <<= 1;
        if high != 0 {
            a ^= 0x1b;
        }
        b >>= 1;
        i += 1;
    }
    r
}
const fn sboxes() -> ([u8; 256], [u8; 256]) {
    let mut forward = [0u8; 256];
    let mut inverse = [0u8; 256];
    let mut x = 0;
    while x < 256 {
        let mut v = if x == 0 { 0 } else { 1 };
        let mut i = 0;
        while i < 254 {
            v = multiply(v, x as u8);
            i += 1;
        }
        let s =
            v ^ v.rotate_left(1) ^ v.rotate_left(2) ^ v.rotate_left(3) ^ v.rotate_left(4) ^ 0x63;
        forward[x] = s;
        inverse[s as usize] = x as u8;
        x += 1;
    }
    (forward, inverse)
}
const BOXES: ([u8; 256], [u8; 256]) = sboxes();

pub(crate) fn exec(cpu: &mut Cpu, insn: u32) -> bool {
    let d = (insn & 31) as usize;
    let n = ((insn >> 5) & 31) as usize;
    let m = ((insn >> 16) & 31) as usize;
    // SHA-3 Boolean and rotation primitives.
    let sha3 = insn & 0xffe0_8000;
    if sha3 == 0xce00_0000 || sha3 == 0xce20_0000 {
        let a = ((insn >> 10) & 31) as usize;
        cpu.v[d] = if sha3 == 0xce00_0000 {
            cpu.v[n] ^ cpu.v[m] ^ cpu.v[a]
        } else {
            cpu.v[n] ^ (cpu.v[m] & !cpu.v[a])
        };
        return true;
    }
    if insn & 0xffe0_fc00 == 0xce60_8c00 {
        let a = cpu.v[n];
        let b = cpu.v[m];
        cpu.v[d] = ((a as u64 ^ (b as u64).rotate_left(1)) as u128)
            | ((((a >> 64) as u64 ^ ((b >> 64) as u64).rotate_left(1)) as u128) << 64);
        return true;
    }
    if insn & 0xffe0_0000 == 0xce80_0000 {
        let r = (insn >> 10) & 63;
        let v = cpu.v[n] ^ cpu.v[m];
        cpu.v[d] = ((v as u64).rotate_right(r) as u128)
            | ((((v >> 64) as u64).rotate_right(r) as u128) << 64);
        return true;
    }
    let sha = insn & 0xffe0_fc00;
    let unpack = |v: u128| -> [u32; 4] {
        [
            v as u32,
            (v >> 32) as u32,
            (v >> 64) as u32,
            (v >> 96) as u32,
        ]
    };
    let pack = |v: [u32; 4]| -> u128 {
        (v[0] as u128) | ((v[1] as u128) << 32) | ((v[2] as u128) << 64) | ((v[3] as u128) << 96)
    };
    if matches!(sha, 0x5e00_0000 | 0x5e00_1000 | 0x5e00_2000) {
        let state = unpack(cpu.v[d]);
        let words = unpack(cpu.v[m]);
        let (mut a, mut b, mut c, mut dd, mut e) =
            (state[0], state[1], state[2], state[3], cpu.v[n] as u32);
        for word in words {
            let f = match sha {
                0x5e00_0000 => (b & c) | (!b & dd),
                0x5e00_1000 => b ^ c ^ dd,
                _ => (b & c) | (b & dd) | (c & dd),
            };
            let t = a
                .rotate_left(5)
                .wrapping_add(f)
                .wrapping_add(e)
                .wrapping_add(word);
            e = dd;
            dd = c;
            c = b.rotate_left(30);
            b = a;
            a = t;
        }
        cpu.v[d] = pack([a, b, c, dd]);
        return true;
    }
    if insn & 0xffff_fc00 == 0x5e28_0800 {
        cpu.v[d] = (cpu.v[n] as u32).rotate_right(2) as u128;
        return true;
    }
    if sha == 0x5e00_3000 {
        let d0 = unpack(cpu.v[d]);
        let n0 = unpack(cpu.v[n]);
        let m0 = unpack(cpu.v[m]);
        cpu.v[d] = pack([
            d0[0] ^ d0[2] ^ m0[0],
            d0[1] ^ d0[3] ^ m0[1],
            d0[2] ^ n0[0] ^ m0[2],
            d0[3] ^ n0[1] ^ m0[3],
        ]);
        return true;
    }
    if insn & 0xffff_fc00 == 0x5e28_1800 {
        let v = unpack(cpu.v[d]);
        let n = unpack(cpu.v[n]);
        let r0 = (v[0] ^ n[1]).rotate_left(1);
        cpu.v[d] = pack([
            r0,
            (v[1] ^ n[2]).rotate_left(1),
            (v[2] ^ n[3]).rotate_left(1),
            (v[3] ^ r0).rotate_left(1),
        ]);
        return true;
    }
    if matches!(sha, 0x5e00_4000 | 0x5e00_5000) {
        let d0 = unpack(cpu.v[d]);
        let n0 = unpack(cpu.v[n]);
        let w = unpack(cpu.v[m]);
        let (mut a, mut b, mut c, mut dd, mut e, mut f, mut g, mut h) = if sha == 0x5e00_4000 {
            (d0[0], d0[1], d0[2], d0[3], n0[0], n0[1], n0[2], n0[3])
        } else {
            (n0[0], n0[1], n0[2], n0[3], d0[0], d0[1], d0[2], d0[3])
        };
        for wi in w {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ (!e & g);
            let t1 = h.wrapping_add(s1).wrapping_add(ch).wrapping_add(wi);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let t2 = s0.wrapping_add(maj);
            h = g;
            g = f;
            f = e;
            e = dd.wrapping_add(t1);
            dd = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
        }
        cpu.v[d] = pack(if sha == 0x5e00_4000 {
            [a, b, c, dd]
        } else {
            [e, f, g, h]
        });
        return true;
    }
    if insn & 0xffff_fc00 == 0x5e28_2800 {
        let mut v = unpack(cpu.v[d]);
        let next = unpack(cpu.v[n]);
        let x = [v[1], v[2], v[3], next[0]];
        for i in 0..4 {
            let s = x[i].rotate_right(7) ^ x[i].rotate_right(18) ^ (x[i] >> 3);
            v[i] = v[i].wrapping_add(s);
        }
        cpu.v[d] = pack(v);
        return true;
    }
    if sha == 0x5e00_6000 {
        let mut v = unpack(cpu.v[d]);
        let n = unpack(cpu.v[n]);
        let m = unpack(cpu.v[m]);
        let mut x = [m[2], m[3], 0, 0];
        let y = [n[1], n[2], n[3], m[0]];
        for i in 0..4 {
            let s = x[i].rotate_right(17) ^ x[i].rotate_right(19) ^ (x[i] >> 10);
            v[i] = v[i].wrapping_add(s).wrapping_add(y[i]);
            if i < 2 {
                x[i + 2] = v[i];
            }
        }
        cpu.v[d] = pack(v);
        return true;
    }
    let key = insn & 0xffff_fc00;
    if matches!(key, 0x4e28_4800 | 0x4e28_5800 | 0x4e28_6800 | 0x4e28_7800) {
        let source = cpu.v[n].to_le_bytes();
        let mut out = [0u8; 16];
        if key == 0x4e28_4800 || key == 0x4e28_5800 {
            let input = (cpu.v[d] ^ cpu.v[n]).to_le_bytes();
            let decrypt = key == 0x4e28_5800;
            for col in 0..4 {
                for row in 0..4 {
                    let src_col = if decrypt {
                        (col + 4 - row) % 4
                    } else {
                        (col + row) % 4
                    };
                    let b = input[4 * src_col + row] as usize;
                    out[4 * col + row] = if decrypt { BOXES.1[b] } else { BOXES.0[b] };
                }
            }
        } else {
            let decrypt = key == 0x4e28_7800;
            for col in 0..4 {
                for row in 0..4 {
                    let coeff = if decrypt {
                        [14, 11, 13, 9]
                    } else {
                        [2, 3, 1, 1]
                    };
                    let mut v = 0;
                    for i in 0..4 {
                        v ^= multiply(source[col * 4 + (row + i) % 4], coeff[i]);
                    }
                    out[col * 4 + row] = v;
                }
            }
        }
        cpu.v[d] = u128::from_le_bytes(out);
        return true;
    }
    if insn & 0xbf20_fc00 == 0x0e20_e000 {
        let size = (insn >> 22) & 3;
        if size != 0 && size != 3 {
            return false;
        }
        let width = if size == 0 { 8 } else { 64 };
        let high = insn & (1 << 30) != 0;
        let mask = (1u128 << width) - 1;
        let mut out = 0;
        for lane in 0..64 / width {
            let shift = lane * width + if high { 64 } else { 0 };
            let a = (cpu.v[n] >> shift) & mask;
            let b = (cpu.v[m] >> shift) & mask;
            let mut product = 0;
            for i in 0..width {
                if b & (1u128 << i) != 0 {
                    product ^= a << i;
                }
            }
            out |= product << (2 * lane * width);
        }
        cpu.v[d] = out;
        return true;
    }
    false
}
