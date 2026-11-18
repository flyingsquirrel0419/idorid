//! Host bridge: loads an APK into the VM, drives the Activity lifecycle, and
//! exposes a C ABI (`include/idorid.h`) for the iOS app.

use std::cell::RefCell;
use std::ffi::{c_char, CStr, CString};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::rc::Rc;

use idorid_apk::axml::{Document, Element};
use idorid_apk::resources::{DimensionUnit, ResValue};
use idorid_apk::{Apk, Manifest, Resources};
use idorid_vm::{ResourceProvider, Throw, Vm};

/// Resources backed by the APK.
pub struct ApkResources {
    apk: Rc<Apk>,
    res: Option<Resources>,
    package: String,
}

fn unit(u: DimensionUnit) -> &'static str {
    match u {
        DimensionUnit::Px => "px",
        DimensionUnit::Dp => "dp",
        DimensionUnit::Sp => "sp",
        DimensionUnit::Pt => "pt",
        DimensionUnit::In => "in",
        DimensionUnit::Mm => "mm",
        DimensionUnit::Unknown(_) => "px",
    }
}

fn dimen_px(value: f32, u: DimensionUnit) -> f32 {
    match u {
        DimensionUnit::Pt => value * 160.0 / 72.0,
        DimensionUnit::In => value * 160.0,
        DimensionUnit::Mm => value * 160.0 / 25.4,
        _ => value,
    }
}

/// Attributes whose references are identities, not values to resolve.
const REF_ATTRS: &[&str] = &[
    "id",
    "layout",
    "src",
    "srcCompat",
    "style",
    "theme",
    "layout_below",
    "layout_above",
    "layout_toLeftOf",
    "layout_toRightOf",
    "layout_toStartOf",
    "layout_toEndOf",
    "layout_alignTop",
    "layout_alignBottom",
    "layout_alignLeft",
    "layout_alignRight",
    "layout_alignStart",
    "layout_alignEnd",
    "labelFor",
    "nextFocusDown",
    "entries",
    "menu",
];

impl ApkResources {
    fn resolve(&self, v: &ResValue) -> ResValue {
        match (v, &self.res) {
            (ResValue::Reference(id), Some(r)) => r.resolve(*id).unwrap_or_else(|| v.clone()),
            _ => v.clone(),
        }
    }

    fn encode(&self, name: &str, v: &ResValue) -> Option<String> {
        if let ResValue::Reference(id) = v {
            if REF_ATTRS.contains(&name) {
                return Some(format!("r:{}", *id as i32));
            }
        }
        let rv = self.resolve(v);
        Some(match rv {
            ResValue::Null | ResValue::Attribute(_) => return None,
            ResValue::String(s) => {
                format!("s:{}", s.replace(['\u{1}', '\u{2}', '\u{3}', '\u{4}'], ""))
            }
            ResValue::Reference(id) => format!("r:{}", id as i32),
            ResValue::Int(i) => format!("i:{i}"),
            ResValue::Hex(h) => format!("i:{}", h as i32),
            ResValue::Bool(b) => format!("b:{b}"),
            ResValue::Dimension { value, unit: u } => {
                format!("d:{}{}", dimen_px(value, u), unit(u))
            }
            ResValue::Fraction { value, .. } => format!("f:{value}"),
            ResValue::Color(c) => format!("c:{}", c as i32),
            ResValue::Float(f) => format!("f:{f}"),
            ResValue::Unknown { data, .. } => format!("i:{}", data as i32),
        })
    }

    fn serialize(&self, e: &Element, out: &mut String) {
        out.push('\u{1}');
        out.push_str(&e.name);
        for a in &e.attributes {
            if a.name.is_empty() {
                continue;
            }
            // Only android:* and app:* (custom view) attributes plus un-namespaced ones (style, class, layout)
            if let Some(v) = self.encode(&a.name, &a.value) {
                out.push('\u{2}');
                out.push_str(&a.name);
                out.push('\u{4}');
                out.push_str(&v);
            }
        }
        for c in &e.children {
            self.serialize(c, out);
        }
        out.push('\u{3}');
    }
}

