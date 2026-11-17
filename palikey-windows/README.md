# PaliKey Windows — Phase 1c Hook Shell

Standalone Windows system tray input method for Pāḷi diacritics using the proven low-level keyboard hook (`WH_KEYBOARD_LL`) and `SendInput` architecture pattern (as pioneered by `vnkey` and `unikey`).

Links `palikey-engine` directly via its native Rust API without external workspace entanglements.

---

## 1. Design & Architecture

`palikey-windows` implements the **core-and-shell** architecture defined in [DESIGN.md](../DESIGN.md):
- **Core Engine (`palikey-engine`)**: Pure, platform-agnostic state machine transducer. Consumes key events and outputs backspace count and commit strings.
- **Windows Shell (`palikey-windows`)**: Standalone Win32 desktop application that manages the system tray icon, window message pump, configuration persistence, and hardware keystroke interception.

```
┌───────────────────────────────────────────────────────────────┐
│                    Windows Subsystem                          │
│                                                               │
│   Hardware Key Down                                           │
│          │                                                    │
│          ▼                                                    │
│   ┌───────────────┐                                           │
│   │WH_KEYBOARD_LL │◄── Injected Keystrokes Guard              │
│   │     Hook      │    (LLKHF_INJECTED / PALIKEY_MAGIC_INFO)  │
│   └──────┬────────┘                                           │
│          │                                                    │
│          ├───────────────────────┬────────────────────────┐   │
│          ▼                       ▼                        ▼   │
│   Modifier / Nav Key        Backspace Key           Printable │
│   (Ctrl/Alt/Win/Arrows)     (Engine Undo)             Char    │
│          │                       │                        │   │
│   Flush & Passthrough      Undo: Suppress                 │   │
│   CallNextHookEx()         & SendInput()                  │   │
│                                  │                        ▼   │
│                                  │             ┌─────────────────────┐
│                                  │             │   ToUnicodeEx()     │
│                                  │             │ Layout + Shift/Caps │
│                                  │             └──────────┬──────────┘
│                                  │                        │           
│                                  │                        ▼           
│                                  │             ┌─────────────────────┐
│                                  │             │   palikey-engine    │
│                                  │             │     transducer      │
│                                  │             └──────────┬──────────┘
│                                  │                        │           
│                                  │         ┌──────────────┴──────────┐
│                                  │         ▼                         ▼
│                                  │    Composition               Passthrough
│                                  │    Replacement                (No Match)
│                                  │         │                         │
│                                  ▼         ▼                         ▼
│                           ┌──────────────────┐             CallNextHookEx()
│                           │    SendInput     │
│                           │ Backspaces +     │
│                           │ KEYEVENTF_UNICODE│
│                           └──────────────────┘
└───────────────────────────────────────────────────────────────┘
```

### Key Technical Highlights:
1. **Zero External Win32 Crate Overhead**:
   Written using std plus direct `extern "system"` Win32 FFI declarations (`user32.dll`, `shell32.dll`, `kernel32.dll`). Compiles in milliseconds, requires zero heavy third-party dependencies, and yields a standalone binary under 1.5 MB.
2. **Layout & Case Aware**:
   Translates virtual-key codes using the foreground window's keyboard layout (`GetKeyboardLayout`) and message-synchronized Shift and CapsLock state via `ToUnicodeEx`. Works identically on QWERTY, QWERTZ, and AZERTY.
3. **Hardware Speed Passthrough**:
   Normal uncomposed keystrokes pass through natively via `CallNextHookEx` with zero synthetic latency and zero cursor flicker. Only matched sequences (e.g. `aa` $\rightarrow$ `ā`, `.t` $\rightarrow$ `ṭ`) suppress the trigger key and emit backspaces + Unicode text.
4. **Anti-Recursion ExtraInfo Tagging**:
   Every synthetic input emitted by `SendInput` is stamped with `dwExtraInfo = PALIKEY_MAGIC_EXTRA_INFO` (`0x50414C49`). The hook immediately passes through any keystroke with `LLKHF_INJECTED` or this magic signature, preventing infinite event loops.
5. **Config Schema Parity**:
   Reads and writes `%APPDATA%\PaliKey\config.json`, conforming exactly to the cross-platform `shared/config.schema.json`.

---

## 2. Limits of the Low-Level Hook Approach

While the `WH_KEYBOARD_LL` + `SendInput` pattern allows shipping an instant, portable, zero-install executable in Phase 1, it has well-understood architectural limitations on modern Windows:

1. **User Interface Privilege Isolation (UIPI) & Elevated Windows**:
   - Windows security prevents standard user processes from sending synthetic inputs or capturing keystrokes in processes running at a higher Integrity Level (e.g. Administrator Command Prompt, PowerShell, Task Manager, or elevated installers).
   - *Workaround*: If typing inside elevated windows is required, launch `palikey-windows.exe` with "Run as Administrator".
2. **Anti-Cheat & Security Software**:
   - Kernel-level gaming anti-cheat engines (such as Riot Vanguard, Easy Anti-Cheat, and BattlEye) intentionally block or unhook `WH_KEYBOARD_LL` to prevent botting, macros, and input spoofing.
3. **UWP & Sandboxed Application Quirks**:
   - Modern Windows App SDK / WinUI 3 / Universal Windows Platform (UWP) text controls and web views (Edge WebView2) may process synthetic `VK_BACK` messages asynchronously, occasionally causing cursor jumping under high typing speeds.
4. **No Native Inline Preedit**:
   - The hook replaces characters after they have hit the target window via backspace deletion. It cannot display OS-native underlined composition strings or native IME candidate windows.

---

## 3. Path to Text Services Framework (TSF) — Phase 4

To eliminate all limitations above, **Phase 4** of the PaliKey roadmap introduces a full **Text Services Framework (TSF)** Text Input Processor (TIP):

- **First-Class OS Integration**:
  TSF is the official Microsoft text input architecture for Windows. It integrates into the Windows Language Bar, Settings, and Touch Keyboard.
- **Native Inline Preedit & Text Store**:
  TSF communicates directly with target applications via COM interfaces (`ITfTextInputProcessor`, `ITfContext`, `ITfRange`). Composition is displayed in-place with native dashed underlines and commits atomically without backspace simulation.
- **Full Privilege & App Compatibility**:
  TSF input processors run in-process within each application context. They work seamlessly across standard Win32 apps, sandboxed UWP apps, Microsoft Office, and elevated windows without security warnings or anti-cheat conflicts.
- **Shared Architecture**:
  The underlying `palikey-engine` transducer and `config.json` schema remain 100% identical. Transitioning to TSF in Phase 4 is strictly an OS shell upgrade.

---

## 4. Building & Running

See [build-on-windows.md](build-on-windows.md) for step-by-step instructions for building the executable on Windows, configuring startup autorun, and resolving SmartScreen notifications.
