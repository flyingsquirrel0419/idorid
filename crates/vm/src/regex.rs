//! A small backtracking regex engine covering the java.util.regex subset that
//! apps typically use with String.split/matches/replaceAll.
//!
//! Supported: literals, `.`, classes `[...]`/`[^...]` with ranges, `\d \D \w
//! \W \s \S \b`, anchors `^ $`, groups `(...)`, `(?:...)`, alternation `|`,
//! quantifiers `* + ? {n} {n,} {n,m}` and their lazy `?` forms.

#[derive(Debug, Clone)]
enum Node {
    Char(u16),
    Any,
    Class(Vec<ClassItem>, bool),
    Start,
    End,
    WordB(bool),
    Group(Box<Node>, Option<usize>),
    Alt(Vec<Node>),
    Concat(Vec<Node>),
    Repeat(Box<Node>, u32, u32, bool),
}

#[derive(Debug, Clone)]
enum ClassItem {
    Range(u16, u16),
    Digit(bool),
    Word(bool),
    Space(bool),
}

#[derive(Debug, Clone)]
pub struct Regex {
    node: Node,
    pub groups: usize,
}

fn is_word(c: u16) -> bool {
    (c as u8 as u16 == c) && ((c as u8).is_ascii_alphanumeric() || c == b'_' as u16)
}
fn is_space(c: u16) -> bool {
    matches!(c, 0x20 | 0x09 | 0x0a | 0x0b | 0x0c | 0x0d)
}

impl ClassItem {
    fn matches(&self, c: u16) -> bool {
        match *self {
            ClassItem::Range(a, b) => a <= c && c <= b,
            ClassItem::Digit(n) => (c >= b'0' as u16 && c <= b'9' as u16) != n,
            ClassItem::Word(n) => is_word(c) != n,
            ClassItem::Space(n) => is_space(c) != n,
        }
    }
}

struct Parser<'a> {
    p: &'a [u16],
    i: usize,
    groups: usize,
}

