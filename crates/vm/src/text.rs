//! Java-compatible number formatting.

/// `Double.toString` (shortest repr, Java layout).
pub fn double_to_string(d: f64) -> String {
    if d.is_nan() {
        return "NaN".into();
    }
    if d.is_infinite() {
        return if d > 0.0 { "Infinity".into() } else { "-Infinity".into() };
    }
    if d == 0.0 {
        return if d.is_sign_negative() { "-0.0".into() } else { "0.0".into() };
    }
    let s = format!("{:e}", d.abs());
    java_layout(d < 0.0, &s, d.abs() >= 1e-3 && d.abs() < 1e7)
}

/// `Float.toString`.
pub fn float_to_string(f: f32) -> String {
    if f.is_nan() {
        return "NaN".into();
    }
    if f.is_infinite() {
        return if f > 0.0 { "Infinity".into() } else { "-Infinity".into() };
    }
    if f == 0.0 {
        return if f.is_sign_negative() { "-0.0".into() } else { "0.0".into() };
    }
    let s = format!("{:e}", f.abs());
    java_layout(f < 0.0, &s, f.abs() >= 1e-3 && f.abs() < 1e7)
}

fn java_layout(neg: bool, sci: &str, plain: bool) -> String {
    let (mant, exp) = sci.split_once('e').unwrap_or((sci, "0"));
    let exp: i32 = exp.parse().unwrap_or(0);
    let digits: String = mant.chars().filter(|c| *c != '.').collect();
    let mut out = String::with_capacity(24);
    if neg {
        out.push('-');
    }
    if plain {
        if exp >= 0 {
            let e = exp as usize;
            if digits.len() > e + 1 {
                out.push_str(&digits[..e + 1]);
                out.push('.');
                out.push_str(&digits[e + 1..]);
            } else {
                out.push_str(&digits);
                for _ in digits.len()..e + 1 {
                    out.push('0');
                }
                out.push_str(".0");
            }
        } else {
            out.push_str("0.");
            for _ in 0..(-exp - 1) {
                out.push('0');
            }
            out.push_str(&digits);
        }
    } else {
        out.push_str(&digits[..1]);
        out.push('.');
        if digits.len() > 1 {
            out.push_str(&digits[1..]);
        } else {
            out.push('0');
        }
        out.push('E');
        out.push_str(&exp.to_string());
    }
    out
}

/// Fixed-point with Java's HALF_UP rounding (as used by `%.Nf`).
pub fn fixed(v: f64, prec: usize) -> String {
    if v.is_nan() {
        return "NaN".into();
    }
    if v.is_infinite() {
        return if v > 0.0 { "Infinity".into() } else { "-Infinity".into() };
    }
    let neg = v.is_sign_negative() && v != 0.0;
    let exact = format!("{:.*}", prec + 30, v.abs());
    let (ip, fp) = exact.split_once('.').unwrap_or((&exact, ""));
    let mut digits: Vec<u8> = ip.bytes().chain(fp.bytes().take(prec)).map(|b| b - b'0').collect();
    let round_up = fp.as_bytes().get(prec).map(|&b| b >= b'5').unwrap_or(false);
    if round_up {
        let mut i = digits.len();
        loop {
            if i == 0 {
                digits.insert(0, 1);
                break;
            }
            i -= 1;
            if digits[i] == 9 {
                digits[i] = 0;
            } else {
                digits[i] += 1;
                break;
            }
        }
    }
    let int_len = digits.len() - prec;
    let mut out = String::new();
    if neg && digits.iter().any(|&d| d != 0) {
        out.push('-');
    }
    for d in &digits[..int_len] {
        out.push((b'0' + d) as char);
    }
    if prec > 0 {
        out.push('.');
        for d in &digits[int_len..] {
            out.push((b'0' + d) as char);
        }
    }
    out
}

/// `%e` formatting: d.ddddde+XX.
pub fn sci(v: f64, prec: usize, upper: bool) -> String {
    if !v.is_finite() {
        return fixed(v, 0);
    }
    let s = format!("{:.*e}", prec, v);
    let (m, e) = s.split_once('e').unwrap();
    let e: i32 = e.parse().unwrap();
    let r = format!("{}e{}{:02}", m, if e < 0 { '-' } else { '+' }, e.abs());
    if upper {
        r.to_uppercase()
    } else {
        r
    }
}

/// Inserts grouping commas into the integer part of a decimal string.
pub fn group(s: &str) -> String {
    let (sign, rest) = if let Some(r) = s.strip_prefix('-') { ("-", r) } else { ("", s) };
    let (ip, fp) = match rest.find('.') {
        Some(i) => (&rest[..i], &rest[i..]),
        None => (rest, ""),
    };
    let mut out = String::new();
    for (i, c) in ip.chars().enumerate() {
        if i > 0 && (ip.len() - i) % 3 == 0 {
            out.push(',');
        }
        out.push(c);
    }
    format!("{sign}{out}{fp}")
}

/// Java `Double.parseDouble` (accepts trailing d/f, leading/trailing whitespace).
pub fn parse_double(s: &str) -> Option<f64> {
    let t = s.trim();
    let t = t.strip_suffix(['d', 'D', 'f', 'F']).unwrap_or(t);
    match t {
        "NaN" => return Some(f64::NAN),
        "Infinity" | "+Infinity" => return Some(f64::INFINITY),
        "-Infinity" => return Some(f64::NEG_INFINITY),
        _ => {}
    }
    if t.contains(|c: char| c.is_ascii_alphabetic() && c != 'e' && c != 'E') {
        return None;
    }
    t.parse::<f64>().ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn doubles() {
        assert_eq!(double_to_string(1.0), "1.0");
        assert_eq!(double_to_string(0.1), "0.1");
        assert_eq!(double_to_string(123456.789), "123456.789");
        assert_eq!(double_to_string(1e7), "1.0E7");
        assert_eq!(double_to_string(1.5e-4), "1.5E-4");
        assert_eq!(double_to_string(0.001), "0.001");
        assert_eq!(double_to_string(-2.5), "-2.5");
        assert_eq!(double_to_string(100.0), "100.0");
        assert_eq!(float_to_string(3.14f32), "3.14");
        assert_eq!(float_to_string(1.0e10f32), "1.0E10");
    }
    #[test]
    fn fixed_half_up() {
        assert_eq!(fixed(0.125, 2), "0.13");
        assert_eq!(fixed(3.14159, 2), "3.14");
        assert_eq!(fixed(9.999, 2), "10.00");
        assert_eq!(fixed(-1.5, 0), "-2");
        assert_eq!(fixed(2.0, 3), "2.000");
        assert_eq!(group("1234567.89"), "1,234,567.89");
    }
}
