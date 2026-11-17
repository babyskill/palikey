//! C Foreign Function Interface (FFI) for PaliKey engine.
//!
//! Provides both singleton and instance APIs per DESIGN.md §3 and §5.

use std::ffi::c_char;
use std::sync::Mutex;

use crate::chars::NiggahitaVariant;
use crate::engine::{Engine, EngineConfig, KeyAction, ProcessResult};
use crate::schemes::SchemeId;

/// C-compatible representation of `ProcessResult`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PalikeyProcessResult {
    /// Number of characters to erase via backspace before committing.
    pub backspaces: u8,
    /// Null-terminated UTF-8 commit string.
    pub commit: [c_char; 64],
    /// Null-terminated UTF-8 preedit string.
    pub preedit: [c_char; 64],
}

impl Default for PalikeyProcessResult {
    fn default() -> Self {
        Self {
            backspaces: 0,
            commit: [0; 64],
            preedit: [0; 64],
        }
    }
}

impl PalikeyProcessResult {
    pub(crate) fn from_engine_result(res: &ProcessResult) -> Self {
        let mut out = Self {
            backspaces: res.backspaces,
            commit: [0; 64],
            preedit: [0; 64],
        };
        copy_to_c_buffer(&res.commit, &mut out.commit);
        copy_to_c_buffer(&res.preedit, &mut out.preedit);
        out
    }
}

fn copy_to_c_buffer(src: &str, dest: &mut [c_char; 64]) {
    let bytes = src.as_bytes();
    let max = bytes.len().min(63);
    for i in 0..max {
        dest[i] = bytes[i] as c_char;
    }
    dest[max] = 0;
}

/// Opaque wrapper for a heap-allocated engine instance.
pub struct PalikeyEngine {
    inner: Mutex<Engine>,
}

// Global singleton engine for single-context shells (e.g. Windows hook)
static SINGLETON: Mutex<Option<Engine>> = Mutex::new(None);

fn with_singleton<F, R>(f: F) -> R
where
    F: FnOnce(&mut Engine) -> R,
{
    let mut guard = SINGLETON.lock().unwrap_or_else(|e| e.into_inner());
    if guard.is_none() {
        *guard = Some(Engine::default());
    }
    f(guard.as_mut().unwrap())
}

// =============================================================================
// Singleton C API (Windows hook style)
// =============================================================================

/// Initializes or configures the global singleton engine.
///
/// - `scheme_id`: 0=Velthuis, 1=PaliTelex, 2=Sil
/// - `niggahita`: 0=DotBelow (ṃ), 1=DotAbove (ṁ)
/// - `extended_set`: true to enable Sanskrit/IAST characters
///
/// Returns 1 on success, 0 on invalid parameters.
#[no_mangle]
pub extern "C" fn palikey_setup(scheme_id: u32, niggahita: u32, extended_set: bool) -> i32 {
    let scheme = match SchemeId::from_u32(scheme_id) {
        Some(s) => s,
        None => return 0,
    };

    let nig = match niggahita {
        0 => NiggahitaVariant::DotBelow,
        1 => NiggahitaVariant::DotAbove,
        _ => return 0,
    };

    let config = EngineConfig {
        scheme,
        niggahita: nig,
        extended_set,
        ..EngineConfig::default()
    };

    let mut guard = SINGLETON.lock().unwrap_or_else(|e| e.into_inner());
    *guard = Some(Engine::new(config));
    1
}

/// Processes a keystroke on the global singleton engine.
///
/// - `key_char`: Unicode codepoint (e.g. 'a' = 97), ignored if `is_backspace` is true.
/// - `is_backspace`: true if Backspace key was pressed.
/// - `out_result`: pointer to `PalikeyProcessResult` populated with output.
///
/// Returns 1 on success, 0 on null pointer or error.
#[no_mangle]
pub extern "C" fn palikey_process(
    key_char: u32,
    is_backspace: bool,
    out_result: *mut PalikeyProcessResult,
) -> i32 {
    if out_result.is_null() {
        return 0;
    }

    let action = if is_backspace {
        KeyAction::Backspace
    } else {
        match char::from_u32(key_char) {
            Some(c) => KeyAction::Char(c),
            None => return 0,
        }
    };

    let result = with_singleton(|eng| eng.process(action));
    unsafe {
        *out_result = PalikeyProcessResult::from_engine_result(&result);
    }
    1
}

/// Resets the global singleton engine buffer and undo state.
#[no_mangle]
pub extern "C" fn palikey_reset() {
    with_singleton(|eng| {
        eng.reset();
    });
}

// =============================================================================
// Instance C API (macOS IMK, Linux Fcitx5/IBus, mobile JNI style)
// =============================================================================

