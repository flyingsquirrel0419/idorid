//! Zero-copy DEX file parser.
//!
//! Everything borrows from the original byte buffer; nothing is decoded until
//! asked for. All offsets are bounds-checked so a malformed file produces an
//! error instead of a panic.

use std::fmt;

mod leb;
pub use leb::{read_sleb128, read_uleb128};

pub const NO_INDEX: u32 = 0xffff_ffff;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DexError {
    BadMagic,
    OutOfBounds(&'static str, usize),
    Malformed(&'static str),
}

impl fmt::Display for DexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DexError::BadMagic => write!(f, "not a dex file"),
            DexError::OutOfBounds(what, off) => write!(f, "{what} out of bounds at {off:#x}"),
            DexError::Malformed(what) => write!(f, "malformed dex: {what}"),
        }
    }
}

impl std::error::Error for DexError {}

pub type Result<T> = std::result::Result<T, DexError>;

#[inline]
fn rd_u16(d: &[u8], off: usize) -> Result<u16> {
    d.get(off..off + 2)
        .map(|b| u16::from_le_bytes([b[0], b[1]]))
        .ok_or(DexError::OutOfBounds("u16", off))
}

#[inline]
fn rd_u32(d: &[u8], off: usize) -> Result<u32> {
    d.get(off..off + 4)
        .map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
        .ok_or(DexError::OutOfBounds("u32", off))
}

