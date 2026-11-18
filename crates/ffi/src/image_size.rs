//! Read bitmap dimensions without decoding the image.

fn le24(b: &[u8]) -> Option<u32> {
    Some(u32::from(*b.first()?) | (u32::from(*b.get(1)?) << 8) | (u32::from(*b.get(2)?) << 16))
}

fn pixels(b: &[u8]) -> Option<(u32, u32)> {
    if b.starts_with(b"\x89PNG\r\n\x1a\n") && b.get(12..16)? == b"IHDR" {
        return Some((
            u32::from_be_bytes(b.get(16..20)?.try_into().ok()?),
            u32::from_be_bytes(b.get(20..24)?.try_into().ok()?),
        ));
    }
    if b.starts_with(b"RIFF") && b.get(8..12)? == b"WEBP" {
        return match b.get(12..16)? {
            b"VP8X" => Some((le24(b.get(24..27)?)? + 1, le24(b.get(27..30)?)? + 1)),
            b"VP8L" if *b.get(20)? == 0x2f => {
                let bits = u32::from_le_bytes(b.get(21..25)?.try_into().ok()?);
                Some(((bits & 0x3fff) + 1, ((bits >> 14) & 0x3fff) + 1))
            }
            b"VP8 " if b.get(23..26)? == b"\x9d\x01\x2a" && b.get(20)? & 1 == 0 => Some((
                u16::from_le_bytes(b.get(26..28)?.try_into().ok()?) as u32 & 0x3fff,
                u16::from_le_bytes(b.get(28..30)?.try_into().ok()?) as u32 & 0x3fff,
            )),
            _ => None,
        };
    }
    if b.starts_with(b"\xff\xd8") {
        let mut pos = 2usize;
        while pos < b.len() {
            if b[pos] != 0xff {
                return None;
            }
            while b.get(pos) == Some(&0xff) {
                pos += 1;
            }
            let marker = *b.get(pos)?;
            pos += 1;
            if marker == 0xd9 || marker == 0xda {
                return None;
            }
            if marker == 0x01 || (0xd0..=0xd8).contains(&marker) {
                continue;
            }
            let len = u16::from_be_bytes(b.get(pos..pos + 2)?.try_into().ok()?) as usize;
            if len < 2 || pos.checked_add(len)? > b.len() {
                return None;
            }
            if matches!(marker, 0xc0..=0xc3 | 0xc5..=0xc7 | 0xc9..=0xcb | 0xcd..=0xcf) {
                if len < 7 {
                    return None;
                }
                let h = u16::from_be_bytes(b.get(pos + 3..pos + 5)?.try_into().ok()?) as u32;
                let w = u16::from_be_bytes(b.get(pos + 5..pos + 7)?.try_into().ok()?) as u32;
                return Some((w, h));
            }
            pos += len;
        }
    }
    None
}

pub(crate) fn size_dp(path: &str, bytes: &[u8]) -> Option<(u32, u32)> {
    let (w, h) = pixels(bytes)?;
    if w == 0 || h == 0 {
        return None;
    }
    let dir = path.split('/').nth(1)?;
    let density = dir
        .split('-')
        .skip(1)
        .find_map(|q| match q {
            "mdpi" | "nodpi" => Some(160u64),
            "hdpi" => Some(240),
            "xhdpi" => Some(320),
            "xxhdpi" => Some(480),
            "xxxhdpi" => Some(640),
            _ => None,
        })
        .unwrap_or(160);
    let scale =
        |px: u32| ((u64::from(px) * 160 + density / 2) / density).min(u64::from(u32::MAX)) as u32;
    Some((scale(w), scale(h)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bitmap_headers_and_density() {
        let png = b"\x89PNG\r\n\x1a\n\0\0\0\rIHDR\0\0\0\x60\0\0\0\x30";
        assert_eq!(
            size_dp("res/drawable-en-xhdpi/icon.png", png),
            Some((48, 24))
        );
        assert_eq!(size_dp("res/drawable-nodpi/icon.png", png), Some((96, 48)));
        let jpeg = b"\xff\xd8\xff\xe0\x00\x04xx\xff\xc2\x00\x07\x08\x00\x30\x00\x60";
        assert_eq!(size_dp("res/drawable-hdpi/icon.jpg", jpeg), Some((64, 32)));
        let vp8x = b"RIFF\0\0\0\0WEBPVP8X\0\0\0\0\0\0\0\0\x5f\0\0\x2f\0\0";
        assert_eq!(pixels(vp8x), Some((96, 48)));
        let vp8l = b"RIFF\0\0\0\0WEBPVP8L\0\0\0\0\x2f\x5f\xc0\x0b\0";
        assert_eq!(pixels(vp8l), Some((96, 48)));
        let vp8 = b"RIFF\0\0\0\0WEBPVP8 \0\0\0\0\0\0\0\x9d\x01\x2a\x60\0\x30\0";
        assert_eq!(pixels(vp8), Some((96, 48)));
        for data in [
            png.as_slice(),
            jpeg.as_slice(),
            vp8x.as_slice(),
            vp8l.as_slice(),
            vp8.as_slice(),
        ] {
            for end in 0..data.len() {
                assert_eq!(pixels(&data[..end]), None);
            }
        }
        assert_eq!(size_dp("res/drawable/icon.xml", b"<vector/>"), None);
    }
}
