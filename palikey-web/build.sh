#!/usr/bin/env bash
set -euo pipefail

# PaliKey Web WASM Shell Build Script
# Builds palikey-web/crate for wasm32-unknown-unknown and prepares dist/ assets.

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORKSPACE_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

export PATH="${HOME}/.cargo/bin:${PATH}"

echo "=== PaliKey Web WASM Build ==="

# 1. Verify cargo availability
if ! command -v cargo &>/dev/null; then
    echo "ERROR: cargo not found in PATH or ~/.cargo/bin" >&2
    exit 1
fi

# 2. Check / add wasm32-unknown-unknown target
if command -v rustup &>/dev/null; then
    if ! rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
        echo "Adding wasm32-unknown-unknown target via rustup..."
        rustup target add wasm32-unknown-unknown
    fi
fi

# 3. Build WebAssembly crate
echo "Compiling palikey-web-wasm..."
cargo build \
    --manifest-path "${WORKSPACE_ROOT}/Cargo.toml" \
    --package palikey-web-wasm \
    --target wasm32-unknown-unknown \
    --release

# 4. Prepare dist directory
mkdir -p "${SCRIPT_DIR}/dist"

WASM_SRC="${WORKSPACE_ROOT}/target/wasm32-unknown-unknown/release/palikey_web.wasm"
if [[ ! -f "${WASM_SRC}" ]]; then
    echo "ERROR: Compiled wasm binary not found at ${WASM_SRC}" >&2
    exit 1
fi

cp "${WASM_SRC}" "${SCRIPT_DIR}/dist/palikey.wasm"
echo "Copied palikey.wasm -> dist/palikey.wasm ($(wc -c < "${SCRIPT_DIR}/dist/palikey.wasm" | tr -d ' ') bytes)"

# Copy schemes.json to dist for cheat-sheet rendering
if [[ -f "${WORKSPACE_ROOT}/shared/schemes.json" ]]; then
    cp "${WORKSPACE_ROOT}/shared/schemes.json" "${SCRIPT_DIR}/dist/schemes.json"
    echo "Copied shared/schemes.json -> dist/schemes.json"
fi

# 5. Generate self-contained single-file dist/index.html demo
echo "Generating dist/index.html..."
python3 "${SCRIPT_DIR}/generate_demo_html.py"

echo "=== Build Complete! ==="