impl ResourceProvider for ApkResources {
    fn string(&self, id: u32) -> Option<String> {
        match self.resolve(&ResValue::Reference(id)) {
            ResValue::String(s) => Some(s),
            ResValue::Int(i) => Some(i.to_string()),
            _ => self.res.as_ref()?.string(id),
        }
    }
    fn layout(&self, id: u32) -> Option<String> {
        let path = self.res.as_ref()?.file_path(id)?;
        let bytes = self.apk.read(&path)?;
        let doc = Document::parse(&bytes).ok()?;
        let mut out = String::with_capacity(bytes.len() / 2);
        self.serialize(&doc.root, &mut out);
        Some(out)
    }
    fn name(&self, id: u32) -> Option<String> {
        let n = self.res.as_ref()?.name(id)?;
        Some(if n.contains(':') {
            n.to_string()
        } else {
            format!("{}:{}", self.package, n)
        })
    }
    fn int(&self, id: u32, _kind: i32) -> Option<i32> {
        match self.resolve(&ResValue::Reference(id)) {
            ResValue::Color(c) => Some(c as i32),
            ResValue::Int(i) => Some(i),
            ResValue::Hex(h) => Some(h as i32),
            ResValue::Bool(b) => Some(b as i32),
            _ => None,
        }
    }
    fn dimen(&self, id: u32) -> Option<f32> {
        match self.resolve(&ResValue::Reference(id)) {
            ResValue::Dimension { value, unit } => Some(dimen_px(value, unit)),
            ResValue::Float(f) => Some(f),
            ResValue::Int(i) => Some(i as f32),
            _ => None,
        }
    }
    fn identifier(&self, name: &str, ty: &str) -> Option<u32> {
        self.res.as_ref()?.id_by_name(ty, name)
    }
    fn string_array(&self, _id: u32) -> Option<Vec<String>> {
        None
    }
}

/// A loaded Android app.
pub struct App {
    pub vm: Vm,
    pub manifest: Manifest,
    pub label: String,
    log: Rc<RefCell<String>>,
    launched: bool,
}

impl App {
    /// Loads an APK. `echo` also forwards log output to stdout/stderr.
    pub fn load(bytes: Vec<u8>, echo: bool) -> Result<App, String> {
        let t0 = std::time::Instant::now();
        let apk = Rc::new(Apk::open(bytes).map_err(|e| format!("bad apk: {e:?}"))?);
        let manifest = apk.manifest().map_err(|e| format!("bad manifest: {e:?}"))?;
        let res = apk.resources().ok();
        let provider = ApkResources {
            apk: apk.clone(),
            res,
            package: manifest.package.clone(),
        };
        let label = match &manifest.label {
            Some(ResValue::String(s)) => s.clone(),
            Some(v) => match provider.resolve(v) {
                ResValue::String(s) => s,
                _ => manifest.package.clone(),
            },
            None => manifest.package.clone(),
        };
        let mut vm = Vm::new();
        for d in apk.dex_files() {
            vm.add_dex(Rc::from(d))?;
        }
        vm.resources = Some(Box::new(provider));
        let log = Rc::new(RefCell::new(String::new()));
        let sink = log.clone();
        vm.out = Box::new(move |fd, s| {
            let mut l = sink.borrow_mut();
            if fd == 2 && !s.is_empty() {
                l.push_str("E/System.err: ");
            } else if fd == 1 && !s.is_empty() {
                l.push_str("I/System.out: ");
            }
            l.push_str(s);
            if echo {
                if fd == 1 {
                    print!("{s}");
                } else {
                    eprint!("{s}");
                }
            }
        });
        if echo {
            eprintln!("[idorid] loaded {} in {:?}", manifest.package, t0.elapsed());
        }
        Ok(App {
            vm,
            manifest,
            label,
            log,
            launched: false,
        })
    }

