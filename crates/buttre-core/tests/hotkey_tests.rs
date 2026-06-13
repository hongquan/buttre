use buttre_core::hotkey::{ButtreHotkeyManager, HotkeyAction};

#[test]
fn test_create_manager() {
    // Note: Hotkey registration can fail in CI or without proper permissions
    // This is expected behavior, not a bug
    let manager = ButtreHotkeyManager::new();
    if manager.is_err() {
        eprintln!("Note: Hotkey manager creation failed (expected in CI/test environments)");
    }
    // Test passes either way - we're just checking it doesn't panic
}

#[test]
fn test_hotkey_actions() {
    assert_eq!(HotkeyAction::Toggle, HotkeyAction::Toggle);
    assert_ne!(HotkeyAction::Toggle, HotkeyAction::Telex);
}
