# PaliKey IBus Engine

Standalone C input method engine daemon for **IBus** (`ibus-engine-palikey`) providing native Pāḷi typing on Linux desktop environments (GNOME, MATE, Cinnamon, XFCE).

PaliKey integrates the clean-room, zero-dependency Rust core (`palikey-engine`) through its C FFI interface (`palikey.h`), supporting three standard transliteration schemes (**Velthuis**, **PaliTelex**, and **SIL**), PTS / VRI niggahīta orthography variants (`ṃ` / `ṁ`), and an **EasyUnicode** direct-insert AltGr/Option layer.

---

## 1. Prerequisites & Required Development Packages

Building `ibus-engine-palikey` requires a C11 compiler, CMake (≥ 3.16), `pkg-config`, the Rust toolchain, and IBus / GLib development libraries.

### Debian / Ubuntu / Linux Mint
```bash
sudo apt update
sudo apt install -y \
    build-essential \
    cmake \
    pkg-config \
    libibus-1.0-dev \
    libglib2.0-dev \
    cargo \
    rustc
```

### Fedora / RHEL
```bash
sudo dnf install -y \
    gcc \
    cmake \
    pkgconf-pkg-config \
    ibus-devel \
    glib2-devel \
    cargo \
    rust
```

### Arch Linux / Manjaro
```bash
sudo pacman -S --needed \
    base-devel \
    cmake \
    pkgconf \
    ibus \
    glib2 \
    rust
```

---

## 2. Building

### Step 1: Build the Rust Engine Static Library
From the root repository directory:
```bash
cargo build --release -p palikey-engine
```
This generates `target/release/libpalikey_engine.a`.

### Step 2: Configure & Build IBus Engine
```bash
cd palikey-linux-ibus
cmake -B build -DCMAKE_BUILD_TYPE=Release
cmake --build build -j$(nproc)
```

---

## 3. Installation

Install system-wide to standard IBus locations:
```bash
sudo cmake --install build
```

This installs:
- Engine daemon executable: `/usr/libexec/ibus-engine-palikey` (or `/usr/lib/ibus/ibus-engine-palikey`)
- D-Bus component manifest: `/usr/share/ibus/component/palikey.xml`
- Scalable SVG icon: `/usr/share/icons/hicolor/scalable/apps/palikey.svg`

### User-local Installation (without root)
```bash
cmake -B build -DCMAKE_INSTALL_PREFIX=$HOME/.local
cmake --build build
cmake --install build
```
*(Ensure `$HOME/.local/share/ibus/component` is included in `IBUS_COMPONENT_PATH` if installing locally).*

---

## 4. Activating & Testing in IBus

1. Restart the IBus daemon:
   ```bash
   ibus restart
   # Or restart with debug logging in background:
   ibus-daemon -drx
   ```
2. Open IBus settings:
   ```bash
   ibus-setup
   ```
3. In the **Input Method** tab:
   - Click **Add**
   - Click the three dots (More)
   - Search for **PaliKey (Pāḷi)** or language **Pali** (`pi`)
   - Click **Add**
4. Switch to PaliKey via `Super+Space` (or your configured hotkey).
5. Type in any application:
   - **Velthuis**: `aa` → `ā`, `.t` → `ṭ`, `~n` → `ñ`, `.m` → `ṃ`
   - **AltGr Direct**: `AltGr + a` → `ā`, `AltGr + t` → `ṭ`

---

## 5. Configuration

PaliKey reads runtime settings from `~/.config/palikey/config.json` (or `$XDG_CONFIG_HOME/palikey/config.json`).

Sample `~/.config/palikey/config.json`:
```json
{
  "$schema": "./config.schema.json",
  "version": "0.1.0",
  "scheme": "velthuis",
  "niggahita": "dot_below",
  "extended_set": false,
  "escape": {
    "enabled": true,
    "escape_char": "\\",
    "undo_by_backspace": true,
    "repeat_key_escapes": true
  }
}
```

### Options
- `scheme`: `"velthuis"` (0, default), `"palitelex"` (1), `"sil"` (2).
- `niggahita`: `"dot_below"` (0, `ṃ` U+1E43, PTS/SuttaCentral standard) or `"dot_above"` (1, `ṁ` U+1E41, VRI tradition).
- `extended_set`: `false` (default) or `true` (enables Sanskrit/IAST characters `ṛ ṝ ḹ ḥ ś ṣ ē ō`).
- `escape`: Controls literal word escaping via `\` and backspace undo behavior.

---

## 6. Packaging (.deb / .rpm)

Generate binary packages via CPack:
```bash
cd palikey-linux-ibus/build
cpack -G DEB   # Generates ibus-palikey-0.1.0-Linux.deb
cpack -G RPM   # Generates ibus-palikey-0.1.0-Linux.rpm
```
