# PaliKey iOS — Container App & Keyboard Extension

PaliKey for iOS provides a fast, offline, and privacy-preserving Pāḷi input method extension and container application for iPhone and iPad (iOS 15.0+).

It embeds the cross-platform Rust `palikey-engine` via an XCFramework (`PaliKeyEngine.xcframework`) linking `libpalikey_engine.a` directly through the stable C ABI declared in `palikey-engine/include/palikey.h`.

---

## 1. Architecture & Design

```
┌────────────────────────────────────────────────────────┐
│                      PaliKey.app                       │
│  (SwiftUI Container: Settings, Scheme Picker, Docs)    │
└───────────────────────────┬────────────────────────────┘
                            │ App Groups UserDefaults
                            │ ("group.org.giacngo.palikey")
┌───────────────────────────▼────────────────────────────┐
│                  PaliKeyKeyboard.appex                 │
│      (UIInputViewController + Pure UIKit Keyboard)     │
├────────────────────────────────────────────────────────┤
│                    PaliEngine.swift                    │
│             (Swift Wrapper & Memory Safety)            │
├────────────────────────────────────────────────────────┤
│               PaliKeyEngine.xcframework                │
│    - ios-arm64 (Device staticlib)                      │
│    - ios-arm64_x86_64-simulator (Universal staticlib)  │
│    - Headers: palikey.h + module.modulemap             │
└────────────────────────────────────────────────────────┘
```

### Key Technical Characteristics:
1. **Low Memory Footprint & Rapid Startup**:
   - iOS imposes an aggressive memory watchdog on keyboard extensions (~30–50MB maximum).
   - Built with pure, programmatic UIKit (`UIStackView`, `UIButton`) with zero storyboards, XIBs, or heavy runtime dependencies. Total extension memory usage remains well under 12MB.
2. **Per-Session Engine Instances**:
   - Each typing session initializes a dedicated `PalikeyEngine` instance via `palikey_new`, avoiding global singleton state conflicts across app switches.
3. **App Groups Synchronization**:
   - User preferences (transliteration scheme, niggahīta variant, keyboard mode) are shared between the container app and keyboard extension using App Groups (`group.org.giacngo.palikey`), with an automatic graceful fallback to `UserDefaults.standard` for un-provisioned development builds and simulators.
4. **Reproducible XcodeGen Spec**:
   - `project.yml` allows deterministic regeneration of `PaliKey.xcodeproj` via `xcodegen generate`. The generated Xcode project is also checked into the repository for immediate opening without third-party tooling.

---

## 2. Input Modes & Typing Schemes

PaliKey provides three input modes, matching the behavior across Android, macOS, and Linux:

| Mode | Indicator | Description |
|---|---|---|
| **Transducer** | `⚡ <Scheme>` | Real-time transliteration via the active scheme table. Backspace reverses composition. |
| **Pāḷi Row** | `🔤 Pāḷi` | Dedicated on-screen diacritic row (`ā ī ū ṅ ñ ṭ ḍ ṇ ḷ ṃ/ṁ`). Tapping inserts immediately and clears composition. |
| **Off** | `⌨️ Off` | Plain passthrough QWERTY mode for typing untransliterated English or symbols. |

### Transliteration Schemes:
- **Velthuis** (PTS standard): `aa` → `ā`, `.t` → `ṭ`, `"n` → `ṅ`, `~n` → `ñ`, `.m` → `ṃ`.
- **PaliTelex** (Vietnamese Telex style): `aa` → `ā`, `tx` → `ṭ`, `ngw` → `ṅ`, `nw`/`ny` → `ñ`, `mx` → `ṃ`.
- **SIL** (Keyman style): `-a` → `ā`, `.t` → `ṭ`, `>n` → `ṅ`, `~n` → `ñ`, `.m` → `ṃ`.

### Niggahīta Variants:
- `ṃ` (U+1E43, Dot Below) — PTS & SuttaCentral convention (default).
- `ṁ` (U+1E41, Dot Above) — VRI tradition.

---

## 3. Strict Privacy Posture — No Full Access

PaliKey is designed with a strict zero-data-collection security architecture:

1. **`RequestsOpenAccess` is FALSE**:
   - Declared in `PaliKeyKeyboard/Resources/Info.plist`:
     ```xml
     <key>RequestsOpenAccess</key>
     <false/>
     ```
2. **No Network Access**:
   - Without Full Access, iOS completely sandboxes the extension from network sockets. PaliKey cannot make HTTP requests or send telemetry beacons.
3. **No Keystroke Logging or Cloud Sync**:
   - Keystrokes are processed strictly in local RAM by the embedded Rust engine. Nothing is saved to disk, recorded, or sent off-device.
4. **Apple Privacy Manifests**:
   - Both `PaliKey` and `PaliKeyKeyboard` include `PrivacyInfo.xcprivacy` declaring `NSPrivacyTracking = false` and an empty `NSPrivacyCollectedDataTypes` list.

### Full Access Trade-offs:
- **Clipboard**: The keyboard cannot paste text from the system clipboard.
- **Microphone / Dictation**: Dictation requires Full Access; PaliKey does not request it.
- **Audio Clicks**: Keyboard click sounds (`UIDevice.current.playInputClick()`) will only sound if the user has enabled "Keyboard Clicks" in iOS **Settings → Sounds & Haptics**.

---

## 4. How to Build & Run in Xcode

### Prerequisites:
- macOS with Xcode 15+ (tested on Xcode 26.3 on macOS 15.7).
- Rust toolchain (1.80+) with iOS targets installed:
  ```bash
  rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios
  ```
- (Optional) `xcodegen` (`brew install xcodegen`) if modifying `project.yml`.

### Step 1: Build Engine & Generate XCFramework
Run the automated build script from the repository root or `palikey-ios/`:
```bash
cd /Users/nhai/Dev/palikey/palikey-ios
./build-engine.sh
```
This script:
1. Compiles `palikey-engine` release static libraries for `aarch64-apple-ios`, `aarch64-apple-ios-sim`, and `x86_64-apple-ios`.
2. Strips debug symbols (`xcrun strip -S`).
3. Uses `lipo` to combine `aarch64-apple-ios-sim` and `x86_64-apple-ios` into a universal simulator static library.
4. Creates `Frameworks/PaliKeyEngine.xcframework` via `xcodebuild -create-xcframework` with C headers and Clang module map.

### Step 2: Open and Build Project in Xcode
```bash
open PaliKey.xcodeproj
```
1. Select the `PaliKey` scheme.
2. Select any target destination:
   - iOS Simulator (e.g. iPhone 16 Pro)
   - Connected iOS device
3. Press **Cmd + B** to build or **Cmd + R** to run.

To regenerate `PaliKey.xcodeproj` from `project.yml`:
```bash
xcodegen generate
```

---

## 5. How to Enable the Keyboard in iOS

1. Launch the **PaliKey** app on your device or simulator to initialize default settings.
2. Open iOS **Settings**.
3. Navigate to **General** → **Keyboard** → **Keyboards**.
4. Tap **Add New Keyboard...**.
5. Under **Third-Party Keyboards**, tap **PaliKey**.
6. Switch to any app (Notes, Messages, Safari, or the live test area inside the PaliKey container app).
7. Tap the globe icon (`🌐`) on the iOS keyboard to switch to PaliKey.
