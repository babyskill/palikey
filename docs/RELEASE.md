# PaliKey — Release Checklist & Distribution Playbook

This document specifies the standard release engineering protocol, artifact packaging, code signing, versioning, and distribution workflows for **PaliKey** across all supported platforms.

---

## 1. Core Principles & Versioning Rule

### 1.1 The Golden Rule: Engine Version = Product Version
PaliKey adheres strictly to Semantic Versioning (`MAJOR.MINOR.PATCH`).

The version declared in **`palikey-engine/Cargo.toml`** is the single canonical source of truth for the entire product:
```toml
[package]
name = "palikey-engine"
version = "0.1.0" # <- Canonical product version
```

Every platform shell and manifest **MUST** match this version string verbatim before tagging a release.

### 1.2 Version Synchronization Checklist
Prior to tagging any release, verify that all version descriptors are synchronized:

| File | Field / Location | Expected Value |
|------|------------------|----------------|
| `palikey-engine/Cargo.toml` | `package.version` | `"0.1.0"` |
| `palikey-windows/Cargo.toml` | `package.version` | `"0.1.0"` |
| `palikey-tsf/CMakeLists.txt` | `project(... VERSION ...)` | `0.1.0` |
| `palikey-tsf/src/palikey_tsf.rc` | `VER_FILEVERSION_STR` | `"0.1.0"` |
| `palikey-macos/Info.plist` | `CFBundleShortVersionString` | `"0.1.0"` |
| `palikey-macos/Info.plist` | `CFBundleVersion` | `"1"` (or build counter) |
| `palikey-android/app/build.gradle.kts` | `versionName` | `"0.1.0"` |
| `palikey-android/app/build.gradle.kts` | `versionCode` | `100` (monotonically increasing) |
| `palikey-ios/project.yml` | `CFBundleShortVersionString` | `"0.1.0"` |
| `palikey-linux-fcitx5/CMakeLists.txt` | `project(... VERSION ...)` | `0.1.0` |
| `palikey-linux-ibus/CMakeLists.txt` | `project(... VERSION ...)` | `0.1.0` |
| `palikey-web/crate/Cargo.toml` | `package.version` | `"0.1.0"` |
| `palikey-keyman/palikey.kmn` | `&VERSION` | `'0.1'` (Keyman uses M.m) |
| `shared/config.schema.json` | `properties.version.default` | `"0.1.0"` |

---

## 2. Per-Platform Release Artifacts

Every official GitHub release (`v<version>`) publishes the following artifacts:

| Platform | Shell / Component | Artifact Filename | Format | Architectures | Target Host / Store |
|----------|-------------------|-------------------|--------|---------------|---------------------|
| **macOS** | Native IMK Shell | `PaliKey-<version>-macOS.dmg` | Apple Disk Image (DMG) | Universal (`arm64` + `x86_64`) | Direct download (`~/Library/Input Methods/`) |
| **Windows** | Text Services Framework (Primary) | `PaliKeyTSF-<version>-Windows.zip` | ZIP archive (`PaliKeyTSF.dll` + `register.bat`) | `x64`, `Win32` | Direct download / System registration |
| **Windows** | Low-Level Hook (Portable Fallback) | `PaliKey-Hook-<version>-x64.zip` | ZIP archive (`palikey.exe` portable) | `x64` | Direct download (zero install) |
| **Linux** | Fcitx5 C++ Addon | `fcitx5-palikey_<version>_amd64.deb`<br>`fcitx5-palikey-<version>-1.x86_64.rpm` | Debian / RPM Packages | `x86_64`, `aarch64` | Apt / Dnf / Pacman repository |
| **Linux** | IBus C Daemon | `ibus-palikey_<version>_amd64.deb`<br>`ibus-palikey-<version>-1.x86_64.rpm` | Debian / RPM Packages | `x86_64`, `aarch64` | Apt / Dnf / Pacman repository |
| **Android** | Kotlin IME | `palikey-<version>-release.aab`<br>`PaliKey-<version>-universal.apk` | AAB (Play Store) & APK (Sideload) | `arm64-v8a`, `armeabi-v7a`, `x86_64` | Google Play Store & direct APK sideload |
| **iOS** | Swift Container + Keyboard Ext | `PaliKey.ipa` (Archive) | App Store submission | `arm64` | Apple App Store / TestFlight |
| **Web** | WASM Standalone Bundle | `palikey-web-<version>.zip` | Web bundle (`.wasm`, `.js`, `index.html`) | `wasm32` | Hosted on `giacngo.org/tools` |
| **Keyman** | Keyman Desktop/Mobile Package | `palikey.kmp` | Keyman Keyboard Package | All | Keyman Keyboard Registry |

