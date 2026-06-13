use buttre_core::events::{AppEvent, LogLevel};

#[test]
fn test_event_creation() {
    let event = AppEvent::method_changed("telex", true);
    match event {
        AppEvent::MethodChanged { method, enabled } => {
            assert_eq!(method, "telex");
            assert!(enabled);
        }
        _ => panic!("Wrong event type"),
    }
}

#[test]
fn test_log_levels() {
    assert!(LogLevel::Error > LogLevel::Warn);
    assert!(LogLevel::Warn > LogLevel::Info);
    assert!(LogLevel::Info > LogLevel::Debug);
}