impl<'a> Parser<'a> {
    fn peek(&self) -> Option<u16> {
        self.p.get(self.i).copied()
    }
    fn eat(&mut self, c: char) -> bool {
        if self.peek() == Some(c as u16) {
            self.i += 1;
            true
        } else {
            false
        }
    }
    fn alt(&mut self) -> Result<Node, String> {
        let mut alts = vec![self.concat()?];
        while self.eat('|') {
            alts.push(self.concat()?);
        }
        Ok(if alts.len() == 1 {
            alts.pop().unwrap()
        } else {
            Node::Alt(alts)
        })
    }
    fn concat(&mut self) -> Result<Node, String> {
        let mut items = Vec::new();
        while let Some(c) = self.peek() {
            if c == b'|' as u16 || c == b')' as u16 {
                break;
            }
            let atom = self.atom()?;
            items.push(self.quant(atom)?);
        }
        Ok(Node::Concat(items))
    }
    fn number(&mut self) -> Option<u32> {
        let s = self.i;
        while matches!(self.peek(), Some(c) if (b'0' as u16..=b'9' as u16).contains(&c)) {
            self.i += 1;
        }
        if s == self.i {
            return None;
        }
        String::from_utf16_lossy(&self.p[s..self.i]).parse().ok()
    }
    fn quant(&mut self, atom: Node) -> Result<Node, String> {
        let (min, max) = match self.peek().map(|c| c as u8 as char) {
            Some('*') => (0, u32::MAX),
            Some('+') => (1, u32::MAX),
            Some('?') => (0, 1),
            Some('{') => {
                let save = self.i;
                self.i += 1;
                let Some(lo) = self.number() else {
                    self.i = save;
                    return Ok(atom);
                };
                let hi = if self.eat(',') {
                    self.number().unwrap_or(u32::MAX)
                } else {
                    lo
                };
                if !self.eat('}') {
                    return Err("bad repetition".into());
                }
                self.i -= 1;
                (lo, hi)
            }
            _ => return Ok(atom),
        };
        self.i += 1;
        let lazy = self.eat('?');
        if !lazy {
            self.eat('+'); // possessive treated as greedy
        }
        Ok(Node::Repeat(Box::new(atom), min, max, lazy))
    }
    fn escape(&mut self, in_class: bool) -> Result<ClassOrChar, String> {
        let c = self.peek().ok_or("trailing backslash")?;
        self.i += 1;
        Ok(match c as u8 as char {
            'd' => ClassOrChar::Class(ClassItem::Digit(false)),
            'D' => ClassOrChar::Class(ClassItem::Digit(true)),
            'w' => ClassOrChar::Class(ClassItem::Word(false)),
            'W' => ClassOrChar::Class(ClassItem::Word(true)),
            's' => ClassOrChar::Class(ClassItem::Space(false)),
            'S' => ClassOrChar::Class(ClassItem::Space(true)),
            'b' if !in_class => ClassOrChar::WordB(false),
            'B' if !in_class => ClassOrChar::WordB(true),
            't' => ClassOrChar::Char(9),
            'n' => ClassOrChar::Char(10),
            'r' => ClassOrChar::Char(13),
            'f' => ClassOrChar::Char(12),
            '0' => ClassOrChar::Char(0),
            'u' => {
                let h = String::from_utf16_lossy(self.p.get(self.i..self.i + 4).ok_or("bad \\u")?);
                self.i += 4;
                ClassOrChar::Char(u16::from_str_radix(&h, 16).map_err(|e| e.to_string())?)
            }
            'Q' => {
                // \Q...\E literal block
                let mut lits = Vec::new();
                while self.i < self.p.len() {
                    if self.p[self.i] == b'\\' as u16
                        && self.p.get(self.i + 1) == Some(&(b'E' as u16))
                    {
                        self.i += 2;
                        break;
                    }
                    lits.push(self.p[self.i]);
                    self.i += 1;
                }
                ClassOrChar::Lit(lits)
            }
            _ => ClassOrChar::Char(c),
        })
    }
    fn atom(&mut self) -> Result<Node, String> {
        let c = self.peek().unwrap();
        self.i += 1;
        Ok(match c as u8 as char {
            '.' if c < 128 => Node::Any,
            '^' if c < 128 => Node::Start,
            '$' if c < 128 => Node::End,
            '(' if c < 128 => {
                let idx = if self.eat('?') {
                    if !self.eat(':') {
                        return Err("unsupported group".into());
                    }
                    None
                } else {
                    self.groups += 1;
                    Some(self.groups)
                };
                let inner = self.alt()?;
                if !self.eat(')') {
                    return Err("unclosed group".into());
                }
                Node::Group(Box::new(inner), idx)
            }
            '[' if c < 128 => {
                let neg = self.eat('^');
                let mut items = Vec::new();
                let mut first = true;
                loop {
                    let Some(c) = self.peek() else {
                        return Err("unclosed class".into());
                    };
                    if c == b']' as u16 && !first {
                        self.i += 1;
                        break;
                    }
                    first = false;
                    self.i += 1;
                    let lo = if c == b'\\' as u16 {
                        match self.escape(true)? {
                            ClassOrChar::Class(ci) => {
                                items.push(ci);
                                continue;
                            }
                            ClassOrChar::Char(ch) => ch,
                            _ => continue,
                        }
                    } else {
                        c
                    };
                    if self.peek() == Some(b'-' as u16)
                        && self.p.get(self.i + 1).is_some_and(|&n| n != b']' as u16)
                    {
                        self.i += 1;
                        let mut hi = self.peek().unwrap();
                        self.i += 1;
                        if hi == b'\\' as u16 {
                            if let ClassOrChar::Char(ch) = self.escape(true)? {
                                hi = ch;
                            }
                        }
                        items.push(ClassItem::Range(lo, hi));
                    } else {
                        items.push(ClassItem::Range(lo, lo));
                    }
                }
                Node::Class(items, neg)
            }
            '\\' if c < 128 => match self.escape(false)? {
                ClassOrChar::Char(ch) => Node::Char(ch),
                ClassOrChar::Class(ci) => Node::Class(vec![ci], false),
                ClassOrChar::WordB(n) => Node::WordB(n),
                ClassOrChar::Lit(l) => Node::Concat(l.into_iter().map(Node::Char).collect()),
            },
            _ => Node::Char(c),
        })
    }
}

enum ClassOrChar {
    Char(u16),
    Class(ClassItem),
    WordB(bool),
    Lit(Vec<u16>),
}

pub type Caps = Vec<Option<(usize, usize)>>;

impl Regex {
    pub fn new(pattern: &[u16]) -> Result<Regex, String> {
        let mut p = Parser {
            p: pattern,
            i: 0,
            groups: 0,
        };
        let node = p.alt()?;
        if p.i != pattern.len() {
            return Err("unmatched )".into());
        }
        Ok(Regex {
            node,
            groups: p.groups,
        })
    }

    /// If the pattern is a plain literal, returns it (fast path for split).
    pub fn literal(pattern: &[u16]) -> Option<Vec<u16>> {
        let meta = b".$|()[{^?*+\\";
        if pattern
            .iter()
            .all(|&c| c >= 128 || !meta.contains(&(c as u8)))
        {
            return Some(pattern.to_vec());
        }
        if pattern.len() == 2 && pattern[0] == b'\\' as u16 {
            let c = pattern[1];
            if c < 128 && !(c as u8).is_ascii_alphanumeric() {
                return Some(vec![c]);
            }
        }
        None
    }

