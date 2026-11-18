#!/usr/bin/env bash
# Compiles core/src (our own closed-world java.* + android.* shim) into
# crates/vm/core.dex, which is embedded in the VM binary.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SDK="${ANDROID_HOME:-${ANDROID_SDK_ROOT:-$HOME/android-sdk}}"
D8="$(ls -d "$SDK"/build-tools/*/ | sort -V | tail -1)d8"
mkdir -p "$ROOT/build"
OUT="$(mktemp -d "$ROOT/build/core.XXXXXX")"
trap 'rm -rf "$OUT"' EXIT
mkdir -p "$OUT/classes" "$OUT/dex"
find "$ROOT/core/src" -name '*.java' > "$OUT/sources.txt"
# Empty bootclasspath: core must be self-contained.
javac -source 8 -target 8 -bootclasspath "" -extdirs "" -Xlint:-options -nowarn \
  -encoding UTF-8 -d "$OUT/classes" @"$OUT/sources.txt"
(cd "$OUT/classes" && find . -name '*.class' > ../classes.txt)
(cd "$OUT/classes" && "$D8" --release --min-api 24 --no-desugaring --output "$OUT/dex" @../classes.txt)
cp "$OUT/dex/classes.dex" "$ROOT/crates/vm/core.dex.tmp.$$" && mv "$ROOT/crates/vm/core.dex.tmp.$$" "$ROOT/crates/vm/core.dex"
echo "core.dex: $(stat -c %s "$ROOT/crates/vm/core.dex" 2>/dev/null || stat -f %z "$ROOT/crates/vm/core.dex") bytes"
