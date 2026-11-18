//! Natives for the android.* shim: logging, resources, inflation helpers.

use std::collections::HashMap;

use crate::heap::ObjData;
use crate::{JResult, NativeFn, Vm};

fn opt_str(vm: &mut Vm, s: Option<String>) -> u64 {
    match s {
        Some(s) => vm.new_string(&s) as u64,
        None => 0,
    }
}

fn mirror_class(vm: &Vm, r: u32) -> Option<crate::ClassId> {
    match vm.heap.get(r).map(|o| &o.data) {
        Some(ObjData::Mirror(c)) => Some(*c),
        _ => None,
    }
}

pub fn register(m: &mut HashMap<&'static str, NativeFn>) {
    m.insert(
        "Landroid/util/Log;->nativeWrite(ILjava/lang/String;Ljava/lang/String;)V",
        |vm, a| {
            let level = match a[0] {
                2 => 'V',
                3 => 'D',
                4 => 'I',
                5 => 'W',
                6 => 'E',
                _ => 'A',
            };
            let tag = vm.rust_string(a[1]).unwrap_or_else(|| "null".into());
            let msg = vm.rust_string(a[2]).unwrap_or_else(|| "null".into());
            let line = format!("{level}/{tag}: {msg}\n");
            (vm.out)(3, &line);
            Ok(0)
        },
    );
    m.insert(
        "Landroid/content/res/Resources;->nativeString(I)Ljava/lang/String;",
        |vm, a| {
            let s = vm.resources.as_ref().and_then(|r| r.string(a[0]));
            Ok(opt_str(vm, s))
        },
    );
    m.insert(
        "Landroid/content/res/Resources;->nativeLayout(I)Ljava/lang/String;",
        |vm, a| {
            let s = vm.resources.as_ref().and_then(|r| r.layout(a[0]));
            Ok(opt_str(vm, s))
        },
    );
    m.insert(
        "Landroid/content/res/Resources;->nativeName(I)Ljava/lang/String;",
        |vm, a| {
            let s = vm.resources.as_ref().and_then(|r| r.name(a[0]));
            Ok(opt_str(vm, s))
        },
    );
    m.insert(
        "Landroid/content/res/Resources;->nativeInt(II)I",
        |vm, a| match vm.resources.as_ref().and_then(|r| r.int(a[0], a[1] as i32)) {
            Some(v) => Ok(v as u32 as u64),
            None => not_found(vm, a[0]),
        },
    );
    m.insert(
        "Landroid/content/res/Resources;->nativeDimen(I)F",
        |vm, a| match vm.resources.as_ref().and_then(|r| r.dimen(a[0])) {
            Some(v) => Ok(v.to_bits() as u64),
            None => not_found(vm, a[0]),
        },
    );
    m.insert(
        "Landroid/content/res/Resources;->nativeIdentifier(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)I",
        |vm, a| {
            let mut name = vm.rust_string(a[0]).unwrap_or_default();
            let mut ty = vm.rust_string(a[1]).unwrap_or_default();
            // "pkg:type/name" or "type/name" forms
            if let Some((_, rest)) = name.split_once(':') {
                name = rest.to_string();
            }
            if let Some((t, n)) = name.clone().split_once('/') {
                ty = t.to_string();
                name = n.to_string();
            }
            let id = vm.resources.as_ref().and_then(|r| r.identifier(&name, &ty)).unwrap_or(0);
            Ok(id as u64)
        },
    );
    m.insert(
        "Landroid/content/res/Resources;->nativeStringArray(I)[Ljava/lang/String;",
        |vm, a| {
            let Some(items) = vm.resources.as_ref().and_then(|r| r.string_array(a[0])) else {
                return Ok(0);
            };
            let arr = vm.new_array("[Ljava/lang/String;", items.len())?;
            vm.pins.push(arr);
            for (i, s) in items.iter().enumerate() {
                let r = vm.new_string(s);
                if let ObjData::ArrL(x) = &mut vm.heap.get_mut(arr).unwrap().data {
                    x[i] = r;
                }
            }
            vm.pins.pop();
            Ok(arr as u64)
        },
    );
    m.insert(
        "Landroid/view/VMInflate;->construct(Ljava/lang/Class;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
        |vm, a| {
            let Some(c) = mirror_class(vm, a[0]) else { return vm.throw_new("Ljava/lang/NullPointerException;", "class") };
            vm.ensure_init(c)?;
            let (ctor, two) = match vm.find_declared(c, "<init>(Landroid/content/Context;Landroid/util/AttributeSet;)V") {
                Some(m) => (m, true),
                None => match vm.find_declared(c, "<init>(Landroid/content/Context;)V") {
                    Some(m) => (m, false),
                    None => {
                        let msg = format!("{} has no (Context, AttributeSet) constructor", vm.class(c).java_name());
                        return vm.throw_new("Landroid/view/InflateException;", &msg);
                    }
                },
            };
            let obj = vm.alloc_instance(c);
            vm.pins.push(obj);
            let r = if two { vm.invoke(ctor, &[obj, a[1], a[2]]) } else { vm.invoke(ctor, &[obj, a[1]]) };
            vm.pins.pop();
            r.map(|_| obj as u64)
        },
    );
    m.insert(
        "Landroid/view/VMBridge;->invokeOnClick(Ljava/lang/Object;Ljava/lang/String;Landroid/view/View;)V",
        |vm, a| {
            let name = vm.rust_string(a[1]).unwrap_or_default();
            let Some(cls) = vm.heap.get(a[0]).map(|o| o.class) else { return Ok(0) };
            let id = vm.sig_id(&format!("{name}(Landroid/view/View;)V"));
            match vm.find_virtual(cls, id) {
                Some(m) => vm.invoke(m, &[a[0], a[2]]).map(|_| 0),
                None => {
                    let msg = format!("Could not find method {name}(View) for android:onClick");
                    vm.throw_new("Ljava/lang/IllegalStateException;", &msg)
                }
            }
        },
    );
}

