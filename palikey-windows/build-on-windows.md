# Building PaliKey on Windows

This guide describes how to build, test, and run the standalone `palikey-windows` tray executable on a native Microsoft Windows machine.

---

## 1. Prerequisites

1. **Operating System**:
   - Windows 10 (64-bit) or Windows 11.
2. **C++ Build Tools (MSVC Linker)**:
   - Install **Visual Studio 2022** or **Visual Studio Build Tools**.
   - Ensure the workload **"Desktop development with C++"** is selected during installation (provides `link.exe` and Windows SDK import libraries `user32.lib`, `shell32.lib`, `kernel32.lib`).
3. **Rust Toolchain**:
   - Install Rust via [rustup.rs](https://rustup.rs):
     ```cmd
     winget install Rustlang.Rustup
     ```
   - Verify toolchain installation:
     ```cmd
     rustup default stable-x86_64-pc-windows-msvc
     rustc --version
     cargo --version
     ```

---

## 2. Build Instructions

Open a Command Prompt (`cmd.exe`) or PowerShell terminal:

```cmd
:: 1. Navigate to the palikey-windows directory
cd C:\path\to\palikey\palikey-windows

:: 2. Run unit tests
cargo test

:: 3. Build optimized release binary
cargo build --release
```

---

## 3. Expected Output Binary

Upon successful compilation, the release binary will be located at:
```
target\release\palikey-windows.exe
```

- **Binary Type**: Native 64-bit Windows GUI/Tray executable.
- **Dependencies**: Zero bundled third-party DLLs. Links exclusively with standard Windows system DLLs (`user32.dll`, `shell32.dll`, `kernel32.dll`).
- **Binary Size**: Under 1.5 MB uncompressed.
- **Portability**: 100% portable single executable — does not require installation.

---

## 4. Windows Defender SmartScreen Note

Because fresh development builds are unsigned, Windows Defender SmartScreen will display an alert on first launch:
> *"Windows protected your PC — Microsoft Defender SmartScreen prevented an unrecognized app from starting."*

### How to run:
1. Click **"More info"**.
2. Click **"Run anyway"**.
3. *Alternative*: Right-click `palikey-windows.exe` $\rightarrow$ **Properties** $\rightarrow$ Check **"Unblock"** at the bottom $\rightarrow$ Click **Apply** $\rightarrow$ **OK**.

*(Official release builds will be signed via SignPath OSS Authenticode code signing).*

---

## 5. How to Run at Startup

Because `palikey-windows` is portable, you can enable auto-start on login using either of the following standard Windows mechanisms:

### Method A: Windows Startup Folder (Recommended, No Admin Required)
1. Press `Win + R` to open the Run dialog.
2. Type `shell:startup` and press **Enter**.
   *(This opens `C:\Users\<Username>\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Startup`)*.
3. Right-click inside the folder $\rightarrow$ **New** $\rightarrow$ **Shortcut**.
4. Browse to your `palikey-windows.exe` file, name it `PaliKey`, and click **Finish**.

### Method B: Windows Registry Autorun
Run PowerShell as current user:
```powershell
Set-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\Run" `
  -Name "PaliKey" -Value "C:\Full\Path\To\palikey-windows.exe"
```

---

## 6. Configuration & Tray Menu

- **Configuration File**:
  Saved automatically to:
  `%APPDATA%\PaliKey\config.json` (e.g. `C:\Users\<Username>\AppData\Roaming\PaliKey\config.json`).
  Conforms to `shared/config.schema.json`.
- **System Tray Icon**:
  - Look for the PaliKey icon in the system notification area (lower right taskbar corner).
  - **Left-click or Right-click** to open the menu:
    - **Enabled (Pāḷi Input)**: Checkbox to toggle active IME hook on/off.
    - **Scheme Picker**:
      - `Velthuis (aa->ā, .t->ṭ, .m->ṃ)`
      - `PaliTelex (aa->ā, tx->ṭ, ngw->ṅ)`
      - `SIL / Keyman (-a->ā, .t->ṭ, >n->ṅ)`
    - **Niggahīta Orthography**:
      - `ṃ (U+1E43 Dot Below - PTS / SuttaCentral standard)`
      - `ṁ (U+1E41 Dot Above - VRI tradition)`
    - **Extended Sanskrit / IAST**: Toggle support for ṛ, ṝ, ḥ, ś, ṣ.
    - **Exit PaliKey**: Uninstalls hook cleanly and exits.
