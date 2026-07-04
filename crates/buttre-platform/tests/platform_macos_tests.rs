#![cfg(platform_macos)]
use buttre_platform::platforms::macos::ffi::*;

#[test]
fn test_engine_lifecycle() {
    let id = buttre_engine_new();
    assert_ne!(id, 0);

    // keycode 0 = 'a', no shift, no capslock
    let result = buttre_engine_process_key(id, 0, false, false);
    assert!(!result.is_null());

    buttre_engine_free(id);
}

#[test]
fn test_invalid_handle() {
    // All functions handle invalid ID gracefully
    assert_eq!(
        buttre_engine_process_key(0, 0, false, false),
        std::ptr::null()
    );
    assert_eq!(
        buttre_engine_process_key(99999, 0, false, false),
        std::ptr::null()
    );
    assert_eq!(buttre_engine_process_backspace(0), std::ptr::null());
    buttre_engine_reset(0); // No crash
    buttre_engine_free(0); // No crash
    buttre_engine_free(99999); // No crash
}

#[test]
fn test_multiple_engines() {
    let id1 = buttre_engine_new();
    let id2 = buttre_engine_new();

    assert_ne!(id1, id2);

    buttre_engine_process_key(id1, 0, false, false); // 'a'
    buttre_engine_process_key(id2, 1, false, false); // 's'

    buttre_engine_free(id1);
    buttre_engine_free(id2);
}