/// Creates a new heap-allocated engine instance.
///
/// Returns a pointer to `PalikeyEngine`, or NULL on failure.
#[no_mangle]
pub extern "C" fn palikey_new(
    scheme_id: u32,
    niggahita: u32,
    extended_set: bool,
) -> *mut PalikeyEngine {
    let scheme = match SchemeId::from_u32(scheme_id) {
        Some(s) => s,
        None => SchemeId::Velthuis,
    };

    let nig = match niggahita {
        1 => NiggahitaVariant::DotAbove,
        _ => NiggahitaVariant::DotBelow,
    };

    let config = EngineConfig {
        scheme,
        niggahita: nig,
        extended_set,
        ..EngineConfig::default()
    };

    let engine = PalikeyEngine {
        inner: Mutex::new(Engine::new(config)),
    };

    Box::into_raw(Box::new(engine))
}

/// Frees an engine instance created by `palikey_new`.
#[no_mangle]
pub extern "C" fn palikey_free(engine: *mut PalikeyEngine) {
    if !engine.is_null() {
        unsafe {
            drop(Box::from_raw(engine));
        }
    }
}

/// Processes a keystroke on a specific engine instance.
///
/// Returns 1 on success, 0 on null pointer or error.
#[no_mangle]
pub extern "C" fn palikey_process_instance(
    engine: *mut PalikeyEngine,
    key_char: u32,
    is_backspace: bool,
    out_result: *mut PalikeyProcessResult,
) -> i32 {
    if engine.is_null() || out_result.is_null() {
        return 0;
    }

    let action = if is_backspace {
        KeyAction::Backspace
    } else {
        match char::from_u32(key_char) {
            Some(c) => KeyAction::Char(c),
            None => return 0,
        }
    };

    let eng = unsafe { &*engine };
    let mut guard = eng.inner.lock().unwrap_or_else(|e| e.into_inner());
    let res = guard.process(action);

    unsafe {
        *out_result = PalikeyProcessResult::from_engine_result(&res);
    }
    1
}

/// Sets the scheme for an engine instance (or the singleton if `engine` is NULL).
///
/// Returns 1 on success, 0 on invalid scheme ID.
#[no_mangle]
pub extern "C" fn palikey_set_scheme(engine: *mut PalikeyEngine, scheme_id: u32) -> i32 {
    let scheme = match SchemeId::from_u32(scheme_id) {
        Some(s) => s,
        None => return 0,
    };

    if engine.is_null() {
        with_singleton(|eng| eng.set_scheme(scheme));
    } else {
        let eng = unsafe { &*engine };
        let mut guard = eng.inner.lock().unwrap_or_else(|e| e.into_inner());
        guard.set_scheme(scheme);
    }
    1
}

/// Resets an engine instance (clears buffer and undo state).
#[no_mangle]
pub extern "C" fn palikey_reset_instance(engine: *mut PalikeyEngine) {
    if !engine.is_null() {
        let eng = unsafe { &*engine };
        let mut guard = eng.inner.lock().unwrap_or_else(|e| e.into_inner());
        guard.reset();
    }
}

/// Returns the engine library version string (null-terminated).
#[no_mangle]
pub extern "C" fn palikey_version() -> *const c_char {
    static VERSION: &[u8] = b"0.1.0\0";
    VERSION.as_ptr() as *const c_char
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CStr;

    #[test]
    fn test_singleton_c_api() {
        assert_eq!(palikey_setup(0, 0, false), 1);

        let mut res = PalikeyProcessResult::default();
        // Send 'a'
        assert_eq!(palikey_process('a' as u32, false, &mut res), 1);
        assert_eq!(res.backspaces, 0);
        let commit_str = unsafe { CStr::from_ptr(res.commit.as_ptr()).to_str().unwrap() };
        assert_eq!(commit_str, "a");

        // Send second 'a' -> 'ā'
        assert_eq!(palikey_process('a' as u32, false, &mut res), 1);
        assert_eq!(res.backspaces, 1);
        let commit_str = unsafe { CStr::from_ptr(res.commit.as_ptr()).to_str().unwrap() };
        assert_eq!(commit_str, "ā");

        // Backspace -> undoes to "aa"
        assert_eq!(palikey_process(0, true, &mut res), 1);
        assert_eq!(res.backspaces, 1);
        let commit_str = unsafe { CStr::from_ptr(res.commit.as_ptr()).to_str().unwrap() };
        assert_eq!(commit_str, "aa");

        palikey_reset();
    }

    #[test]
    fn test_instance_c_api() {
        let eng = palikey_new(1, 0, false); // PaliTelex
        assert!(!eng.is_null());

        let mut res = PalikeyProcessResult::default();
        // Send 't'
        assert_eq!(palikey_process_instance(eng, 't' as u32, false, &mut res), 1);
        assert_eq!(res.backspaces, 0);

        // Send 'x' -> 'ṭ'
        assert_eq!(palikey_process_instance(eng, 'x' as u32, false, &mut res), 1);
        assert_eq!(res.backspaces, 1);
        let commit_str = unsafe { CStr::from_ptr(res.commit.as_ptr()).to_str().unwrap() };
        assert_eq!(commit_str, "ṭ");

        palikey_free(eng);
    }
}
