use crate::{DexError, Result};

#[inline]
pub fn read_uleb128(d: &[u8], p: &mut usize) -> Result<u32> {
    let mut result: u32 = 0;
    for i in 0..5 {
        let b = *d.get(*p).ok_or(DexError::OutOfBounds("uleb128", *p))?;
        *p += 1;
        result |= ((b & 0x7f) as u32) << (i * 7);
        if b & 0x80 == 0 {
            return Ok(result);
        }
    }
    Err(DexError::Malformed("uleb128 too long"))
}

#[inline]
pub fn read_sleb128(d: &[u8], p: &mut usize) -> Result<i32> {
    let mut result: i32 = 0;
    let mut shift = 0;
    loop {
        let b = *d.get(*p).ok_or(DexError::OutOfBounds("sleb128", *p))?;
        *p += 1;
        result |= ((b & 0x7f) as i32) << shift;
        shift += 7;
        if b & 0x80 == 0 {
            if shift < 32 && b & 0x40 != 0 {
                result |= -1i32 << shift;
            }
            return Ok(result);
        }
        if shift >= 35 {
            return Err(DexError::Malformed("sleb128 too long"));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn leb() {
        let mut p = 0;
        assert_eq!(read_uleb128(&[0xe5, 0x8e, 0x26], &mut p).unwrap(), 624485);
        p = 0;
        assert_eq!(read_sleb128(&[0x7f], &mut p).unwrap(), -1);
        p = 0;
        assert_eq!(read_sleb128(&[0x80, 0x7f], &mut p).unwrap(), -128);
    }
}
