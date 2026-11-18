# Pāḷi Keyboard

> **Product Name:** Pāḷi Keyboard (internal codename: `palikey`)

One Pāḷi input method for every platform — one Rust engine, one shared transliteration standard, and native shells for Windows, macOS, Linux, Android, iOS, and Web.

Clean-room open-source implementation (MIT License) following the core-and-shell architecture specified in [DESIGN.md](DESIGN.md).

---

## Download

Official pre-built binaries are attached to each [GitHub Release](https://github.com/babyskill/palikey/releases):

- **macOS (Universal arm64 + x86_64)**: `PaliKeyboard-macOS.zip`
  Universal InputMethodKit bundle (`PaliKey.app`). Extract to `~/Library/Input Methods/`, log out and back in, then enable in *System Settings $\rightarrow$ Keyboard $\rightarrow$ Text Input $\rightarrow$ Input Sources*.
- **Windows (x64)**: `PaliKeyboard-Windows.zip`
  Contains both the primary modern Text Services Framework COM service (`PaliKeyTSF.dll` with `register.bat` / `unregister.bat`) and the standalone zero-install portable tray hook (`palikey-windows.exe`), along with `INSTALL.txt`.

*Other platforms (Linux, Android, iOS, Web, Keyman) are currently build-from-source — see [Build From Source](#build-from-source-summary) below.*

---

## Platform Status Table

Verified levels below are factual records:

| Platform / Shell | Implementation | Phase | Verification Level | Host Environment |
|---|---|---|---|---|
| **Core Engine** (`palikey-engine`) | Rust transducer state machine + C FFI | Phase 0 | **Verified** — 36/36 cargo tests passed (18 unit tests, 18 golden corpus tests: Velthuis, PaliTelex, SIL, Vietnamese Telex). | Intel iMac (`x86_64-apple-darwin`) & CI |
| **macOS** (`palikey-macos`) | Native Swift InputMethodKit (IMK) universal bundle | Phase 1a & 8 | **CI-built release binaries; community testing welcome** — Universal (`arm64` + `x86_64`) bundle built via GitHub Actions / `build.sh`; headless smoke tests pass; live macOS end-user testing welcome. | macOS 15.7.9 / GitHub Actions runner |
| **Windows TSF** (`palikey-tsf`) | C++17 Text Services Framework COM DLL (`PaliKeyTSF.dll`) | Phase 4 | **CI-built release binaries; community testing welcome** — Implements `ITfTextInputProcessorEx`, `ITfKeyEventSink`, `ITfCompositionSink` (preedit underline). Built via GitHub Actions MSVC toolchain; Windows live registration & runtime testing welcome. | Windows Server 2022 / Windows 11 runner |
| **Windows Hook** (`palikey-windows`) | Standalone Rust system tray app (`WH_KEYBOARD_LL` + `SendInput`) | Phase 1c | **CI-built release binaries; community testing welcome** — Zero-install portable fallback built via GitHub Actions MSVC toolchain; Windows runtime testing welcome. | Windows Server 2022 / Windows 11 runner |
| **Linux Fcitx5** (`palikey-linux-fcitx5`) | Modern C++17 addon (`InputMethodEngineV2`) | Phase 3 | **Code-only verified** — CMake + CPack packaging. Compiled against stubs; shared config parser & EasyUnicode unit test passed linking `libpalikey_engine.a`; live Linux desktop runtime testing pending. | Code-only (Clang 17.0, Ubuntu/Debian/Arch) |
| **Linux IBus** (`palikey-linux-ibus`) | C11 standalone GObject daemon (`IBusEngine`) | Phase 3 | **Code-only verified** — CMake + CPack packaging, D-Bus component manifest. Compiled against stubs; unit test passed linking `libpalikey_engine.a`; live Linux desktop runtime testing pending. | Code-only (Clang 17.0, GNOME/Fedora/Ubuntu) |
| **Keyman** (`palikey-keyman`) | Keyman package (`palikey.kmn`, `palikey.kpj`) | Phase 1b | **Code-only verified** — Synthesized from `shared/schemes.json` via `generate_kmn.py`; unified Velthuis and PaliTelex with zero conflicts; Keyman Developer binary compilation pending. | Code-only (Keyman Developer) |
| **Web** (`palikey-web`) | WebAssembly (`wasm32-unknown-unknown`) + JS glue | Phase 1b | **Verified** — 41 KB standalone `.wasm`, single-page interactive UI with offline base64 fallback, 52/52 Node.js integration tests passed (`test.mjs`). | Node.js v20+, Chrome/Safari |
| **Android** (`palikey-android`) | Kotlin `InputMethodService` + C++ JNI bridge | Phase 2a | **Verified** — Zero network permissions (`INTERNET` absent). Dual layouts (Transducer + Dedicated Pāḷi row + Off). Native staticlibs built for `arm64-v8a`, `armeabi-v7a`, `x86_64`. Output debug APK verified (4.8 MB). | Android SDK 35, NDK 26.3 |
| **iOS** (`palikey-ios`) | Swift container app + programmatic UIKit keyboard extension | Phase 2b | **Verified** — `RequestsOpenAccess = false` (NO Full Access). Universal `PaliKeyEngine.xcframework` (`arm64`, `x86_64-sim`, `arm64-sim`). Full build succeeded in Xcode targeting iOS Simulator. | Xcode 26.3, iOS SDK 26.2 |

---

## Build From Source Summary

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
# Output executable: target/release/palikey-windows.exe
```

---

## Supported Input Schemes

All shells support the unified transliteration schemes defined in `shared/schemes.json`:

1. **Velthuis** (Scholar standard):
   `aa` $\rightarrow$ ā, `ii` $\rightarrow$ ī, `uu` $\rightarrow$ ū, `.t` $\rightarrow$ ṭ, `.d` $\rightarrow$ ḍ, `.n` $\rightarrow$ ṇ, `.m` $\rightarrow$ ṃ, `"n` $\rightarrow$ ṅ, `~n` $\rightarrow$ ñ, `.l` $\rightarrow$ ḷ.
2. **PaliTelex** (Vietnamese Telex muscle memory):
   `aa` $\rightarrow$ ā, `ii` $\rightarrow$ ī, `uu` $\rightarrow$ ū, `tx` $\rightarrow$ ṭ, `dx` $\rightarrow$ ḍ, `nx` $\rightarrow$ ṇ, `mx` $\rightarrow$ ṃ, `ngw` $\rightarrow$ ṅ, `nw` / `ny` $\rightarrow$ ñ, `lx` $\rightarrow$ ḷ.
3. **SIL / Keyman** (Traditional transliteration):
   `-a` $\rightarrow$ ā, `-i` $\rightarrow$ ī, `-u` $\rightarrow$ ū, `.t` $\rightarrow$ ṭ, `.d` $\rightarrow$ ḍ, `.n` $\rightarrow$ ṇ, `.m` $\rightarrow$ ṃ, `>n` $\rightarrow$ ṅ, `~n` $\rightarrow$ ñ, `.l` $\rightarrow$ ḷ.

**Desktop Shell Extra**:
- **EasyUnicode**: `AltGr + key` (Windows/Linux) or `⌥ Option + key` (macOS) directly inserts diacritics bypassing the transducer.

---

## License & Credits

- **License**: MIT License ([LICENSE](LICENSE)) — clean-room development.
- **Project Design**: Inspired by `vnkey`'s core-and-shell architecture pattern; no vnkey code is used.
- **Author**: Duc Kien Nguyen / giacngo.org tools initiative.
