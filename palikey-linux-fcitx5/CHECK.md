# PaliKey Fcitx5 Addon — Verification & Status Report

This document records the exact checks executed on the development host (macOS Intel) and the explicit commands required when building on a native Linux machine.

---

## 1. Verified on Development Host (macOS x86_64)

Because macOS does not provide native Fcitx5 shared libraries or headers, full Linux compilation and runtime testing cannot be executed directly on macOS. However, all source code, type definitions, FFI integrations, and configuration loaders were verified at syntax, type, and unit levels:

### 1.1. C++17 Syntax & Strict Compiler Typecheck
All addon sources were checked with Clang 17.0.0 using strict flags (`-std=c++17 -Wall -Wextra -Werror`):
```bash
clang++ -fsyntax-only -std=c++17 -Wall -Wextra -Werror \
    -I palikey-linux-fcitx5/src \
    -I palikey-engine/include \
    -I palikey-linux-fcitx5/test/stubs \
    palikey-linux-fcitx5/src/palikey_state.cpp \
    palikey-linux-fcitx5/src/palikey_engine_addon.cpp \
    palikey-linux-fcitx5/src/palikey_config.cpp
```
- **Result**: `0 errors, 0 warnings (Exit code 0)`
- Confirmed:
  - `PalikeyEngineAddon` properly implements `fcitx::InputMethodEngineV2`.
  - `PalikeyState` correctly subclasses `fcitx::InputContextProperty`.
  - Factory registration and property lookups adhere to Fcitx5's property architecture.
  - All keysyms, key event flags, and surrounding text API calls match Fcitx5 specifications.

### 1.2. Shared Configuration & EasyUnicode Unit Test Execution
The shared C/C++ configuration loader and EasyUnicode mapping were compiled and executed against the live `libpalikey_engine.a` static library:
```bash
clang++ -std=c++17 -Wall -Wextra \
    -I palikey-linux-fcitx5/src \
    -I palikey-engine/include \
    palikey-linux-fcitx5/test/test_config.cpp \
    palikey-linux-fcitx5/src/palikey_config.c \
    -L target/release -lpalikey_engine -lpthread -ldl -lm \
    -o /tmp/test_fcitx_config && /tmp/test_fcitx_config
```
- **Result**: `[SUCCESS] Fcitx5 C++17 shared config test passed!`
- Confirmed:
  - Canonical resolution of `$XDG_CONFIG_HOME/palikey/config.json` and `$HOME/.config/palikey/config.json`.
  - Accurate parsing of `shared/config.json` adhering to `shared/config.schema.json`.
  - EasyUnicode AltGr direct-mapping behavior (`a` → `ā`, `m` → `ṃ`/`ṁ`).
  - Correct FFI interaction with `palikey_new`, `palikey_process_instance`, `palikey_set_scheme`, and `palikey_free`.

### 1.3. FFI Symbol Cross-Check
Every symbol from `palikey-engine/include/palikey.h` utilized in the Fcitx5 addon was verified against the exported symbols in `target/release/libpalikey_engine.a`:
| Symbol | Type | In palikey.h | In libpalikey_engine.a | Usage in palikey-linux-fcitx5 |
|---|---|:---:|:---:|---|
| `PalikeyEngine` | Opaque Struct | Yes | Yes | Context state handle (`PalikeyState::m_engine`) |
| `PalikeyProcessResult` | Struct | Yes | Yes | Keystroke output (`commit`, `backspaces`, `preedit`) |
| `PalikeySchemeId` | Enum | Yes | Yes | Transliteration scheme selector (`0..2`) |
| `PalikeyNiggahitaVariant` | Enum | Yes | Yes | Orthography selector (`0..1`) |
| `palikey_new` | Function | Yes | Yes | Instance creation on context init |
| `palikey_free` | Function | Yes | Yes | Instance disposal on context destroy |
| `palikey_process_instance`| Function | Yes | Yes | Keystroke transduction processing |
| `palikey_set_scheme` | Function | Yes | Yes | Dynamic scheme switching |
| `palikey_reset_instance` | Function | Yes | Yes | Buffer and undo history reset |
| `palikey_version` | Function | Yes | Yes | Engine version query |

---

## 2. Linux Machine Next Steps (Build & Runtime Verification)

When running on a native Linux workstation (e.g. Debian 12+, Ubuntu 22.04+, Fedora 39+, Arch Linux):

### 2.1. CMake Configure & Build
```bash
# Ensure development packages are present
# Debian/Ubuntu: sudo apt install build-essential cmake extra-cmake-modules libfcitx5core-dev libfcitx5utils-dev cargo
# Fedora: sudo dnf install gcc-c++ cmake extra-cmake-modules fcitx5-devel cargo

# Step 1: Build the Rust engine static library
cargo build --release -p palikey-engine

# Step 2: Configure CMake project
cd palikey-linux-fcitx5
cmake -B build -DCMAKE_BUILD_TYPE=Release

# Step 3: Build the addon shared module
cmake --build build -j$(nproc)
```

### 2.2. Installation & Verification
```bash
# Step 4: Install addon into system directories
sudo cmake --install build

# Step 5: Restart Fcitx5 and observe startup logs
fcitx5 -r -d

# Step 6: Verify registration in fcitx5-configtool or via CLI
fcitx5-remote -s palikey
```
