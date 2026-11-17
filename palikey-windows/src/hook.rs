//! Low-level keyboard hook (WH_KEYBOARD_LL) and SendInput engine integration.
//!
//! Follows the vnkey/unikey architecture pattern:
//! - Intercepts hardware keystrokes before target applications receive them.
//! - Queries active keyboard layout and modifier state (Shift, CapsLock) to translate
//!   virtual keys to Unicode characters via `ToUnicodeEx`.
//! - Feeds characters and backspaces to the `palikey-engine` transducer.
//! - If composition occurs, suppresses the original keystroke and emits replacement
//!   backspaces + Unicode text via `SendInput(KEYEVENTF_UNICODE)`.
//! - Tags all synthetic SendInput events with `PALIKEY_MAGIC_EXTRA_INFO` and checks
//!   `LLKHF_INJECTED` to guarantee zero recursive re-entry.

use std::sync::Mutex;

use palikey_engine::engine::{Engine, KeyAction, ProcessResult};

use crate::config::Config;

#[cfg(windows)]
use crate::win32::*;

/// Internal state of the keyboard hook.
struct HookState {
    enabled: bool,
    engine: Engine,
}

static HOOK_STATE: Mutex<Option<HookState>> = Mutex::new(None);

#[cfg(windows)]
static HOOK_HANDLE: Mutex<Option<usize>> = Mutex::new(None);

/// Initializes the engine state and installs the WH_KEYBOARD_LL hook.
#[cfg(windows)]
pub fn install(config: &Config) -> bool {
    let mut state_guard = HOOK_STATE.lock().unwrap_or_else(|e| e.into_inner());
    *state_guard = Some(HookState {
        enabled: true,
        engine: Engine::new(config.to_engine_config()),
    });

    let mut hook_guard = HOOK_HANDLE.lock().unwrap_or_else(|e| e.into_inner());
    if hook_guard.is_none() {
        unsafe {
            let h_instance = GetModuleHandleW(std::ptr::null());
            let hook = SetWindowsHookExW(
                WH_KEYBOARD_LL,
                low_level_keyboard_proc,
                h_instance,
                0, // Global thread hook
            );
            if hook.is_null() {
                return false;
            }
            *hook_guard = Some(hook as usize);
        }
    }
    true
}

/// Non-Windows stub for compilation check and testing.
#[cfg(not(windows))]
pub fn install(config: &Config) -> bool {
    let mut state_guard = HOOK_STATE.lock().unwrap_or_else(|e| e.into_inner());
    *state_guard = Some(HookState {
        enabled: true,
        engine: Engine::new(config.to_engine_config()),
    });
    true
}

/// Unhooks WH_KEYBOARD_LL and cleans up hook state.
#[cfg(windows)]
pub fn uninstall() {
    let mut hook_guard = HOOK_HANDLE.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(hook_val) = hook_guard.take() {
        unsafe {
            UnhookWindowsHookEx(hook_val as HHOOK);
        }
    }
    let mut state_guard = HOOK_STATE.lock().unwrap_or_else(|e| e.into_inner());
    *state_guard = None;
}

/// Non-Windows stub for compilation check.
#[cfg(not(windows))]
pub fn uninstall() {
    let mut state_guard = HOOK_STATE.lock().unwrap_or_else(|e| e.into_inner());
    *state_guard = None;
}

/// Toggles whether PaliKey active composition is enabled.
pub fn set_enabled(enabled: bool) {
    let mut guard = HOOK_STATE.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(state) = guard.as_mut() {
        state.enabled = enabled;
        if !enabled {
            state.engine.reset();
        }
    }
}

/// Returns whether PaliKey is currently enabled.
pub fn is_enabled() -> bool {
    let guard = HOOK_STATE.lock().unwrap_or_else(|e| e.into_inner());
    guard.as_ref().map(|s| s.enabled).unwrap_or(false)
}

/// Updates the active engine configuration from a new `Config`.
pub fn update_config(config: &Config) {
    let mut guard = HOOK_STATE.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(state) = guard.as_mut() {
        state.engine.set_config(config.to_engine_config());
        state.engine.reset();
    }
}

/// Resets the engine composition buffer and undo history.
pub fn reset() {
    let mut guard = HOOK_STATE.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(state) = guard.as_mut() {
        state.engine.reset();
    }
}

/// Helper to process a key action through the global engine.
pub fn process_key_action(action: KeyAction) -> Option<ProcessResult> {
    let mut guard = HOOK_STATE.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(state) = guard.as_mut() {
        if state.enabled {
            return Some(state.engine.process(action));
        }
    }
    None
}