#[derive(Debug, Clone, Copy)]
struct Section {
    size: u32,
    off: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProtoId {
    pub shorty_idx: u32,
    pub return_type_idx: u32,
    pub parameters_off: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FieldId {
    pub class_idx: u16,
    pub type_idx: u16,
    pub name_idx: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MethodId {
    pub class_idx: u16,
    pub proto_idx: u16,
    pub name_idx: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClassDef {
    pub class_idx: u32,
    pub access_flags: u32,
    pub superclass_idx: u32,
    pub interfaces_off: u32,
    pub source_file_idx: u32,
    pub annotations_off: u32,
    pub class_data_off: u32,
    pub static_values_off: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EncodedField {
    pub field_idx: u32,
    pub access_flags: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EncodedMethod {
    pub method_idx: u32,
    pub access_flags: u32,
    pub code_off: u32,
}

#[derive(Debug, Clone, Default)]
pub struct ClassData {
    pub static_fields: Vec<EncodedField>,
    pub instance_fields: Vec<EncodedField>,
    pub direct_methods: Vec<EncodedMethod>,
    pub virtual_methods: Vec<EncodedMethod>,
}

/// A method body. `insns` borrows the code units from the file.
#[derive(Debug, Clone, Copy)]
pub struct CodeItem<'a> {
    pub registers_size: u16,
    pub ins_size: u16,
    pub outs_size: u16,
    pub tries_size: u16,
    pub debug_info_off: u32,
    /// Raw little-endian code units (2 bytes each).
    pub insns_bytes: &'a [u8],
    tries_off: usize,
    handlers_off: usize,
}

impl<'a> CodeItem<'a> {
    #[inline]
    pub fn insns_len(&self) -> usize {
        self.insns_bytes.len() / 2
    }

    /// Decode code units into an owned Vec (cheap; done once per method).
    pub fn insns(&self) -> Vec<u16> {
        self.insns_bytes
            .chunks_exact(2)
            .map(|c| u16::from_le_bytes([c[0], c[1]]))
            .collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TryItem {
    pub start_addr: u32,
    pub insn_count: u16,
    pub handler_off: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CatchHandler {
    /// (type_idx, handler address in code units)
    pub typed: Vec<(u32, u32)>,
    pub catch_all: Option<u32>,
}

/// Decoded `encoded_value`.
#[derive(Debug, Clone, PartialEq)]
pub enum EncodedValue {
    Byte(i8),
    Short(i16),
    Char(u16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    MethodType(u32),
    MethodHandle(u32),
    String(u32),
    Type(u32),
    Field(u32),
    Method(u32),
    Enum(u32),
    Array(Vec<EncodedValue>),
    Annotation(u32, Vec<(u32, EncodedValue)>),
    Null,
    Boolean(bool),
}

pub struct Dex<'a> {
    data: &'a [u8],
    strings: Section,
    types: Section,
    protos: Section,
    fields: Section,
    methods: Section,
    class_defs: Section,
}

impl<'a> fmt::Debug for Dex<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Dex")
            .field("len", &self.data.len())
            .field("strings", &self.strings.size)
            .field("types", &self.types.size)
            .field("methods", &self.methods.size)
            .field("classes", &self.class_defs.size)
            .finish()
    }
}

impl<'a> Dex<'a> {
    pub fn parse(data: &'a [u8]) -> Result<Self> {
        if data.len() < 0x70 || &data[0..4] != b"dex\n" || data[7] != 0 {
            return Err(DexError::BadMagic);
        }
        if rd_u32(data, 40)? != 0x1234_5678 {
            return Err(DexError::Malformed("big-endian dex not supported"));
        }
        let sec = |o: usize, elem: usize, what: &'static str| -> Result<Section> {
            let s = Section {
                size: rd_u32(data, o)?,
                off: rd_u32(data, o + 4)?,
            };
            let end = s.off as usize + s.size as usize * elem;
            if s.size > 0 && end > data.len() {
                return Err(DexError::OutOfBounds(what, s.off as usize));
            }
            Ok(s)
        };
        Ok(Dex {
            data,
            strings: sec(56, 4, "string_ids")?,
            types: sec(64, 4, "type_ids")?,
            protos: sec(72, 12, "proto_ids")?,
            fields: sec(80, 8, "field_ids")?,
            methods: sec(88, 8, "method_ids")?,
            class_defs: sec(96, 32, "class_defs")?,
        })
    }

    #[inline]
    pub fn data(&self) -> &'a [u8] {
        self.data
    }

    pub fn string_count(&self) -> u32 {
        self.strings.size
    }
    pub fn type_count(&self) -> u32 {
        self.types.size
    }
    pub fn field_count(&self) -> u32 {
        self.fields.size
    }
    pub fn method_count(&self) -> u32 {
        self.methods.size
    }
    pub fn class_def_count(&self) -> u32 {
        self.class_defs.size
    }

    fn check(&self, idx: u32, s: Section, what: &'static str) -> Result<()> {
        if idx >= s.size {
            Err(DexError::OutOfBounds(what, idx as usize))
        } else {
            Ok(())
        }
    }

    /// Raw MUTF-8 bytes of a string (without the trailing NUL) and its UTF-16 length.
    pub fn string_raw(&self, idx: u32) -> Result<(&'a [u8], u32)> {
        self.check(idx, self.strings, "string idx")?;
        let off = rd_u32(self.data, self.strings.off as usize + idx as usize * 4)? as usize;
        let mut p = off;
        let utf16_len = read_uleb128(self.data, &mut p)?;
        let rest = self
            .data
            .get(p..)
            .ok_or(DexError::OutOfBounds("string data", p))?;
        let end = rest
            .iter()
            .position(|&b| b == 0)
            .ok_or(DexError::Malformed("unterminated string"))?;
        Ok((&rest[..end], utf16_len))
    }

    /// String as Rust `str` when it is plain ASCII/UTF-8 compatible (the common
    /// case for descriptors and names). Falls back to lossy MUTF-8 decoding.
    pub fn string(&self, idx: u32) -> Result<std::borrow::Cow<'a, str>> {
        let (raw, _) = self.string_raw(idx)?;
        if raw.is_ascii() {
            // SAFETY-free: ascii is valid utf8
            return Ok(std::borrow::Cow::Borrowed(
                std::str::from_utf8(raw).expect("ascii"),
            ));
        }
        Ok(std::borrow::Cow::Owned(String::from_utf16_lossy(
            &mutf8_to_utf16(raw),
        )))
    }

    /// String decoded into UTF-16 code units (Java semantics).
    pub fn string_utf16(&self, idx: u32) -> Result<Vec<u16>> {
        let (raw, _) = self.string_raw(idx)?;
        Ok(mutf8_to_utf16(raw))
    }

    pub fn type_descriptor_idx(&self, type_idx: u32) -> Result<u32> {
        self.check(type_idx, self.types, "type idx")?;
        rd_u32(self.data, self.types.off as usize + type_idx as usize * 4)
    }

    pub fn type_descriptor(&self, type_idx: u32) -> Result<std::borrow::Cow<'a, str>> {
        self.string(self.type_descriptor_idx(type_idx)?)
    }

    pub fn proto_id(&self, idx: u32) -> Result<ProtoId> {
        self.check(idx, self.protos, "proto idx")?;
        let o = self.protos.off as usize + idx as usize * 12;
        Ok(ProtoId {
            shorty_idx: rd_u32(self.data, o)?,
            return_type_idx: rd_u32(self.data, o + 4)?,
            parameters_off: rd_u32(self.data, o + 8)?,
        })
    }

    pub fn field_id(&self, idx: u32) -> Result<FieldId> {
        self.check(idx, self.fields, "field idx")?;
        let o = self.fields.off as usize + idx as usize * 8;
        Ok(FieldId {
            class_idx: rd_u16(self.data, o)?,
            type_idx: rd_u16(self.data, o + 2)?,
            name_idx: rd_u32(self.data, o + 4)?,
        })
    }

    pub fn method_id(&self, idx: u32) -> Result<MethodId> {
        self.check(idx, self.methods, "method idx")?;
        let o = self.methods.off as usize + idx as usize * 8;
        Ok(MethodId {
            class_idx: rd_u16(self.data, o)?,
            proto_idx: rd_u16(self.data, o + 2)?,
            name_idx: rd_u32(self.data, o + 4)?,
        })
    }

    pub fn class_def(&self, idx: u32) -> Result<ClassDef> {
        self.check(idx, self.class_defs, "class_def idx")?;
        let o = self.class_defs.off as usize + idx as usize * 32;
        let r = |k: usize| rd_u32(self.data, o + k * 4);
        Ok(ClassDef {
            class_idx: r(0)?,
            access_flags: r(1)?,
            superclass_idx: r(2)?,
            interfaces_off: r(3)?,
            source_file_idx: r(4)?,
            annotations_off: r(5)?,
            class_data_off: r(6)?,
            static_values_off: r(7)?,
        })
    }

    pub fn class_defs(&self) -> impl Iterator<Item = Result<ClassDef>> + '_ {
        (0..self.class_defs.size).map(move |i| self.class_def(i))
    }

    /// Reads a `type_list` (used for interfaces and parameters).
    pub fn type_list(&self, off: u32) -> Result<Vec<u16>> {
        if off == 0 {
            return Ok(Vec::new());
        }
        let off = off as usize;
        let n = rd_u32(self.data, off)? as usize;
        (0..n).map(|i| rd_u16(self.data, off + 4 + i * 2)).collect()
    }

    /// Parameter type indices of a proto.
    pub fn proto_params(&self, proto_idx: u32) -> Result<Vec<u16>> {
        self.type_list(self.proto_id(proto_idx)?.parameters_off)
    }

    /// Java-style method signature, e.g. `(ILjava/lang/String;)V`.
    pub fn proto_signature(&self, proto_idx: u32) -> Result<String> {
        let p = self.proto_id(proto_idx)?;
        let mut s = String::from("(");
        for t in self.type_list(p.parameters_off)? {
            s.push_str(&self.type_descriptor(t as u32)?);
        }
        s.push(')');
        s.push_str(&self.type_descriptor(p.return_type_idx)?);
        Ok(s)
    }

    pub fn class_data(&self, off: u32) -> Result<ClassData> {
        let mut cd = ClassData::default();
        if off == 0 {
            return Ok(cd);
        }
        let d = self.data;
        let mut p = off as usize;
        let sf = read_uleb128(d, &mut p)?;
        let inf = read_uleb128(d, &mut p)?;
        let dm = read_uleb128(d, &mut p)?;
        let vm = read_uleb128(d, &mut p)?;
        let fields = |n: u32, p: &mut usize| -> Result<Vec<EncodedField>> {
            let mut idx = 0u32;
            let mut v = Vec::with_capacity(n.min(4096) as usize);
            for _ in 0..n {
                idx = idx.wrapping_add(read_uleb128(d, p)?);
                v.push(EncodedField {
                    field_idx: idx,
                    access_flags: read_uleb128(d, p)?,
                });
            }
            Ok(v)
        };
        cd.static_fields = fields(sf, &mut p)?;
        cd.instance_fields = fields(inf, &mut p)?;
        let methods = |n: u32, p: &mut usize| -> Result<Vec<EncodedMethod>> {
            let mut idx = 0u32;
            let mut v = Vec::with_capacity(n.min(4096) as usize);
            for _ in 0..n {
                idx = idx.wrapping_add(read_uleb128(d, p)?);
                v.push(EncodedMethod {
                    method_idx: idx,
                    access_flags: read_uleb128(d, p)?,
                    code_off: read_uleb128(d, p)?,
                });
            }
            Ok(v)
        };
        cd.direct_methods = methods(dm, &mut p)?;
        cd.virtual_methods = methods(vm, &mut p)?;
        Ok(cd)
    }

    pub fn code_item(&self, off: u32) -> Result<CodeItem<'a>> {
        let o = off as usize;
        let d = self.data;
        let tries_size = rd_u16(d, o + 6)?;
        let n = rd_u32(d, o + 12)? as usize;
        let start = o + 16;
        let end = start + n * 2;
        let insns_bytes = d
            .get(start..end)
            .ok_or(DexError::OutOfBounds("insns", start))?;
        let tries_off = if tries_size > 0 && n % 2 == 1 {
            end + 2
        } else {
            end
        };
        Ok(CodeItem {
            registers_size: rd_u16(d, o)?,
            ins_size: rd_u16(d, o + 2)?,
            outs_size: rd_u16(d, o + 4)?,
            tries_size,
            debug_info_off: rd_u32(d, o + 8)?,
            insns_bytes,
            tries_off,
            handlers_off: tries_off + tries_size as usize * 8,
        })
    }

