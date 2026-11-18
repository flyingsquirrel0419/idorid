use crate::{
    binary::{chunk, err, pool, str_at, u16at, u32at, u8at},
    ApkError,
};
use std::collections::{HashMap, HashSet};
use std::sync::OnceLock;

pub const DEVICE_SDK: u16 = 30;

#[derive(Debug, Clone, PartialEq)]
pub enum ResValue {
    Null,
    String(String),
    Reference(u32),
    Attribute(u32),
    Int(i32),
    Hex(u32),
    Bool(bool),
    Dimension { value: f32, unit: DimensionUnit },
    Fraction { value: f32, unit: FractionUnit },
    Color(u32),
    Float(f32),
    Unknown { data_type: u8, data: u32 },
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DimensionUnit {
    Px,
    Dp,
    Sp,
    Pt,
    In,
    Mm,
    Unknown(u8),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FractionUnit {
    Parent,
    Self_,
    Unknown(u8),
}
pub(crate) fn value(kind: u8, data: u32, strings: &[String]) -> Result<ResValue, ApkError> {
    let complex = || {
        let mantissa = (data as i32) >> 8;
        let radix = ((data >> 4) & 3) as usize;
        mantissa as f32 * [1.0, 1.0 / 128.0, 1.0 / 32768.0, 1.0 / 8388608.0][radix]
    };
    Ok(match kind {
        0 => ResValue::Null,
        1 => ResValue::Reference(data),
        2 => ResValue::Attribute(data),
        3 => ResValue::String(str_at(strings, data).ok_or_else(err)?.to_owned()),
        4 => ResValue::Float(f32::from_bits(data)),
        5 => ResValue::Dimension {
            value: complex(),
            unit: match data & 15 {
                0 => DimensionUnit::Px,
                1 => DimensionUnit::Dp,
                2 => DimensionUnit::Sp,
                3 => DimensionUnit::Pt,
                4 => DimensionUnit::In,
                5 => DimensionUnit::Mm,
                n => DimensionUnit::Unknown(n as u8),
            },
        },
        6 => ResValue::Fraction {
            value: complex(),
            unit: match data & 15 {
                0 => FractionUnit::Self_,
                1 => FractionUnit::Parent,
                n => FractionUnit::Unknown(n as u8),
            },
        },
        0x10 => ResValue::Int(data as i32),
        0x11 => ResValue::Hex(data),
        0x12 => ResValue::Bool(data != 0),
        0x1c..=0x1f => ResValue::Color(data),
        _ => ResValue::Unknown {
            data_type: kind,
            data,
        },
    })
}
#[derive(Debug, Clone, PartialEq)]
pub struct Bag {
    pub parent: u32,
    pub items: Vec<(u32, ResValue)>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
struct Config {
    language: u8,
    country: u8,
    direction: u8,
    sw: u16,
    width: u16,
    height: u16,
    orientation: u8,
    night: u8,
    density: u16,
    sdk: u16,
}
impl Config {
    fn parse(c: &[u8]) -> Option<Self> {
        let byte = |i: usize| c.get(i).copied().unwrap_or(0);
        let word = |i: usize| u16::from_le_bytes([byte(i), byte(i + 1)]);
        let lang = [byte(8), byte(9)];
        let country = [byte(10), byte(11)];
        let orientation = byte(12);
        let density = word(14);
        let sdk = word(24);
        let direction = byte(28) & 0xc0;
        let night = byte(29) & 0x30;
        let sw = word(30);
        let width = word(32);
        let height = word(34);
        if (lang != [0, 0] && lang != *b"en")
            || orientation > 1
            || sdk > DEVICE_SDK
            || direction & 0x80 != 0
            || night & 0x20 != 0
            || sw > 390
            || width > 390
            || height > 844
        {
            return None;
        }
        // Unspecified country is preferred: the simulated device has no country.
        let density_score = if density == 0 {
            1
        } else if density == 0xffff {
            2
        } else if density == 480 {
            u16::MAX
        } else {
            let distance = density.abs_diff(480).min(16000);
            32000 - distance * 2 + u16::from(density > 480)
        };
        Some(Self {
            language: u8::from(lang != [0, 0]),
            country: u8::from(country == [0, 0]),
            direction: u8::from(direction != 0),
            sw,
            width,
            height,
            orientation: u8::from(orientation != 0),
            night: u8::from(night != 0),
            density: density_score,
            sdk,
        })
    }
}
#[derive(Debug, Clone)]
struct Entry {
    name: String,
    value: Option<ResValue>,
    bag: Option<Bag>,
    config: Config,
}
#[derive(Debug, Clone, Default)]
pub struct Resources {
    entries: HashMap<u32, Entry>,
    names: HashMap<String, u32>,
}
fn framework() -> &'static Resources {
    static FRAMEWORK: OnceLock<Resources> = OnceLock::new();
    FRAMEWORK.get_or_init(|| Resources::decode_framework(include_bytes!("../framework-res.bin")))
}
impl Resources {
    pub fn parse(data: &[u8]) -> Result<Self, ApkError> {
        let (t, h, s) = chunk(data, 0)?;
        if t != 2 || h < 12 {
            return Err(err());
        }
        let data = &data[..s];
        let mut p = h;
        let mut global = Vec::new();
        let mut result = Self::default();
        while p < s {
            let (t, _, size) = chunk(data, p)?;
            let c = &data[p..p + size];
            match t {
                1 => global = pool(c)?,
                0x200 => result.package(c, &global)?,
                _ => {}
            }
            p += size;
        }
        Ok(result)
    }
    fn package(&mut self, d: &[u8], global: &[String]) -> Result<(), ApkError> {
        let (_, h, s) = chunk(d, 0)?;
        if h < 284 {
            return Err(err());
        }
        let pkg = u32at(d, 8)?;
        if pkg > 255 {
            return Err(err());
        }
        let types_off = u32at(d, 268)? as usize;
        let keys_off = u32at(d, 276)? as usize;
        let type_offset = if h >= 288 { u32at(d, 284)? } else { 0 };
        let types = pool(d.get(types_off..).ok_or_else(err)?)?;
        let keys = pool(d.get(keys_off..).ok_or_else(err)?)?;
        let mut p = h;
        while p < s {
            let (kind, header, size) = chunk(d, p)?;
            let c = &d[p..p + size];
            if kind == 0x201 {
                if header < 24 {
                    return Err(err());
                }
                let ty = u8at(c, 8)? as u32;
                let flags = u8at(c, 9)?;
                let count = u32at(c, 12)? as usize;
                let start = u32at(c, 16)? as usize;
                let config_size = u32at(c, 20)? as usize;
                if config_size < 4 || 20 + config_size > header {
                    return Err(err());
                }
                let Some(config) = Config::parse(&c[20..20 + config_size]) else {
                    p += size;
                    continue;
                };
                let type_name =
                    str_at(&types, ty.checked_sub(1).ok_or_else(err)?).ok_or_else(err)?;
                let width = if flags & 2 != 0 && flags & 1 == 0 {
                    2
                } else {
                    4
                };
                if count > size.saturating_sub(header) / width {
                    return Err(err());
                }
                for i in 0..count {
                    let (index, offset) = if flags & 1 != 0 {
                        (
                            u16at(c, header + i * 4)? as u32,
                            u16at(c, header + i * 4 + 2)? as u32 * 4,
                        )
                    } else if flags & 2 != 0 {
                        let off = u16at(c, header + i * 2)?;
                        if off == u16::MAX {
                            continue;
                        }
                        (i as u32, off as u32 * 4)
                    } else {
                        let off = u32at(c, header + i * 4)?;
                        if off == u32::MAX {
                            continue;
                        }
                        (i as u32, off)
                    };
                    if index > 65535 {
                        return Err(err());
                    }
                    let ep = start.checked_add(offset as usize).ok_or_else(err)?;
                    let es = u16at(c, ep)? as usize;
                    let ef = u16at(c, ep + 2)?;
                    let key = if ef & 8 != 0 {
                        es as u32
                    } else {
                        u32at(c, ep + 4)?
                    };
                    if ef & 8 == 0 && es < 8 {
                        return Err(err());
                    }
                    let vp = ep.checked_add(es).ok_or_else(err)?;
                    let (v, bag) = if ef & 8 != 0 {
                        (
                            Some(value((ef >> 8) as u8, u32at(c, ep + 4)?, global)?),
                            None,
                        )
                    } else if ef & 1 != 0 {
                        if es < 16 {
                            return Err(err());
                        }
                        let parent = u32at(c, ep + 8)?;
                        let count = u32at(c, ep + 12)? as usize;
                        if count > c.len().saturating_sub(vp) / 12 {
                            return Err(err());
                        }
                        let mut items = Vec::with_capacity(count);
                        for j in 0..count {
                            let pos = vp + j * 12;
                            if u16at(c, pos + 4)? < 8 {
                                return Err(err());
                            }
                            items.push((
                                u32at(c, pos)?,
                                value(u8at(c, pos + 7)?, u32at(c, pos + 8)?, global)?,
                            ));
                        }
                        (None, Some(Bag { parent, items }))
                    } else {
                        if u16at(c, vp)? < 8 {
                            return Err(err());
                        }
                        (
                            Some(value(u8at(c, vp + 3)?, u32at(c, vp + 4)?, global)?),
                            None,
                        )
                    };
                    let id = (pkg << 24) | ((ty + type_offset) << 16) | index;
                    let name = format!("{}/{}", type_name, str_at(&keys, key).ok_or_else(err)?);
                    let entry = Entry {
                        name: name.clone(),
                        value: v,
                        bag,
                        config: config.clone(),
                    };
                    if self.entries.get(&id).is_none_or(|old| old.config < config) {
                        self.names.insert(name, id);
                        self.entries.insert(id, entry);
                    }
                }
            }
            p += size;
        }
        Ok(())
    }
    fn entry(&self, id: u32) -> Option<&Entry> {
        if id >> 24 == 1 {
            framework().entries.get(&id)
        } else {
            self.entries.get(&id)
        }
    }
    pub fn bag(&self, id: u32) -> Option<&Bag> {
        self.entry(id)?.bag.as_ref()
    }
    pub fn style(&self, id: u32) -> Vec<(u32, ResValue)> {
        let mut chain = Vec::new();
        let mut seen = HashSet::new();
        let mut current = id;
        while current != 0 && seen.insert(current) {
            let Some(bag) = self.bag(current) else {
                break;
            };
            chain.push(bag);
            current = bag.parent;
        }
        let mut out = Vec::new();
        let mut indices = HashMap::new();
        for bag in chain.into_iter().rev() {
            for (key, val) in &bag.items {
                if let Some(&idx) = indices.get(key) {
                    out[idx] = (*key, val.clone());
                } else {
                    indices.insert(*key, out.len());
                    out.push((*key, val.clone()));
                }
            }
        }
        out
    }
    pub fn string_array(&self, id: u32) -> Option<Vec<String>> {
        self.bag(id)?
            .items
            .iter()
            .map(|(_, v)| match v {
                ResValue::String(s) => Some(s.clone()),
                ResValue::Reference(id) => self.string(*id),
                ResValue::Int(i) => Some(i.to_string()),
                _ => None,
            })
            .collect()
    }
    pub fn get(&self, id: u32) -> Option<ResValue> {
        self.entry(id)?.value.clone()
    }
    pub fn resolve(&self, id: u32) -> Option<ResValue> {
        let mut seen = HashSet::new();
        let mut current = id;
        loop {
            if !seen.insert(current) {
                return None;
            }
            match self.get(current)? {
                ResValue::Reference(next) => current = next,
                v => return Some(v),
            }
        }
    }
    pub fn string(&self, id: u32) -> Option<String> {
        match self.resolve(id)? {
            ResValue::String(s) => Some(s),
            _ => None,
        }
    }
    pub fn file_path(&self, id: u32) -> Option<String> {
        self.string(id).filter(|s| s.starts_with("res/"))
    }
    pub fn name(&self, id: u32) -> Option<&str> {
        self.entry(id).map(|e| e.name.as_str())
    }
    pub fn id_by_name(&self, kind: &str, name: &str) -> Option<u32> {
        let (kind, fw) = if let Some(k) = kind.strip_prefix("android:") {
            (k, true)
        } else {
            (kind, false)
        };
        let (name, fw) = if let Some(n) = name.strip_prefix("android:") {
            (n, true)
        } else {
            (name, fw)
        };
        let target = format!("{kind}/{name}");
        if fw {
            framework().names.get(&target).copied()
        } else {
            self.names.get(&target).copied()
        }
    }
}

impl Resources {
    /// Generate the compact framework snapshot from a platform resource table.
    pub fn framework_snapshot(&self) -> Vec<u8> {
        use std::io::Write;
        fn number(out: &mut Vec<u8>, n: u32) {
            out.extend(n.to_le_bytes());
        }
        fn string(out: &mut Vec<u8>, s: &str) {
            number(out, s.len() as u32);
            out.extend(s.as_bytes());
        }
        fn val(out: &mut Vec<u8>, v: &ResValue) {
            let (tag, n) = match v {
                ResValue::Null => (0, 0),
                ResValue::Reference(x) => (1, *x),
                ResValue::Attribute(x) => (2, *x),
                ResValue::String(s) => {
                    out.push(3);
                    string(out, s);
                    return;
                }
                ResValue::Float(x) => (4, x.to_bits()),
                ResValue::Int(x) => (5, *x as u32),
                ResValue::Hex(x) => (6, *x),
                ResValue::Bool(x) => (7, *x as u32),
                ResValue::Color(x) => (8, *x),
                ResValue::Dimension { value, unit } => {
                    out.push(9);
                    number(out, value.to_bits());
                    out.push(match unit {
                        DimensionUnit::Px => 0,
                        DimensionUnit::Dp => 1,
                        DimensionUnit::Sp => 2,
                        DimensionUnit::Pt => 3,
                        DimensionUnit::In => 4,
                        DimensionUnit::Mm => 5,
                        DimensionUnit::Unknown(x) => *x,
                    });
                    return;
                }
                ResValue::Fraction { value, unit } => {
                    out.push(10);
                    number(out, value.to_bits());
                    out.push(match unit {
                        FractionUnit::Self_ => 0,
                        FractionUnit::Parent => 1,
                        FractionUnit::Unknown(x) => *x,
                    });
                    return;
                }
                ResValue::Unknown { data_type, data } => {
                    out.push(11);
                    number(out, *data);
                    out.push(*data_type);
                    return;
                }
            };
            out.push(tag);
            number(out, n);
        }
        let mut raw = Vec::new();
        let mut entries: Vec<_> = self
            .entries
            .iter()
            .filter(|(id, e)| {
                **id >> 24 == 1
                    && matches!(
                        e.name.split('/').next().unwrap_or(""),
                        "attr"
                            | "style"
                            | "color"
                            | "dimen"
                            | "integer"
                            | "bool"
                            | "fraction"
                            | "string"
                            | "array"
                            | "plurals"
                            | "layout"
                            | "drawable"
                            | "xml"
                    )
            })
            .collect();
        entries.sort_by_key(|(id, _)| **id);
        number(&mut raw, entries.len() as u32);
        for (id, e) in entries {
            number(&mut raw, *id);
            string(&mut raw, &e.name);
            if let Some(b) = &e.bag {
                raw.push(2);
                number(&mut raw, b.parent);
                number(&mut raw, b.items.len() as u32);
                for (k, v) in &b.items {
                    number(&mut raw, *k);
                    if matches!(v,ResValue::String(s) if s.len()>200 || s.starts_with("res/")) {
                        val(&mut raw, &ResValue::Null);
                    } else {
                        val(&mut raw, v);
                    }
                }
            } else if let Some(v) = &e.value {
                if matches!(v,ResValue::String(s) if s.len()>200 || s.starts_with("res/")) {
                    raw.push(0);
                } else {
                    raw.push(1);
                    val(&mut raw, v);
                }
            } else {
                raw.push(0);
            }
        }
        let mut z = zip::ZipWriter::new(std::io::Cursor::new(Vec::new()));
        z.start_file(
            "resources",
            zip::write::SimpleFileOptions::default()
                .compression_method(zip::CompressionMethod::Deflated),
        )
        .unwrap();
        z.write_all(&raw).unwrap();
        z.finish().unwrap().into_inner()
    }
    fn decode_framework(data: &[u8]) -> Self {
        use std::io::Read;
        let mut raw = Vec::new();
        let mut zip = zip::ZipArchive::new(std::io::Cursor::new(data)).expect("framework snapshot");
        zip.by_index(0)
            .expect("framework snapshot entry")
            .read_to_end(&mut raw)
            .expect("framework snapshot data");
        struct Reader<'a> {
            d: &'a [u8],
            p: usize,
        }
        impl Reader<'_> {
            fn byte(&mut self) -> u8 {
                let v = self.d[self.p];
                self.p += 1;
                v
            }
            fn num(&mut self) -> u32 {
                let n = u32::from_le_bytes(self.d[self.p..self.p + 4].try_into().unwrap());
                self.p += 4;
                n
            }
            fn string(&mut self) -> String {
                let n = self.num() as usize;
                let s = std::str::from_utf8(&self.d[self.p..self.p + n])
                    .unwrap()
                    .to_owned();
                self.p += n;
                s
            }
            fn val(&mut self) -> ResValue {
                let t = self.byte();
                if t == 3 {
                    return ResValue::String(self.string());
                }
                let n = self.num();
                match t {
                    0 => ResValue::Null,
                    1 => ResValue::Reference(n),
                    2 => ResValue::Attribute(n),
                    4 => ResValue::Float(f32::from_bits(n)),
                    5 => ResValue::Int(n as i32),
                    6 => ResValue::Hex(n),
                    7 => ResValue::Bool(n != 0),
                    8 => ResValue::Color(n),
                    9 => ResValue::Dimension {
                        value: f32::from_bits(n),
                        unit: match self.byte() {
                            0 => DimensionUnit::Px,
                            1 => DimensionUnit::Dp,
                            2 => DimensionUnit::Sp,
                            3 => DimensionUnit::Pt,
                            4 => DimensionUnit::In,
                            5 => DimensionUnit::Mm,
                            x => DimensionUnit::Unknown(x),
                        },
                    },
                    10 => ResValue::Fraction {
                        value: f32::from_bits(n),
                        unit: match self.byte() {
                            0 => FractionUnit::Self_,
                            1 => FractionUnit::Parent,
                            x => FractionUnit::Unknown(x),
                        },
                    },
                    11 => ResValue::Unknown {
                        data: n,
                        data_type: self.byte(),
                    },
                    _ => panic!("bad framework value"),
                }
            }
        }
        let mut r = Reader { d: &raw, p: 0 };
        let count = r.num();
        let mut result = Self::default();
        for _ in 0..count {
            let id = r.num();
            let name = r.string();
            let tag = r.byte();
            let (value, bag) = match tag {
                1 => (Some(r.val()), None),
                2 => {
                    let parent = r.num();
                    let n = r.num();
                    let mut items = Vec::new();
                    for _ in 0..n {
                        let key = r.num();
                        items.push((key, r.val()));
                    }
                    (None, Some(Bag { parent, items }))
                }
                _ => (None, None),
            };
            result.names.insert(name.clone(), id);
            result.entries.insert(
                id,
                Entry {
                    name,
                    value,
                    bag,
                    config: Config::default(),
                },
            );
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn cfg() -> Vec<u8> {
        vec![0; 64]
    }
    fn set16(c: &mut [u8], i: usize, v: u16) {
        c[i..i + 2].copy_from_slice(&v.to_le_bytes());
    }
    #[test]
    fn select_device_config() {
        let default = cfg();
        let mut v21 = cfg();
        set16(&mut v21, 24, 21);
        assert!(Config::parse(&v21) > Config::parse(&default));
        set16(&mut v21, 24, 31);
        assert!(Config::parse(&v21).is_none());
        let mut c = cfg();
        c[8..10].copy_from_slice(b"fr");
        assert!(Config::parse(&c).is_none());
        c = cfg();
        c[12] = 2;
        assert!(Config::parse(&c).is_none());
        c = cfg();
        c[29] = 0x20;
        assert!(Config::parse(&c).is_none());
        c = cfg();
        set16(&mut c, 30, 391);
        assert!(Config::parse(&c).is_none());
        let mut exact = cfg();
        set16(&mut exact, 14, 480);
        let mut low = cfg();
        set16(&mut low, 14, 320);
        assert!(Config::parse(&exact) > Config::parse(&low));
    }
    #[test]
    fn merge_bags_and_cycles() {
        let mut r = Resources::default();
        let entry = |parent, items| Entry {
            name: String::new(),
            value: None,
            bag: Some(Bag { parent, items }),
            config: Config::default(),
        };
        r.entries.insert(
            0x7f010001,
            entry(
                0x7f010002,
                vec![(1, ResValue::Int(3)), (3, ResValue::Int(4))],
            ),
        );
        r.entries.insert(
            0x7f010002,
            entry(
                0x7f010001,
                vec![(1, ResValue::Int(1)), (2, ResValue::Int(2))],
            ),
        );
        assert_eq!(
            r.style(0x7f010001),
            vec![
                (1, ResValue::Int(3)),
                (2, ResValue::Int(2)),
                (3, ResValue::Int(4))
            ]
        );
    }
    #[test]
    fn parses_androidx_bags() {
        let apk = crate::Apk::open(
            include_bytes!("../../../tests/fixtures/androidx-hello-release.apk").to_vec(),
        )
        .unwrap();
        let resources = apk.resources().unwrap();
        let style = resources
            .entries
            .iter()
            .find_map(|(&id, e)| {
                (e.name.starts_with("style/")
                    && e.bag
                        .as_ref()
                        .is_some_and(|b| b.parent != 0 && !b.items.is_empty()))
                .then_some(id)
            })
            .unwrap();
        let bag = resources.bag(style).unwrap();
        assert_ne!(bag.parent, 0);
        assert!(!bag.items.is_empty());
        assert!(!resources.style(style).is_empty());
    }
    #[test]
    fn framework_resources() {
        let r = Resources::default();
        assert_eq!(r.id_by_name("android:attr", "textColor"), Some(0x01010098));
        let theme = r
            .id_by_name("android:style", "Theme.Material.Light")
            .unwrap();
        assert_ne!(r.bag(theme).unwrap().parent, 0);
        let color = r.id_by_name("android:color", "black").unwrap();
        assert!(matches!(r.resolve(color), Some(ResValue::Color(_))));
    }
}
