# Pāḷi Keyboard

> **Product Name:** Pāḷi Keyboard (internal codename: `palikey`)

One Pāḷi input method for every platform — one Rust engine, one shared transliteration standard, and native shells for Windows, macOS, Linux, Android, iOS, and Web.

Clean-room implementation (MIT License) following the core-and-shell architecture specified in [DESIGN.md](DESIGN.md).

---

## 1. Platform Status Table

Verified levels below are factual records copied directly from each phase's completion report (`.agy/done/`):

| Platform / Shell | Implementation | Phase | Verification Level | Host Environment |
|---|---|---|---|---|
| **Core Engine** (`palikey-engine`) | Rust transducer state machine + C FFI | Phase 0 | **Verified** — 31/31 cargo tests passed (14 unit tests, 17 golden corpus tests: Velthuis, PaliTelex, SIL). | Intel iMac (`x86_64-apple-darwin`) |
| **macOS** (`palikey-macos`) | Native Swift InputMethodKit (IMK) bundle | Phase 1a | **Verified** — Compiled with `swiftc`, linked `libpalikey_engine.a` via C FFI, ad-hoc codesigned, headless smoke test passed (`sammaadi.t.thi` $\rightarrow$ `sammādiṭṭhi`, backspace undo, PaliTelex, SIL, EasyUnicode). | macOS 15.7.9, x86_64 |
| **Web** (`palikey-web`) | WebAssembly (`wasm32-unknown-unknown`) + JS glue | Phase 1b | **Verified** — 41 KB standalone `.wasm`, single-page interactive UI with offline base64 fallback, 52/52 Node.js integration tests passed (`test.mjs`). | Node.js v20+, Chrome/Safari |
| **Keyman** (`palikey-keyman`) | Keyman package (`palikey.kmn`, `palikey.kpj`) | Phase 1b | **Verified** — Synthesized from `shared/schemes.json` via `generate_kmn.py`; unified Velthuis and PaliTelex with zero conflicts. | Keyman Developer |
| **Windows Hook** (`palikey-windows`) | Standalone Rust system tray app (`WH_KEYBOARD_LL` + `SendInput`) | Phase 1c | **Code & Cross-Check Verified** — 6/6 tests pass on host; `cargo check --target x86_64-pc-windows-msvc` passed cleanly (0 errors, 0 warnings). Portable fallback. | Host cross-check; Windows runtime pending |
| **Android** (`palikey-android`) | Kotlin `InputMethodService` + C++ JNI bridge | Phase 2a | **Verified** — Zero network permissions (`INTERNET` absent). Dual layouts (Transducer + Dedicated Pāḷi row + Off). Native staticlibs built for `arm64-v8a`, `armeabi-v7a`, `x86_64`. Output debug APK verified (4.8 MB). | Android SDK 35, NDK 26.3 |
| **iOS** (`palikey-ios`) | Swift container app + programmatic UIKit keyboard extension | Phase 2b | **Verified** — `RequestsOpenAccess = false` (NO Full Access). Universal `PaliKeyEngine.xcframework` (`arm64`, `x86_64-sim`, `arm64-sim`). Full build succeeded in Xcode targeting iOS Simulator. | Xcode 26.3, iOS SDK 26.2 |
| **Linux Fcitx5** (`palikey-linux-fcitx5`) | Modern C++17 addon (`InputMethodEngineV2`) | Phase 3 | **Verified** — CMake + CPack DEB/RPM packaging. Compiled with Clang `-Wall -Wextra -Werror` against stubs; shared config parser & EasyUnicode unit test passed linking `libpalikey_engine.a`. | Clang 17.0, Ubuntu/Debian/Arch |
| **Linux IBus** (`palikey-linux-ibus`) | C11 standalone GObject daemon (`IBusEngine`) | Phase 3 | **Verified** — CMake + CPack DEB/RPM packaging, D-Bus component manifest. Compiled with Clang against stubs; shared config parser & EasyUnicode unit test passed linking `libpalikey_engine.a`. | Clang 17.0, GNOME/Fedora/Ubuntu |
| **Windows TSF** (`palikey-tsf`) | C++17 Text Services Framework COM DLL (`PaliKeyTSF.dll`) | Phase 4 | **Code & Cross-Check Verified** — Implements `ITfTextInputProcessorEx`, `ITfKeyEventSink`, `ITfCompositionSink` (preedit underline), and `ITfDisplayAttributeProvider`. Per-context engine instances. FFI cross-check complete. Windows targets typecheck cleanly. | Host cross-check; Windows runtime pending |

---

## 2. Installation Links (Distribution Hub)

In accordance with product distribution policy, all downloads are hosted with direct links via the official hub:

### [giacngo.org/tools/palikey](https://giacngo.org/tools)

