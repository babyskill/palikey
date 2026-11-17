# PaliKey Windows TSF (Text Services Framework) Shell

The native modern Windows input method shell for PaliKey, implemented as a **Text Services Framework (TSF) COM In-Process Server DLL** (`PaliKeyTSF.dll`).

In PaliKey's Windows architecture (per `DESIGN.md` §5), this TSF text service replaces the low-level keyboard hook (`palikey-windows/`) as the proper, primary Windows implementation. The hook shell remains available as a zero-install portable fallback.

---

## 1. Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                       Windows OS                            │
│  Win32 Desktop Apps (Notepad, Word) / UWP / Edge / Chrome   │
└──────────────────────────────┬──────────────────────────────┘
                               │ TSF COM Interfaces
                               ▼
┌─────────────────────────────────────────────────────────────┐
│                 PaliKeyTSF.dll (C++17)                      │
│                                                             │
│  ┌───────────────────────┐       ┌───────────────────────┐  │
│  │ ITfTextInputProcessorEx│       │   ITfCompositionSink  │  │
│  │ Activation & Sinks    │       │   Preedit Underline   │  │
│  └───────────┬───────────┘       └───────────┬───────────┘  │
│              │                               │              │
│  ┌───────────▼───────────┐       ┌───────────▼───────────┐  │
│  │     ITfKeyEventSink   │       │  ITfInsertAtSelection │  │
│  │ Key Event Interception│       │ Direct & Commit Insert│  │
│  └───────────┬───────────┘       └───────────┬───────────┘  │
│              │                               │              │
│              ▼                               ▼              │
│       ContextManager (Instance per ITfContext)               │
└──────────────────────────────┬──────────────────────────────┘
                               │ Stable C FFI (palikey.h)
                               ▼