#[cfg(windows)]
/// Emits synthetic Backspaces via Win32 `SendInput`.
pub unsafe fn send_backspaces(count: u8) {
    if count == 0 {
        return;
    }
    let mut inputs: Vec<INPUT> = Vec::with_capacity((count as usize) * 2);
    for _ in 0..count {
        // Key down
        inputs.push(INPUT {
            r#type: INPUT_KEYBOARD,
            u: INPUT_UNION {
                ki: KEYBDINPUT {
                    wVk: VK_BACK as u16,
                    wScan: 0,
                    dwFlags: 0,
                    time: 0,
                    dwExtraInfo: PALIKEY_MAGIC_EXTRA_INFO,
                },
            },
        });
        // Key up
        inputs.push(INPUT {
            r#type: INPUT_KEYBOARD,
            u: INPUT_UNION {
                ki: KEYBDINPUT {
                    wVk: VK_BACK as u16,
                    wScan: 0,
                    dwFlags: KEYEVENTF_KEYUP,
                    time: 0,
                    dwExtraInfo: PALIKEY_MAGIC_EXTRA_INFO,
                },
            },
        });
    }
    SendInput(
        inputs.len() as u32,
        inputs.as_mut_ptr(),
        std::mem::size_of::<INPUT>() as i32,
    );
}

#[cfg(windows)]
/// Emits synthetic Unicode characters via Win32 `SendInput` with KEYEVENTF_UNICODE.
pub unsafe fn send_unicode_str(s: &str) {
    if s.is_empty() {
        return;
    }
    let utf16: Vec<u16> = s.encode_utf16().collect();
    let mut inputs: Vec<INPUT> = Vec::with_capacity(utf16.len() * 2);
    for &code_unit in &utf16 {
        // Key down
        inputs.push(INPUT {
            r#type: INPUT_KEYBOARD,
            u: INPUT_UNION {
                ki: KEYBDINPUT {
                    wVk: 0,
                    wScan: code_unit,
                    dwFlags: KEYEVENTF_UNICODE,
                    time: 0,
                    dwExtraInfo: PALIKEY_MAGIC_EXTRA_INFO,
                },
            },
        });
        // Key up
        inputs.push(INPUT {
            r#type: INPUT_KEYBOARD,
            u: INPUT_UNION {
                ki: KEYBDINPUT {
                    wVk: 0,
                    wScan: code_unit,
                    dwFlags: KEYEVENTF_UNICODE | KEYEVENTF_KEYUP,
                    time: 0,
                    dwExtraInfo: PALIKEY_MAGIC_EXTRA_INFO,
                },
            },
        });
    }
    SendInput(
        inputs.len() as u32,
        inputs.as_mut_ptr(),
        std::mem::size_of::<INPUT>() as i32,
    );
}