    /// Anchored match starting at `start`; returns end and captures.
    pub fn match_at(&self, s: &[u16], start: usize) -> Option<(usize, Caps)> {
        let mut caps: Caps = vec![None; self.groups + 1];
        let end = m(&self.node, s, start, &mut caps, &mut |e, _| Some(e))?;
        caps[0] = Some((start, end));
        Some((end, caps))
    }

    pub fn full_match(&self, s: &[u16]) -> bool {
        let mut caps: Caps = vec![None; self.groups + 1];
        let len = s.len();
        m(&self.node, s, 0, &mut caps, &mut |e, _| {
            if e == len {
                Some(e)
            } else {
                None
            }
        })
        .is_some()
    }

    /// Finds the leftmost match at or after `from`.
    pub fn find(&self, s: &[u16], from: usize) -> Option<(usize, usize, Caps)> {
        (from..=s.len()).find_map(|i| self.match_at(s, i).map(|(e, c)| (i, e, c)))
    }
}

type Cont<'a> = dyn FnMut(usize, &mut Caps) -> Option<usize> + 'a;

fn m(n: &Node, s: &[u16], i: usize, caps: &mut Caps, k: &mut Cont<'_>) -> Option<usize> {
    match n {
        Node::Char(c) => {
            if s.get(i) == Some(c) {
                k(i + 1, caps)
            } else {
                None
            }
        }
        Node::Any => match s.get(i) {
            Some(&c) if c != 10 && c != 13 => k(i + 1, caps),
            _ => None,
        },
        Node::Class(items, neg) => match s.get(i) {
            Some(&c) if items.iter().any(|it| it.matches(c)) != *neg => k(i + 1, caps),
            _ => None,
        },
        Node::Start => {
            if i == 0 {
                k(i, caps)
            } else {
                None
            }
        }
        Node::End => {
            if i == s.len() {
                k(i, caps)
            } else {
                None
            }
        }
        Node::WordB(neg) => {
            let before = i > 0 && is_word(s[i - 1]);
            let after = i < s.len() && is_word(s[i]);
            if (before != after) != *neg {
                k(i, caps)
            } else {
                None
            }
        }
        Node::Group(inner, idx) => {
            let idx = *idx;
            m(inner, s, i, caps, &mut |e, caps: &mut Caps| {
                let saved = idx.map(|g| caps[g]);
                if let Some(g) = idx {
                    caps[g] = Some((i, e));
                }
                let r = k(e, caps);
                if r.is_none() {
                    if let (Some(g), Some(sv)) = (idx, saved) {
                        caps[g] = sv;
                    }
                }
                r
            })
        }
        Node::Alt(alts) => alts.iter().find_map(|a| m(a, s, i, caps, k)),
        Node::Concat(items) => concat(items, s, i, caps, k),
        Node::Repeat(inner, min, max, lazy) => rep(inner, *min, *max, *lazy, s, i, 0, caps, k),
    }
}

fn concat(items: &[Node], s: &[u16], i: usize, caps: &mut Caps, k: &mut Cont<'_>) -> Option<usize> {
    match items.split_first() {
        None => k(i, caps),
        Some((first, rest)) => m(first, s, i, caps, &mut |e, caps: &mut Caps| {
            concat(rest, s, e, caps, k)
        }),
    }
}

#[allow(clippy::too_many_arguments)]
fn rep(
    n: &Node,
    min: u32,
    max: u32,
    lazy: bool,
    s: &[u16],
    i: usize,
    count: u32,
    caps: &mut Caps,
    k: &mut Cont<'_>,
) -> Option<usize> {
    let try_more = |caps: &mut Caps, k: &mut Cont<'_>| -> Option<usize> {
        if count >= max {
            return None;
        }
        m(n, s, i, caps, &mut |e, caps: &mut Caps| {
            if e == i && count >= min {
                return None; // empty iteration: stop looping
            }
            rep(n, min, max, lazy, s, e, count + 1, caps, k)
        })
    };
    if count < min {
        return try_more(caps, k);
    }
    if lazy {
        if let Some(r) = k(i, caps) {
            return Some(r);
        }
        try_more(caps, k)
    } else {
        if let Some(r) = try_more(caps, k) {
            return Some(r);
        }
        k(i, caps)
    }
}

