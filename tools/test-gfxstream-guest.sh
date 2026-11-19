#!/usr/bin/env bash
# End-to-end arm64 DRM smoke test on lavapipe; requires aarch64-linux-gnu-gcc.
set -euo pipefail
repo=$(cd "$(dirname "$0")/.." && pwd)
work=$(mktemp -d "${TMPDIR:-/tmp}/idorid-gfx-test.XXXXXX")
trap 'rm -rf "$work"' EXIT
mkdir -p "$work/rom/bin" "$work/state"
aarch64-linux-gnu-gcc -O2 -static "$repo/crates/kernel/tests/c/virtgpu-real.c" -o "$work/rom/bin/guest"
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-"$repo/target/gfx"}
cargo build --manifest-path "$repo/Cargo.toml" --profile fast -j2 -p idorid-cli
export IDORID_GFXSTREAM=${IDORID_GFXSTREAM:-"$HOME/gfx/out/lib/libgfxstream_backend.so"}
export VK_ICD_FILENAMES=${VK_ICD_FILENAMES:-/usr/share/vulkan/icd.d/lvp_icd.json}
"$CARGO_TARGET_DIR/fast/idorid" neo --rom "$work/rom" --state "$work/state" -- /bin/guest > "$work/log" 2>&1 || { cat "$work/log"; exit 1; }
cat "$work/log"
grep -q 'GPU_CONTEXT_BLOB_OK' "$work/log"
grep -q 'GPU_TRANSFER_OK' "$work/log"
grep -q 'GPU_PRIME_RESOURCE_FENCE_OK' "$work/log"