    fn call(&mut self, name_sig: &str, args: &[CallArg]) -> String {
        let mut raw = Vec::with_capacity(args.len());
        let pins = self.vm.pins.len();
        for a in args {
            match a {
                CallArg::Int(i) => raw.push(*i as u32),
                CallArg::Str(s) => match s {
                    Some(s) => {
                        let r = self.vm.new_string(s);
                        self.vm.pins.push(r);
                        raw.push(r);
                    }
                    None => raw.push(0),
                },
            }
        }
        let r = self
            .vm
            .call_static("Landroid/app/ActivityThread;", name_sig, &raw);
        self.vm.pins.truncate(pins);
        match r {
            Ok(v) => self
                .vm
                .rust_string(v as u32)
                .unwrap_or_else(|| error_json("no UI")),
            Err(Throw) => {
                let e = self.vm.describe_exception();
                self.log.borrow_mut().push_str(&format!("E/idorid: {e}\n"));
                error_json(&e)
            }
        }
    }

    pub fn launch(&mut self) -> String {
        let Some(activity) = self.manifest.main_activity.clone() else {
            return error_json("no launcher activity in AndroidManifest.xml");
        };
        self.launched = true;
        let pkg = self.manifest.package.clone();
        let label = self.label.clone();
        let app_class = self.manifest.application_class.clone();
        self.call(
            "launch(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
            &[CallArg::Str(Some(pkg)), CallArg::Str(Some(label)), CallArg::Str(Some(activity)), CallArg::Str(app_class)],
        )
    }
    pub fn click(&mut self, id: i32) -> String {
        self.call("click(I)Ljava/lang/String;", &[CallArg::Int(id)])
    }
    pub fn long_click(&mut self, id: i32) -> String {
        self.call("longClick(I)Ljava/lang/String;", &[CallArg::Int(id)])
    }
    pub fn set_text(&mut self, id: i32, text: &str) -> String {
        self.call(
            "setText(ILjava/lang/String;)Ljava/lang/String;",
            &[CallArg::Int(id), CallArg::Str(Some(text.into()))],
        )
    }
    pub fn editor_action(&mut self, id: i32) -> String {
        self.call("editorAction(I)Ljava/lang/String;", &[CallArg::Int(id)])
    }
    pub fn set_progress(&mut self, id: i32, p: i32) -> String {
        self.call(
            "setProgress(II)Ljava/lang/String;",
            &[CallArg::Int(id), CallArg::Int(p)],
        )
    }
    pub fn back(&mut self) -> String {
        self.call("back()Ljava/lang/String;", &[])
    }
    pub fn tick(&mut self) -> String {
        self.call("tick()Ljava/lang/String;", &[])
    }
    pub fn ui(&mut self) -> String {
        self.call("ui()Ljava/lang/String;", &[])
    }
    pub fn take_log(&mut self) -> String {
        std::mem::take(&mut *self.log.borrow_mut())
    }
    pub fn info_json(&self) -> String {
        format!(
            "{{\"package\":{},\"label\":{},\"activity\":{}}}",
            json_str(&self.manifest.package),
            json_str(&self.label),
            self.manifest
                .main_activity
                .as_deref()
                .map(json_str)
                .unwrap_or_else(|| "null".into())
        )
    }
}

enum CallArg {
    Int(i32),
    Str(Option<String>),
}

pub fn json_str(s: &str) -> String {
    let mut o = String::with_capacity(s.len() + 2);
    o.push('"');
    for c in s.chars() {
        match c {
            '"' => o.push_str("\\\""),
            '\\' => o.push_str("\\\\"),
            '\n' => o.push_str("\\n"),
            '\r' => o.push_str("\\r"),
            '\t' => o.push_str("\\t"),
            c if (c as u32) < 0x20 => o.push_str(&format!("\\u{:04x}", c as u32)),
            c => o.push(c),
        }
    }
    o.push('"');
    o
}

