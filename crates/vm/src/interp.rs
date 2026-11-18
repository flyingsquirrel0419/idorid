//! The bytecode interpreter: a non-recursive `match` loop over 16-bit code
//! units. Java calls push a `Frame`; they never recurse on the Rust stack, so
//! deep Java recursion is bounded only by `max_depth`.

use std::rc::Rc;

use crate::{ClassSrc, Code, Frame, JResult, MKind, MethodIdx, ObjData, Throw, Vm};
use idorid_dex::NO_INDEX;

const NPE: &str = "Ljava/lang/NullPointerException;";
const AIOOBE: &str = "Ljava/lang/ArrayIndexOutOfBoundsException;";

struct State {
    code: Rc<Code>,
    pc: usize,
    fp: usize,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    Virtual,
    Super,
    Direct,
    Static,
    Interface,
}

impl Vm {
    /// Invokes a method with raw argument slots (receiver first for instance methods).
    pub fn invoke(&mut self, m: MethodIdx, args: &[u32]) -> JResult<u64> {
        let kind = self.methods[m as usize].kind.clone();
        match kind {
            MKind::Native(f) => return f(self, args),
            MKind::Abstract => {
                let msg = self.method_desc(m);
                return self.throw_new("Ljava/lang/AbstractMethodError;", &msg);
            }
            _ => {}
        }
        let Some(code) = self.method_code(m) else {
            let msg = format!("No implementation found for {}", self.method_desc(m));
            return self.throw_new("Ljava/lang/UnsatisfiedLinkError;", &msg);
        };
        if self.frames.len() >= self.max_depth {
            return self.throw_new("Ljava/lang/StackOverflowError;", "stack size exceeded");
        }
        let fp = self.sp;
        let regs = code.regs as usize;
        let ins = code.ins as usize;
        self.ensure_stack(fp + regs);
        self.stack[fp..fp + regs].fill(0);
        let n = args.len().min(ins);
        self.stack[fp + regs - ins..fp + regs - ins + n].copy_from_slice(&args[..n]);
        self.sp = fp + regs;
        self.frames.push(Frame {
            method: m,
            code: code.clone(),
            pc: 0,
            fp,
        });
        let base = self.frames.len() - 1;
        let mut st = State { code, pc: 0, fp };
        loop {
            match self.run(&mut st, base) {
                Ok(v) => return Ok(v),
                Err(Throw) => {
                    if !self.unwind(&mut st, base) {
                        return Err(Throw);
                    }
                }
            }
        }
    }

    pub fn method_desc(&self, m: MethodIdx) -> String {
        let meth = &self.methods[m as usize];
        format!(
            "{}.{}{}",
            self.class(meth.class).java_name(),
            meth.name,
            meth.sig
        )
    }

    #[inline]
    fn ensure_stack(&mut self, n: usize) {
        if n > self.stack.len() {
            let new = (n * 2).max(self.stack.len() * 2);
            self.stack.resize(new, 0);
        }
    }

    /// Finds a handler for the pending exception, popping frames down to `base`.
    fn unwind(&mut self, st: &mut State, base: usize) -> bool {
        loop {
            if self.exit_code.is_none() {
                if let Some(h) = self.find_handler(&st.code.clone(), st.pc) {
                    st.pc = h as usize;
                    return true;
                }
            }
            let f = self.frames.pop().expect("frame");
            self.sp = f.fp;
            if self.frames.len() == base {
                return false;
            }
            let top = self.frames.last().unwrap();
            st.code = top.code.clone();
            st.fp = top.fp;
            st.pc = top.pc;
        }
    }

    fn find_handler(&mut self, code: &Code, pc: usize) -> Option<u32> {
        let pc = pc as u32;
        let exc = self.exception;
        if exc == 0 {
            return None;
        }
        let t = code.tries.iter().find(|t| t.start <= pc && pc < t.end)?;
        for &(ty, addr) in t.handlers.iter() {
            if ty == NO_INDEX {
                return Some(addr);
            }
            // Resolution failure must not clobber the in-flight exception.
            match self.resolve_type(code.dex, ty) {
                Ok(c) => {
                    if self.instance_of(exc, c) {
                        return Some(addr);
                    }
                }
                Err(_) => self.exception = exc,
            }
        }
        None
    }

    #[inline]
    fn npe<T>(&mut self, what: &str) -> JResult<T> {
        self.throw_new(NPE, what)
    }

