<div align="center">

# idorid

**Run Android apps on a stock iPhone. No JIT, no emulator, no jailbreak.**

A Dalvik bytecode interpreter and a small Android framework, written in Rust,
rendering real Android apps as native UIKit views.

[![CI](https://github.com/flyingsquirrel0419/idorid/actions/workflows/ci.yml/badge.svg)](https://github.com/flyingsquirrel0419/idorid/actions/workflows/ci.yml)
[![iOS IPA](https://github.com/flyingsquirrel0419/idorid/actions/workflows/ios.yml/badge.svg)](https://github.com/flyingsquirrel0419/idorid/actions/workflows/ios.yml)
![License](https://img.shields.io/badge/license-Apache--2.0-blue)

</div>

---

```text
$ idorid run examples/todo.apk
[idorid] loaded dev.idorid.todo in 1.0ms
== Todo (Android) ==
#1 FrameLayout
  #2 LinearLayout
    #3 TextView "Todo (Android)"
    #4 TextView "2 open · running for 0s"
    #5 LinearLayout
      #6 EditText "" hint="What needs doing?"
      #7 Button "Add"
    #8 LinearLayout
      #9 Spinner
      #10 CheckBox "Urgent only" checked=false
    #12 ScrollView
        #17 TextView "🔥 Try idorid on an iPhone" [clickable]
        #18 TextView "• Star the repo" [clickable]
    #13 Button "About"
[idorid] launch 7.6ms, 143 classes, 1002 objects, heap 61 KB
```

That is an **unmodified APK** built with the stock Android SDK (`aapt2` + `d8`):
its `classes.dex` is interpreted, its `resources.arsc` and binary layout XML are
parsed, and its `Activity` goes through the real lifecycle. On iOS the same view
tree becomes native `UILabel`s, `UIButton`s and `UITextField`s placed at the
frames computed by Android's own measure/layout algorithm.

**New in 0.2: real AndroidX apps.** The Android Studio *Empty Views Activity*
template, built by Gradle with R8 (`AppCompatActivity`, Material 3 theme,
`MaterialButton`, `ConstraintLayout`), runs unmodified: the AndroidX and
Material Components code inside the APK is executed as-is, themes and styles
resolve through `resources.arsc`, and `ConstraintLayout` from the APK lays the
screen out. Kotlin apps work too (the Kotlin stdlib is just more bytecode).

## Why this is different

iOS does not let ordinary apps create executable memory. Every Android-on-iOS
attempt so far either emulates a whole ARM machine and needs JIT (StikDebug,
debugger tricks, iOS-version roulette) or is not open source.

idorid takes the other road:

| | Full-system emulators | **idorid** |
|---|---|---|
| Needs JIT / debugger attach | usually | **never**: only signed code runs; bytecode is data |
| Boots Android | yes (seconds to minutes, GBs) | **no**: an app starts in milliseconds |
| Memory | hundreds of MB+ | **~12 MB RSS** for the VM + app |
| UI | a framebuffer in a window | **native UIKit views** |
| Runs native `.so` libraries | yes | no (see [limits](#what-works-today)) |

## How it works

```
APK ──zip──▶ classes*.dex (zero-copy)        AndroidManifest.xml / resources.arsc / res/layout/*.xml
                   │                                         │ binary XML (AXML) + resource table
                   ▼                                         ▼
     idorid-dex: zero-copy DEX parser          idorid-apk: launcher activity, strings, layouts
                   │
                   ▼
     idorid-vm:  non-recursive interpreter (all ~220 Dalvik opcodes)
                 · dense jump-table dispatch, monomorphic inline caches
                 · load-time bytecode verifier → unchecked register access
                 · 32-bit object handles, compact Latin-1/UTF-16 strings
                 · precise-heap / conservative-frame mark-sweep GC
                 · lazy class linking, lazy method decoding
                   │  + core.dex: our own java.lang / java.util / android.* (Java, ~360 KB)
                   ▼
     Android measure/layout in Java (real LinearLayout, RelativeLayout, ConstraintLayout from the APK, ...)
                   │
     UI tree + frames (JSON) ──▶ idorid-ffi (C ABI) ──▶ Swift/UIKit host on iPhone
                    └─▶ idorid-cli (headless; used by tests)
```

* **No AOSP framework.** Booting `framework.jar` would drag in thousands of JNI
  natives and system services. Instead `core/src/android` is a compact
  re-implementation of the parts apps actually touch (Activity lifecycle,
  Views, layouts, adapters, dialogs, Handler/Looper, SharedPreferences, …)
  written in plain Java and compiled to `core.dex`, which is embedded in the
  binary.
* **Correctness by differential testing.** Every program in `tests/java` is run
  on the host JVM and on idorid, and the outputs must match byte for byte
  (`tests/oracle.sh`). Real APKs are driven end to end in
  `crates/ffi/tests/apps.rs`.

## What works today

**Runtime:** the whole Dalvik instruction set except `invoke-polymorphic`
(`invoke-custom` lambdas via `LambdaMetafactory` are supported), reflection
(`Constructor`/`Method`/`Field`), Kotlin and its stdlib, exceptions with stack
traces and line numbers, static initializers, interfaces with default methods,
inner/anonymous classes, enums, string switch, multi-dimensional arrays,
`String.format`, a regex subset (`split`, `matches`, `replaceAll`), boxing,
`java.util` collections with JDK-identical iteration order, `java.util.concurrent`
basics, cooperative `Thread`s, `Handler.postDelayed` timers.

**AndroidX:** `AppCompatActivity`, `AppCompatDelegate` (theme checks, sub-decor,
view inflation factory), Material 3 themes and `MaterialButton` (shape
backgrounds drawn through `Canvas`), `ConstraintLayout`, `RecyclerView`
(scrolling and recycling through Android's own touch dispatch),
`MaterialToolbar` with menus, `TextInputLayout`, `Chip`, `MaterialSwitch`,
`MaterialCardView`, `CoordinatorLayout`, `Snackbar`.

**Framework:** Android's measure/layout pass, themes and styles
(`obtainStyledAttributes`, `TypedArray`, `?attr`, `textAppearance`,
configuration-qualified resources, framework resources), drawable XML
(shape, selector, layer-list, inset, ripple, vector), `Canvas` custom views
with touch, real touch dispatch (`onInterceptTouchEvent`), `ValueAnimator`,
`java.math`, `java.text`.

**Android:** `Activity` (lifecycle, `startActivity` with extras,
`startActivityForResult`, back stack), layout XML inflation (`<include>`,
`<merge>`, `android:onClick`), `LinearLayout`, `FrameLayout`, `RelativeLayout`
(approximate), `ScrollView`, `TextView`, `Button`, `EditText`, `ImageView`,
`CheckBox`, `Switch`, `RadioButton`/`RadioGroup`, `ProgressBar`, `SeekBar`,
`Spinner`, `ListView`/`GridView` + `ArrayAdapter`/`BaseAdapter`, `AlertDialog`,
`Toast`, `Log`, string/color/dimen resources, `SharedPreferences` (in memory).

**Not yet:** native `.so` libraries (JNI), Jetpack Compose, networking,
files, databases, sound, sensors, real threads (threads run cooperatively),
WebView. Apps that need these will start and then hit a clear error. See the
[roadmap](#roadmap). `idorid missing app.apk` lists every framework API an app
references that idorid does not provide yet.

Native libraries are the hard wall: they are ARM64 ELF code, and iOS will only
execute code signed into the app. Future options are build-time static
translation into the IPA, or an ARM64 interpreter. Neither is a weekend project.

## Install on iPhone

1. Download `idorid.ipa` from the latest
   [release](https://github.com/flyingsquirrel0419/idorid/releases) (or from the
   artifacts of the latest [iOS workflow run](https://github.com/flyingsquirrel0419/idorid/actions/workflows/ios.yml)).
2. Sideload it with SideStore, AltStore, Sideloadly or TrollStore. It is
   unsigned; your tool signs it with your Apple ID. No JIT enabler is needed.
3. Open idorid, tap **+**, pick an `.apk` from Files (or share an APK to
   idorid), and tap it to run.

The app ships with demos: Todo (views, dialogs, preferences), Paint (Canvas
custom view, touch, animation), Calc (Kotlin), Hello AndroidX (the Android
Studio template), Planets (AndroidX showcase: RecyclerView, MaterialToolbar,
TextInputLayout, chips, switches, cards, a second activity).

## Build

Requirements: Rust (stable), JDK 17+, Android SDK build-tools (for `d8` and
`aapt2`; only needed to rebuild `core.dex` or test APKs).

```sh
cargo build --release                                   # VM, CLI, C library
./target/release/idorid run examples/todo.apk           # interactive: click <id>, type <id> <text>, back, log
./target/release/idorid main app.dex com.example.Main   # run a plain main()

tools/build-core.sh                                     # rebuild crates/vm/core.dex from core/src
tools/build-apk.sh examples/todo examples/todo.apk      # build an example APK with the stock SDK
tests/oracle.sh                                         # JVM vs idorid differential tests
cargo test --workspace

ios/build-ipa.sh                                        # on macOS: build/idorid.ipa (unsigned)
```

## Performance

Micro-benchmarks (`tests/bench/run.sh`, x86-64 Linux). The comparison is
HotSpot's `-Xint` mode, which is a template interpreter generated as machine
code at startup, a technique iOS forbids:

| benchmark | idorid | JVM `-Xint` |
|---|---:|---:|
| `fib(27)` | 35 ms | 15 ms |
| 10M-iteration arithmetic loop | 301 ms | 150 ms |
| sieve, 2M | 194 ms | 110 ms |
| 1M `new int[8]` | 96 ms | 76 ms |
| 5M interface calls | 478 ms | 183 ms |
| **peak RSS** | **12 MB** | 66 MB |

App launch (load APK, link classes, run `onCreate`, produce the UI) is 3–8 ms
for the examples.

## Roadmap

- [x] Canvas / custom views, touch dispatch, vector drawables
- [x] Kotlin apps, `invoke-custom` lambdas, reflection
- [x] Android measure/layout, themes/styles, AndroidX AppCompat + Material 3 + ConstraintLayout
- [x] RecyclerView, MaterialToolbar menus, TextInputLayout, CoordinatorLayout
- [ ] Fragments, Navigation, ViewPager2, Jetpack Compose
- [ ] Faster interpreter: pre-decoded handlers, field-offset quickening, string fast paths
- [ ] Green threads (the non-recursive interpreter is already built for it)
- [ ] Networking (`HttpURLConnection`/OkHttp) over `URLSession`, files, SQLite
- [ ] Research: static ARM64 → signed-code translation for JNI libraries

## Project layout

| path | what |
|---|---|
| `crates/dex` | DEX parser |
| `crates/vm` | interpreter, heap/GC, class loader, natives |
| `crates/apk` | APK, binary XML, resources.arsc |
| `crates/ffi` | `App` driver + C ABI (`include/idorid.h`) |
| `crates/cli` | `idorid` command-line runner |
| `core/src` | Java class library + Android framework shim → `core.dex` |
| `ios/` | Swift UIKit host and IPA build script |
| `tests/java` | differential tests, `tests/bench` benchmarks |
| `examples/` | demo apps (plain Android SDK projects) |

## Contributing

The fastest way to help is to **try an APK and open an issue with the log**
(the Log button in the app, or `idorid run app.apk` then `log`). Missing
framework APIs are usually small, self-contained Java additions in
`core/src/android`.

## Credits

Built by a human with Claude Code and OpenAI Codex (`gpt-6.1-terra`) working as
pair programmers. idorid re-implements Android APIs from their public
documentation. The only AOSP-derived data is a trimmed copy of the framework
resource table (`crates/apk/framework-res.bin`, Apache-2.0), used to resolve
`android:` themes and attributes. Not affiliated with Google or Apple. Android is a trademark
of Google LLC.

## License

Apache-2.0
