use crate::ApkError;
pub(crate) fn err() -> ApkError {
    ApkError("invalid or truncated Android binary resource".into())
}
pub(crate) fn u8at(d: &[u8], p: usize) -> Result<u8, ApkError> {
    d.get(p).copied().ok_or_else(err)
}
pub(crate) fn u16at(d: &[u8], p: usize) -> Result<u16, ApkError> {
    let b = d
        .get(p..p.checked_add(2).ok_or_else(err)?)
        .ok_or_else(err)?;
    Ok(u16::from_le_bytes([b[0], b[1]]))
}
pub(crate) fn u32at(d: &[u8], p: usize) -> Result<u32, ApkError> {
    let b = d
        .get(p..p.checked_add(4).ok_or_else(err)?)
        .ok_or_else(err)?;
    Ok(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
}
pub(crate) fn chunk(d: &[u8], p: usize) -> Result<(u16, usize, usize), ApkError> {
    let t = u16at(d, p)?;
    let h = u16at(d, p + 2)? as usize;
    let s = u32at(d, p + 4)? as usize;
    if h < 8 || s < h || p.checked_add(s).filter(|e| *e <= d.len()).is_none() {
        return Err(err());
    }
    Ok((t, h, s))
}
pub(crate) fn pool(d: &[u8]) -> Result<Vec<String>, ApkError> {
    let (t, h, s) = chunk(d, 0)?;
    if t != 1 || h < 28 {
        return Err(err());
    }
    let count = u32at(d, 8)? as usize;
    let flags = u32at(d, 16)?;
    let start = u32at(d, 20)? as usize;
    if count > s.saturating_sub(h) / 4 {
        return Err(err());
    }
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let mut p = start
            .checked_add(u32at(d, h + i * 4)? as usize)
            .ok_or_else(err)?;
        if p >= s {
            return Err(err());
        }
        if flags & 0x100 != 0 {
            fn len(d: &[u8], p: &mut usize) -> Result<usize, ApkError> {
                let a = u8at(d, *p)?;
                *p += 1;
                if a & 128 != 0 {
                    let b = u8at(d, *p)?;
                    *p += 1;
                    Ok((((a & 127) as usize) << 8) | b as usize)
                } else {
                    Ok(a as usize)
                }
            }
            let _ = len(d, &mut p)?;
            let n = len(d, &mut p)?;
            let end = p.checked_add(n).ok_or_else(err)?;
            let bytes = d.get(p..end).filter(|_| end < s).ok_or_else(err)?;
            out.push(std::str::from_utf8(bytes).map_err(|_| err())?.to_owned());
        } else {
            fn len(d: &[u8], p: &mut usize) -> Result<usize, ApkError> {
                let a = u16at(d, *p)?;
                *p += 2;
                if a & 0x8000 != 0 {
                    let b = u16at(d, *p)?;
                    *p += 2;
                    Ok((((a & 0x7fff) as usize) << 16) | b as usize)
                } else {
                    Ok(a as usize)
                }
            }
            let n = len(d, &mut p)?;
            let end = p
                .checked_add(n.checked_mul(2).ok_or_else(err)?)
                .ok_or_else(err)?;
            if end.checked_add(2).filter(|e| *e <= s).is_none() {
                return Err(err());
            }
            let mut chars = Vec::with_capacity(n);
            for j in 0..n {
                chars.push(u16at(d, p + j * 2)?)
            }
            out.push(String::from_utf16(&chars).map_err(|_| err())?);
        }
    }
    Ok(out)
}
pub(crate) fn str_at(pool: &[String], i: u32) -> Option<&str> {
    if i == u32::MAX {
        None
    } else {
        pool.get(i as usize).map(String::as_str)
    }
}
