#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
sdk="${ANDROID_HOME:-$HOME/android-sdk}"
build="$sdk/build-tools/35.0.0"
jar="$sdk/platforms/android-35/android.jar"
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT
mkdir -p "$tmp/compiled" "$tmp/gen" "$tmp/classes" "$tmp/dex" "$root/tests/fixtures"
"$build/aapt2" compile --dir "$root/tests/sample-app/res" -o "$tmp/compiled"
"$build/aapt2" link -I "$jar" --manifest "$root/tests/sample-app/AndroidManifest.xml" --java "$tmp/gen" -o "$tmp/app.apk" "$tmp/compiled"/*.flat
find "$root/tests/sample-app/src" "$tmp/gen" -name '*.java' -print0 | xargs -0 javac --release 8 -classpath "$jar" -d "$tmp/classes"
find "$tmp/classes" -name '*.class' -print0 | xargs -0 "$build/d8" --lib "$jar" --min-api 21 --output "$tmp/dex"
cp "$tmp/app.apk" "$root/tests/fixtures/sample.apk"
(cd "$tmp/dex" && zip -q "$root/tests/fixtures/sample.apk" classes*.dex)
