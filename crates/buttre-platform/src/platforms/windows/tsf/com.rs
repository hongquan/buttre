//! COM helper utilities

use std::sync::atomic::{AtomicU32, Ordering};
use windows::core::BOOL;
use windows::Win32::Foundation::HINSTANCE;

use super::logging::{init_logging, log_debug};

/// Global DLL reference count
static DLL_REF_COUNT: AtomicU32 = AtomicU32::new(0);

/// Increment DLL reference count
pub fn dll_add_ref() {
    DLL_REF_COUNT.fetch_add(1, Ordering::SeqCst);
}

/// Decrement DLL reference count
pub fn dll_release() {
    DLL_REF_COUNT.fetch_sub(1, Ordering::SeqCst);
}

/// Get current DLL reference count
pub fn dll_get_ref_count() -> u32 {
    DLL_REF_COUNT.load(Ordering::SeqCst)
}

/// Check if DLL can be unloaded
pub fn dll_can_unload() -> bool {
    dll_get_ref_count() == 0
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn DllMain(
    _hinst_dll: HINSTANCE,
    fdw_reason: u32,
    _lpv_reserved: *const core::ffi::c_void,
) -> BOOL {
    const DLL_PROCESS_ATTACH: u32 = 1;
    const DLL_PROCESS_DETACH: u32 = 0;

    // A panic crossing an FFI boundary is undefined behaviour. Catch and swallow
    // any panic here so a bug in the logging path cannot crash the host process.
    let ok = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        match fdw_reason {
            DLL_PROCESS_ATTACH => {
                init_logging();
                log_debug("DLL_PROCESS_ATTACH - buttre TSF loaded");
            }
            DLL_PROCESS_DETACH => {
                log_debug("DLL_PROCESS_DETACH - buttre TSF unloaded");
            }
            _ => {}
        }
    }));

    BOOL(ok.is_ok() as i32)
}