    /// Runs until the frame at index `base` returns.
    fn run(&mut self, st: &mut State, base: usize) -> JResult<u64> {
        'frame: loop {
            let code = st.code.clone();
            let insns: &[u16] = &code.insns;
            let fp = st.fp;
            let dex = code.dex;
            let mut pc = st.pc;

            macro_rules! r {
                ($i:expr) => {
                    self.stack[fp + ($i) as usize]
                };
            }
            macro_rules! ri {
                ($i:expr) => {
                    self.stack[fp + ($i) as usize] as i32
                };
            }
            macro_rules! rf {
                ($i:expr) => {
                    f32::from_bits(self.stack[fp + ($i) as usize])
                };
            }
            macro_rules! rw {
                ($i:expr) => {{
                    let i = fp + ($i) as usize;
                    (self.stack[i] as u64) | ((self.stack[i + 1] as u64) << 32)
                }};
            }
            macro_rules! rl {
                ($i:expr) => {
                    rw!($i) as i64
                };
            }
            macro_rules! rd {
                ($i:expr) => {
                    f64::from_bits(rw!($i))
                };
            }
            macro_rules! setw {
                ($i:expr, $v:expr) => {{
                    let v: u64 = $v;
                    let i = fp + ($i) as usize;
                    self.stack[i] = v as u32;
                    self.stack[i + 1] = (v >> 32) as u32;
                }};
            }
            macro_rules! w {
                ($k:expr) => {
                    insns[pc + $k] as u32
                };
            }
            macro_rules! w32 {
                ($k:expr) => {
                    (insns[pc + $k] as u32) | ((insns[pc + $k + 1] as u32) << 16)
                };
            }

            loop {
                st.pc = pc;
                let ins = insns[pc];
                let op = ins & 0xff;
                let aa = (ins >> 8) as usize;
                let a = ((ins >> 8) & 0xf) as usize;
                let b = (ins >> 12) as usize;
                match op {
                    0x00 => pc += 1,
                    0x01 | 0x07 => {
                        r!(a) = r!(b);
                        pc += 1;
                    }
                    0x02 | 0x08 => {
                        r!(aa) = r!(w!(1));
                        pc += 2;
                    }
                    0x03 | 0x09 => {
                        r!(w!(1)) = r!(w!(2));
                        pc += 3;
                    }
                    0x04 => {
                        let v = rw!(b);
                        setw!(a, v);
                        pc += 1;
                    }
                    0x05 => {
                        let v = rw!(w!(1));
                        setw!(aa, v);
                        pc += 2;
                    }
                    0x06 => {
                        let v = rw!(w!(2));
                        setw!(w!(1), v);
                        pc += 3;
                    }
                    0x0a | 0x0c => {
                        r!(aa) = self.ret as u32;
                        pc += 1;
                    }
                    0x0b => {
                        setw!(aa, self.ret);
                        pc += 1;
                    }
                    0x0d => {
                        r!(aa) = self.exception;
                        self.exception = 0;
                        pc += 1;
                    }
                    0x0e..=0x11 => {
                        let v = match op {
                            0x0e => 0,
                            0x10 => rw!(aa),
                            _ => r!(aa) as u64,
                        };
                        let f = self.frames.pop().expect("frame");
                        self.sp = f.fp;
                        if self.frames.len() == base {
                            return Ok(v);
                        }
                        self.ret = v;
                        let top = self.frames.last().unwrap();
                        st.code = top.code.clone();
                        st.fp = top.fp;
                        st.pc = top.pc + 3;
                        continue 'frame;
                    }
                    0x12 => {
                        r!(a) = ((ins as i16) >> 12) as i32 as u32;
                        pc += 1;
                    }
                    0x13 => {
                        r!(aa) = insns[pc + 1] as i16 as i32 as u32;
                        pc += 2;
                    }
                    0x14 => {
                        r!(aa) = w32!(1);
                        pc += 3;
                    }
                    0x15 => {
                        r!(aa) = w!(1) << 16;
                        pc += 2;
                    }
                    0x16 => {
                        setw!(aa, insns[pc + 1] as i16 as i64 as u64);
                        pc += 2;
                    }
                    0x17 => {
                        setw!(aa, w32!(1) as i32 as i64 as u64);
                        pc += 3;
                    }
                    0x18 => {
                        let v = (w32!(1) as u64) | ((w32!(3) as u64) << 32);
                        setw!(aa, v);
                        pc += 5;
                    }
                    0x19 => {
                        setw!(aa, (w!(1) as u64) << 48);
                        pc += 2;
                    }
                    0x1a => {
                        self.maybe_gc();
                        r!(aa) = self.const_string(dex, w!(1))?;
                        pc += 2;
                    }
                    0x1b => {
                        self.maybe_gc();
                        r!(aa) = self.const_string(dex, w32!(1))?;
                        pc += 3;
                    }
                    0x1c => {
                        let c = self.resolve_type(dex, w!(1))?;
                        r!(aa) = self.mirror(c);
                        pc += 2;
                    }
                    0x1d | 0x1e => {
                        if r!(aa) == 0 {
                            return self.npe("monitor on null");
                        }
                        pc += 1;
                    }
                    0x1f => {
                        let v = r!(aa);
                        if v != 0 {
                            let c = self.resolve_type(dex, w!(1))?;
                            if !self.instance_of(v, c) {
                                let from = self.class(self.heap.get(v).unwrap().class).java_name();
                                let msg = format!(
                                    "{} cannot be cast to {}",
                                    from,
                                    self.class(c).java_name()
                                );
                                return self.throw_new("Ljava/lang/ClassCastException;", &msg);
                            }
                        }
                        pc += 2;
                    }
                    0x20 => {
                        let v = r!(b);
                        let res = if v == 0 {
                            false
                        } else {
                            let c = self.resolve_type(dex, w!(1))?;
                            self.instance_of(v, c)
                        };
                        r!(a) = res as u32;
                        pc += 2;
                    }
                    0x21 => {
                        let v = r!(b);
                        let len = match self.heap.get(v).and_then(|o| o.data.array_len()) {
                            Some(l) => l,
                            None => return self.npe("array-length on null"),
                        };
                        r!(a) = len as u32;
                        pc += 1;
                    }
                    0x22 => {
                        let c = self.resolve_type(dex, w!(1))?;
                        if self.class(c).flags
                            & (idorid_dex::access::ABSTRACT | idorid_dex::access::INTERFACE)
                            != 0
                        {
                            let msg = self.class(c).java_name();
                            return self.throw_new("Ljava/lang/InstantiationError;", &msg);
                        }
                        self.ensure_init(c)?;
                        self.maybe_gc();
                        r!(aa) = self.alloc_instance(c);
                        pc += 2;
                    }
                    0x23 => {
                        let n = ri!(b);
                        if n < 0 {
                            return self.throw_new(
                                "Ljava/lang/NegativeArraySizeException;",
                                &n.to_string(),
                            );
                        }
                        let c = self.resolve_type(dex, w!(1))?;
                        self.maybe_gc();
                        r!(a) = self.alloc_array(c, n as usize);
                        pc += 2;
                    }
                    0x24 | 0x25 => {
                        let c = self.resolve_type(dex, w!(1))?;
                        let mut vals = [0u32; 256];
                        let count;
                        if op == 0x24 {
                            count = b;
                            let rw = insns[pc + 2] as usize;
                            let list = [rw & 0xf, (rw >> 4) & 0xf, (rw >> 8) & 0xf, rw >> 12, a];
                            for i in 0..count {
                                vals[i] = r!(list[i]);
                            }
                        } else {
                            count = aa;
                            let first = w!(2) as usize;
                            for i in 0..count {
                                vals[i] = r!(first + i);
                            }
                        }
                        self.maybe_gc();
                        let arr = self.alloc_array(c, count);
                        match &mut self.heap.get_mut(arr).unwrap().data {
                            ObjData::ArrI(x) => {
                                for i in 0..count {
                                    x[i] = vals[i] as i32;
                                }
                            }
                            ObjData::ArrL(x) => x.copy_from_slice(&vals[..count]),
                            _ => {
                                return self.throw_new(
                                    "Ljava/lang/InternalError;",
                                    "filled-new-array type",
                                )
                            }
                        }
                        self.ret = arr as u64;
                        pc += 3;
                    }
                    0x26 => {
                        let off = (pc as i64 + w32!(1) as i32 as i64) as usize;
                        let width = insns[off + 1] as usize;
                        let size = (insns[off + 2] as usize) | ((insns[off + 3] as usize) << 16);
                        let data = &insns[off + 4..];
                        let byte = |i: usize| -> u8 {
                            let u = data[i / 2];
                            if i.is_multiple_of(2) {
                                u as u8
                            } else {
                                (u >> 8) as u8
                            }
                        };
                        let arr = r!(aa);
                        let Some(obj) = self.heap.get_mut(arr) else {
                            return self.npe("fill-array-data on null");
                        };
                        let ok = obj.data.array_len().map(|l| l >= size).unwrap_or(false);
                        if !ok {
                            return self.throw_new(AIOOBE, "fill-array-data");
                        }
                        let le = |i: usize, n: usize| -> u64 {
                            let mut v = 0u64;
                            for k in 0..n {
                                v |= (byte(i * n + k) as u64) << (8 * k);
                            }
                            v
                        };
                        match (&mut obj.data, width) {
                            (ObjData::ArrB(x), 1) => (0..size).for_each(|i| x[i] = byte(i) as i8),
                            (ObjData::ArrC(x), 2) => (0..size).for_each(|i| x[i] = le(i, 2) as u16),
                            (ObjData::ArrS(x), 2) => (0..size).for_each(|i| x[i] = le(i, 2) as i16),
                            (ObjData::ArrI(x), 4) => (0..size).for_each(|i| x[i] = le(i, 4) as i32),
                            (ObjData::ArrJ(x), 8) => (0..size).for_each(|i| x[i] = le(i, 8) as i64),
                            _ => {
                                return self.throw_new(
                                    "Ljava/lang/InternalError;",
                                    "fill-array-data width",
                                )
                            }
                        }
                        pc += 3;
                    }
                    0x27 => {
                        let v = r!(aa);
                        if v == 0 {
                            return self.npe("throw null");
                        }
                        self.exception = v;
                        return Err(Throw);
                    }
                    0x28 => pc = (pc as isize + (aa as u8 as i8) as isize) as usize,
                    0x29 => pc = (pc as isize + insns[pc + 1] as i16 as isize) as usize,
                    0x2a => pc = (pc as isize + w32!(1) as i32 as isize) as usize,
                    0x2b => {
                        let off = (pc as i64 + w32!(1) as i32 as i64) as usize;
                        let size = insns[off + 1] as usize;
                        let first =
                            ((insns[off + 2] as u32) | ((insns[off + 3] as u32) << 16)) as i32;
                        let idx = (ri!(aa) as i64) - first as i64;
                        if idx >= 0 && (idx as usize) < size {
                            let t = off + 4 + idx as usize * 2;
                            let rel = ((insns[t] as u32) | ((insns[t + 1] as u32) << 16)) as i32;
                            pc = (pc as isize + rel as isize) as usize;
                        } else {
                            pc += 3;
                        }
                    }
                    0x2c => {
                        let off = (pc as i64 + w32!(1) as i32 as i64) as usize;
                        let size = insns[off + 1] as usize;
                        let key = ri!(aa);
                        let rd32 =
                            |p: usize| ((insns[p] as u32) | ((insns[p + 1] as u32) << 16)) as i32;
                        let keys = off + 2;
                        let targets = keys + size * 2;
                        let (mut lo, mut hi) = (0usize, size);
                        let mut found = None;
                        while lo < hi {
                            let mid = (lo + hi) / 2;
                            let k = rd32(keys + mid * 2);
                            if k == key {
                                found = Some(mid);
                                break;
                            } else if k < key {
                                lo = mid + 1;
                            } else {
                                hi = mid;
                            }
                        }
                        match found {
                            Some(i) => pc = (pc as isize + rd32(targets + i * 2) as isize) as usize,
                            None => pc += 3,
                        }
                    }
                    0x2d..=0x31 => {
                        let bb = w!(1) & 0xff;
                        let cc = w!(1) >> 8;
                        let res: i32 = match op {
                            0x2d | 0x2e => {
                                let (x, y) = (rf!(bb), rf!(cc));
                                if x < y {
                                    -1
                                } else if x > y {
                                    1
                                } else if x == y {
                                    0
                                } else if op == 0x2d {
                                    -1
                                } else {
                                    1
                                }
                            }
                            0x2f | 0x30 => {
                                let (x, y) = (rd!(bb), rd!(cc));
                                if x < y {
                                    -1
                                } else if x > y {
                                    1
                                } else if x == y {
                                    0
                                } else if op == 0x2f {
                                    -1
                                } else {
                                    1
                                }
                            }
                            _ => rl!(bb).cmp(&rl!(cc)) as i32,
                        };
                        r!(aa) = res as u32;
                        pc += 2;
                    }
                    0x32..=0x37 => {
                        let (x, y) = (ri!(a), ri!(b));
                        let t = match op {
                            0x32 => x == y,
                            0x33 => x != y,
                            0x34 => x < y,
                            0x35 => x >= y,
                            0x36 => x > y,
                            _ => x <= y,
                        };
                        pc = if t {
                            (pc as isize + insns[pc + 1] as i16 as isize) as usize
                        } else {
                            pc + 2
                        };
                    }
                    0x38..=0x3d => {
                        let x = ri!(aa);
                        let t = match op {
                            0x38 => x == 0,
                            0x39 => x != 0,
                            0x3a => x < 0,
                            0x3b => x >= 0,
                            0x3c => x > 0,
                            _ => x <= 0,
                        };
                        pc = if t {
                            (pc as isize + insns[pc + 1] as i16 as isize) as usize
                        } else {
                            pc + 2
                        };
                    }
                    0x44..=0x51 => {
                        let arr = r!(w!(1) & 0xff);
                        let idx = ri!(w!(1) >> 8);
                        let put = op >= 0x4b;
                        let kind = if put { op - 0x4b } else { op - 0x44 };
                        // aput-object store check
                        if put && kind == 2 {
                            let v = r!(aa);
                            if v != 0 && arr != 0 {
                                let ac = self.heap.get(arr).map(|o| o.class);
                                if let Some(ac) = ac {
                                    if let ClassSrc::Array { component } = self.class(ac).src {
                                        if !self.instance_of(v, component) {
                                            let msg = self
                                                .class(self.heap.get(v).unwrap().class)
                                                .java_name();
                                            return self.throw_new(
                                                "Ljava/lang/ArrayStoreException;",
                                                &msg,
                                            );
                                        }
                                    }
                                }
                            }
                        }
                        let val = if put {
                            if kind == 1 {
                                rw!(aa)
                            } else {
                                r!(aa) as u64
                            }
                        } else {
                            0
                        };
                        let Some(obj) = self.heap.get_mut(arr) else {
                            return self.npe("array access on null");
                        };
                        let len = obj.data.array_len().unwrap_or(0);
                        if idx < 0 || idx as usize >= len {
                            let msg = format!("length={len}; index={idx}");
                            return self.throw_new(AIOOBE, &msg);
                        }
                        let i = idx as usize;
                        let mut out: Option<u64> = None;
                        match (&mut obj.data, kind, put) {
                            (ObjData::ArrI(x), 0, false) => out = Some(x[i] as u32 as u64),
                            (ObjData::ArrI(x), 0, true) => x[i] = val as i32,
                            (ObjData::ArrJ(x), 1, false) => out = Some(x[i] as u64),
                            (ObjData::ArrJ(x), 1, true) => x[i] = val as i64,
                            (ObjData::ArrL(x), 2, false) => out = Some(x[i] as u64),
                            (ObjData::ArrL(x), 2, true) => x[i] = val as u32,
                            (ObjData::ArrB(x), 3, false) => out = Some(x[i] as u8 as u64),
                            (ObjData::ArrB(x), 4, false) => out = Some(x[i] as i32 as u32 as u64),
                            (ObjData::ArrB(x), 3 | 4, true) => x[i] = val as i8,
                            (ObjData::ArrC(x), 5, false) => out = Some(x[i] as u64),
                            (ObjData::ArrC(x), 5, true) => x[i] = val as u16,
                            (ObjData::ArrS(x), 6, false) => out = Some(x[i] as i32 as u32 as u64),
                            (ObjData::ArrS(x), 6, true) => x[i] = val as i16,
                            _ => {
                                return self
                                    .throw_new("Ljava/lang/VerifyError;", "array type mismatch")
                            }
                        }
                        if let Some(v) = out {
                            if kind == 1 {
                                setw!(aa, v);
                            } else {
                                r!(aa) = v as u32;
                            }
                        }
                        pc += 2;
                    }
                    0x52..=0x5f => {
                        let put = op >= 0x59;
                        let kind = if put { op - 0x59 } else { op - 0x52 };
                        let fr = self.resolve_field(dex, w!(1), false)?;
                        let obj = r!(b);
                        let wide = kind == 1;
                        let val = if put {
                            if wide {
                                rw!(a)
                            } else {
                                narrow(r!(a), kind) as u64
                            }
                        } else {
                            0
                        };
                        let Some(o) = self.heap.get_mut(obj) else {
                            return self.npe("field access on null object");
                        };
                        let ObjData::Inst(fs) = &mut o.data else {
                            return self.throw_new(
                                "Ljava/lang/IncompatibleClassChangeError;",
                                "field on non-instance",
                            );
                        };
                        let s = fr.slot as usize;
                        if put {
                            fs[s] = val as u32;
                            if wide {
                                fs[s + 1] = (val >> 32) as u32;
                            }
                        } else if wide {
                            let v = fs[s] as u64 | ((fs[s + 1] as u64) << 32);
                            setw!(a, v);
                        } else {
                            let v = fs[s];
                            r!(a) = v;
                        }
                        pc += 2;
                    }
                    0x60..=0x6d => {
                        let put = op >= 0x67;
                        let kind = if put { op - 0x67 } else { op - 0x60 };
                        let fr = self.resolve_field(dex, w!(1), true)?;
                        if self.class(fr.owner).state != crate::ClassState::Initialized {
                            self.ensure_init(fr.owner)?;
                        }
                        let s = fr.slot as usize;
                        let wide = kind == 1;
                        if put {
                            if wide {
                                let v = rw!(aa);
                                let stc = &mut self.class_mut(fr.owner).statics;
                                stc[s] = v as u32;
                                stc[s + 1] = (v >> 32) as u32;
                            } else {
                                let v = narrow(r!(aa), kind);
                                self.class_mut(fr.owner).statics[s] = v;
                            }
                        } else if wide {
                            let stc = &self.class(fr.owner).statics;
                            let v = stc[s] as u64 | ((stc[s + 1] as u64) << 32);
                            setw!(aa, v);
                        } else {
                            r!(aa) = self.class(fr.owner).statics[s];
                        }
                        pc += 2;
                    }
                    0x6e..=0x72 | 0x74..=0x78 => {
                        let range = op >= 0x74;
                        let kind = match if range { op - 0x74 } else { op - 0x6e } {
                            0 => Kind::Virtual,
                            1 => Kind::Super,
                            2 => Kind::Direct,
                            3 => Kind::Static,
                            _ => Kind::Interface,
                        };
                        self.maybe_gc();
                        // Gather argument register numbers.
                        let mut regs = [0usize; 5];
                        let (count, first) = if range {
                            (aa, w!(2) as usize)
                        } else {
                            let rw = insns[pc + 2] as usize;
                            regs = [rw & 0xf, (rw >> 4) & 0xf, (rw >> 8) & 0xf, rw >> 12, a];
                            (b, 0)
                        };
                        let reg_of = |i: usize| if range { first + i } else { regs[i] };
                        let midx = w!(1);
                        let mref = self.resolve_method_ref(dex, midx)?;
                        let target = match kind {
                            Kind::Static => match mref.direct {
                                Some(m) => {
                                    let c = self.methods[m as usize].class;
                                    if self.class(c).state != crate::ClassState::Initialized {
                                        self.ensure_init(c)?;
                                    }
                                    m
                                }
                                None => return self.no_such_method(mref.class, mref.sig_id),
                            },
                            Kind::Direct => {
                                if r!(reg_of(0)) == 0 {
                                    return self.npe_invoke(mref.sig_id);
                                }
                                match mref.direct {
                                    Some(m) => m,
                                    None => return self.no_such_method(mref.class, mref.sig_id),
                                }
                            }
                            Kind::Virtual | Kind::Interface => {
                                let recv = r!(reg_of(0));
                                let Some(rc) = self.heap.get(recv).map(|o| o.class) else {
                                    return self.npe_invoke(mref.sig_id);
                                };
                                // Monomorphic inline cache on the method ref.
                                let ic = self.ic_get(dex, midx);
                                if ic.0 == rc.0 + 1 {
                                    ic.1
                                } else {
                                    match self.find_virtual(rc, mref.sig_id) {
                                        Some(m) => {
                                            self.ic_set(dex, midx, rc.0 + 1, m);
                                            m
                                        }
                                        None => return self.no_such_method(rc, mref.sig_id),
                                    }
                                }
                            }
                            Kind::Super => {
                                if r!(reg_of(0)) == 0 {
                                    return self.npe_invoke(mref.sig_id);
                                }
                                let found = if self.class(mref.class).is_interface() {
                                    mref.direct
                                } else {
                                    let cur = self.frames.last().unwrap().method;
                                    let cc = self.methods[cur as usize].class;
                                    match self.class(cc).super_ {
                                        Some(s) => self.find_virtual(s, mref.sig_id),
                                        None => None,
                                    }
                                };
                                match found {
                                    Some(m) => m,
                                    None => return self.no_such_method(mref.class, mref.sig_id),
                                }
                            }
                        };
                        if let Some(f) = self.frames.last_mut() {
                            f.pc = pc;
                        }
                        match self.methods[target as usize].kind {
                            MKind::Native(f) => {
                                let mut buf = [0u32; 16];
                                let v = if count <= 16 {
                                    for (i, slot) in buf.iter_mut().enumerate().take(count) {
                                        *slot = r!(reg_of(i));
                                    }
                                    f(self, &buf[..count])?
                                } else {
                                    let v: Vec<u32> = (0..count).map(|i| r!(reg_of(i))).collect();
                                    f(self, &v)?
                                };
                                self.ret = v;
                                pc += 3;
                            }
                            MKind::Abstract => {
                                let msg = self.method_desc(target);
                                return self.throw_new("Ljava/lang/AbstractMethodError;", &msg);
                            }
                            _ => {
                                let Some(callee) = self.method_code(target) else {
                                    let msg = format!(
                                        "No implementation found for {}",
                                        self.method_desc(target)
                                    );
                                    return self
                                        .throw_new("Ljava/lang/UnsatisfiedLinkError;", &msg);
                                };
                                if self.frames.len() >= self.max_depth {
                                    return self.throw_new(
                                        "Ljava/lang/StackOverflowError;",
                                        "stack size exceeded",
                                    );
                                }
                                let nfp = self.sp;
                                let regs_n = callee.regs as usize;
                                let ins_n = callee.ins as usize;
                                self.ensure_stack(nfp + regs_n);
                                self.stack[nfp..nfp + regs_n].fill(0);
                                let dst = nfp + regs_n - ins_n;
                                if range {
                                    self.stack.copy_within(
                                        fp + first..fp + first + count.min(ins_n),
                                        dst,
                                    );
                                } else {
                                    for i in 0..count.min(ins_n) {
                                        self.stack[dst + i] = self.stack[fp + regs[i]];
                                    }
                                }
                                self.sp = nfp + regs_n;
                                self.frames.push(Frame {
                                    method: target,
                                    code: callee.clone(),
                                    pc: 0,
                                    fp: nfp,
                                });
                                st.code = callee;
                                st.fp = nfp;
                                st.pc = 0;
                                continue 'frame;
                            }
                        }
                    }
                    0x7b..=0x8f => {
                        match op {
                            0x7b => r!(a) = ri!(b).wrapping_neg() as u32,
                            0x7c => r!(a) = !r!(b),
                            0x7d => setw!(a, rl!(b).wrapping_neg() as u64),
                            0x7e => setw!(a, !rw!(b)),
                            0x7f => r!(a) = (-rf!(b)).to_bits(),
                            0x80 => setw!(a, (-rd!(b)).to_bits()),
                            0x81 => setw!(a, ri!(b) as i64 as u64),
                            0x82 => r!(a) = (ri!(b) as f32).to_bits(),
                            0x83 => setw!(a, (ri!(b) as f64).to_bits()),
                            0x84 => r!(a) = rw!(b) as u32,
                            0x85 => r!(a) = (rl!(b) as f32).to_bits(),
                            0x86 => setw!(a, (rl!(b) as f64).to_bits()),
                            0x87 => r!(a) = rf!(b) as i32 as u32,
                            0x88 => setw!(a, rf!(b) as i64 as u64),
                            0x89 => setw!(a, (rf!(b) as f64).to_bits()),
                            0x8a => r!(a) = rd!(b) as i32 as u32,
                            0x8b => setw!(a, rd!(b) as i64 as u64),
                            0x8c => r!(a) = (rd!(b) as f32).to_bits(),
                            0x8d => r!(a) = ri!(b) as i8 as i32 as u32,
                            0x8e => r!(a) = r!(b) & 0xffff,
                            _ => r!(a) = ri!(b) as i16 as i32 as u32,
                        }
                        pc += 1;
                    }
                    0x90..=0xcf => {
                        let two = op >= 0xb0;
                        let o = if two { op - 0xb0 } else { op - 0x90 };
                        let (d, x, y) = if two {
                            (a, a, b)
                        } else {
                            (aa, (w!(1) & 0xff) as usize, (w!(1) >> 8) as usize)
                        };
                        match o {
                            0..=10 => {
                                let v = self.int_op(o, ri!(x), ri!(y))?;
                                r!(d) = v as u32;
                            }
                            11..=21 => {
                                let k = o - 11;
                                // shifts take an int shift amount
                                let yv = if k >= 8 { ri!(y) as i64 } else { rl!(y) };
                                let v = self.long_op(k, rl!(x), yv)?;
                                setw!(d, v as u64);
                            }
                            22..=26 => {
                                let (p, q) = (rf!(x), rf!(y));
                                let v = match o - 22 {
                                    0 => p + q,
                                    1 => p - q,
                                    2 => p * q,
                                    3 => p / q,
                                    _ => p % q,
                                };
                                r!(d) = v.to_bits();
                            }
                            _ => {
                                let (p, q) = (rd!(x), rd!(y));
                                let v = match o - 27 {
                                    0 => p + q,
                                    1 => p - q,
                                    2 => p * q,
                                    3 => p / q,
                                    _ => p % q,
                                };
                                setw!(d, v.to_bits());
                            }
                        }
                        pc += if two { 1 } else { 2 };
                    }
                    0xd0..=0xd7 => {
                        let lit = insns[pc + 1] as i16 as i32;
                        let o = op - 0xd0;
                        let v = if o == 1 {
                            lit.wrapping_sub(ri!(b))
                        } else {
                            self.int_op(o, ri!(b), lit)?
                        };
                        r!(a) = v as u32;
                        pc += 2;
                    }
                    0xd8..=0xe2 => {
                        let src = (w!(1) & 0xff) as usize;
                        let lit = (insns[pc + 1] >> 8) as u8 as i8 as i32;
                        let o = op - 0xd8;
                        let v = if o == 1 {
                            lit.wrapping_sub(ri!(src))
                        } else {
                            self.int_op(o, ri!(src), lit)?
                        };
                        r!(aa) = v as u32;
                        pc += 2;
                    }
                    0xfa..=0xff => {
                        let msg = format!("unsupported opcode {op:#x} (invoke-polymorphic/custom)");
                        return self.throw_new("Ljava/lang/UnsupportedOperationException;", &msg);
                    }
                    _ => {
                        let msg = format!("bad opcode {op:#x} at {pc}");
                        return self.throw_new("Ljava/lang/VerifyError;", &msg);
                    }
                }
                self.instructions += 1;
            }
        }
    }

    #[inline]
    fn int_op(&mut self, o: u16, x: i32, y: i32) -> JResult<i32> {
        Ok(match o {
            0 => x.wrapping_add(y),
            1 => x.wrapping_sub(y),
            2 => x.wrapping_mul(y),
            3 | 4 => {
                if y == 0 {
                    return self.throw_new("Ljava/lang/ArithmeticException;", "divide by zero");
                }
                if o == 3 {
                    x.wrapping_div(y)
                } else {
                    x.wrapping_rem(y)
                }
            }
            5 => x & y,
            6 => x | y,
            7 => x ^ y,
            8 => x.wrapping_shl(y as u32 & 31),
            9 => x.wrapping_shr(y as u32 & 31),
            _ => ((x as u32) >> (y as u32 & 31)) as i32,
        })
    }

    #[inline]
    fn long_op(&mut self, o: u16, x: i64, y: i64) -> JResult<i64> {
        Ok(match o {
            0 => x.wrapping_add(y),
            1 => x.wrapping_sub(y),
            2 => x.wrapping_mul(y),
            3 | 4 => {
                if y == 0 {
                    return self.throw_new("Ljava/lang/ArithmeticException;", "divide by zero");
                }
                if o == 3 {
                    x.wrapping_div(y)
                } else {
                    x.wrapping_rem(y)
                }
            }
            5 => x & y,
            6 => x | y,
            7 => x ^ y,
            8 => x.wrapping_shl(y as u32 & 63),
            9 => x.wrapping_shr(y as u32 & 63),
            _ => ((x as u64) >> (y as u32 & 63)) as i64,
        })
    }

    fn ic_get(&self, dex: u16, midx: u32) -> (u32, MethodIdx) {
        self.dexes[dex as usize]
            .ic
            .get(midx as usize)
            .copied()
            .unwrap_or((0, 0))
    }

    fn ic_set(&mut self, dex: u16, midx: u32, cls: u32, m: MethodIdx) {
        let df = &mut self.dexes[dex as usize];
        if df.ic.is_empty() {
            df.ic = vec![(0, 0); df.methods.len()];
        }
        df.ic[midx as usize] = (cls, m);
    }

    fn npe_invoke<T>(&mut self, sig_id: u32) -> JResult<T> {
        let name = self.method_ref_name(sig_id);
        let msg = format!("Attempt to invoke method '{name}' on a null object reference");
        self.throw_new(NPE, &msg)
    }

    fn no_such_method<T>(&mut self, c: crate::ClassId, sig_id: u32) -> JResult<T> {
        let msg = format!(
            "No method {} in class {}",
            self.method_ref_name(sig_id),
            self.class(c).name
        );
        self.throw_new("Ljava/lang/NoSuchMethodError;", &msg)
    }
}

/// Truncates a register value for sub-int field stores.
#[inline]
fn narrow(v: u32, kind: u16) -> u32 {
    match kind {
        3 => v & 1,
        4 => v as i8 as i32 as u32,
        5 => v & 0xffff,
        6 => v as i16 as i32 as u32,
        _ => v,
    }
}
