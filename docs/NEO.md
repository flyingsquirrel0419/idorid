# idorid neo: booting real Android on iOS without JIT

idorid's original path (`crates/vm` + `core/`) runs APKs on our own
re-implementation of the Android framework. **neo** is the second path: it boots
an unmodified **AOSP aarch64 userspace** (init → servicemanager → zygote/ART →
system_server → SurfaceFlinger → launcher) inside the iOS app process.

## Architecture

```
 iOS app (Swift)  ── Metal/CALayer display, touch, audio, files
        │  C ABI
 crates/kernel    ── "Linux in a library": processes/threads, VFS over the ROM,
        │            mmap, futex, signals, epoll, sockets, binder, ashmem,
        │            property service, /proc, /dev/graphics, input devices
 crates/a64       ── AArch64 EL0 CPU interpreter + software MMU (no JIT)
 (later) AOT      ── build-time translation of the fixed ROM's code to host code
```

- **No kernel is emulated at the hardware level.** Guest Linux syscalls (`svc`)
  exit the CPU and are implemented in Rust (user-mode emulation, like
  qemu-user, but with every Android device node implemented in-process).
- **Memory.** A software MMU with 4 KiB guest pages (iOS uses 16 KiB pages, so
  host mmap cannot be used directly). Pages are host buffers. Each guest thread
  caches translations in a direct-mapped TLB; mapping changes bump a
  generation counter.
- **Processes.** Guest processes and threads live in the one iOS process. Each
  guest thread runs on its own host thread. `fork()` shares pages copy-on-write
  (zygote forks every app).
- **No JIT.** The interpreter decodes and executes guest instructions. Speed
  comes from block caching first. Later, because the ROM is fixed, its
  `.so`/`.oat` code is translated to host code at build time and linked into
  the signed app binary. ART runs with its own AOT-compiled boot image, so
  most Java executes as (translated) native code, not double interpretation.
- **Display.** First milestone: software rendering (SwiftShader/`gralloc`
  in the guest) into a framebuffer the host shows as a texture. Later:
  virtio-gpu-style command streaming to Metal.

## Milestones

1. `crates/a64`: run static aarch64 Linux binaries; differential tests against
   `qemu-aarch64-static` (`cargo test -p idorid-a64`).
2. `crates/kernel`: dynamic ELF (the guest's own `linker64`), files, threads,
   futex, signals, epoll; run AOSP command-line binaries (`toybox`, `sh`).
3. binder, ashmem, properties; `servicemanager`, then `init second_stage`.
4. zygote + system_server boot; SurfaceFlinger to a host framebuffer.
5. Launcher on screen, touch input; then AOT and GPU.

## Status (2026-11-19)

The unmodified Android 13 emulator ROM boots on our CPU and kernel up to
`system_server`:

- `init second_stage`: properties, SELinux (permissive emulation with real
  file labels), ~120 rc files, ueventd, logd, lmkd;
- servicemanager, hwservicemanager, vold, keystore2, keymint, ~40 HALs;
- zygote: ART initializes, preloads classes, resources and libraries, and
  forks `system_server`, which starts its bootstrap services;
- netd runs on our bpf(2) emulation;
- **graphics:** SurfaceFlinger renders with ANGLE on Vulkan through
  virtio-gpu + gfxstream (host: lavapipe on Linux, MoltenVK on iOS), the
  ranchu HWC composes, and the boot animation runs:

  <img src="img/neo-bootanim.png" width="240" alt="Android 13 boot animation on idorid neo">

Next: system_server past the boot phases to SystemUI and the launcher.

Speed today: ~40 M guest instructions/s (interpreter with a decoded block
cache). In progress: build-time AOT of profile-selected hot ROM pages.

`idorid neo-install <rom.zip> <dir>` installs a Google system image (sparse
images, super partition, ext4, APEX flattening, SELinux labels) in pure Rust;
the iOS app uses the same code to install Android on the device.

## Running

```
tools/fetch-rom.sh                       # ~/aosp/rom13 (once)
cargo build --profile fast -p idorid-cli
./target/fast/idorid neo -- /system/bin/sh -c 'echo hi; ls /system'
IDORID_LOGCAT=1 ./target/fast/idorid neo -- /system/bin/init second_stage
```

Debug switches: `kill -USR1 <idorid pid>` (dump every guest thread's
syscall, fd and backtrace), `IDORID_GPU_DEBUG=1` (fence/sync tracing),
`IDORID_FB_DUMP=out.png` (write composed frames), `IDORID_LOGCAT=1` (mirror logd to stderr),
`IDORID_STRACE=1` (syscalls), `IDORID_TRACE_FAULTS=1` (registers and
file+offset backtraces for guest faults), `IDORID_PROFILE=out.txt` (hot code
pages, input for AOT).

`rom-overlay/` holds our patches to the ROM (layered over it read-only):
64-bit-only zygote, flattened APEX, an fstab without logical partitions, and
stubs for apexd and iptables.

## ROM

Development uses an AOSP arm64 system image unpacked to a directory
(`tools/fetch-rom.sh`). The ROM is not committed to this repository.