| Platform | Channel / Package | Direct Link Placeholder |
|---|---|---|
| **macOS** | `.dmg` Drag-and-Drop Installer | [Download for macOS](https://github.com/giacngo/palikey/releases/download/v0.1.0/PaliKey-0.1.0-macOS.dmg) |
| **Windows** | Text Services Framework (TSF) DLL | [Download for Windows (TSF)](https://github.com/giacngo/palikey/releases/download/v0.1.0/PaliKeyTSF-0.1.0-Windows.zip) |
| **Windows** | Portable Executable (Hook Fallback) | [Download for Windows (Portable)](https://github.com/giacngo/palikey/releases/download/v0.1.0/PaliKey-Hook-0.1.0-x64.zip) |
| **Linux** | Debian / Ubuntu Package (`.deb`) | [Download for Linux (.deb)](https://github.com/giacngo/palikey/releases/download/v0.1.0/fcitx5-palikey_0.1.0_amd64.deb) |
| **Linux** | Fedora / RHEL Package (`.rpm`) | [Download for Linux (.rpm)](https://github.com/giacngo/palikey/releases/download/v0.1.0/fcitx5-palikey-0.1.0-1.x86_64.rpm) |
| **Android** | Google Play Store | [Get on Google Play](https://play.google.com/store/apps/details?id=org.giacngo.palikey) |
| **Android** | Direct Sideload APK | [Download Android APK](https://github.com/giacngo/palikey/releases/download/v0.1.0/PaliKey-0.1.0-universal.apk) |
| **iOS** | Apple App Store | [Download on the App Store](https://apps.apple.com/app/palikey-pali-keyboard/id6740000000) |
| **Web** | In-Browser Typing Box | [Use Online Web Tool](https://giacngo.org/tools/palikey/) |
| **Keyman** | Unified Keyboard Package (`.kmp`) | [Install Keyman Package](https://keyman.com/keyboards/install/palikey) |

---

## 3. Build From Source Summary

### Prerequisites
- **Rust Toolchain**: `cargo` 1.80+ (`rustup default stable`)
- **CMake**: 3.15+
- **Platform-Specific SDKs**:
  - macOS: Xcode 15+ / Command Line Tools (`swiftc`, `codesign`)
  - Linux: GCC 11+ or Clang 14+, `fcitx5-devel`, `libibus-1.0-dev`
  - Android: Android SDK 35, NDK 26+, JDK 17
  - iOS: Xcode 16+ on macOS
  - Windows: Visual Studio 2022 with C++ workload and Windows SDK

### Build Commands by Platform

#### 1. Core Rust Engine
```bash
cd palikey-engine
cargo build --release
cargo test
```

#### 2. macOS IMK Shell
```bash
cd palikey-macos
./build.sh
# Output bundle: build/PaliKey.app
```

#### 3. Web WASM Bundle
```bash
cd palikey-web
./build.sh
# Open dist/index.html in any browser, or run: node test.mjs
```

#### 4. Android Input Method
```bash
cd palikey-android
./build-engine.sh
./gradlew assembleDebug
# Output APK: app/build/outputs/apk/debug/app-debug.apk
```

#### 5. iOS Keyboard Extension
```bash
cd palikey-ios
./build-engine.sh
xcodegen generate # (if modifying project.yml)
xcodebuild -project PaliKey.xcodeproj -scheme PaliKey -destination 'generic/platform=iOS Simulator' build
```

#### 6. Linux Fcitx5 Addon
```bash
cd palikey-linux-fcitx5
mkdir build && cd build
cmake -DCMAKE_BUILD_TYPE=Release ..
make -j$(nproc)
# Optional packaging: cpack -G "DEB;RPM"
```

#### 7. Linux IBus Daemon
```bash
cd palikey-linux-ibus
mkdir build && cd build
cmake -DCMAKE_BUILD_TYPE=Release ..
make -j$(nproc)
# Optional packaging: cpack -G "DEB;RPM"
```

#### 8. Windows TSF Text Service (C++ COM DLL)
On Windows in an **x64 Native Tools Command Prompt for VS 2022**:
```cmd
cd palikey-engine
cargo build --target x86_64-pc-windows-msvc --release
cd ..\palikey-tsf
mkdir build & cd build
cmake -G "Visual Studio 17 2022" -A x64 ..
cmake --build . --config Release
# Register with admin rights:
copy Release\PaliKeyTSF.dll ..\PaliKeyTSF.dll
cd ..
register.bat
```

#### 9. Windows Portable Hook App (Rust Fallback)
```cmd
cd palikey-windows
cargo build --release
# Output executable: target/release/palikey.exe
```

---

## 4. Supported Input Schemes

All shells support the three unified transliteration schemes defined in `shared/schemes.json`:

1. **Velthuis** (Scholar standard):
   `aa` $\rightarrow$ ā, `ii` $\rightarrow$ ī, `uu` $\rightarrow$ ū, `.t` $\rightarrow$ ṭ, `.d` $\rightarrow$ ḍ, `.n` $\rightarrow$ ṇ, `.m` $\rightarrow$ ṃ, `"n` $\rightarrow$ ṅ, `~n` $\rightarrow$ ñ, `.l` $\rightarrow$ ḷ.
2. **PaliTelex** (Vietnamese Telex muscle memory):
   `aa` $\rightarrow$ ā, `ii` $\rightarrow$ ī, `uu` $\rightarrow$ ū, `tx` $\rightarrow$ ṭ, `dx` $\rightarrow$ ḍ, `nx` $\rightarrow$ ṇ, `mx` $\rightarrow$ ṃ, `ngw` $\rightarrow$ ṅ, `nw` / `ny` $\rightarrow$ ñ, `lx` $\rightarrow$ ḷ.
3. **SIL / Keyman** (Traditional transliteration):
   `-a` $\rightarrow$ ā, `-i` $\rightarrow$ ī, `-u` $\rightarrow$ ū, `.t` $\rightarrow$ ṭ, `.d` $\rightarrow$ ḍ, `.n` $\rightarrow$ ṇ, `.m` $\rightarrow$ ṃ, `>n` $\rightarrow$ ṅ, `~n` $\rightarrow$ ñ, `.l` $\rightarrow$ ḷ.

**Desktop Shell Extra**:
- **EasyUnicode**: `AltGr + key` (Windows/Linux) or `⌥ Option + key` (macOS) directly inserts diacritics bypassing the transducer.

---

## 5. License & Credits

- **License**: MIT License ([LICENSE](LICENSE)) — clean-room development.
- **Project Design**: Inspired by `vnkey`'s core-and-shell architecture pattern; no vnkey code is used.
- **Author**: Duc Kien Nguyen / giacngo.org tools initiative.
