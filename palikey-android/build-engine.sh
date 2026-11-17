#!/usr/bin/env bash
set -euo pipefail

# ==============================================================================
# build-engine.sh — Cross-compile PaliKey Rust engine & JNI bridge for Android
# ==============================================================================

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
ENGINE_DIR="${REPO_ROOT}/palikey-engine"
TARGET_DIR="${REPO_ROOT}/target"
JNILIBS_DIR="${SCRIPT_DIR}/app/src/main/jniLibs"
CPP_DIR="${SCRIPT_DIR}/app/src/main/cpp"

echo "=== PaliKey Android Engine Build ==="
echo "Repo root:   ${REPO_ROOT}"
echo "Engine dir:  ${ENGINE_DIR}"
echo "Output jni:  ${JNILIBS_DIR}"

# 1. Detect Android NDK
NDK_ROOT=""
if [ -n "${ANDROID_NDK_HOME:-}" ] && [ -d "${ANDROID_NDK_HOME}" ]; then
    NDK_ROOT="${ANDROID_NDK_HOME}"
elif [ -n "${ANDROID_NDK_ROOT:-}" ] && [ -d "${ANDROID_NDK_ROOT}" ]; then
    NDK_ROOT="${ANDROID_NDK_ROOT}"
elif [ -n "${NDK:-}" ] && [ -d "${NDK}" ]; then
    NDK_ROOT="${NDK}"
elif [ -n "${ANDROID_HOME:-}" ] && [ -d "${ANDROID_HOME}/ndk" ]; then
    # Pick latest NDK under $ANDROID_HOME/ndk
    NDK_ROOT="$(ls -td "${ANDROID_HOME}/ndk/"* 2>/dev/null | head -n 1 || true)"
elif [ -d "${HOME}/Library/Android/sdk/ndk" ]; then
    # macOS default SDK location
    NDK_ROOT="$(ls -td "${HOME}/Library/Android/sdk/ndk/"* 2>/dev/null | head -n 1 || true)"
fi

if [ -z "${NDK_ROOT}" ] || [ ! -d "${NDK_ROOT}" ]; then
    echo "ERROR: Android NDK not found!"
    echo "Please set ANDROID_NDK_HOME or install NDK via Android Studio / SDK Manager:"
    echo "  sdkmanager --install 'ndk;26.3.11579264'"
    exit 1
fi

echo "Found NDK:   ${NDK_ROOT}"

# 2. Detect Host LLVM Toolchain inside NDK
HOST_OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
HOST_ARCH="$(uname -m)"

LLVM_PREBUILT=""
if [ -d "${NDK_ROOT}/toolchains/llvm/prebuilt/darwin-x86_64/bin" ]; then
    LLVM_PREBUILT="${NDK_ROOT}/toolchains/llvm/prebuilt/darwin-x86_64/bin"
elif [ -d "${NDK_ROOT}/toolchains/llvm/prebuilt/darwin-aarch64/bin" ]; then
    LLVM_PREBUILT="${NDK_ROOT}/toolchains/llvm/prebuilt/darwin-aarch64/bin"
elif [ -d "${NDK_ROOT}/toolchains/llvm/prebuilt/linux-x86_64/bin" ]; then
    LLVM_PREBUILT="${NDK_ROOT}/toolchains/llvm/prebuilt/linux-x86_64/bin"
else
    echo "ERROR: NDK LLVM toolchain directory not found under ${NDK_ROOT}/toolchains/llvm/prebuilt"
    exit 1
fi

echo "Toolchain:   ${LLVM_PREBUILT}"

# 3. Ensure cargo & rustup in PATH
if ! command -v cargo &> /dev/null; then
    if [ -f "${HOME}/.cargo/bin/cargo" ]; then
        export PATH="${HOME}/.cargo/bin:${PATH}"
    else
        echo "ERROR: cargo not found in PATH or ~/.cargo/bin"
        exit 1
    fi
fi

