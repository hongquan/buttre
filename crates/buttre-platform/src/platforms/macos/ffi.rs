//! buttre macOS - Zero-Unsafe FFI Bridge (Handle-Based)
//!
//! **Tests**: Integration tests for this module are located in `crates/buttre-platform/tests/platform_macos_tests.rs`.
//!
//! Uses integer handles instead of raw pointers to achieve ZERO unsafe
//! for memory management operations.

use std::collections::HashMap;
use std::ffi::CString;
use std::os::raw::c_char;
use std::sync::{Mutex, atomic::{AtomicU64, Ordering}};
use buttre_core::Action;
use buttre_core::{Keyboard, KeyboardBuilder};

// ============================================================================
// GLOBAL STATE (Thread-Safe)
// ============================================================================

static ENGINES: Mutex<Option<HashMap<u64, EngineState>>> = Mutex::new(None);
static NEXT_ID: AtomicU64 = AtomicU64::new(1);

/// Keeps the last returned CString alive across the FFI boundary.
static LAST_RESULT: Mutex<Option<CString>> = Mutex::new(None);

struct EngineState {
    keyboard: Keyboard,
    enabled:  bool,
}

impl EngineState {
    fn new() -> Self {
        let keyboard = KeyboardBuilder::telex()
            .expect("Failed to create Telex keyboard");
        Self {
            keyboard,
            enabled: true,
        }
    }
}

fn init_engines() {
    let mut engines = ENGINES.lock().unwrap();
    if engines.is_none() {
        *engines = Some(HashMap::new());
    }
}

// ============================================================================
// PUBLIC FFI FUNCTIONS
// ============================================================================

/// Create new engine instance.
///
/// Returns a non-zero handle, or 0 on failure.
#[no_mangle]
pub extern "C" fn buttre_engine_new() -> u64 {
    init_engines();
    let id = NEXT_ID.fetch_add(1, Ordering::SeqCst);
    let mut engines = ENGINES.lock().unwrap();
    if let Some(ref mut map) = *engines {
        map.insert(id, EngineState::new());
        return id;
    }
    0
}

/// Free engine instance.
///
/// Passing 0 or an invalid ID is a safe no-op.
#[no_mangle]
pub extern "C" fn buttre_engine_free(engine_id: u64) {
    if engine_id == 0 { return; }
    let mut engines = ENGINES.lock().unwrap();
    if let Some(ref mut map) = *engines {
        map.remove(&engine_id);
    }
}

/// Process a key press.
///
/// # Parameters
/// - `engine_id`: handle from `buttre_engine_new`
/// - `keycode`: macOS virtual keycode
/// - `shift`: Shift key held
/// - `capslock`: Caps Lock active — uppercase = `capslock XOR shift`
///
/// **ABI BREAK (since 0.6.3-alpha):** `capslock` is the new 4th parameter.
/// Swift host call sites must be updated to pass the fourth argument.
///
/// Returns a pointer to a UTF-8 string (valid until next call), or null.
#[no_mangle]
pub extern "C" fn buttre_engine_process_key(
    engine_id: u64,
    keycode: u16,
    shift: bool,
    capslock: bool,
) -> *const c_char {
    if engine_id == 0 { return std::ptr::null(); }

    let mut engines = ENGINES.lock().unwrap();
    let engine = match engines.as_mut().and_then(|m| m.get_mut(&engine_id)) {
        Some(e) => e,
        None => return std::ptr::null(),
    };

    if !engine.enabled { return std::ptr::null(); }

    let ch = match keycode_to_char(keycode, shift, capslock) {
        Some(c) => c,
        None => return std::ptr::null(),
    };

    let action = match engine.keyboard.process(ch) {
        Ok(actions) => actions.into_iter().next().unwrap_or(Action::DoNothing),
        Err(e) => {
            tracing::warn!("buttre_engine_process_key: keyboard error: {}", e);
            return std::ptr::null();
        }
    };

    match action {
        Action::Replace { text, .. } | Action::Commit(text) => store_and_return_cstring(text),
        _ => std::ptr::null(),
    }
}

/// Process backspace.
///
/// Returns a pointer to a UTF-8 string (new preedit), or null.
#[no_mangle]
pub extern "C" fn buttre_engine_process_backspace(engine_id: u64) -> *const c_char {
    if engine_id == 0 { return std::ptr::null(); }
    let mut engines = ENGINES.lock().unwrap();
    let engine = match engines.as_mut().and_then(|m| m.get_mut(&engine_id)) {
        Some(e) => e,
        None => return std::ptr::null(),
    };
    let action = match engine.keyboard.backspace() {
        Ok(a) => a,
        Err(e) => {
            tracing::warn!("buttre_engine_process_backspace: keyboard error: {}", e);
            return std::ptr::null();
        }
    };
    match action {
        Action::Replace { text, .. } if !text.is_empty() => store_and_return_cstring(text),
        _ => std::ptr::null(),
    }
}