    pub fn tries(&self, code: &CodeItem<'a>) -> Result<Vec<TryItem>> {
        (0..code.tries_size as usize)
            .map(|i| {
                let o = code.tries_off + i * 8;
                Ok(TryItem {
                    start_addr: rd_u32(self.data, o)?,
                    insn_count: rd_u16(self.data, o + 4)?,
                    handler_off: rd_u16(self.data, o + 6)?,
                })
            })
            .collect()
    }

    /// Decodes the catch handler at `handler_off` (relative to the handler list).
    pub fn catch_handler(&self, code: &CodeItem<'a>, handler_off: u16) -> Result<CatchHandler> {
        let d = self.data;
        let mut p = code.handlers_off + handler_off as usize;
        let size = read_sleb128(d, &mut p)?;
        let mut h = CatchHandler::default();
        for _ in 0..size.unsigned_abs() {
            let t = read_uleb128(d, &mut p)?;
            let a = read_uleb128(d, &mut p)?;
            h.typed.push((t, a));
        }
        if size <= 0 {
            h.catch_all = Some(read_uleb128(d, &mut p)?);
        }
        Ok(h)
    }

    /// Static initial values for a class (`static_values_off`).
    pub fn encoded_array(&self, off: u32) -> Result<Vec<EncodedValue>> {
        if off == 0 {
            return Ok(Vec::new());
        }
        let mut p = off as usize;
        self.read_array(&mut p)
    }

