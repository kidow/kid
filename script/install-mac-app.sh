#!/usr/bin/env bash
#
# Build a local Kid.app and install it to /Applications (macOS, Apple Silicon).
# Ad-hoc signed (unsigned dev build) — on first launch, right-click > Open once.
#
# Usage:  script/install-mac-app.sh           # release build + install
#         SKIP_BUILD=1 script/install-mac-app.sh   # reuse existing target/release/zed
#
set -euo pipefail
cd "$(dirname "$0")/.."

APP="/Applications/Kid.app"
ICON_SRC="crates/zed/resources/app-icon@2x.png"

if [[ "${SKIP_BUILD:-0}" != "1" ]]; then
  echo "==> Building release binary (cargo build --release -p zed)..."
  cargo build --release -p zed
fi
[[ -f target/release/kid ]] || { echo "target/release/kid not found — build first"; exit 1; }

echo "==> Generating icon..."
work="$(mktemp -d)"
iset="$work/Kid.iconset"; mkdir -p "$iset"
for s in 16 32 128 256 512; do
  sips -z "$s" "$s" "$ICON_SRC" --out "$iset/icon_${s}x${s}.png" >/dev/null
  d=$((s * 2))
  sips -z "$d" "$d" "$ICON_SRC" --out "$iset/icon_${s}x${s}@2x.png" >/dev/null
done
iconutil -c icns "$iset" -o "$work/kid.icns"

echo "==> Assembling $APP..."
rm -rf "$APP"
mkdir -p "$APP/Contents/MacOS" "$APP/Contents/Resources"
cp target/release/kid "$APP/Contents/MacOS/kid"
cp "$work/kid.icns" "$APP/Contents/Resources/kid.icns"

cat > "$APP/Contents/Info.plist" <<'PLIST'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleName</key><string>Kid</string>
    <key>CFBundleDisplayName</key><string>Kid</string>
    <key>CFBundleIdentifier</key><string>dev.kid.Kid</string>
    <key>CFBundleExecutable</key><string>kid</string>
    <key>CFBundleIconFile</key><string>kid.icns</string>
    <key>CFBundlePackageType</key><string>APPL</string>
    <key>CFBundleShortVersionString</key><string>1.6.0</string>
    <key>CFBundleVersion</key><string>1.6.0</string>
    <key>LSMinimumSystemVersion</key><string>10.15.7</string>
    <key>NSHighResolutionCapable</key><true/>
    <key>CFBundleURLTypes</key>
    <array><dict>
        <key>CFBundleURLName</key><string>dev.kid.Kid</string>
        <key>CFBundleURLSchemes</key><array><string>kid</string></array>
    </dict></array>
</dict>
</plist>
PLIST

echo "==> Signing (ad-hoc) + registering..."
xattr -dr com.apple.quarantine "$APP" 2>/dev/null || true
codesign --force --deep --sign - "$APP"
LSREG="/System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/LaunchServices.framework/Versions/A/Support/lsregister"
"$LSREG" -f "$APP"
rm -rf "$work"

echo "==> Done. Open 'Kid' from Spotlight / Launchpad / Applications."
echo "    First launch: right-click Kid.app > Open (unsigned dev build)."