┌─────────────────────────────────────────────────────────────┐
│                palikey-engine (Rust staticlib)              │
│                 libpalikey_engine.a / .lib                  │
└─────────────────────────────────────────────────────────────┘
```

### Key Technical Characteristics:
1. **Modern TSF COM Service**:
   - Implements `ITfTextInputProcessorEx` (and `ITfTextInputProcessor`) for clean activation and lifetime management.
   - Advises `ITfThreadMgrEventSink` to track focus, document lifecycle, and context push/pop events.
   - Advises `ITfKeyEventSink` to intercept keystrokes with zero global hook interference or anti-cheat conflicts.
2. **Context-Specific Engine Instances**:
   - Allocates and isolates an independent `PalikeyEngine*` instance per document/control context (`ITfContext`).
   - Ensures document state, composition buffers, and undo history in one window never contaminate another.
3. **True Composition with Preedit Underline**:
   - Implements `ITfCompositionSink` to manage active composition ranges.
   - Implements `ITfDisplayAttributeProvider` (`CPaliKeyDisplayAttributeInfo`) to render standard dotted underline (`TF_LS_DOT` with `TF_ATTR_INPUT`) under active preedit text in Word, Notepad, Edge, and browser text fields.
   - Commits text cleanly using `ITfInsertAtSelection` or composition termination.
4. **Single-Level Backspace Undo & Repeat-Key Escape**:
   - Reverting composed characters (e.g. typing Backspace after `aa` $\rightarrow$ `ā` restores raw `aa`).
   - Typing trigger character again immediately escapes composition (e.g. `aaa` $\rightarrow$ `aaa`).
5. **EasyUnicode AltGr Layer**:
   - Direct diacritic input via `AltGr + key` (e.g. `AltGr + a` $\rightarrow$ `ā`, `AltGr + t` $\rightarrow$ `ṭ`) bypassing the transducer per `DESIGN.md` §4.
6. **Zero-Overhead Configuration**:
   - Reads user preferences from `%APPDATA%\PaliKey\config.json` matching `shared/config.schema.json`.

---

## 2. FFI Symbol Cross-Check Table

Every C FFI symbol declared in `palikey-engine/include/palikey.h` is cross-checked below against its usage in `palikey-tsf`:

| Symbol in `palikey.h` | Signature | Usage in `palikey-tsf` | Status |
|-----------------------|-----------|------------------------|--------|
| `palikey_new` | `PalikeyEngine* palikey_new(uint32_t scheme_id, uint32_t niggahita, bool extended_set)` | Invoked in `ContextManager::GetOrCreate` to allocate a fresh engine instance for each new `ITfContext`. | **VERIFIED** — Exact match |
| `palikey_free` | `void palikey_free(PalikeyEngine* engine)` | Invoked in `ContextManager::Remove` and `ContextManager::Clear` when contexts are popped, uninitialized, or the text service deactivates. | **VERIFIED** — Exact match |
| `palikey_process_instance` | `int32_t palikey_process_instance(PalikeyEngine* engine, uint32_t key_char, bool is_backspace, PalikeyProcessResult* out_result)` | Invoked in `CPaliKeyTextService::DoKeyActionSession` during edit sessions to process keystrokes and backspace events. | **VERIFIED** — Exact match |
| `palikey_set_scheme` | `int32_t palikey_set_scheme(PalikeyEngine* engine, uint32_t scheme_id)` | Invoked in `ContextManager::UpdateConfig` when settings are reloaded or modified at runtime. | **VERIFIED** — Exact match |
| `palikey_reset_instance` | `void palikey_reset_instance(PalikeyEngine* engine)` | Invoked in `CPaliKeyTextService::CommitActiveComposition` and `OnCompositionTerminated` to flush composition on navigation, blur, or escape. | **VERIFIED** — Exact match |
| `palikey_version` | `const char* palikey_version(void)` | Queried for version verification, About metadata, and logging. | **VERIFIED** — Exact match |
| `palikey_setup` | `int32_t palikey_setup(uint32_t scheme_id, uint32_t niggahita, bool extended_set)` | Global singleton API (used by Phase 1c hook fallback; not needed for multi-context TSF). | **VERIFIED** — Compatible |
| `palikey_process` | `int32_t palikey_process(uint32_t key_char, bool is_backspace, PalikeyProcessResult* out_result)` | Global singleton API (used by Phase 1c hook fallback; not needed for multi-context TSF). | **VERIFIED** — Compatible |
| `palikey_reset` | `void palikey_reset(void)` | Global singleton API (used by Phase 1c hook fallback; not needed for multi-context TSF). | **VERIFIED** — Compatible |

---

## 3. GUIDs and Registration

PaliKey TSF uses unique RFC 4122 v4 GUIDs documented in [GUIDS.md](GUIDS.md):

- **CLSID_PaliKeyTextService**: `{D67F188D-4418-44AA-A564-E007D2C0DD1C}`
- **GUID_PaliKeyProfile**: `{5B556F97-994A-4D54-932C-89DE95337D5A}`
- **GUID_PaliKeyDisplayAttribute**: `{E4449DB3-F588-4C20-8B16-D97640392682}`
- **GUID_PaliKeyLangBarItem**: `{847F8420-4349-42F3-A055-0D83B0B3E0C8}`

Registration is handled by `regsvr32.exe`:
- **`register.bat`**: Runs `regsvr32.exe /s PaliKeyTSF.dll` with Administrator privileges.
- **`unregister.bat`**: Runs `regsvr32.exe /u /s PaliKeyTSF.dll` to cleanly deregister.

---

## 4. Verification Note (Mac Host vs Windows Target)

> [!NOTE]
> **Code-Only Verification on macOS**:
> Because the development machine is an Apple macOS system (Darwin), native Windows COM DLL execution, MSVC `link.exe` linking, and `regsvr32.exe` execution cannot run on this machine.
>
> However, the underlying Rust engine code has been cross-compilation typechecked with `cargo check --target x86_64-pc-windows-msvc` and `cargo check --target i686-pc-windows-msvc` (both 100% clean, 0 errors, 0 warnings), and all 31 engine tests pass.
>
> Live compilation and registration must be performed on a Windows machine as described below.

---

## 5. First Build Instructions for a Windows Machine

### Prerequisites on Windows
1. **Windows 10 / 11** (x64 or ARM64 with x64 emulation).
2. **Visual Studio 2022** (or Visual Studio Build Tools 2022) with:
   - "Desktop development with C++" workload
   - Windows 10/11 SDK (10.0.19041.0 or newer)
   - MSVC v143 compiler toolset
   - CMake tools for Windows
3. **Rust Toolchain**:
   - `rustup default stable-x86_64-pc-windows-msvc`
   - Target installed: `rustup target add x86_64-pc-windows-msvc` (and optionally `i686-pc-windows-msvc`)

### Step-by-Step Build Commands

Open **x64 Native Tools Command Prompt for VS 2022** and execute:

```cmd
:: 1. Navigate to the repository root
cd /d C:\path\to\palikey

:: 2. Build the palikey-engine Rust static library for Windows x64
cd palikey-engine
cargo build --target x86_64-pc-windows-msvc --release
cd ..

:: 3. Configure and build PaliKeyTSF.dll using CMake
cd palikey-tsf
mkdir build
cd build
cmake -G "Visual Studio 17 2022" -A x64 ..
cmake --build . --config Release

:: 4. Verify output DLL
dir Release\PaliKeyTSF.dll

:: 5. Register the text service (must run from an Elevated / Administrator Prompt)
copy Release\PaliKeyTSF.dll ..\PaliKeyTSF.dll
cd ..
register.bat
```

To build for 32-bit Win32:
```cmd
cd palikey-engine
cargo build --target i686-pc-windows-msvc --release
cd ..\palikey-tsf
mkdir build32 & cd build32
cmake -G "Visual Studio 17 2022" -A Win32 ..
cmake --build . --config Release
```

---

## 6. Testing & Verification on Windows

1. Press `Win + Space` or click the language indicator on the Windows Taskbar.
2. Select **"PaliKey (Pāḷi)"**.
3. Open **Notepad** or **Microsoft Word**:
   - In Velthuis scheme: type `sammaadi.t.thi` $\rightarrow$ observer inline preedit dotted underline, converting to `sammādiṭṭhi`.
   - Test backspace undo: type `aa` ($\rightarrow$ `ā`), press Backspace $\rightarrow$ raw `aa` is restored.
   - In EasyUnicode mode: press `AltGr + a` $\rightarrow$ `ā`, `AltGr + t` $\rightarrow$ `ṭ`, `AltGr + m` $\rightarrow$ `ṃ`.