    fn read_array(&self, p: &mut usize) -> Result<Vec<EncodedValue>> {
        let n = read_uleb128(self.data, p)?;
        let mut v = Vec::with_capacity(n.min(4096) as usize);
        for _ in 0..n {
            v.push(self.read_value(p)?);
        }
        Ok(v)
    }

    fn read_value(&self, p: &mut usize) -> Result<EncodedValue> {
        let d = self.data;
        let b = *d.get(*p).ok_or(DexError::OutOfBounds("encoded_value", *p))?;
        *p += 1;
        let ty = b & 0x1f;
        let arg = (b >> 5) as usize;
        let mut take = |n: usize| -> Result<u64> {
            let bytes = d
                .get(*p..*p + n)
                .ok_or(DexError::OutOfBounds("encoded_value", *p))?;
            *p += n;
            let mut v = 0u64;
            for (i, &x) in bytes.iter().enumerate() {
                v |= (x as u64) << (i * 8);
            }
            Ok(v)
        };
        let sext = |v: u64, n: usize| -> i64 {
            let shift = 64 - n * 8;
            ((v << shift) as i64) >> shift
        };
        Ok(match ty {
            0x00 => EncodedValue::Byte(take(1)? as i8),
            0x02 => EncodedValue::Short(sext(take(arg + 1)?, arg + 1) as i16),
            0x03 => EncodedValue::Char(take(arg + 1)? as u16),
            0x04 => EncodedValue::Int(sext(take(arg + 1)?, arg + 1) as i32),
            0x06 => EncodedValue::Long(sext(take(arg + 1)?, arg + 1)),
            0x10 => {
                // zero-extended to the right
                let v = take(arg + 1)? << ((3 - arg) * 8);
                EncodedValue::Float(f32::from_bits(v as u32))
            }
            0x11 => {
                let v = take(arg + 1)? << ((7 - arg) * 8);
                EncodedValue::Double(f64::from_bits(v))
            }
            0x15 => EncodedValue::MethodType(take(arg + 1)? as u32),
            0x16 => EncodedValue::MethodHandle(take(arg + 1)? as u32),
            0x17 => EncodedValue::String(take(arg + 1)? as u32),
            0x18 => EncodedValue::Type(take(arg + 1)? as u32),
            0x19 => EncodedValue::Field(take(arg + 1)? as u32),
            0x1a => EncodedValue::Method(take(arg + 1)? as u32),
            0x1b => EncodedValue::Enum(take(arg + 1)? as u32),
            0x1c => EncodedValue::Array(self.read_array(p)?),
            0x1d => {
                let t = read_uleb128(d, p)?;
                let n = read_uleb128(d, p)?;
                let mut els = Vec::new();
                for _ in 0..n {
                    let name = read_uleb128(d, p)?;
                    els.push((name, self.read_value(p)?));
                }
                EncodedValue::Annotation(t, els)
            }
            0x1e => EncodedValue::Null,
            0x1f => EncodedValue::Boolean(arg != 0),
            _ => return Err(DexError::Malformed("encoded_value type")),
        })
    }
}