# 4. Target definitions: Triple, ABI, Clang prefix, Min API
TARGETS=(
    "aarch64-linux-android:arm64-v8a:aarch64-linux-android26:aarch64-linux-android"
    "armv7-linux-androideabi:armeabi-v7a:armv7a-linux-androideabi26:armv7a-linux-androideabi"
    "x86_64-linux-android:x86_64:x86_64-linux-android26:x86_64-linux-android"
)

# 5. Build for each architecture
for entry in "${TARGETS[@]}"; do
    IFS=":" read -r RUST_TRIPLE ANDROID_ABI CLANG_PREFIX LINKER_NAME <<< "${entry}"
    echo ""
    echo "--- Building ${ANDROID_ABI} (${RUST_TRIPLE}) ---"

    # Ensure rustup target is installed
    if ! rustup target list | grep -q "${RUST_TRIPLE} (installed)"; then
        echo "Adding rustup target ${RUST_TRIPLE}..."
        rustup target add "${RUST_TRIPLE}"
    fi

    # Set architecture-specific cargo linker
    CLANG_BIN="${LLVM_PREBUILT}/${CLANG_PREFIX}-clang"
    CLANGXX_BIN="${LLVM_PREBUILT}/${CLANG_PREFIX}-clang++"

    if [ ! -f "${CLANG_BIN}" ]; then
        echo "ERROR: Compiler binary ${CLANG_BIN} does not exist!"
        exit 1
    fi

    # Export cargo linker variable according to Rust convention
    # e.g. CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER
    VAR_NAME="CARGO_TARGET_$(echo "${RUST_TRIPLE}" | tr '[:lower:]-' '[:upper:]_')_LINKER"
    export "${VAR_NAME}=${CLANG_BIN}"

    # Build engine staticlib & cdylib
    echo "Compiling palikey-engine via cargo (${RUST_TRIPLE})..."
    (cd "${REPO_ROOT}" && cargo build --package palikey-engine --release --target "${RUST_TRIPLE}")

    STATIC_LIB="${TARGET_DIR}/${RUST_TRIPLE}/release/libpalikey_engine.a"
    SHARED_LIB="${TARGET_DIR}/${RUST_TRIPLE}/release/libpalikey_engine.so"

    if [ ! -f "${STATIC_LIB}" ]; then
        echo "ERROR: Expected static library not found at ${STATIC_LIB}"
        exit 1
    fi

    # Prepare jniLibs destination
    ABI_DEST="${JNILIBS_DIR}/${ANDROID_ABI}"
    mkdir -p "${ABI_DEST}"

    # Compile native-lib.cpp into libpalikey_jni.so linking libpalikey_engine.a
    echo "Compiling JNI bridge ${ABI_DEST}/libpalikey_jni.so..."
    "${CLANGXX_BIN}" -O3 -shared -fPIC \
        -I"${ENGINE_DIR}/include" \
        "${CPP_DIR}/native-lib.cpp" \
        "${STATIC_LIB}" \
        -static-libstdc++ \
        -llog \
        -o "${ABI_DEST}/libpalikey_jni.so"

    # Also copy the standalone libpalikey_engine.so for direct FFI if needed
    if [ -f "${SHARED_LIB}" ]; then
        cp "${SHARED_LIB}" "${ABI_DEST}/libpalikey_engine.so"
    fi

    # Strip symbols
    if [ -f "${LLVM_PREBUILT}/llvm-strip" ]; then
        "${LLVM_PREBUILT}/llvm-strip" "${ABI_DEST}/libpalikey_jni.so"
        if [ -f "${ABI_DEST}/libpalikey_engine.so" ]; then
            "${LLVM_PREBUILT}/llvm-strip" "${ABI_DEST}/libpalikey_engine.so"
        fi
    fi

    echo "Generated: $(ls -lh "${ABI_DEST}/libpalikey_jni.so")"
done

echo ""
echo "=== Android Engine & JNI Build Complete ==="
ls -lh "${JNILIBS_DIR}"/*/*.so
