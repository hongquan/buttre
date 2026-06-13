#![cfg(platform_linux)]
use buttre_platform::platforms::linux::ibus::*;

#[test]
fn test_engine_creation() {
    let engine = ButtreEngine::new();
    assert_eq!(*engine.preedit.lock().unwrap(), "");
}

#[test]
fn test_keyval_conversion() {
    // Test lowercase
    assert_eq!(keyval_to_char(0x0061, 0), Some('a'));

    // Test uppercase (with shift)
    assert_eq!(keyval_to_char(0x0061, 0x1), Some('A'));

    // Test space
    assert_eq!(keyval_to_char(0x0020, 0), Some(' '));
}
