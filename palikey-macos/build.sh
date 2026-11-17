#!/usr/bin/env bash
#
# build.sh — Build script for PaliKey macOS InputMethodKit app + smoke tests.
#
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
BUILD_DIR="$SCRIPT_DIR/build"
APP_BUNDLE="$BUILD_DIR/PaliKey.app"

echo "============================================================"
echo " Building PaliKey (macOS InputMethodKit Shell)"
echo " Platform: $(uname -m), $(sw_vers -productName) $(sw_vers -productVersion)"
echo "============================================================"

# Ensure Rust & Cargo in PATH
export PATH="$HOME/.cargo/bin:/usr/local/bin:/usr/bin:/bin:$PATH"

if ! command -v cargo >/dev/null 2>&1; then
    echo "ERROR: cargo not found in PATH." >&2
    exit 1
fi

if ! command -v swiftc >/dev/null 2>&1; then
    echo "ERROR: swiftc not found in PATH." >&2
    exit 1
fi

SDK_PATH="$(xcrun --show-sdk-path)"
echo "Using SDK: $SDK_PATH"

# 1. Build Rust engine staticlib
echo ""
echo "--> [1/6] Building palikey-engine (cargo build --release)..."
(cd "$REPO_ROOT/palikey-engine" && cargo build --release)

# 2. Verify engine cargo tests
echo ""
echo "--> [2/6] Running cargo test in palikey-engine..."
(cd "$REPO_ROOT/palikey-engine" && cargo test)

# 3. Assemble bundle directories
echo ""
echo "--> [3/6] Assembling bundle structure..."
rm -rf "$BUILD_DIR"
mkdir -p "$APP_BUNDLE/Contents/MacOS"
mkdir -p "$APP_BUNDLE/Contents/Resources"

# Copy Info.plist and verify
cp "$SCRIPT_DIR/Info.plist" "$APP_BUNDLE/Contents/Info.plist"
plutil -lint "$APP_BUNDLE/Contents/Info.plist"

# Copy icon if available or generate placeholder
if [ -f "$SCRIPT_DIR/PaliKey.tiff" ]; then
    cp "$SCRIPT_DIR/PaliKey.tiff" "$APP_BUNDLE/Contents/Resources/PaliKey.tiff"
fi

# 4. Compile Swift IMK application binary
echo ""
echo "--> [4/6] Compiling PaliKey Swift sources with swiftc..."
swiftc \
    -O \
    -target x86_64-apple-macos15.0 \
    -sdk "$SDK_PATH" \
    -framework Cocoa \
    -framework InputMethodKit \
    -framework Carbon \
    -import-objc-header "$REPO_ROOT/palikey-engine/include/palikey.h" \
    -I "$REPO_ROOT/palikey-engine/include" \
    -L "$REPO_ROOT/target/release" \
    -lpalikey_engine \
    "$SCRIPT_DIR/PaliEngine.swift" \
    "$SCRIPT_DIR/PaliConfig.swift" \
    "$SCRIPT_DIR/EasyUnicode.swift" \
    "$SCRIPT_DIR/PaliKeyInputController.swift" \
    "$SCRIPT_DIR/main.swift" \
    -o "$APP_BUNDLE/Contents/MacOS/PaliKey"

# Ad-hoc codesign
echo "Ad-hoc codesigning bundle..."
codesign --force --deep --sign - "$APP_BUNDLE"
codesign --verify --deep --strict "$APP_BUNDLE"

# 5. Build and execute headless smoke test
echo ""
echo "--> [5/6] Building and running smoke test harness..."
swiftc \
    -O \
    -target x86_64-apple-macos15.0 \
    -sdk "$SDK_PATH" \
    -framework Foundation \
    -import-objc-header "$REPO_ROOT/palikey-engine/include/palikey.h" \
    -I "$REPO_ROOT/palikey-engine/include" \
    -L "$REPO_ROOT/target/release" \
    -lpalikey_engine \
    "$SCRIPT_DIR/PaliEngine.swift" \
    "$SCRIPT_DIR/EasyUnicode.swift" \
    "$SCRIPT_DIR/smoke_test.swift" \
    -o "$BUILD_DIR/smoke_test"

"$BUILD_DIR/smoke_test"

# 6. Bundle verification
echo ""
echo "--> [6/6] Verifying bundle structure..."
file "$APP_BUNDLE/Contents/MacOS/PaliKey"
plutil -lint "$APP_BUNDLE/Contents/Info.plist"

echo ""
echo "============================================================"
echo " PaliKey macOS Build & Verification: SUCCESS (Exit 0)"
echo " Output: $APP_BUNDLE"
echo "============================================================"
