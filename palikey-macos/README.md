# PaliKey for macOS

Native macOS Input Method (InputMethodKit) for typing Romanized Pāḷi in any application.

Built in Swift with a high-performance Rust core transducer engine (`palikey-engine`) linked as a static library.

---

## Features

- **Three Built-in Schemes:**
  - **Velthuis:** Scholar standard (`aa` $\rightarrow$ ā, `.t` $\rightarrow$ ṭ, `"n` $\rightarrow$ ṅ, `~n` $\rightarrow$ ñ, `.m` $\rightarrow$ ṃ).
  - **PaliTelex:** Vietnamese Telex muscle memory (`aa` $\rightarrow$ ā, `tx` $\rightarrow$ ṭ, `ngw` $\rightarrow$ ṅ, `nw` $\rightarrow$ ñ, `mx` $\rightarrow$ ṃ).
  - **SIL / Keyman:** Compatibility mode (`-a` $\rightarrow$ ā, `.t` $\rightarrow$ ṭ, `>n` $\rightarrow$ ṅ, `~n` $\rightarrow$ ñ, `.m` $\rightarrow$ ṃ).
- **EasyUnicode Direct-Insert Layer:**
  - Hold `Option` (`⌥`) or `Option + Shift` for instant single-keystroke Pāḷi insertion (e.g. `⌥a` $\rightarrow$ ā, `⌥t` $\rightarrow$ ṭ, `⌥m` $\rightarrow$ ṃ, `⌥g` $\rightarrow$ ṅ, `⌥j` $\rightarrow$ ñ).
- **Single-Level Backspace Undo:**
  - Pressing `Backspace` immediately after a composition reverts the character back to its raw sequence (e.g. `ā` $\rightarrow$ `aa`).
- **Niggahīta Variant Toggle:**
  - Switch between `ṃ` (U+1E43, PTS / SuttaCentral dot below standard) and `ṁ` (U+1E41, VRI dot above tradition) directly from the menu.
- **Extended Sanskrit / IAST Support:**
  - Toggle support for Sanskrit characters (`ṛ`, `ṝ`, `ḷ`, `ḥ`, `ś`, `ṣ`, `ē`, `ō`).
- **100% Offline & Private:**
  - Zero network permissions, no telemetry, no background network traffic.

---

## Installation

### 1. Build the App Bundle
Ensure Rust (`cargo`) and Xcode Command Line Tools (`swiftc`) are installed, then run:
```bash
cd palikey-macos
./build.sh
```
This builds `build/PaliKey.app`.

### 2. Install to Input Methods
Copy the bundle into your user's Input Methods directory:
```bash
cp -R build/PaliKey.app ~/Library/Input\ Methods/
```

### 3. Activate the Input Method
1. **Log out of macOS and log back in** (or kill the system text input server: `killall -9 "PaliKey" AppleSpell 2>/dev/null || true`).
2. Open **System Settings** $\rightarrow$ **Keyboard** $\rightarrow$ **Text Input** $\rightarrow$ click **Edit...** next to Input Sources.
3. Click the **`+`** (Add) button at the bottom left.
4. Search for **PaliKey** (under *English* or *Multiple Languages* / *Others*).
5. Select **PaliKey** and click **Add**.
6. Switch to PaliKey via the menu bar input menu or `Control + Space`.

---

## Configuration & Usage

Click the **PaliKey** icon in the macOS menu bar to:
- Switch active transliteration scheme (**Velthuis**, **PaliTelex**, or **SIL**).
- Toggle Niggahīta orthography (`ṃ` vs `ṁ`).
- Enable or disable the Extended Sanskrit / IAST character set.
- View the About dialog.

Configuration is persisted in `~/Library/Application Support/PaliKey/config.json`.

---

## Keyboard Schemes Reference

### 1. Velthuis
| Pāḷi | Typed Sequence | Pāḷi (Cap) | Typed Sequence |
|---|---|---|---|
| ā | `aa` | Ā | `AA` or `Aa` |
| ī | `ii` | Ī | `II` or `Ii` |
| ū | `uu` | Ū | `UU` or `Uu` |
| ṭ | `.t` | Ṭ | `.T` |
| ḍ | `.d` | Ḍ | `.D` |
| ṇ | `.n` | Ṇ | `.N` |
| ḷ | `.l` | Ḷ | `.L` |
| ṃ / ṁ | `.m` | Ṃ / Ṁ | `.M` |
| ṅ | `"n` | Ṅ | `"N` |
| ñ | `~n` | Ñ | `~N` |

### 2. PaliTelex
| Pāḷi | Typed Sequence | Pāḷi (Cap) | Typed Sequence |
|---|---|---|---|
| ā, ī, ū | `aa`, `ii`, `uu` | Ā, Ī, Ū | `AA`, `II`, `UU` |
| ṭ, ḍ, ṇ | `tx`, `dx`, `nx` | Ṭ, Ḍ, Ṇ | `TX`, `DX`, `NX` |
| ḷ, ṃ | `lx`, `mx` | Ḷ, Ṃ | `LX`, `MX` |
| ṅ | `ngw` (or `ngx`) | Ṅ | `NGW` (or `NGX`) |
| ñ | `nw` (or `ny`, `~n`) | Ñ | `NW` (or `NY`, `~N`) |

### 3. EasyUnicode (Option / AltGr Layer)
Works at any time regardless of the active scheme:
- `⌥a` $\rightarrow$ ā (`⌥⇧a` $\rightarrow$ Ā)
- `⌥i` $\rightarrow$ ī (`⌥⇧i` $\rightarrow$ Ī)
- `⌥u` $\rightarrow$ ū (`⌥⇧u` $\rightarrow$ Ū)
- `⌥t` $\rightarrow$ ṭ (`⌥⇧t` $\rightarrow$ Ṭ)
- `⌥d` $\rightarrow$ ḍ (`⌥⇧d` $\rightarrow$ Ḍ)
- `⌥n` $\rightarrow$ ṇ (`⌥⇧n` $\rightarrow$ Ṇ)
- `⌥m` $\rightarrow$ ṃ / ṁ (`⌥⇧m` $\rightarrow$ Ṃ / Ṁ)
- `⌥l` $\rightarrow$ ḷ (`⌥⇧l` $\rightarrow$ Ḷ)
- `⌥g` or `⌥k` $\rightarrow$ ṅ (`⌥⇧g` $\rightarrow$ Ṅ)
- `⌥j` or `⌥c` $\rightarrow$ ñ (`⌥⇧j` $\rightarrow$ Ñ)

---

## Known Limits & Notes

1. **Non-TSM Applications:** Applications that do not implement macOS `TSMDocumentAccess` (e.g. certain raw terminal emulators or legacy X11 ports) do not report exact cursor selection offsets; PaliKey falls back to standard deletion commands (`deleteBackward`).
2. **First-time Install:** macOS caches input methods upon login. If PaliKey does not immediately appear in System Settings after copying to `~/Library/Input Methods/`, log out and log back in.
3. **Codesigning:** The bundle produced by `build.sh` is ad-hoc signed (`codesign --sign -`) for local development and testing. Release distribution on other Macs requires a valid Apple Developer ID Application certificate and notarization.
