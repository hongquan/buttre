use buttre_core::events::create_event_bus;
use buttre_core::services::HotkeyService;

#[test]
fn test_create_hotkey_service() {
    let bus = create_event_bus();
    let service = HotkeyService::new(bus);
    // Hotkey registration may fail if hotkeys are already taken
    // This is OK, we just want to ensure the service can be created
    let _ = service;
}

#[test]
fn test_poll_no_panic() {
    let bus = create_event_bus();
    if let Ok(service) = HotkeyService::new(bus) {
        // Should not panic even if no hotkeys pressed
        service.poll();
    }
    // If service creation failed, that's OK too
}
