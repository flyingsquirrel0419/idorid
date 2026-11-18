use crate::{
    android_attrs,
    binary::{chunk, err, pool, str_at, u16at, u32at, u8at},
    resources::{value, ResValue},
    ApkError,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Attribute {
    pub namespace: Option<String>,
    pub name: String,
    pub resource_id: Option<u32>,
    pub value: ResValue,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Element {
    pub namespace: Option<String>,
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub children: Vec<Element>,
}
impl Element {
    pub fn attribute(&self, name: &str) -> Option<&Attribute> {
        self.attributes.iter().find(|a| a.name == name)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Document {
    pub root: Element,
}
impl Document {
    pub fn parse(data: &[u8]) -> Result<Self, ApkError> {
        let (t, h, s) = chunk(data, 0)?;
        if t != 3 {
            return Err(err());
        }
        let data = &data[..s];
        let mut p = h;
        let mut strings = Vec::new();
        let mut ids = Vec::new();
        let mut stack: Vec<Element> = Vec::new();
        let mut root = None;
        while p < s {
            let (t, header, size) = chunk(data, p)?;
            let c = &data[p..p + size];
            match t {
                1 => strings = pool(c)?,
                0x180 => {
                    if (size - header) % 4 != 0 {
                        return Err(err());
                    }
                    for i in (header..size).step_by(4) {
                        ids.push(u32at(c, i)?)
                    }
                }
                0x102 => {
                    if header < 16 || size < header + 20 {
                        return Err(err());
                    }
                    let ns = u32at(c, header)?;
                    let name = u32at(c, header + 4)?;
                    let start = u16at(c, header + 8)? as usize;
                    let stride = u16at(c, header + 10)? as usize;
                    let count = u16at(c, header + 12)? as usize;
                    if stride < 20 || start < 20 {
                        return Err(err());
                    }
                    let base = header.checked_add(start).ok_or_else(err)?;
                    if base
                        .checked_add(count.checked_mul(stride).ok_or_else(err)?)
                        .filter(|end| *end <= size)
                        .is_none()
                    {
                        return Err(err());
                    }
                    let mut attrs = Vec::with_capacity(count);
                    for i in 0..count {
                        let a = base + i * stride;
                        let ns = u32at(c, a)?;
                        let name_idx = u32at(c, a + 4)?;
                        let raw = u32at(c, a + 8)?;
                        let id = ids.get(name_idx as usize).copied().filter(|x| *x != 0);
                        let original = str_at(&strings, name_idx).unwrap_or("");
                        let name = id
                            .and_then(android_attrs::name)
                            .unwrap_or(original)
                            .to_owned();
                        let kind = u8at(c, a + 15)?;
                        let data = u32at(c, a + 16)?;
                        let val =
                            if kind == 3 && str_at(&strings, data).is_none() && raw != u32::MAX {
                                ResValue::String(str_at(&strings, raw).ok_or_else(err)?.to_owned())
                            } else {
                                value(kind, data, &strings)?
                            };
                        attrs.push(Attribute {
                            namespace: str_at(&strings, ns).map(str::to_owned),
                            name,
                            resource_id: id,
                            value: val,
                        });
                    }
                    stack.push(Element {
                        namespace: str_at(&strings, ns).map(str::to_owned),
                        name: str_at(&strings, name).ok_or_else(err)?.to_owned(),
                        attributes: attrs,
                        children: Vec::new(),
                    });
                }
                0x103 => {
                    if header < 16 || size < header + 8 {
                        return Err(err());
                    }
                    let element = stack.pop().ok_or_else(err)?;
                    if str_at(&strings, u32at(c, header + 4)?) != Some(element.name.as_str()) {
                        return Err(err());
                    }
                    if let Some(parent) = stack.last_mut() {
                        parent.children.push(element)
                    } else if root.replace(element).is_some() {
                        return Err(err());
                    }
                }
                _ => {}
            }
            p += size;
        }
        if !stack.is_empty() {
            return Err(err());
        }
        Ok(Self {
            root: root.ok_or_else(err)?,
        })
    }
}