/// Reset engine state (clears the composition buffer).
#[no_mangle]
pub extern "C" fn buttre_engine_reset(engine_id: u64) {
    if engine_id == 0 { return; }
    let mut engines = ENGINES.lock().unwrap();
    if let Some(engine) = engines.as_mut().and_then(|m| m.get_mut(&engine_id)) {
        engine.keyboard.reset();
    }
}

/// Switch the input method.
///
/// - `method`: `0` = telex, `1` = vni, `2` = nom (no dictionary). Other values are rejected.
///
/// Returns `true` on success.
#[no_mangle]
pub extern "C" fn buttre_engine_set_method(engine_id: u64, method: u8) -> bool {
    if engine_id == 0 { return false; }
    let mut engines = ENGINES.lock().unwrap();
    let engine = match engines.as_mut().and_then(|m| m.get_mut(&engine_id)) {
        Some(e) => e,
        None => return false,
    };
    let result = match method {
        0 => KeyboardBuilder::telex(),
        1 => KeyboardBuilder::vni(),
        2 => KeyboardBuilder::nom(None),
        _ => {
            tracing::warn!("buttre_engine_set_method: unknown method {}", method);
            return false;
        }
    };
    match result {
        Ok(kb) => {
            engine.keyboard = kb;
            tracing::debug!("Engine {} switched to method {}", engine_id, method);
            true
        }
        Err(e) => {
            tracing::warn!("buttre_engine_set_method: failed to build keyboard: {}", e);
            false
        }
    }
}

/// Enable or disable an engine.
///
/// A disabled engine returns null for all `process_key` calls.
/// Call `buttre_engine_free` to release memory — disabling alone does not free.
#[no_mangle]
pub extern "C" fn buttre_engine_set_enabled(engine_id: u64, enabled: bool) {
    if engine_id == 0 { return; }
    let mut engines = ENGINES.lock().unwrap();
    if let Some(engine) = engines.as_mut().and_then(|m| m.get_mut(&engine_id)) {
        if engine.enabled && !enabled {
            engine.keyboard.reset();
        }
        engine.enabled = enabled;
        tracing::debug!("Engine {} enabled={}", engine_id, enabled);
    }
}

// ============================================================================
// INTERNAL HELPERS
// ============================================================================

fn store_and_return_cstring(text: String) -> *const c_char {
    match CString::new(text) {
        Ok(cstring) => {
            let ptr = cstring.as_ptr();
            *LAST_RESULT.lock().unwrap() = Some(cstring);
            ptr
        }
        Err(e) => {
            eprintln!("[buttre FFI] ERROR: Invalid string: {}", e);
            std::ptr::null()
        }
    }
}

/// Map macOS virtual keycode to character (US ANSI layout).
///
/// Letter case uses `capslock XOR shift` so CapsLock+Shift = lowercase,
/// matching system behavior and the gonhanh Engine.cpp reference.
///
/// Tab (48), Return (36), Space (49), Escape (53) are intentionally omitted;
/// break-key handling is the responsibility of the Swift host.
fn keycode_to_char(keycode: u16, shift: bool, capslock: bool) -> Option<char> {
    // Letter keycodes — apply CapsLock XOR Shift for case
    let letter = match keycode {
        0 => 'a', 1 => 's', 2 => 'd', 3 => 'f', 4 => 'h', 5 => 'g', 6 => 'z',
        7 => 'x', 8 => 'c', 9 => 'v', 11 => 'b', 12 => 'q', 13 => 'w', 14 => 'e',
        15 => 'r', 16 => 'y', 17 => 't', 31 => 'o', 32 => 'u', 34 => 'i', 35 => 'p',
        37 => 'l', 38 => 'j', 40 => 'k', 45 => 'n', 46 => 'm',
        _ => return keycode_to_char_non_letter(keycode, shift),
    };
    let uppercase = capslock != shift;
    Some(if uppercase { letter.to_ascii_uppercase() } else { letter })
}

fn keycode_to_char_non_letter(keycode: u16, shift: bool) -> Option<char> {
    Some(if shift {
        match keycode {
            // Shifted digits
            18 => '!', 19 => '@', 20 => '#', 21 => '$', 23 => '%',
            22 => '^', 26 => '&', 28 => '*', 25 => '(', 29 => ')',
            // Shifted punctuation
            27 => '_', 24 => '+',
            33 => '{', 30 => '}', 42 => '|',
            41 => ':', 39 => '"',
            43 => '<', 47 => '>', 44 => '?', 50 => '~',
            _ => return None,
        }
    } else {
        match keycode {
            // Unshifted digits
            18 => '1', 19 => '2', 20 => '3', 21 => '4', 23 => '5',
            22 => '6', 26 => '7', 28 => '8', 25 => '9', 29 => '0',
            // Unshifted punctuation
            27 => '-', 24 => '=',
            33 => '[', 30 => ']', 42 => '\\',
            41 => ';', 39 => '\'',
            43 => ',', 47 => '.', 44 => '/', 50 => '`',
            _ => return None,
        }
    })
}

// ============================================================================
// TESTS
// ============================================================================
