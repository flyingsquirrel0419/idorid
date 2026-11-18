//! Text and image metrics used by the framework's layout pass.
//!
//! The host supplies real font metrics through `Vm::text_measurer`; without
//! one (CLI, tests) a deterministic approximation of the system font is used
//! so that layouts are stable across machines.

/// (text, size, flags, max_width) -> (width, height). flags: bit0 bold,
/// bit1 italic, bit2 monospace, bits 8..15 max lines (0 = unlimited).
pub type TextMeasurer = Box<dyn Fn(&str, f32, i32, f32) -> (f32, f32)>;

/// Bitmap resource id -> intrinsic size in dp.
pub type ImageSizer = Box<dyn Fn(u32) -> Option<(u32, u32)>>;

fn advance(c: char, size: f32, flags: i32) -> f32 {
    let em = if flags & 4 != 0 {
        0.6
    } else {
        match c {
            ' ' => 0.28,
            'i' | 'l' | 'j' | '.' | ',' | '\'' | '!' | '|' | ':' | ';' => 0.27,
            'f' | 't' | 'r' | 'I' | '(' | ')' | '[' | ']' | '-' => 0.36,
            'm' | 'w' | 'M' | 'W' | '@' => 0.85,
            '0'..='9' => 0.56,
            'A'..='Z' => 0.66,
            c if (c as u32) >= 0x1100 => 1.0, // CJK, emoji: full width
            _ => 0.53,
        }
    };
    em * size * if flags & 1 != 0 { 1.05 } else { 1.0 }
}

/// Approximate layout: greedy word wrap, line height 1.17 em.
pub fn approximate(text: &str, size: f32, flags: i32, max_width: f32) -> (f32, f32) {
    let line_h = (size * 1.17).ceil();
    let max_lines = ((flags >> 8) & 0xff) as usize;
    let limit = if max_width > 0.0 { max_width } else { f32::INFINITY };
    let mut widest: f32 = 0.0;
    let mut lines = 0usize;
    for para in text.split('\n') {
        let mut line_w: f32 = 0.0;
        lines += 1;
        for word in para.split_inclusive(' ') {
            let ww: f32 = word.chars().map(|c| advance(c, size, flags)).sum();
            let trimmed: f32 = word.trim_end().chars().map(|c| advance(c, size, flags)).sum();
            if line_w > 0.0 && line_w + trimmed > limit {
                widest = widest.max(line_w);
                lines += 1;
                line_w = 0.0;
            }
            if ww > limit && line_w == 0.0 {
                // A single word wider than the line breaks by character.
                let mut w = 0.0;
                for c in word.chars() {
                    let a = advance(c, size, flags);
                    if w + a > limit && w > 0.0 {
                        widest = widest.max(w);
                        lines += 1;
                        w = 0.0;
                    }
                    w += a;
                }
                line_w = w;
            } else {
                line_w += ww;
            }
        }
        // trailing spaces do not count toward the width
        let trail: f32 = para
            .chars()
            .rev()
            .take_while(|&c| c == ' ')
            .map(|c| advance(c, size, flags))
            .sum();
        widest = widest.max(line_w - trail);
    }
    if max_lines > 0 {
        lines = lines.min(max_lines);
    }
    (widest.min(limit).ceil(), lines.max(1) as f32 * line_h)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn wraps_and_limits_lines() {
        let (w1, h1) = approximate("Hello world", 14.0, 0, 0.0);
        assert!(w1 > 50.0 && w1 < 100.0);
        assert_eq!(h1, 17.0);
        let (w2, h2) = approximate("Hello world", 14.0, 0, 45.0);
        assert!(w2 <= 45.0);
        assert_eq!(h2, 34.0);
        let (_, h3) = approximate("a b c d e f g h", 14.0, 1 << 8, 10.0);
        assert_eq!(h3, 17.0);
        assert_eq!(approximate("", 14.0, 0, 0.0), (0.0, 17.0));
    }
}