fn not_found(vm: &mut Vm, id: u32) -> JResult<u64> {
    let msg = format!("Resource ID #0x{id:x}");
    vm.throw_new("Landroid/content/res/Resources$NotFoundException;", &msg)
}

/// java.lang.reflect.Array natives (registered with the core set).
pub fn register_reflect(m: &mut HashMap<&'static str, NativeFn>) {
    m.insert(
        "Ljava/lang/reflect/Array;->newInstance(Ljava/lang/Class;I)Ljava/lang/Object;",
        |vm, a| {
            let Some(c) = mirror_class(vm, a[0]) else {
                return vm.throw_new("Ljava/lang/NullPointerException;", "componentType");
            };
            if (a[1] as i32) < 0 {
                return vm.throw_new(
                    "Ljava/lang/NegativeArraySizeException;",
                    &(a[1] as i32).to_string(),
                );
            }
            let desc = format!("[{}", vm.class(c).name);
            Ok(vm.new_array(&desc, a[1] as usize)? as u64)
        },
    );
    m.insert(
        "Ljava/lang/reflect/Array;->arrayClassOf(Ljava/lang/Class;I)Ljava/lang/Class;",
        |vm, a| {
            let c = mirror_class(vm, a[0]).unwrap();
            let desc = format!("{}{}", "[".repeat(a[1] as usize), vm.class(c).name);
            let ac = vm.find_class(&desc)?;
            Ok(vm.mirror(ac) as u64)
        },
    );
    m.insert(
        "Ljava/lang/reflect/Array;->getLength(Ljava/lang/Object;)I",
        |vm, a| match vm.heap.get(a[0]).and_then(|o| o.data.array_len()) {
            Some(l) => Ok(l as u64),
            None if a[0] == 0 => vm.throw_new("Ljava/lang/NullPointerException;", "array"),
            None => vm.throw_new(
                "Ljava/lang/IllegalArgumentException;",
                "Argument is not an array",
            ),
        },
    );
    m.insert(
        "Ljava/lang/reflect/Array;->get(Ljava/lang/Object;I)Ljava/lang/Object;",
        |vm, a| {
            let i = a[1] as i32;
            let Some(o) = vm.heap.get(a[0]) else {
                return vm.throw_new("Ljava/lang/NullPointerException;", "array");
            };
            let len = o.data.array_len().unwrap_or(0);
            if i < 0 || i as usize >= len {
                return vm.throw_new(
                    "Ljava/lang/ArrayIndexOutOfBoundsException;",
                    &format!("length={len}; index={i}"),
                );
            }
            let i = i as usize;
            let comp = match vm.class(o.class).src {
                crate::ClassSrc::Array { component } => vm.class(component).name.clone(),
                _ => return vm.throw_new("Ljava/lang/IllegalArgumentException;", "not an array"),
            };
            let (cls, sig, args): (&str, &str, Vec<u32>) = match (&o.data, &*comp) {
                (ObjData::ArrL(x), _) => return Ok(x[i] as u64),
                (ObjData::ArrI(x), "I") => (
                    "Ljava/lang/Integer;",
                    "valueOf(I)Ljava/lang/Integer;",
                    vec![x[i] as u32],
                ),
                (ObjData::ArrI(x), _) => (
                    "Ljava/lang/Float;",
                    "valueOf(F)Ljava/lang/Float;",
                    vec![x[i] as u32],
                ),
                (ObjData::ArrJ(x), "J") => (
                    "Ljava/lang/Long;",
                    "valueOf(J)Ljava/lang/Long;",
                    vec![x[i] as u32, (x[i] >> 32) as u32],
                ),
                (ObjData::ArrJ(x), _) => (
                    "Ljava/lang/Double;",
                    "valueOf(D)Ljava/lang/Double;",
                    vec![x[i] as u32, (x[i] >> 32) as u32],
                ),
                (ObjData::ArrB(x), "Z") => (
                    "Ljava/lang/Boolean;",
                    "valueOf(Z)Ljava/lang/Boolean;",
                    vec![x[i] as u32],
                ),
                (ObjData::ArrB(x), _) => (
                    "Ljava/lang/Byte;",
                    "valueOf(B)Ljava/lang/Byte;",
                    vec![x[i] as i32 as u32],
                ),
                (ObjData::ArrC(x), _) => (
                    "Ljava/lang/Character;",
                    "valueOf(C)Ljava/lang/Character;",
                    vec![x[i] as u32],
                ),
                (ObjData::ArrS(x), _) => (
                    "Ljava/lang/Short;",
                    "valueOf(S)Ljava/lang/Short;",
                    vec![x[i] as i32 as u32],
                ),
                _ => return Ok(0),
            };
            vm.call_static(cls, sig, &args)
        },
    );
    m.insert(
        "Ljava/lang/reflect/Array;->set(Ljava/lang/Object;ILjava/lang/Object;)V",
        |vm, a| {
            let i = a[1] as i32;
            let Some(o) = vm.heap.get(a[0]) else {
                return vm.throw_new("Ljava/lang/NullPointerException;", "array");
            };
            let len = o.data.array_len().unwrap_or(0);
            if i < 0 || i as usize >= len {
                return vm.throw_new(
                    "Ljava/lang/ArrayIndexOutOfBoundsException;",
                    &format!("length={len}; index={i}"),
                );
            }
            let v = a[2];
            let first = vm.get_field(v, "value").unwrap_or(0);
            let second = vm
                .heap
                .get(v)
                .and_then(|o| match &o.data {
                    ObjData::Inst(f) => f.get(1).copied(),
                    _ => None,
                })
                .unwrap_or(0);
            match &mut vm.heap.get_mut(a[0]).unwrap().data {
                ObjData::ArrL(x) => x[i as usize] = v,
                ObjData::ArrI(x) => x[i as usize] = first as i32,
                ObjData::ArrJ(x) => x[i as usize] = (first as u64 | ((second as u64) << 32)) as i64,
                ObjData::ArrB(x) => x[i as usize] = first as i8,
                ObjData::ArrC(x) => x[i as usize] = first as u16,
                ObjData::ArrS(x) => x[i as usize] = first as i16,
                _ => {}
            }
            Ok(0)
        },
    );
}
