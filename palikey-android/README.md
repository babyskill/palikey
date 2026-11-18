# PaliKey for Android

PaliKey Android is a clean-room, privacy-first Android Input Method Service (IME) for typing Pāḷi with diacritics. It embeds the shared cross-platform `palikey-engine` written in Rust via a high-performance JNI bridge.

---

## 1. Architecture & Design Principles

```
┌────────────────────────────────────────────────────────────────────────┐
│                        Android User Interface                          │
│                                                                        │
│   ┌────────────────────────────────────────────────────────────────┐   │
│   │               Pāḷi Row (Toggleable / Dedicated)                │   │
│   │    [ ā ] [ ī ] [ ū ] [ ṅ ] [ ñ ] [ ṭ ] [ ḍ ] [ ṇ ] [ ḷ ] [ ṃ ]  │   │
│   └────────────────────────────────────────────────────────────────┘   │
│   ┌────────────────────────────────────────────────────────────────┐   │
│   │                      QWERTY Letter Rows                        │   │
│   │     q  w  e  r  t  y  u  i  o  p / a  s  d  f  g  h  j  k  l   │   │
│   │     [⇧]  z  x  c  v  b  n  m  [⌫]                              │   │
│   └────────────────────────────────────────────────────────────────┘   │
│   ┌────────────────────────────────────────────────────────────────┐   │
│   │     [?123]   [⚡ Mode]   [ Pāḷi Keyboard ]   [.]   [↵]        │   │
│   └────────────────────────────────────────────────────────────────┘   │
└────────────────────────────────────┬───────────────────────────────────┘
                                     │ Touch Keystrokes
                                     ▼
┌────────────────────────────────────────────────────────────────────────┐
│             PaliKeyService (InputMethodService, Kotlin)                │
│                                                                        │
│  - Mode 0 (Transducer): Transliterates letter sequences (Velthuis/     │
│                         PaliTelex/SIL) via Rust state machine.         │
│  - Mode 1 (Pāḷi Row):   Dedicated row commits Pāḷi chars directly;     │
│                         letter keys pass through.                      │
│  - Mode 2 (Off):        Plain QWERTY passthrough.                      │
└────────────────────────────────────┬───────────────────────────────────┘
                                     │ JNI Calls
                                     ▼
┌────────────────────────────────────────────────────────────────────────┐
│             JNI Bridge (native-lib.cpp & NativeEngine.kt)              │
│                                                                        │
│  - Statically links libpalikey_engine.a via C ABI (include/palikey.h)  │
│  - Zero overhead, isolated session handles, no dynamic linker issues   │
└────────────────────────────────────┬───────────────────────────────────┘
                                     │ C ABI FFI
                                     ▼
┌────────────────────────────────────────────────────────────────────────┐
│                     palikey-engine (Rust Core)                         │
│                                                                        │
│  - Deterministic state machine & scheme tables                         │
│  - Single-step backspace undo and buffer flushing                      │
└────────────────────────────────────────────────────────────────────────┘
```

### Key Design Tenets:
1. **Strictly Offline & Zero Permissions**: No `android.permission.INTERNET`. Zero telemetry, zero analytics, zero network access.
2. **Zero Third-Party Dependencies**: Built exclusively with plain Android framework views (`android.widget.*`, `android.view.*`). No deprecated `KeyboardView` XML inflate overhead or heavy UI dependencies.
3. **Dual Input Layouts**:
   - **Transducer Mode**: Standard QWERTY typing with automatic transliteration via Velthuis (`aa`→ā, `.t`→ṭ, `"n`→ṅ, `~n`→ñ, `.m`→ṃ), PaliTelex (`aa`→ā, `tx`→ṭ, `ngw`→ṅ, `nw`→ñ, `mx`→ṃ), or SIL (`-a`→ā, `.t`→ṭ, `>n`→ṅ).
   - **Pāḷi Row Mode**: Dedicated on-screen row for all diacritic characters (`ā ī ū ṅ ñ ṭ ḍ ṇ ḷ ṃ`) with uppercase shift support.
   - **Mode Key**: Single-tap cycling: `Transducer` → `Pāḷi Row` → `Off (plain)`.
4. **Clean JNI Architecture**: C++ JNI bridge (`native-lib.cpp`) statically links `libpalikey_engine.a` compiled directly from `palikey-engine`. The engine source code remains completely untouched and platform-agnostic.

---

## 2. Directory Structure

