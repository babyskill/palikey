#!/usr/bin/env bash
set -euo pipefail

# ==============================================================================
# build-engine.sh — Cross-compile PaliKey Rust engine & create XCFramework for iOS
# ==============================================================================
# Targets:
#   - aarch64-apple-ios (Physical iOS devices: iPhone, iPad)
#   - aarch64-apple-ios-sim (iOS Simulator on Apple Silicon)
#   - x86_64-apple-ios (iOS Simulator on Intel Macs, including this iMac)
#
# Slices produced:
#   - ios-arm64
#   - ios-arm64_x86_64-simulator (Universal fat library via lipo)
# ==============================================================================

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
ENGINE_DIR="${REPO_ROOT}/palikey-engine"
TARGET_DIR="${REPO_ROOT}/target"
OUTPUT_DIR="${SCRIPT_DIR}/Frameworks"
XCFRAMEWORK_NAME="PaliKeyEngine.xcframework"
XCFRAMEWORK_PATH="${OUTPUT_DIR}/${XCFRAMEWORK_NAME}"

echo "=== PaliKey iOS Engine Build ==="
echo "Repo root:       ${REPO_ROOT}"
echo "Engine dir:      ${ENGINE_DIR}"
echo "Output path:     ${XCFRAMEWORK_PATH}"

# 1. Ensure cargo & rustup in PATH
if ! command -v cargo &> /dev/null; then
    if [ -f "${HOME}/.cargo/bin/cargo" ]; then
        export PATH="${HOME}/.cargo/bin:${PATH}"
    else
        echo "ERROR: cargo not found in PATH or ~/.cargo/bin"
        exit 1
    fi
fi

# 2. Check Xcode Developer Tools
if ! command -v xcodebuild &> /dev/null; then
    echo "ERROR: xcodebuild not found. Please ensure Xcode is installed and active via xcode-select."
    exit 1
fi

DEVELOPER_DIR="$(xcode-select -p)"
XCODE_VER="$(xcodebuild -version | tr '\n' ' ')"
echo "Developer dir:   ${DEVELOPER_DIR}"
echo "Xcode version:   ${XCODE_VER}"

# 3. Ensure required Rust targets are installed
REQUIRED_TARGETS=("aarch64-apple-ios" "aarch64-apple-ios-sim" "x86_64-apple-ios")

for tgt in "${REQUIRED_TARGETS[@]}"; do
    if ! rustup target list --installed | grep -q "^${tgt}$"; then
        echo "Adding missing rustup target: ${tgt}..."
        rustup target add "${tgt}"
    fi
done

# 4. Build release staticlib for each target
echo ""
echo "--- Compiling palikey-engine for iOS targets ---"
for tgt in "${REQUIRED_TARGETS[@]}"; do
    echo "Building ${tgt} (release)..."
    (cd "${REPO_ROOT}" && cargo build --package palikey-engine --release --target "${tgt}")
    echo "Stripping debug symbols from ${tgt} library..."
    xcrun strip -S "${TARGET_DIR}/${tgt}/release/libpalikey_engine.a"
done

DEVICE_LIB="${TARGET_DIR}/aarch64-apple-ios/release/libpalikey_engine.a"
SIM_ARM64_LIB="${TARGET_DIR}/aarch64-apple-ios-sim/release/libpalikey_engine.a"
SIM_X86_64_LIB="${TARGET_DIR}/x86_64-apple-ios/release/libpalikey_engine.a"

for lib in "${DEVICE_LIB}" "${SIM_ARM64_LIB}" "${SIM_X86_64_LIB}"; do
    if [ ! -f "${lib}" ]; then
        echo "ERROR: Expected static library not found at ${lib}"
        exit 1
    fi
done

# 5. Create universal simulator library via lipo
echo ""
echo "--- Creating universal iOS Simulator library via lipo ---"
STAGE_DIR="$(mktemp -d /tmp/palikey-ios-build.XXXXXX)"
trap 'rm -rf "${STAGE_DIR}"' EXIT

SIM_UNIVERSAL_DIR="${STAGE_DIR}/sim-universal"
mkdir -p "${SIM_UNIVERSAL_DIR}"
SIM_UNIVERSAL_LIB="${SIM_UNIVERSAL_DIR}/libpalikey_engine.a"

xcrun lipo -create \
    "${SIM_ARM64_LIB}" \
    "${SIM_X86_64_LIB}" \
    -output "${SIM_UNIVERSAL_LIB}"

echo "Universal simulator staticlib created: ${SIM_UNIVERSAL_LIB}"
xcrun lipo -info "${SIM_UNIVERSAL_LIB}"

# 6. Prepare Headers & Module Map
echo ""
echo "--- Staging C Headers & Clang Module Map ---"
HEADERS_DIR="${STAGE_DIR}/Headers"
mkdir -p "${HEADERS_DIR}"

cp "${ENGINE_DIR}/include/palikey.h" "${HEADERS_DIR}/palikey.h"

cat << 'EOF' > "${HEADERS_DIR}/module.modulemap"
module PaliKeyEngine {
    header "palikey.h"
    export *
}
EOF

# 7. Package PaliKeyEngine.xcframework
echo ""
echo "--- Packaging XCFramework via xcodebuild ---"
mkdir -p "${OUTPUT_DIR}"
rm -rf "${XCFRAMEWORK_PATH}"

xcodebuild -create-xcframework \
    -library "${DEVICE_LIB}" \
    -headers "${HEADERS_DIR}" \
    -library "${SIM_UNIVERSAL_LIB}" \
    -headers "${HEADERS_DIR}" \
    -output "${XCFRAMEWORK_PATH}"

echo ""
echo "=== XCFramework Successfully Generated ==="
echo "Location: ${XCFRAMEWORK_PATH}"
echo ""
echo "Slice info:"
xcrun lipo -info "${XCFRAMEWORK_PATH}/ios-arm64/libpalikey_engine.a"
xcrun lipo -info "${XCFRAMEWORK_PATH}/ios-arm64_x86_64-simulator/libpalikey_engine.a"