fn error_json(msg: &str) -> String {
    format!(
        "{{\"title\":\"idorid\",\"finished\":false,\"error\":{},\"next\":-1}}",
        json_str(msg)
    )
}

// ----------------------------------------------------------------------
// C ABI
// ----------------------------------------------------------------------

/// Opaque handle for C.
pub struct IdoridVm(App);

fn to_c(s: String) -> *mut c_char {
    CString::new(s.replace('\0', ""))
        .map(|c| c.into_raw())
        .unwrap_or(std::ptr::null_mut())
}

fn guard(f: impl FnOnce() -> String) -> *mut c_char {
    match catch_unwind(AssertUnwindSafe(f)) {
        Ok(s) => to_c(s),
        Err(p) => {
            let msg = p
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| p.downcast_ref::<&str>().map(|s| s.to_string()))
                .unwrap_or_else(|| "VM panic".into());
            to_c(error_json(&format!("internal error: {msg}")))
        }
    }
}

/// # Safety
/// `apk` must point to `len` readable bytes; `err` may be null.
#[no_mangle]
pub unsafe extern "C" fn idorid_vm_new(
    apk: *const u8,
    len: usize,
    err: *mut *mut c_char,
) -> *mut IdoridVm {
    if apk.is_null() {
        return std::ptr::null_mut();
    }
    let bytes = std::slice::from_raw_parts(apk, len).to_vec();
    match catch_unwind(AssertUnwindSafe(|| App::load(bytes, false))) {
        Ok(Ok(app)) => Box::into_raw(Box::new(IdoridVm(app))),
        Ok(Err(e)) => {
            if !err.is_null() {
                *err = to_c(e);
            }
            std::ptr::null_mut()
        }
        Err(_) => {
            if !err.is_null() {
                *err = to_c("internal error while loading".into());
            }
            std::ptr::null_mut()
        }
    }
}

macro_rules! vm_fn {
    ($(#[$m:meta])* $name:ident ($($arg:ident : $ty:ty),*) => |$app:ident| $body:expr) => {
        $(#[$m])*
        /// # Safety
        /// `vm` must come from `idorid_vm_new` and be used from one thread.
        #[no_mangle]
        pub unsafe extern "C" fn $name(vm: *mut IdoridVm $(, $arg: $ty)*) -> *mut c_char {
            let Some(IdoridVm($app)) = vm.as_mut() else { return std::ptr::null_mut() };
            guard(|| $body)
        }
    };
}

vm_fn!(idorid_vm_launch() => |app| app.launch());
vm_fn!(idorid_vm_click(view_id: i32) => |app| app.click(view_id));
vm_fn!(idorid_vm_long_click(view_id: i32) => |app| app.long_click(view_id));
vm_fn!(idorid_vm_editor_action(view_id: i32) => |app| app.editor_action(view_id));
vm_fn!(idorid_vm_set_progress(view_id: i32, progress: i32) => |app| app.set_progress(view_id, progress));
vm_fn!(idorid_vm_back() => |app| app.back());
vm_fn!(idorid_vm_tick() => |app| app.tick());
vm_fn!(idorid_vm_take_log() => |app| app.take_log());
vm_fn!(idorid_vm_info() => |app| app.info_json());
vm_fn!(idorid_vm_set_text(view_id: i32, utf8: *const c_char) => |app| {
    let text = if utf8.is_null() { String::new() } else { CStr::from_ptr(utf8).to_string_lossy().into_owned() };
    app.set_text(view_id, &text)
});

/// # Safety
/// `s` must come from an idorid function (or be null).
#[no_mangle]
pub unsafe extern "C" fn idorid_string_free(s: *mut c_char) {
    if !s.is_null() {
        drop(CString::from_raw(s));
    }
}

/// # Safety
/// `vm` must come from `idorid_vm_new` (or be null); it is invalid afterwards.
#[no_mangle]
pub unsafe extern "C" fn idorid_vm_free(vm: *mut IdoridVm) {
    if !vm.is_null() {
        drop(Box::from_raw(vm));
    }
}