/// Decodes Modified UTF-8 into UTF-16 code units.
pub fn mutf8_to_utf16(raw: &[u8]) -> Vec<u16> {
    let mut out = Vec::with_capacity(raw.len());
    let mut i = 0;
    while i < raw.len() {
        let a = raw[i] as u16;
        if a < 0x80 {
            out.push(a);
            i += 1;
        } else if a & 0xe0 == 0xc0 && i + 1 < raw.len() {
            out.push(((a & 0x1f) << 6) | (raw[i + 1] as u16 & 0x3f));
            i += 2;
        } else if a & 0xf0 == 0xe0 && i + 2 < raw.len() {
            out.push(((a & 0x0f) << 12) | ((raw[i + 1] as u16 & 0x3f) << 6) | (raw[i + 2] as u16 & 0x3f));
            i += 3;
        } else {
            out.push(0xfffd);
            i += 1;
        }
    }
    out
}

/// Number of 32-bit register slots an argument of this descriptor uses.
#[inline]
pub fn descriptor_slots(desc: &str) -> usize {
    match desc.as_bytes().first() {
        Some(b'J') | Some(b'D') => 2,
        _ => 1,
    }
}

pub mod access {
    pub const PUBLIC: u32 = 0x1;
    pub const PRIVATE: u32 = 0x2;
    pub const PROTECTED: u32 = 0x4;
    pub const STATIC: u32 = 0x8;
    pub const FINAL: u32 = 0x10;
    pub const SYNCHRONIZED: u32 = 0x20;
    pub const NATIVE: u32 = 0x100;
    pub const INTERFACE: u32 = 0x200;
    pub const ABSTRACT: u32 = 0x400;
    pub const ENUM: u32 = 0x4000;
    pub const CONSTRUCTOR: u32 = 0x10000;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_garbage() {
        assert_eq!(Dex::parse(b"nope").unwrap_err(), DexError::BadMagic);
        let mut v = vec![0u8; 0x70];
        v[..8].copy_from_slice(b"dex\n035\0");
        assert!(Dex::parse(&v).is_err()); // bad endian tag
    }

    #[test]
    fn mutf8() {
        assert_eq!(mutf8_to_utf16(b"hi"), vec![104, 105]);
        // U+0000 encoded as C0 80
        assert_eq!(mutf8_to_utf16(&[0xc0, 0x80]), vec![0]);
        // U+D55C (한)
        assert_eq!(mutf8_to_utf16(&[0xed, 0x95, 0x9c]), vec![0xd55c]);
    }
}
