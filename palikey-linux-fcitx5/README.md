# PaliKey Fcitx5 Addon

C++17 input method addon for **Fcitx5** providing native Pāḷi typing on modern Linux desktops (GNOME, KDE Plasma 5/6, XFCE, Sway, Hyprland).

PaliKey integrates the clean-room, zero-dependency Rust core (`palikey-engine`) through its C FFI interface (`palikey.h`), supporting three standard transliteration schemes (**Velthuis**, **PaliTelex**, and **SIL**), PTS / VRI niggahīta orthography variants (`ṃ` / `ṁ`), and an **EasyUnicode** direct-insert AltGr/Option layer.

---

## 1. Prerequisites & Required Development Packages

To build and run the Fcitx5 addon, your Linux system requires a C++17 compiler, CMake (≥ 3.16), the Rust toolchain (for the engine static library), and Fcitx5 development headers.

### Debian / Ubuntu / Linux Mint
```bash
sudo apt update
sudo apt install -y \
    build-essential \
    cmake \
    extra-cmake-modules \
    libfcitx5core-dev \
    libfcitx5utils-dev \
    cargo \
    rustc
```

### Fedora / RHEL
```bash
sudo dnf install -y \
    gcc-c++ \
    cmake \
    extra-cmake-modules \
    fcitx5-devel \
    cargo \
    rust
```

### Arch Linux / Manjaro
```bash
sudo pacman -S --needed \
    base-devel \
    cmake \
    extra-cmake-modules \
    fcitx5 \
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

### Step 2: Configure & Build Fcitx5 Addon
```bash
cd palikey-linux-fcitx5
cmake -B build -DCMAKE_BUILD_TYPE=Release
cmake --build build -j$(nproc)
```

---

## 3. Installation

Install system-wide to standard Fcitx5 directories:
```bash
sudo cmake --install build
```

This installs:
- Shared addon module: `/usr/lib/fcitx5/palikey.so` (or system libdir)
- Addon metadata: `/usr/share/fcitx5/addon/palikey.conf`
- Input method descriptor: `/usr/share/fcitx5/inputmethod/palikey.conf`
- Scalable SVG icon: `/usr/share/icons/hicolor/scalable/apps/org.giacngo.palikey.svg`

### User-local Installation (without root)
You can also install into your user profile:
```bash
cmake -B build -DCMAKE_INSTALL_PREFIX=$HOME/.local
cmake --build build
cmake --install build
```

---

## 4. Activating & Testing in Fcitx5

1. Restart the Fcitx5 daemon:
   ```bash
   fcitx5 -r -d
   ```
2. Open the Fcitx5 configuration tool:
   ```bash
   fcitx5-configtool
   ```
3. In the **Input Method** tab:
   - Uncheck *"Only Show Current Language"*
   - Search for **PaliKey** (Pāḷi language `pi`)
   - Add it to your active input method list.
4. Switch to PaliKey using your configured toggle shortcut (e.g. `Ctrl+Space` or `Super+Space`).
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
cd palikey-linux-fcitx5/build
cpack -G DEB   # Generates fcitx5-palikey-0.1.0-Linux.deb
cpack -G RPM   # Generates fcitx5-palikey-0.1.0-Linux.rpm
```
