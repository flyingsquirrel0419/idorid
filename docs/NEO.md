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

## ROM

Development uses an AOSP arm64 system image unpacked to a directory
(`tools/fetch-rom.sh`). The ROM is not committed to this repository.
