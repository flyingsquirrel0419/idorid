# idorid — plan

Goal: run Android apps (APK) on a stock iPhone **without JIT**: no executable
memory, no dynamic codegen. Everything that executes is either code signed in
the IPA or *data* interpreted by it. Written in Rust, tuned for fast startup and
low memory.

## Hard constraints (iOS)

- No `MAP_JIT`, no RWX pages, no runtime-generated machine code.
- So: Dalvik bytecode is **interpreted**; native `.so` (ARM64 ELF + Bionic +
  JNI) **cannot be loaded**. Apps that need native libraries are out of scope
  for now (future: build-time static translation or an ARM64 interpreter).
- We do not run AOSP `framework.jar`. We ship a small, hand-written Android
  framework in Java (`core/`), compiled to `core.dex` and embedded in the
  binary. Its UI calls bridge to the host (UIKit on iOS, headless on Linux).

## Architecture

```
APK ──zip──▶ classes*.dex (mmap/borrowed)   AndroidManifest.xml (AXML)
                   │                               │ launcher activity
                   ▼                               ▼
     idorid-dex: zero-copy parser      idorid-apk: zip + AXML + arsc
                   │
                   ▼
     idorid-vm: lazy class linking, switch interpreter over 32-bit
                register slots, 32-bit object handles, precise-heap /
                conservative-frame mark-sweep GC, Rust natives
                   │  core.dex (java.lang / java.util / android.* shim)
                   ▼
     UI tree (view id, type, props)  ──▶ idorid-ffi (C ABI) ──▶ iOS UIKit host
                                     └─▶ idorid-cli (headless, tests)
```

Crates:

| crate | role |
|---|---|
| `idorid-dex` | DEX parser: header, ids, class data, code items, try/catch, encoded values, MUTF-8 |
| `idorid-vm` | interpreter, heap/GC, class loader, natives, UI bridge |
| `idorid-apk` | APK zip reading, binary XML (AndroidManifest/layouts), resources.arsc (strings/layouts) |
| `idorid-ffi` | C ABI for the iOS host (`include/idorid.h`) |
| `idorid-cli` | `idorid run app.apk` / `idorid dex file.dex` headless runner |

## Memory / speed choices

- Object references are `u32` handles into an object table (like compressed
  oops): 4-byte refs everywhere, no raw pointers across FFI, GC can stay simple.
- Registers are `u32` slots in one contiguous per-thread stack; wide values use
  pairs, exactly like Dalvik.
- Strings: compact Latin-1 when possible, UTF-16 otherwise.
- Classes are linked lazily on first use; methods are never pre-decoded until
  profiling says it pays off.
- DEX is borrowed from the APK buffer (stored entry) or decompressed once.

## FFI contract (iOS host ⇄ Rust)

```c
typedef struct IdoridVm IdoridVm;
// Load an APK. On failure returns NULL and sets *err (free with idorid_string_free).
IdoridVm *idorid_vm_new(const uint8_t *apk, size_t len, char **err);
// Instantiate the launcher activity and run onCreate/onStart/onResume.
// Returns the UI JSON (see below). Must be freed.
char *idorid_vm_launch(IdoridVm *vm);
// Deliver a click to the view with this id; returns the new UI JSON.
char *idorid_vm_click(IdoridVm *vm, int32_t view_id);
// Text changed in an EditText; returns new UI JSON.
char *idorid_vm_set_text(IdoridVm *vm, int32_t view_id, const char *utf8);
// Back button. Returns new UI JSON ("finished": true when activity finished).
char *idorid_vm_back(IdoridVm *vm);
// Drain logcat-style output (android.util.Log + System.out).
char *idorid_vm_take_log(IdoridVm *vm);
// App label + package from the manifest, as JSON {"package":..,"label":..,"activity":..}.
char *idorid_vm_info(IdoridVm *vm);
void idorid_string_free(char *s);
void idorid_vm_free(IdoridVm *vm);
```

All calls for one VM must come from the same thread (host uses a dedicated
thread with a large stack, 16 MB).

UI JSON:

```json
{"title":"My App","finished":false,"error":null,
 "root":{"id":1,"type":"LinearLayout","orientation":"vertical",
   "padding":[l,t,r,b],"bg":"#ffffffff","visible":true,
   "children":[
     {"id":2,"type":"TextView","text":"Hello","textSize":18,"color":"#ff000000","gravity":"center"},
     {"id":3,"type":"Button","text":"Tap me","clickable":true},
     {"id":4,"type":"EditText","text":"","hint":"name"},
     {"id":5,"type":"ImageView"}]}}
```

Types: `LinearLayout`, `FrameLayout`, `ScrollView`, `TextView`, `Button`,
`EditText`, `ImageView`, `View`. Unknown props are ignored by the host.
Layout params: `"width"`/`"height"`: `"match"`, `"wrap"`, or dp number;
`"weight"`: number.

## Milestones

1. DEX parser + CLI dump. ✅ test: parse every dex produced by d8.
2. Interpreter: arithmetic, branches, calls, arrays, strings, exceptions,
   virtual/interface dispatch, statics/<clinit>. Oracle test: the same Java
   program run on the JVM and on idorid must print the same thing.
3. Core library (java.lang, java.util collections, StringBuilder, boxing).
4. Android shim: Activity lifecycle, View tree, OnClickListener, Log, Toast,
   programmatic UI; then layout XML inflation + string resources.
5. iOS host app + GitHub Actions unsigned IPA.
6. Perf: inline caches, pre-decoded handlers, boot image; measure.
7. Compatibility growth: Kotlin stdlib apps, more widgets, Canvas.
