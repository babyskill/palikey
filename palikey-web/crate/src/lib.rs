//! WebAssembly bindings for PaliKey engine.
//!
//! Exposes a clean C-ABI WebAssembly interface with zero external dependencies.
//! Works without wasm-bindgen for offline/zero-network environments.

use std::cell::RefCell;
use palikey_engine::chars::NiggahitaVariant;
use palikey_engine::engine::{Engine, EngineConfig, KeyAction, ProcessResult};
use palikey_engine::schemes::SchemeId;

/// Wrapper around `Engine` holding the latest `ProcessResult`.
pub struct WasmEngine {
    engine: Engine,
    last_result: ProcessResult,
}

impl WasmEngine {
    fn new(scheme_id: u32, niggahita: u32, extended: u32) -> Self {
        let scheme = SchemeId::from_u32(scheme_id).unwrap_or(SchemeId::Velthuis);
        let nig = match niggahita {
            1 => NiggahitaVariant::DotAbove,
            _ => NiggahitaVariant::DotBelow,
        };
        let config = EngineConfig {
            scheme,
            niggahita: nig,
            extended_set: extended != 0,
            ..EngineConfig::default()
        };
        Self {
            engine: Engine::new(config),
            last_result: ProcessResult::default(),
        }
    }
}

// Global default engine for simple single-context use cases
thread_local! {
    static DEFAULT_ENGINE: RefCell<WasmEngine> = RefCell::new(WasmEngine::new(0, 0, 0));
}

// =============================================================================
// Instance-based WASM API
// =============================================================================

/// Creates a new engine instance.
/// - `scheme_id`: 0=Velthuis, 1=PaliTelex, 2=Sil
/// - `niggahita`: 0=DotBelow (ṃ), 1=DotAbove (ṁ)
/// - `extended`: 1=enable Sanskrit/IAST, 0=disable
#[no_mangle]
pub extern "C" fn palikey_wasm_new(scheme_id: u32, niggahita: u32, extended: u32) -> *mut WasmEngine {
    let inst = Box::new(WasmEngine::new(scheme_id, niggahita, extended));
    Box::into_raw(inst)
}

/// Frees an engine instance.
#[no_mangle]
pub extern "C" fn palikey_wasm_free(ptr: *mut WasmEngine) {
    if !ptr.is_null() {
        unsafe {
            drop(Box::from_raw(ptr));
        }
    }
}

/// Sets the transliteration scheme for this engine.
#[no_mangle]
pub extern "C" fn palikey_wasm_set_scheme(ptr: *mut WasmEngine, scheme_id: u32) {
    if let Some(eng) = unsafe { ptr.as_mut() } {
        if let Some(s) = SchemeId::from_u32(scheme_id) {
            eng.engine.set_scheme(s);
        }
    }
}

/// Sets the niggahīta variant: 0=DotBelow (ṃ), 1=DotAbove (ṁ).
#[no_mangle]
pub extern "C" fn palikey_wasm_set_niggahita(ptr: *mut WasmEngine, niggahita: u32) {
    if let Some(eng) = unsafe { ptr.as_mut() } {
        let nig = match niggahita {
            1 => NiggahitaVariant::DotAbove,
            _ => NiggahitaVariant::DotBelow,
        };
        eng.engine.set_niggahita(nig);
    }
}

/// Enables or disables the extended Sanskrit/IAST set.
#[no_mangle]
pub extern "C" fn palikey_wasm_set_extended(ptr: *mut WasmEngine, extended: u32) {
    if let Some(eng) = unsafe { ptr.as_mut() } {
        eng.engine.set_extended_set(extended != 0);
    }
}

/// Resets the engine word buffer and state.
#[no_mangle]
pub extern "C" fn palikey_wasm_reset(ptr: *mut WasmEngine) {
    if let Some(eng) = unsafe { ptr.as_mut() } {
        eng.engine.reset();
        eng.last_result = ProcessResult::default();
    }
}

/// Processes a single Unicode code point.
/// Returns 1 on success, 0 on invalid char code or null engine.
#[no_mangle]
pub extern "C" fn palikey_wasm_process_char(ptr: *mut WasmEngine, code_point: u32) -> u32 {
    let eng = match unsafe { ptr.as_mut() } {
        Some(e) => e,
        None => return 0,
    };

    let c = match char::from_u32(code_point) {
        Some(ch) => ch,
        None => return 0,
    };

    eng.last_result = eng.engine.process(KeyAction::Char(c));
    1
}

/// Processes a Backspace key press.
/// Returns 1 on success, 0 on null engine.
#[no_mangle]
pub extern "C" fn palikey_wasm_backspace(ptr: *mut WasmEngine) -> u32 {
    let eng = match unsafe { ptr.as_mut() } {
        Some(e) => e,
        None => return 0,
    };

    eng.last_result = eng.engine.process(KeyAction::Backspace);
    1
}

