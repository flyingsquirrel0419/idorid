#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."

if [[ "${SIMULATOR:-0}" == "1" ]]; then
  rust_target=aarch64-apple-ios-sim
  sdk=iphonesimulator
  swift_target=arm64-apple-ios15.0-simulator
  app=build/sim/idorid.app
else
  rust_target=aarch64-apple-ios
  sdk=iphoneos
  swift_target=arm64-apple-ios15.0
  app=build/Payload/idorid.app
  rm -rf build/Payload
fi

cargo build --profile release-ffi -p idorid-ffi --target "$rust_target"
rm -rf "$app"
mkdir -p "$app"
xcrun -sdk "$sdk" swiftc -sdk "$(xcrun --sdk "$sdk" --show-sdk-path)" \
  -target "$swift_target" -O -import-objc-header ios/Bridging-Header.h \
  -L "target/$rust_target/release-ffi" -lidorid_ffi \
  -framework UIKit -framework UniformTypeIdentifiers -framework Security -framework SystemConfiguration -lc++ \
  ios/Sources/*.swift -o "$app/idorid"
cp ios/Info.plist "$app/Info.plist"
mkdir -p "$app/Examples"
cp examples/*.apk "$app/Examples/" 2>/dev/null || true
cp tests/fixtures/sample.apk "$app/Examples/Hello Sample.apk" 2>/dev/null || true
cp tests/fixtures/androidx-hello-release.apk "$app/Examples/Hello AndroidX.apk" 2>/dev/null || true
printf 'APPL????' > "$app/PkgInfo"
if [[ "${SIMULATOR:-0}" != "1" ]]; then
  rm -f build/idorid.ipa
  (cd build && zip -qry idorid.ipa Payload)
fi
