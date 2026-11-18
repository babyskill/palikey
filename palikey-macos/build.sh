#!/usr/bin/env bash
#
# build.sh — Build script for PaliKey macOS InputMethodKit app (Universal: arm64 + x86_64) + smoke tests.
#
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
BUILD_DIR="$SCRIPT_DIR/build"
APP_BUNDLE="$BUILD_DIR/PaliKey.app"

echo "============================================================"
echo " Building PaliKey (macOS InputMethodKit Shell - Universal)"
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

# Ensure rustup targets for both architectures are installed
if command -v rustup >/dev/null 2>&1; then
    if ! rustup target list | grep -q "aarch64-apple-darwin (installed)"; then
        echo "Adding rustup target aarch64-apple-darwin..."
        rustup target add aarch64-apple-darwin
    fi
    if ! rustup target list | grep -q "x86_64-apple-darwin (installed)"; then
        echo "Adding rustup target x86_64-apple-darwin..."
        rustup target add x86_64-apple-darwin
    fi
fi

SDK_PATH="$(xcrun --show-sdk-path)"
echo "Using SDK: $SDK_PATH"

# 1. Build Rust engine staticlibs for BOTH targets
echo ""
echo "--> [1/6] Building palikey-engine release staticlibs (x86_64 and aarch64)..."
(cd "$REPO_ROOT/palikey-engine" && cargo build --release --target x86_64-apple-darwin)
(cd "$REPO_ROOT/palikey-engine" && cargo build --release --target aarch64-apple-darwin)

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
mkdir -p "$BUILD_DIR/arch"

# Copy Info.plist and verify
cp "$SCRIPT_DIR/Info.plist" "$APP_BUNDLE/Contents/Info.plist"
plutil -lint "$APP_BUNDLE/Contents/Info.plist"

# Copy icons and resources
if [ -f "$SCRIPT_DIR/PaliKey.tiff" ]; then
    cp "$SCRIPT_DIR/PaliKey.tiff" "$APP_BUNDLE/Contents/Resources/PaliKey.tiff"
fi
if [ -f "$SCRIPT_DIR/PaliKey.icns" ]; then
    cp "$SCRIPT_DIR/PaliKey.icns" "$APP_BUNDLE/Contents/Resources/PaliKey.icns"
fi

# Copy localized resources (InfoPlist.strings, etc.)
if [ -d "$SCRIPT_DIR/Resources" ]; then
    cp -R "$SCRIPT_DIR/Resources/" "$APP_BUNDLE/Contents/Resources/"
fi

# Validate strings files
for strings_file in "$APP_BUNDLE/Contents/Resources"/*/*.strings; do
    if [ -f "$strings_file" ]; then
        plutil -lint "$strings_file"
    fi
done

# 4. Compile Swift IMK application binary for both architectures and lipo merge
echo ""
echo "--> [4/6] Compiling PaliKey Swift sources (arm64 + x86_64) & creating universal binary..."

SWIFT_SOURCES=(
    "$SCRIPT_DIR/PaliEngine.swift"
    "$SCRIPT_DIR/PaliConfig.swift"
    "$SCRIPT_DIR/EasyUnicode.swift"
    "$SCRIPT_DIR/PaliKeyInputController.swift"
    "$SCRIPT_DIR/main.swift"
)

# 4a. Compile x86_64 slice
swiftc \
    -O \
    -target x86_64-apple-macos15.0 \
    -sdk "$SDK_PATH" \
    -framework Cocoa \
    -framework InputMethodKit \
    -framework Carbon \
    -import-objc-header "$REPO_ROOT/palikey-engine/include/palikey.h" \
    -I "$REPO_ROOT/palikey-engine/include" \
    "$REPO_ROOT/target/x86_64-apple-darwin/release/libpalikey_engine.a" \
    "${SWIFT_SOURCES[@]}" \
    -o "$BUILD_DIR/arch/PaliKey_x86_64"

# 4b. Compile arm64 slice
swiftc \
    -O \
    -target arm64-apple-macos15.0 \
    -sdk "$SDK_PATH" \
    -framework Cocoa \
    -framework InputMethodKit \
    -framework Carbon \
    -import-objc-header "$REPO_ROOT/palikey-engine/include/palikey.h" \
    -I "$REPO_ROOT/palikey-engine/include" \
    "$REPO_ROOT/target/aarch64-apple-darwin/release/libpalikey_engine.a" \
    "${SWIFT_SOURCES[@]}" \
    -o "$BUILD_DIR/arch/PaliKey_arm64"

# 4c. Merge into universal binary with lipo
lipo -create \
    "$BUILD_DIR/arch/PaliKey_x86_64" \
    "$BUILD_DIR/arch/PaliKey_arm64" \
    -output "$APP_BUNDLE/Contents/MacOS/PaliKey"

rm -rf "$BUILD_DIR/arch"

# Ad-hoc codesign universal bundle (AFTER lipo)
echo "Ad-hoc codesigning universal bundle..."
codesign --force --deep --sign - "$APP_BUNDLE"
codesign --verify --deep --strict "$APP_BUNDLE"

# 5. Build and execute headless smoke test (host-arch slice)
echo ""
echo "--> [5/6] Building and running smoke test harness (host-arch slice)..."
HOST_ARCH="$(uname -m)"
if [ "$HOST_ARCH" = "arm64" ]; then
    SMOKE_TARGET="arm64-apple-macos15.0"
    SMOKE_ENGINE="$REPO_ROOT/target/aarch64-apple-darwin/release/libpalikey_engine.a"
else
    SMOKE_TARGET="x86_64-apple-macos15.0"
    SMOKE_ENGINE="$REPO_ROOT/target/x86_64-apple-darwin/release/libpalikey_engine.a"
fi

swiftc \
    -O \
    -target "$SMOKE_TARGET" \
    -sdk "$SDK_PATH" \
    -framework Foundation \
    -import-objc-header "$REPO_ROOT/palikey-engine/include/palikey.h" \
    -I "$REPO_ROOT/palikey-engine/include" \
    "$SMOKE_ENGINE" \
    "$SCRIPT_DIR/PaliEngine.swift" \
    "$SCRIPT_DIR/EasyUnicode.swift" \
    "$SCRIPT_DIR/smoke_test.swift" \
    -o "$BUILD_DIR/smoke_test"

"$BUILD_DIR/smoke_test"

# 6. Bundle verification
echo ""
echo "--> [6/6] Verifying universal bundle structure..."
file "$APP_BUNDLE/Contents/MacOS/PaliKey"
lipo -info "$APP_BUNDLE/Contents/MacOS/PaliKey"
plutil -lint "$APP_BUNDLE/Contents/Info.plist"

echo ""
echo "============================================================"
echo " PaliKey macOS Universal Build & Verification: SUCCESS (Exit 0)"
echo " Output: $APP_BUNDLE"
echo "============================================================"
