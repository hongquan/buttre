use buttre_core::ButtreCore;

#[test]
fn test_create_buttre_core() {
    let core = ButtreCore::new();
    assert!(core.is_ok());
}

#[test]
fn test_init() {
    let mut core = ButtreCore::new().unwrap();
    let result = core.init();
    assert!(result.is_ok());
}

#[test]
fn test_switch_method() {
    let mut core = ButtreCore::new().unwrap();
    core.init().unwrap();

    let result = core.switch_method("telex");
    assert!(result.is_ok());
    assert_eq!(core.current_method(), "telex");
}

#[test]
fn test_toggle() {
    let mut core = ButtreCore::new().unwrap();
    core.init().unwrap();

    // Start with telex
    core.switch_method("telex").unwrap();
    assert!(core.is_enabled());

    // Toggle to english
    core.toggle().unwrap();
    assert!(!core.is_enabled());

    // Toggle back to telex
    core.toggle().unwrap();
    assert!(core.is_enabled());
}

#[test]
fn test_list_methods() {
    let core = ButtreCore::new().unwrap();
    let methods = core.list_methods();

    // Should have at least telex and vni
    assert!(methods.len() >= 2);
}