```
palikey-android/
├── app/
│   ├── build.gradle.kts           // Application Gradle config (minSdk 26, targetSdk 35)
│   ├── proguard-rules.pro         // Proguard preservation rules for JNI classes
│   └── src/
│       └── main/
│           ├── AndroidManifest.xml // Manifest declaring PaliKeyService & SettingsActivity
│           ├── cpp/
│           │   ├── CMakeLists.txt  // Native CMake configuration
│           │   └── native-lib.cpp  // JNI bridge linking palikey-engine via C ABI
│           ├── java/org/giacngo/palikey/
│           │   ├── AppPreferences.kt    // SharedPreferences manager (schemes, niggahīta)
│           │   ├── KeyboardView.kt      // Programmatic QWERTY + Pāḷi row virtual keyboard
│           │   ├── NativeEngine.kt      // High-level Kotlin wrapper for JNI calls
│           │   ├── PaliKeyService.kt    // InputMethodService implementation
│           │   └── SettingsActivity.kt  // Configuration activity & live test area
│           ├── jniLibs/                 // Prebuilt native libraries
│           │   ├── arm64-v8a/           // aarch64 (libpalikey_jni.so, libpalikey_engine.so)
│           │   ├── armeabi-v7a/         // armv7 (libpalikey_jni.so, libpalikey_engine.so)
│           │   └── x86_64/              // x86_64 emulator & Chromebooks
│           └── res/
│               ├── drawable/            // Vector icons (ic_launcher.xml)
│               ├── values/              // Strings, colors, styles
│               └── xml/
│                   └── method.xml       // Input method subtype declaration
├── build-engine.sh                // Multi-arch Rust + JNI compiler script
├── build.gradle.kts               // Root build script (AGP 9.1.0)
├── settings.gradle.kts            // Root settings
├── gradle.properties              // JVM args & AndroidX flags
├── gradlew / gradlew.bat          // Gradle wrapper executable
└── gradle/wrapper/                // Gradle wrapper distribution (9.3.1)
```

---

## 3. Prerequisites & Setup

1. **Java Development Kit**: JDK 17 (verified with Oracle JDK 17 / OpenJDK 17).
2. **Android SDK**: API level 26 to 35.
   - Set `ANDROID_HOME` or create `local.properties`:
     ```properties
     sdk.dir=/Users/nhai/Library/Android/sdk
     ```
3. **Android NDK**: NDK r26 or newer (verified with NDK `26.3.11579264`).
4. **Rust Toolchain**:
   ```bash
   rustup target add aarch64-linux-android
   rustup target add armv7-linux-androideabi
   rustup target add x86_64-linux-android
   ```

---

## 4. Build Instructions

### Step 1: Compile Native Engine & JNI Libraries
Run `build-engine.sh` from the repository root or `palikey-android/`:

```bash
cd palikey-android
./build-engine.sh
```

This script:
- Auto-detects the active Android NDK.
- Uses NDK Clang wrappers as Cargo linkers.
- Cross-compiles `palikey-engine` in `--release` mode for all 3 architectures:
  - `aarch64-linux-android` (`arm64-v8a`)
  - `armv7-linux-androideabi` (`armeabi-v7a`)
  - `x86_64-linux-android` (`x86_64`)
- Compiles `native-lib.cpp` into self-contained `libpalikey_jni.so` libraries linking `libpalikey_engine.a`.
- Strips debug symbols with `llvm-strip` into `app/src/main/jniLibs/<abi>/`.

### Step 2: Build the Android APK
Run Gradle wrapper:

```bash
cd palikey-android
./gradlew assembleDebug
```

The output APK will be generated at:
```
app/build/outputs/apk/debug/app-debug.apk
```

To assemble a release APK:
```bash
./gradlew assembleRelease
```

---

## 5. Settings & Activation Flow

1. **Settings Screen**:
   - Open the **Pāḷi Keyboard** app from your launcher.
   - Tap **"1. Enable Pāḷi Keyboard in Settings"** → opens Android's system "Manage on-screen keyboards" settings page. Toggle Pāḷi Keyboard on.
   - Tap **"2. Select Pāḷi Keyboard"** → prompts the Android input method picker. Select Pāḷi Keyboard.
2. **Configuration Options**:
   - **Transliteration Scheme**: Choose between Velthuis, PaliTelex, or SIL.
   - **Niggahīta Variant**: Choose `ṃ` (U+1E43, Dot Below - PTS / SuttaCentral) or `ṁ` (U+1E41, Dot Above - VRI).
   - **Extended Set**: Enable/disable Sanskrit IAST characters (`ṛ ṝ ḹ ḥ ś ṣ ē ō`).
   - **Live Test Area**: An interactive input field is embedded directly in the Settings screen to immediately test typing without leaving the app.

---

## 6. Honest Verification Status (Tested on this iMac)

| Check | Tool / Version | Result |
|---|---|---|
| **Host OS** | macOS 15.7.9 (x86_64) | Verified |
| **JDK** | Java 17.0.14 LTS (`/usr/bin/java`) | Verified |
| **Android SDK** | Platforms API 26–37, Build-Tools 35.0.0 | Verified |
| **Android NDK** | NDK `26.3.11579264` (`darwin-x86_64` LLVM) | Verified |
| **Rust Toolchains** | `aarch64-linux-android`, `armv7-linux-androideabi`, `x86_64-linux-android` | Verified (installed) |
| **`build-engine.sh`** | Cargo release + Clang++ JNI compile | **PASSED** (all 3 ABIs generated & stripped) |
| **Engine Tests** | `cargo test --package palikey-engine` | **31/31 PASSED** (14 unit + 17 golden) |
| **Gradle Build** | `./gradlew assembleDebug` (AGP 9.1.0, Gradle 9.3.1) | **PASSED** (in 36s) |
| **Generated APK** | `app/build/outputs/apk/debug/app-debug.apk` | **4.8 MB** (contains all 3 ABIs, DEX, resources) |
