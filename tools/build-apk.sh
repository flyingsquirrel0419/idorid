#!/usr/bin/env bash
# Builds a plain Android app (res/, src/, AndroidManifest.xml) into an APK
# with the stock SDK tools: tools/build-apk.sh <app-dir> <out.apk>
set -euo pipefail
app="$(cd "$1" && pwd)"
out="$(cd "$(dirname "$2")" && pwd)/$(basename "$2")"
sdk="${ANDROID_HOME:-$HOME/android-sdk}"
build="$(ls -d "$sdk"/build-tools/*/ | sort -V | tail -1)"
jar="$(ls -d "$sdk"/platforms/android-*/ | sort -V | tail -1)android.jar"
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT
mkdir -p "$tmp/compiled" "$tmp/gen" "$tmp/classes" "$tmp/dex" "$(dirname "$out")"
"$build/aapt2" compile --dir "$app/res" -o "$tmp/compiled"
"$build/aapt2" link -I "$jar" --manifest "$app/AndroidManifest.xml" --java "$tmp/gen" -o "$tmp/app.apk" "$tmp/compiled"/*.flat
extra=()
if [ -n "$(find "$app/src" -name '*.kt' -print -quit)" ]; then
  # Kotlin sources: compile with kotlinc (Java sources visible for mixed projects)
  # and dex the Kotlin stdlib into the app, like a Gradle debug build does.
  kotlinc="${KOTLINC:-$( [ -x "$HOME/tools/kotlinc/bin/kotlinc" ] && echo "$HOME/tools/kotlinc/bin/kotlinc" || command -v kotlinc)}"
  kotlinc="$(readlink -f "$kotlinc")"
  stdlib="$(dirname "$kotlinc")/../lib/kotlin-stdlib.jar"
  "$kotlinc" -jvm-target 1.8 -nowarn -classpath "$jar" -d "$tmp/classes" \
    $(find "$app/src" "$tmp/gen" -name '*.kt' -o -name '*.java') 2>&1 | grep -v '^warning' || true
  extra=("$stdlib")
fi
if [ -n "$(find "$app/src" -name '*.java' -print -quit)" ]; then
  find "$app/src" "$tmp/gen" -name '*.java' -print0 | xargs -0 javac --release 8 -nowarn -classpath "$jar:$tmp/classes" -d "$tmp/classes"
else
  find "$tmp/gen" -name '*.java' -print0 | xargs -0 javac --release 8 -nowarn -classpath "$jar" -d "$tmp/classes"
fi
find "$tmp/classes" -name '*.class' -print0 | xargs -0 "$build/d8" --lib "$jar" --min-api 21 --output "$tmp/dex" "${extra[@]}" > /dev/null
cp "$tmp/app.apk" "$out"
(cd "$tmp/dex" && zip -q "$out" classes*.dex)
echo "built $out ($(wc -c < "$out") bytes)"