---

## 3. Code Signing & Notarization Workflows

Machine assignment rule (per `DESIGN.md` §8): all automated compilation occurs on the build node (iMac); release signing, device certification, and store submissions occur via the designated release node (MacBook / Mac mini).

### 3.1 macOS: Developer ID Application & Apple Notarization
PaliKey uses the Developer ID Application certificate registered for PaliDict (`A9K2C955CY`):

1. **Codesign the `.app` bundle with Hardened Runtime**:
   ```bash
   codesign --force --deep --options runtime \
     --sign "Developer ID Application: Duc Kien Nguyen (A9K2C955CY)" \
     build/PaliKey.app
   ```
2. **Build and codesign the DMG**:
   ```bash
   hdiutil create -volname "PaliKey" -srcfolder build/PaliKey.app -ov -format UDZO PaliKey-0.1.0-macOS.dmg
   codesign --sign "Developer ID Application: Duc Kien Nguyen (A9K2C955CY)" PaliKey-0.1.0-macOS.dmg
   ```
3. **Submit for Apple Notarization**:
   ```bash
   xcrun notarytool submit PaliKey-0.1.0-macOS.dmg \
     --keychain-profile "notary-palikey" \
     --wait
   ```
4. **Staple the notarization ticket**:
   ```bash
   xcrun stapler staple PaliKey-0.1.0-macOS.dmg
   spctl -a -t open --context context:primary-signature -v PaliKey-0.1.0-macOS.dmg
   ```

### 3.2 Windows: SignPath / Authenticode EV Certificate
Windows binaries (`PaliKeyTSF.dll` and `palikey.exe`) must be signed to prevent SmartScreen warnings:

1. **Free OSS Signing via SignPath.io** (used by vnkey) or internal Authenticode certificate.
2. **Sign with RFC 3161 Timestamping**:
   ```cmd
   signtool.exe sign /v /fd SHA256 /tr http://timestamp.digicert.com /td SHA256 /sha1 <CERT_THUMBPRINT> PaliKeyTSF.dll
   signtool.exe sign /v /fd SHA256 /tr http://timestamp.digicert.com /td SHA256 /sha1 <CERT_THUMBPRINT> palikey.exe
   ```
3. **Verify Signatures**:
   ```cmd
   signtool.exe verify /pa /v PaliKeyTSF.dll
   ```

### 3.3 Android: Google Play App Signing & Sideload Keystore
1. **Build release bundles**:
   ```bash
   cd palikey-android
   ./gradlew bundleRelease assembleRelease
   ```
2. **Sign APKs for direct sideloading via `apksigner`**:
   ```bash
   apksigner sign --ks palikey-release.jks \
     --ks-key-alias palikey \
     --out PaliKey-0.1.0-universal.apk \
     app/build/outputs/apk/release/app-release-unsigned.apk
   ```
3. **Verify APK integrity**:
   ```bash
   apksigner verify --verbose PaliKey-0.1.0-universal.apk
   ```
4. **Upload AAB**: Submit `app/build/outputs/bundle/release/app-release.aab` to Google Play Console.

### 3.4 iOS: App Store Submission via Xcode
1. In `palikey-ios/`, open `PaliKey.xcodeproj` in Xcode on the designated release Mac.
2. Select target scheme **`PaliKey`** $\rightarrow$ **Any iOS Device (arm64)**.
3. Select **Product** $\rightarrow$ **Archive**.
4. In Organizer, select **Distribute App** $\rightarrow$ **App Store Connect** $\rightarrow$ **Upload**.
5. Ensure `RequestsOpenAccess = false` in `PaliKeyKeyboard/Resources/Info.plist` (NO Full Access).
6. Verify Apple Privacy Manifest (`PrivacyInfo.xcprivacy`) validates with 0 tracking data types.

---