#[cfg(windows)]
/// WH_KEYBOARD_LL hook callback procedure.
pub unsafe extern "system" fn low_level_keyboard_proc(
    n_code: i32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    if n_code != HC_ACTION {
        return CallNextHookEx(std::ptr::null_mut(), n_code, w_param, l_param);
    }

    let kbd = *(l_param as *const KBDLLHOOKSTRUCT);

    // Guard against recursion: ignore synthetic keystrokes injected by SendInput
    if (kbd.flags & LLKHF_INJECTED) != 0 || kbd.dwExtraInfo == PALIKEY_MAGIC_EXTRA_INFO {
        return CallNextHookEx(std::ptr::null_mut(), n_code, w_param, l_param);
    }

    // Only intercept keydown events
    if w_param != WM_KEYDOWN as usize && w_param != WM_SYSKEYDOWN as usize {
        return CallNextHookEx(std::ptr::null_mut(), n_code, w_param, l_param);
    }

    // Pass through and flush buffer when modifier shortcuts (Ctrl, Alt, Win) are active
    let ctrl_down = GetKeyState(VK_CONTROL as i32) < 0;
    let alt_down = GetKeyState(VK_MENU as i32) < 0;
    let win_down = GetKeyState(VK_LWIN as i32) < 0 || GetKeyState(VK_RWIN as i32) < 0;

    if ctrl_down || alt_down || win_down {
        reset();
        return CallNextHookEx(std::ptr::null_mut(), n_code, w_param, l_param);
    }

    // Check boundary / navigation keys that should flush composition buffer
    match kbd.vkCode {
        VK_ESCAPE | VK_TAB | VK_RETURN | VK_SPACE | VK_LEFT | VK_RIGHT | VK_UP | VK_DOWN
        | VK_HOME | VK_END | VK_PRIOR | VK_NEXT | VK_DELETE => {
            reset();
            return CallNextHookEx(std::ptr::null_mut(), n_code, w_param, l_param);
        }
        // Modifier key presses themselves: ignore without flushing
        VK_SHIFT | VK_CAPITAL | VK_CONTROL | VK_MENU | VK_LWIN | VK_RWIN => {
            return CallNextHookEx(std::ptr::null_mut(), n_code, w_param, l_param);
        }
        _ => {}
    }

    if !is_enabled() {
        return CallNextHookEx(std::ptr::null_mut(), n_code, w_param, l_param);
    }

    // Handle Backspace (engine undo vs standard backspace)
    if kbd.vkCode == VK_BACK {
        if let Some(res) = process_key_action(KeyAction::Backspace) {
            if !res.commit.is_empty() {
                // Undo composition: replace composed character with raw sequence
                send_backspaces(res.backspaces);
                send_unicode_str(&res.commit);
                // Suppress original backspace keystroke
                return 1;
            }
        }
        // Standard deletion: pass through to target window
        return CallNextHookEx(std::ptr::null_mut(), n_code, w_param, l_param);
    }

    // Translate virtual-key + keyboard state to Unicode character
    let hwnd = GetForegroundWindow();
    let thread_id = GetWindowThreadProcessId(hwnd, std::ptr::null_mut());
    let hkl = GetKeyboardLayout(thread_id);

    let mut key_state = [0u8; 256];
    GetKeyboardState(key_state.as_mut_ptr());

    // In WH_KEYBOARD_LL, synchronize Shift and CapsLock state from message queue
    if GetKeyState(VK_SHIFT as i32) < 0 {
        key_state[VK_SHIFT as usize] = 0x80;
    }
    if (GetKeyState(VK_CAPITAL as i32) & 0x0001) != 0 {
        key_state[VK_CAPITAL as usize] = 0x01;
    }

    let mut buf = [0u16; 16];
    let count = ToUnicodeEx(
        kbd.vkCode,
        kbd.scanCode,
        key_state.as_ptr(),
        buf.as_mut_ptr(),
        buf.len() as i32,
        0,
        hkl,
    );

    let maybe_char = if count == 1 {
        char::decode_utf16(std::iter::once(buf[0]))
            .next()
            .and_then(|r| r.ok())
    } else if count == 2 {
        char::decode_utf16(buf[0..2].iter().copied())
            .next()
            .and_then(|r| r.ok())
    } else {
        None
    };

    if let Some(c) = maybe_char {
        if let Some(res) = process_key_action(KeyAction::Char(c)) {
            // Check if this was a direct pass-through or a composition replacement
            let is_direct_passthrough = res.backspaces == 0
                && res.commit.chars().count() == 1
                && res.commit.chars().next() == Some(c);

            if !is_direct_passthrough {
                // Suppress original hardware keystroke
                if res.backspaces > 0 {
                    send_backspaces(res.backspaces);
                }
                if !res.commit.is_empty() {
                    send_unicode_str(&res.commit);
                }
                return 1;
            }
        }
    }

    // Default: pass through original keystroke to next hook / application
    CallNextHookEx(std::ptr::null_mut(), n_code, w_param, l_param)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::SchemeConfig;

    #[test]
    fn test_hook_state_lifecycle() {
        let mut cfg = Config::default();
        cfg.scheme = SchemeConfig::Velthuis;

        assert!(install(&cfg));
        assert!(is_enabled());

        // Process 'a' -> direct passthrough
        let res1 = process_key_action(KeyAction::Char('a')).expect("res1");
        assert_eq!(res1.backspaces, 0);
        assert_eq!(res1.commit, "a");

        // Process second 'a' -> replaces 1 char with 'ā'
        let res2 = process_key_action(KeyAction::Char('a')).expect("res2");
        assert_eq!(res2.backspaces, 1);
        assert_eq!(res2.commit, "ā");

        // Process Backspace -> undoes 'ā' back to "aa"
        let res_bs = process_key_action(KeyAction::Backspace).expect("res_bs");
        assert_eq!(res_bs.backspaces, 1);
        assert_eq!(res_bs.commit, "aa");

        // Disable hook
        set_enabled(false);
        assert!(!is_enabled());
        assert!(process_key_action(KeyAction::Char('a')).is_none());

        uninstall();
    }

    #[test]
    fn test_palitelex_and_sil_hook_simulation() {
        let mut cfg = Config::default();
        cfg.scheme = SchemeConfig::Palitelex;
        install(&cfg);

        // 't', then 'x' -> ṭ
        let _ = process_key_action(KeyAction::Char('t'));
        let res = process_key_action(KeyAction::Char('x')).expect("tx match");
        assert_eq!(res.backspaces, 1);
        assert_eq!(res.commit, "ṭ");

        // Switch to SIL
        cfg.scheme = SchemeConfig::Sil;
        update_config(&cfg);

        // '-', then 'a' -> ā
        let _ = process_key_action(KeyAction::Char('-'));
        let res_sil = process_key_action(KeyAction::Char('a')).expect("-a match");
        assert_eq!(res_sil.backspaces, 1);
        assert_eq!(res_sil.commit, "ā");

        uninstall();
    }
}
