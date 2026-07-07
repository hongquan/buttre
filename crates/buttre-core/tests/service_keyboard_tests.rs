use buttre_core::events::create_event_bus;
use buttre_core::services::{KeyboardService, Preset};
use buttre_core::Action;

#[test]
fn test_create_preset() {
    let bus = create_event_bus();
    let mut service = KeyboardService::new(bus);

    assert!(service.create_preset(Preset::Telex).is_ok());
    assert!(service.has("telex"));
}

#[test]
fn test_switch_keyboard() {
    let bus = create_event_bus();
    let mut service = KeyboardService::new(bus);

    service.create_preset(Preset::Telex).unwrap();
    service.create_preset(Preset::Vni).unwrap();

    assert!(service.switch("telex").is_ok());
    assert_eq!(service.current(), Some("telex"));

    assert!(service.switch("vni").is_ok());
    assert_eq!(service.current(), Some("vni"));
}

#[test]
fn test_switch_nonexistent() {
    let bus = create_event_bus();
    let mut service = KeyboardService::new(bus);

    assert!(service.switch("nonexistent").is_err());
}

#[test]
fn test_list_keyboards() {
    let bus = create_event_bus();
    let mut service = KeyboardService::new(bus);

    service.create_preset(Preset::Telex).unwrap();
    service.create_preset(Preset::Vni).unwrap();

    let list = service.list();
    assert_eq!(list.len(), 2);
    assert!(list.contains(&"telex"));
    assert!(list.contains(&"vni"));
}

#[test]
fn test_process_without_keyboard() {
    let bus = create_event_bus();
    let mut service = KeyboardService::new(bus);

    let action = service.process('a').unwrap();
    assert!(matches!(action, Action::DoNothing));
}