/// Java `String.split` semantics.
pub fn split(s: &[u16], pattern: &[u16], limit: i32) -> Result<Vec<Vec<u16>>, String> {
    let mut parts: Vec<Vec<u16>> = Vec::new();
    let mut last = 0usize;
    let mut pos = 0usize;
    let lit = Regex::literal(pattern);
    let re = if lit.is_none() {
        Some(Regex::new(pattern)?)
    } else {
        None
    };
    while limit <= 0 || (parts.len() as i32) < limit - 1 {
        let found = match (&lit, &re) {
            (Some(l), _) => {
                if l.is_empty() {
                    if pos < s.len() {
                        Some((pos, pos))
                    } else {
                        None
                    }
                } else {
                    s[pos..]
                        .windows(l.len())
                        .position(|w| w == &l[..])
                        .map(|p| (pos + p, pos + p + l.len()))
                }
            }
            (_, Some(r)) => r.find(s, pos).map(|(a, b, _)| (a, b)),
            _ => None,
        };
        let Some((a, b)) = found else { break };
        if b == 0 {
            // zero-width match at the beginning never produces a leading empty string
            pos = if a == b { a + 1 } else { b };
            if pos > s.len() {
                break;
            }
            continue;
        }
        if a == b && a >= s.len() {
            break;
        }
        parts.push(s[last..a].to_vec());
        last = b;
        pos = if a == b { b + 1 } else { b };
        if pos > s.len() {
            break;
        }
    }
    parts.push(s[last..].to_vec());
    if limit == 0 {
        while parts.len() > 1 && parts.last().is_some_and(|p| p.is_empty()) {
            parts.pop();
        }
        if parts.len() == 1 && parts[0].is_empty() && !s.is_empty() {
            parts.clear();
        }
    }
    Ok(parts)
}

/// Java `replaceAll`/`replaceFirst` with `$n` and `\` escapes in the replacement.
pub fn replace(s: &[u16], pattern: &[u16], repl: &[u16], all: bool) -> Result<Vec<u16>, String> {
    let re = Regex::new(pattern)?;
    let mut out = Vec::with_capacity(s.len());
    let mut pos = 0;
    let mut last = 0;
    while pos <= s.len() {
        let Some((a, b, caps)) = re.find(s, pos) else {
            break;
        };
        out.extend_from_slice(&s[last..a]);
        let mut i = 0;
        while i < repl.len() {
            let c = repl[i];
            if c == b'\\' as u16 && i + 1 < repl.len() {
                out.push(repl[i + 1]);
                i += 2;
            } else if c == b'$' as u16
                && i + 1 < repl.len()
                && (b'0' as u16..=b'9' as u16).contains(&repl[i + 1])
            {
                let g = (repl[i + 1] - b'0' as u16) as usize;
                if let Some(Some((x, y))) = caps.get(g) {
                    out.extend_from_slice(&s[*x..*y]);
                }
                i += 2;
            } else {
                out.push(c);
                i += 1;
            }
        }
        last = b;
        pos = if a == b {
            if b < s.len() {
                out.push(s[b]);
            }
            last = b + 1;
            b + 1
        } else {
            b
        };
        if !all {
            break;
        }
    }
    if last <= s.len() {
        out.extend_from_slice(&s[last.min(s.len())..]);
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn u(s: &str) -> Vec<u16> {
        s.encode_utf16().collect()
    }
    fn st(v: &[Vec<u16>]) -> Vec<String> {
        v.iter().map(|x| String::from_utf16_lossy(x)).collect()
    }
    #[test]
    fn splits() {
        assert_eq!(
            st(&split(&u("a,b,,c,,"), &u(","), 0).unwrap()),
            ["a", "b", "", "c"]
        );
        assert_eq!(
            st(&split(&u("a  b c"), &u("\\s+"), 0).unwrap()),
            ["a", "b", "c"]
        );
        assert_eq!(
            st(&split(&u("a1b22c"), &u("\\d+"), 0).unwrap()),
            ["a", "b", "c"]
        );
        assert_eq!(st(&split(&u("a|b"), &u("\\|"), 0).unwrap()), ["a", "b"]);
        assert_eq!(st(&split(&u("abc"), &u(""), 0).unwrap()), ["a", "b", "c"]);
        assert_eq!(st(&split(&u("a,b,c"), &u(","), 2).unwrap()), ["a", "b,c"]);
        assert_eq!(st(&split(&u(""), &u(","), 0).unwrap()), [""]);
    }
    #[test]
    fn matching() {
        let r = Regex::new(&u("[a-z]+@[a-z]+\\.(com|org)")).unwrap();
        assert!(r.full_match(&u("me@site.com")));
        assert!(!r.full_match(&u("me@site.net")));
        assert!(Regex::new(&u("a.*?b")).unwrap().full_match(&u("axxb")));
        assert!(Regex::new(&u("\\d{3}-\\d{4}"))
            .unwrap()
            .full_match(&u("555-1234")));
    }
    #[test]
    fn replacing() {
        let out = replace(&u("hello world"), &u("(\\w+) (\\w+)"), &u("$2 $1"), true).unwrap();
        assert_eq!(String::from_utf16_lossy(&out), "world hello");
        let out = replace(&u("a1b2c3"), &u("\\d"), &u(""), true).unwrap();
        assert_eq!(String::from_utf16_lossy(&out), "abc");
    }
}
