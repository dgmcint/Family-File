#!/usr/bin/env bash
# Builds "dist/Family File.app" as one app that runs on both Apple Silicon and Intel Macs.
# Add --zip to also make dist/Family-File-macOS.zip for a GitHub release.
set -euo pipefail
cd "$(dirname "$0")/.."

VERSION=$(grep -m1 '^version' Cargo.toml | cut -d'"' -f2)
APP="dist/Family File.app"

rustup target add aarch64-apple-darwin x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin
cargo build --release --target x86_64-apple-darwin

rm -rf "$APP"
mkdir -p "$APP/Contents/MacOS" "$APP/Contents/Resources"
lipo -create -output "$APP/Contents/MacOS/family-file" \
  target/aarch64-apple-darwin/release/family-file \
  target/x86_64-apple-darwin/release/family-file
cp assets/icon.icns "$APP/Contents/Resources/icon.icns"
sed "s/@VERSION@/$VERSION/g" packaging/macos/Info.plist > "$APP/Contents/Info.plist"

# Ad-hoc signature: required for the app to run on Apple Silicon.
codesign --force --deep --sign - "$APP"
echo "Built $APP (version $VERSION)"

if [[ "${1:-}" == "--zip" ]]; then
  rm -f dist/Family-File-macOS.zip
  ditto -c -k --keepParent "$APP" dist/Family-File-macOS.zip
  echo "Built dist/Family-File-macOS.zip"
fi
