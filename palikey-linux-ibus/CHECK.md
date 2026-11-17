# PaliKey IBus Engine — Verification & Status Report

This document records the exact checks executed on the development host (macOS Intel) and the explicit commands required when building on a native Linux machine.

---

## 1. Verified on Development Host (macOS x86_64)

Because macOS does not provide native IBus/GLib packages or D-Bus system daemons, full Linux compilation and runtime testing cannot be executed directly on macOS. However, all source code, type definitions, FFI integrations, and configuration loaders were verified at syntax, type, and unit levels:

### 1.1. C11 Syntax & Strict Compiler Typecheck
All IBus engine sources were checked with Clang 17.0.0 using strict flags (`-std=c11 -Wall -Wextra -Werror`):
```bash
clang -fsyntax-only -std=c11 -Wall -Wextra -Werror \
    -I palikey-linux-ibus/src \
    -I palikey-engine/include \
    -I palikey-linux-ibus/test/stubs \
    palikey-linux-ibus/src/palikey_engine_ibus.c \
    palikey-linux-ibus/src/palikey_config.c \
    palikey-linux-ibus/src/main.c
```
- **Result**: `0 errors, 0 warnings (Exit code 0)`
- Confirmed:
  - `PalikeyEngineIBus` cleanly implements an `IBusEngine` GObject subclass.
  - Signal callbacks and D-Bus component registration match IBus 1.5 specifications.
  - All keysyms, state masks, and surrounding text API calls match standard IBus headers.

### 1.2. Shared Configuration & EasyUnicode Unit Test Execution
The shared C configuration loader and EasyUnicode mapping were compiled and executed against the live `libpalikey_engine.a` static library:
```bash
clang -std=c11 -Wall -Wextra \
    -I palikey-linux-ibus/src \
    -I palikey-engine/include \
    palikey-linux-ibus/test/test_config.c \
    palikey-linux-ibus/src/palikey_config.c \
    -L target/release -lpalikey_engine -lpthread -ldl -lm \
    -o /tmp/test_linux_config && /tmp/test_linux_config
```
- **Result**: `[SUCCESS] All shared Linux shell tests passed!`
- Confirmed:
  - Canonical resolution of `$XDG_CONFIG_HOME/palikey/config.json` and `$HOME/.config/palikey/config.json`.
  - Accurate parsing of `shared/config.json` adhering to `shared/config.schema.json`.
  - EasyUnicode AltGr direct-mapping behavior (`a` → `ā`, `m` → `ṃ`/`ṁ`).
  - Correct FFI interaction with `palikey_new`, `palikey_process_instance`, `palikey_set_scheme`, and `palikey_free`.

### 1.3. FFI Symbol Cross-Check
Every symbol from `palikey-engine/include/palikey.h` utilized in the IBus engine was verified against the exported symbols in `target/release/libpalikey_engine.a`:
| Symbol | Type | In palikey.h | In libpalikey_engine.a | Usage in palikey-linux-ibus |
|---|---|:---:|:---:|---|
| `PalikeyEngine` | Opaque Struct | Yes | Yes | Context state handle (`PalikeyEngineIBus::engine`) |
| `PalikeyProcessResult` | Struct | Yes | Yes | Keystroke output (`commit`, `backspaces`, `preedit`) |
| `PalikeySchemeId` | Enum | Yes | Yes | Transliteration scheme selector (`0..2`) |
| `PalikeyNiggahitaVariant` | Enum | Yes | Yes | Orthography selector (`0..1`) |
| `palikey_new` | Function | Yes | Yes | Instance creation on context init |
| `palikey_free` | Function | Yes | Yes | Instance disposal on context finalize |
| `palikey_process_instance`| Function | Yes | Yes | Keystroke transduction processing |
| `palikey_set_scheme` | Function | Yes | Yes | Dynamic scheme switching |
| `palikey_reset_instance` | Function | Yes | Yes | Buffer and undo history reset |
| `palikey_version` | Function | Yes | Yes | Engine version query in `--version` |

---

## 2. Linux Machine Next Steps (Build & Runtime Verification)

When running on a native Linux workstation (e.g. Debian 12+, Ubuntu 22.04+, Fedora 39+, Arch Linux):

### 2.1. CMake Configure & Build
```bash
# Ensure development packages are present
# Debian/Ubuntu: sudo apt install build-essential cmake pkg-config libibus-1.0-dev libglib2.0-dev cargo
# Fedora: sudo dnf install gcc cmake pkgconf-pkg-config ibus-devel glib2-devel cargo

# Step 1: Build the Rust engine static library
cargo build --release -p palikey-engine

# Step 2: Configure CMake project
cd palikey-linux-ibus
cmake -B build -DCMAKE_BUILD_TYPE=Release

# Step 3: Build the executable daemon
cmake --build build -j$(nproc)
```

### 2.2. Installation & Verification
```bash
# Step 4: Install daemon and XML manifest into system directories
sudo cmake --install build

# Step 5: Restart IBus daemon
ibus restart

# Step 6: Verify registration in IBus engines list
ibus list-engine | grep palikey

# Step 7: Activate PaliKey engine
ibus engine palikey
```
