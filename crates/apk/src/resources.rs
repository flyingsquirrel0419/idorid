use crate::{
    binary::{chunk, err, pool, str_at, u16at, u32at, u8at},
    ApkError,
};
use std::collections::{HashMap, HashSet};

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
#[derive(Debug, Clone)]
struct Entry {
    name: String,
    value: ResValue,
    default: bool,
}
#[derive(Debug, Clone, Default)]
pub struct Resources {
    entries: HashMap<u32, Entry>,
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
                let default = c[24..20 + config_size].iter().all(|b| *b == 0);
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
                    let key = u32at(c, ep + 4)?;
                    if es < 8 || ef & 1 != 0 {
                        continue;
                    }
                    let vp = ep.checked_add(es).ok_or_else(err)?;
                    if u16at(c, vp)? < 8 {
                        return Err(err());
                    }
                    let v = value(u8at(c, vp + 3)?, u32at(c, vp + 4)?, global)?;
                    let id = (pkg << 24) | ((ty + type_offset) << 16) | index;
                    let name = format!("{}/{}", type_name, str_at(&keys, key).ok_or_else(err)?);
                    let entry = Entry {
                        name,
                        value: v,
                        default,
                    };
                    match self.entries.get(&id) {
                        Some(old) if old.default || !default => {}
                        _ => {
                            self.entries.insert(id, entry);
                        }
                    }
                }
            }
            p += size;
        }
        Ok(())
    }
    pub fn get(&self, id: u32) -> Option<ResValue> {
        self.entries.get(&id).map(|e| e.value.clone())
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
        self.entries.get(&id).map(|e| e.name.as_str())
    }
    pub fn id_by_name(&self, kind: &str, name: &str) -> Option<u32> {
        let target = format!("{kind}/{name}");
        self.entries
            .iter()
            .find_map(|(&id, e)| (e.name == target).then_some(id))
    }
}
