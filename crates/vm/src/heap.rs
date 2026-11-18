//! Object table + mark-sweep GC.
//!
//! References are `u32` handles (0 = null) into `objects`. Handles are stable,
//! so the collector never moves anything and nothing outside the VM ever sees a
//! raw pointer.

use crate::ClassId;

pub type Ref = u32;

#[derive(Debug, Clone)]
pub enum StrData {
    Latin1(Box<[u8]>),
    Utf16(Box<[u16]>),
}

#[derive(Debug, Clone)]
pub struct JStr {
    pub data: StrData,
    pub hash: u32,
}

impl JStr {
    pub fn from_utf16(v: &[u16]) -> JStr {
        let data = if v.iter().all(|&c| c < 0x100) {
            StrData::Latin1(v.iter().map(|&c| c as u8).collect())
        } else {
            StrData::Utf16(v.into())
        };
        JStr { data, hash: 0 }
    }
    pub fn from_rust(s: &str) -> JStr {
        if s.is_ascii() {
            JStr {
                data: StrData::Latin1(s.as_bytes().into()),
                hash: 0,
            }
        } else {
            let v: Vec<u16> = s.encode_utf16().collect();
            JStr::from_utf16(&v)
        }
    }
    #[inline]
    pub fn len(&self) -> usize {
        match &self.data {
            StrData::Latin1(b) => b.len(),
            StrData::Utf16(b) => b.len(),
        }
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    #[inline]
    pub fn at(&self, i: usize) -> u16 {
        match &self.data {
            StrData::Latin1(b) => b[i] as u16,
            StrData::Utf16(b) => b[i],
        }
    }
    pub fn to_utf16(&self) -> Vec<u16> {
        match &self.data {
            StrData::Latin1(b) => b.iter().map(|&c| c as u16).collect(),
            StrData::Utf16(b) => b.to_vec(),
        }
    }
    pub fn to_rust(&self) -> String {
        match &self.data {
            StrData::Latin1(b) => b.iter().map(|&c| c as char).collect(),
            StrData::Utf16(b) => String::from_utf16_lossy(b),
        }
    }
    pub fn java_hash(&mut self) -> u32 {
        if self.hash == 0 {
            let mut h: u32 = 0;
            for i in 0..self.len() {
                h = h.wrapping_mul(31).wrapping_add(self.at(i) as u32);
            }
            self.hash = h;
        }
        self.hash
    }
    pub fn equals(&self, o: &JStr) -> bool {
        if self.len() != o.len() {
            return false;
        }
        match (&self.data, &o.data) {
            (StrData::Latin1(a), StrData::Latin1(b)) => a == b,
            (StrData::Utf16(a), StrData::Utf16(b)) => a == b,
            _ => (0..self.len()).all(|i| self.at(i) == o.at(i)),
        }
    }
    fn bytes(&self) -> usize {
        match &self.data {
            StrData::Latin1(b) => b.len(),
            StrData::Utf16(b) => b.len() * 2,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ObjData {
    Inst(Box<[u32]>),
    Str(JStr),
    /// boolean[] and byte[]
    ArrB(Box<[i8]>),
    ArrC(Box<[u16]>),
    ArrS(Box<[i16]>),
    /// int[] and float[] (float bits)
    ArrI(Box<[i32]>),
    /// long[] and double[] (double bits)
    ArrJ(Box<[i64]>),
    ArrL(Box<[u32]>),
    /// java.lang.Class mirror
    Mirror(ClassId),
}

impl ObjData {
    pub fn array_len(&self) -> Option<usize> {
        Some(match self {
            ObjData::ArrB(a) => a.len(),
            ObjData::ArrC(a) => a.len(),
            ObjData::ArrS(a) => a.len(),
            ObjData::ArrI(a) => a.len(),
            ObjData::ArrJ(a) => a.len(),
            ObjData::ArrL(a) => a.len(),
            _ => return None,
        })
    }
    fn bytes(&self) -> usize {
        24 + match self {
            ObjData::Inst(f) => f.len() * 4,
            ObjData::Str(s) => s.bytes() + 8,
            ObjData::ArrB(a) => a.len(),
            ObjData::ArrC(a) => a.len() * 2,
            ObjData::ArrS(a) => a.len() * 2,
            ObjData::ArrI(a) => a.len() * 4,
            ObjData::ArrJ(a) => a.len() * 8,
            ObjData::ArrL(a) => a.len() * 4,
            ObjData::Mirror(_) => 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Obj {
    pub class: ClassId,
    pub data: ObjData,
}

pub struct Heap {
    pub objects: Vec<Option<Obj>>,
    free: Vec<u32>,
    marks: Vec<u64>,
    pub live_bytes: usize,
    pub allocated_since_gc: usize,
    pub threshold: usize,
    pub collections: u32,
}

impl Default for Heap {
    fn default() -> Self {
        Self::new()
    }
}

impl Heap {
    pub fn new() -> Heap {
        Heap {
            objects: vec![None], // handle 0 = null
            free: Vec::new(),
            marks: Vec::new(),
            live_bytes: 0,
            allocated_since_gc: 0,
            threshold: 4 << 20,
            collections: 0,
        }
    }

    #[inline]
    pub fn alloc(&mut self, obj: Obj) -> Ref {
        let b = obj.data.bytes();
        self.live_bytes += b;
        self.allocated_since_gc += b;
        if let Some(h) = self.free.pop() {
            self.objects[h as usize] = Some(obj);
            h
        } else {
            let h = self.objects.len() as u32;
            self.objects.push(Some(obj));
            h
        }
    }

    #[inline]
    pub fn wants_gc(&self) -> bool {
        self.allocated_since_gc > self.threshold
    }

    #[inline]
    pub fn get(&self, r: Ref) -> Option<&Obj> {
        self.objects.get(r as usize).and_then(|o| o.as_ref())
    }
    #[inline]
    pub fn get_mut(&mut self, r: Ref) -> Option<&mut Obj> {
        self.objects.get_mut(r as usize).and_then(|o| o.as_mut())
    }

    /// Is `r` a handle to a live object? (Used for conservative frame scanning.)
    #[inline]
    pub fn is_live(&self, r: u32) -> bool {
        r != 0 && matches!(self.objects.get(r as usize), Some(Some(_)))
    }

    pub fn object_count(&self) -> usize {
        self.objects.len() - 1 - self.free.len()
    }

    /// Mark from `roots`; `ref_slots(class)` lists which instance slots hold refs.
    pub fn collect<F>(&mut self, roots: &mut dyn Iterator<Item = u32>, ref_slots: F)
    where
        F: Fn(ClassId) -> *const [u32],
    {
        let n = self.objects.len();
        self.marks.clear();
        self.marks.resize(n.div_ceil(64), 0);
        let mut stack: Vec<u32> = Vec::with_capacity(256);
        for r in roots {
            if self.is_live(r) && !self.test_set(r) {
                stack.push(r);
            }
        }
        let marks = &mut self.marks;
        let objects = &self.objects;
        while let Some(r) = stack.pop() {
            let obj = objects[r as usize].as_ref().unwrap();
            match &obj.data {
                ObjData::Inst(fields) => {
                    // SAFETY: the slot list lives in the class table, which is not
                    // mutated during collection.
                    let slots = unsafe { &*ref_slots(obj.class) };
                    for &s in slots {
                        let c = fields[s as usize];
                        if c != 0 && !mark(marks, c) {
                            stack.push(c);
                        }
                    }
                }
                ObjData::ArrL(a) => {
                    for &c in a.iter() {
                        if c != 0 && !mark(marks, c) {
                            stack.push(c);
                        }
                    }
                }
                _ => {}
            }
        }
        let mut live = 0;
        for h in 1..n {
            if self.objects[h].is_some() {
                if self.test(h as u32) {
                    live += self.objects[h].as_ref().unwrap().data.bytes();
                } else {
                    self.objects[h] = None;
                    self.free.push(h as u32);
                }
            }
        }
        self.live_bytes = live;
        self.allocated_since_gc = 0;
        self.threshold = (live * 2).max(4 << 20);
        self.collections += 1;
    }

    #[inline]
    fn test(&self, r: u32) -> bool {
        self.marks[(r / 64) as usize] & (1 << (r % 64)) != 0
    }
    #[inline]
    fn test_set(&mut self, r: u32) -> bool {
        mark(&mut self.marks, r)
    }
}

/// Sets the mark bit; returns whether it was already set.
#[inline]
fn mark(marks: &mut [u64], r: u32) -> bool {
    let w = &mut marks[(r / 64) as usize];
    let bit = 1 << (r % 64);
    let was = *w & bit != 0;
    *w |= bit;
    was
}