## 4. Distribution Hub Update Protocol (`giacngo.org/tools`)

### 4.1 Strict Direct Links Rule
Per Kien's explicit product requirement (`DESIGN.md` §6):
> **DIRECT FILE LINKS ONLY.**
> No links to vendor homepages, generic product landing pages, or third-party intermediary sites.
> Every single download button on `giacngo.org/tools` MUST either trigger an immediate file download from GitHub Releases or directly open the platform's native store page.

### 4.2 Canonical Distribution URLs Table
When publishing release `v0.1.0`, update the links on `giacngo.org/tools` using this exact mapping:

| Button Label on Hub | Target Direct Link URL | Description |
|---------------------|------------------------|-------------|
| **Download for macOS** | `https://github.com/giacngo/palikey/releases/download/v0.1.0/PaliKey-0.1.0-macOS.dmg` | Direct DMG download |
| **Download for Windows (TSF)** | `https://github.com/giacngo/palikey/releases/download/v0.1.0/PaliKeyTSF-0.1.0-Windows.zip` | Direct TSF ZIP download |
| **Download for Windows (Portable)** | `https://github.com/giacngo/palikey/releases/download/v0.1.0/PaliKey-Hook-0.1.0-x64.zip` | Direct Portable EXE download |
| **Download for Linux (.deb)** | `https://github.com/giacngo/palikey/releases/download/v0.1.0/fcitx5-palikey_0.1.0_amd64.deb` | Direct Debian package |
| **Download for Linux (.rpm)** | `https://github.com/giacngo/palikey/releases/download/v0.1.0/fcitx5-palikey-0.1.0-1.x86_64.rpm` | Direct RPM package |
| **Get on Google Play** | `https://play.google.com/store/apps/details?id=org.giacngo.palikey` | Direct Google Play Store listing |
| **Download Android APK** | `https://github.com/giacngo/palikey/releases/download/v0.1.0/PaliKey-0.1.0-universal.apk` | Direct APK sideload |
| **Download on the App Store** | `https://apps.apple.com/app/palikey-pali-keyboard/id6740000000` | Direct iOS App Store listing |
| **Use Online Web Tool** | `https://giacngo.org/tools/palikey/` | Embedded WebAssembly typing tool |
| **Install Keyman Package** | `https://keyman.com/keyboards/install/palikey` | One-click direct Keyman package install |

---

## 5. Step-by-Step Release Execution Checklist

### Phase 1: Pre-Release Verification (on iMac Build Node)
- [ ] Run core engine tests:
  ```bash
  cargo test --package palikey-engine
  ```
  *(Must report 31 passed: 14 unit tests + 17 golden tests, 0 failed).*
- [ ] Verify `wasm32` Web tests:
  ```bash
  cd palikey-web && node test.mjs
  ```
  *(Must report 52 passed, 0 failed).*
- [ ] Verify macOS smoke test:
  ```bash
  cd palikey-macos && ./build.sh
  ```
  *(Must report `Smoke test passed: all transliteration assertions succeeded`).*
- [ ] Verify Linux config parser & EasyUnicode tests:
  ```bash
  cd palikey-linux-fcitx5 && ./test.sh
  cd ../palikey-linux-ibus && ./test.sh
  ```
- [ ] Verify Windows targets typecheck:
  ```bash
  cargo check --package palikey-engine --target x86_64-pc-windows-msvc
  cargo check --package palikey-engine --target i686-pc-windows-msvc
  ```

### Phase 2: Git Tagging & GitHub Release Creation
- [ ] Ensure working directory is clean on `main` branch.
- [ ] Tag the release:
  ```bash
  git tag -a v0.1.0 -m "PaliKey v0.1.0 release"
  git push origin v0.1.0
  ```
- [ ] Upload all packaged, signed artifacts to the GitHub Release.

### Phase 3: Hub Deployment (`giacngo.org/tools`)
- [ ] Deploy updated WebAssembly bundle (`palikey.wasm`, `palikey.js`, `schemes.json`) to `https://giacngo.org/tools/palikey/`.
- [ ] Update download button links on `giacngo.org/tools` matching the Canonical Direct Links table (§4.2).
- [ ] Click every link on a live browser to confirm each triggers direct download without redirect loops or 404s.