/// Returns the number of characters to backspace before inserting commit.
#[no_mangle]
pub extern "C" fn palikey_wasm_get_backspaces(ptr: *const WasmEngine) -> u32 {
    unsafe { ptr.as_ref() }
        .map(|e| e.last_result.backspaces as u32)
        .unwrap_or(0)
}

/// Returns a pointer to the UTF-8 commit string in WASM memory.
#[no_mangle]
pub extern "C" fn palikey_wasm_get_commit_ptr(ptr: *const WasmEngine) -> *const u8 {
    unsafe { ptr.as_ref() }
        .map(|e| e.last_result.commit.as_ptr())
        .unwrap_or(std::ptr::null())
}

/// Returns the length in bytes of the UTF-8 commit string.
#[no_mangle]
pub extern "C" fn palikey_wasm_get_commit_len(ptr: *const WasmEngine) -> u32 {
    unsafe { ptr.as_ref() }
        .map(|e| e.last_result.commit.len() as u32)
        .unwrap_or(0)
}

/// Returns a pointer to the UTF-8 preedit string in WASM memory.
#[no_mangle]
pub extern "C" fn palikey_wasm_get_preedit_ptr(ptr: *const WasmEngine) -> *const u8 {
    unsafe { ptr.as_ref() }
        .map(|e| e.last_result.preedit.as_ptr())
        .unwrap_or(std::ptr::null())
}

/// Returns the length in bytes of the UTF-8 preedit string.
#[no_mangle]
pub extern "C" fn palikey_wasm_get_preedit_len(ptr: *const WasmEngine) -> u32 {
    unsafe { ptr.as_ref() }
        .map(|e| e.last_result.preedit.len() as u32)
        .unwrap_or(0)
}

// =============================================================================
// Default singleton WASM API (no pointer tracking needed by JS)
// =============================================================================

#[no_mangle]
pub extern "C" fn palikey_default_init(scheme_id: u32, niggahita: u32, extended: u32) {
    DEFAULT_ENGINE.with(|cell| {
        *cell.borrow_mut() = WasmEngine::new(scheme_id, niggahita, extended);
    });
}

#[no_mangle]
pub extern "C" fn palikey_default_set_scheme(scheme_id: u32) {
    DEFAULT_ENGINE.with(|cell| {
        if let Some(s) = SchemeId::from_u32(scheme_id) {
            cell.borrow_mut().engine.set_scheme(s);
        }
    });
}

#[no_mangle]
pub extern "C" fn palikey_default_set_niggahita(niggahita: u32) {
    DEFAULT_ENGINE.with(|cell| {
        let nig = match niggahita {
            1 => NiggahitaVariant::DotAbove,
            _ => NiggahitaVariant::DotBelow,
        };
        cell.borrow_mut().engine.set_niggahita(nig);
    });
}

#[no_mangle]
pub extern "C" fn palikey_default_set_extended(extended: u32) {
    DEFAULT_ENGINE.with(|cell| {
        cell.borrow_mut().engine.set_extended_set(extended != 0);
    });
}

#[no_mangle]
pub extern "C" fn palikey_default_reset() {
    DEFAULT_ENGINE.with(|cell| {
        let mut eng = cell.borrow_mut();
        eng.engine.reset();
        eng.last_result = ProcessResult::default();
    });
}

#[no_mangle]
pub extern "C" fn palikey_default_process_char(code_point: u32) -> u32 {
    let c = match char::from_u32(code_point) {
        Some(ch) => ch,
        None => return 0,
    };
    DEFAULT_ENGINE.with(|cell| {
        let mut eng = cell.borrow_mut();
        eng.last_result = eng.engine.process(KeyAction::Char(c));
    });
    1
}

#[no_mangle]
pub extern "C" fn palikey_default_backspace() -> u32 {
    DEFAULT_ENGINE.with(|cell| {
        let mut eng = cell.borrow_mut();
        eng.last_result = eng.engine.process(KeyAction::Backspace);
    });
    1
}

#[no_mangle]
pub extern "C" fn palikey_default_get_backspaces() -> u32 {
    DEFAULT_ENGINE.with(|cell| cell.borrow().last_result.backspaces as u32)
}

#[no_mangle]
pub extern "C" fn palikey_default_get_commit_ptr() -> *const u8 {
    DEFAULT_ENGINE.with(|cell| cell.borrow().last_result.commit.as_ptr())
}

#[no_mangle]
pub extern "C" fn palikey_default_get_commit_len() -> u32 {
    DEFAULT_ENGINE.with(|cell| cell.borrow().last_result.commit.len() as u32)
}

#[no_mangle]
pub extern "C" fn palikey_default_get_preedit_ptr() -> *const u8 {
    DEFAULT_ENGINE.with(|cell| cell.borrow().last_result.preedit.as_ptr())
}

#[no_mangle]
pub extern "C" fn palikey_default_get_preedit_len() -> u32 {
    DEFAULT_ENGINE.with(|cell| cell.borrow().last_result.preedit.len() as u32)
}
