use buttre_core::services::MethodRegistry;
use buttre_core::events::{MethodInfo, MethodSource};

#[test]
fn test_scan_methods() {
    let mut registry = MethodRegistry::new().unwrap();
    registry.scan().unwrap();
    
    // Should have at least telex and vni
    assert!(registry.count() >= 2);
    assert!(registry.contains("telex"));
    assert!(registry.contains("vni"));
}

#[test]
fn test_get_method() {
    let mut registry = MethodRegistry::new().unwrap();
    registry.scan().unwrap();
    
    let telex = registry.get("telex");
    assert!(telex.is_some());
    assert_eq!(telex.unwrap().id, "telex");
}

#[test]
fn test_by_language() {
    let mut registry = MethodRegistry::new().unwrap();
    registry.scan().unwrap();
    
    let vietnamese = registry.by_language("vietnamese");
    assert!(vietnamese.len() >= 2); // At least telex and vni
}

#[test]
fn test_builtins() {
    let mut registry = MethodRegistry::new().unwrap();
    registry.scan().unwrap();
    
    let builtins = registry.builtins();
    assert!(builtins.len() >= 2); // telex and vni
    
    // All should be builtin
    for method in builtins {
        assert!(matches!(method.source, MethodSource::Builtin));
    }
}

#[test]
fn test_register_unregister() {
    let mut registry = MethodRegistry::new().unwrap();
    
    let custom = MethodInfo {
        id: "custom".to_string(),
        name: "Custom Method".to_string(),
        language: "vietnamese".to_string(),
        source: MethodSource::Custom("/path/to/custom.toml".to_string()),
    };
    
    registry.register(custom);
    assert!(registry.contains("custom"));
    assert_eq!(registry.count(), 1);
    
    let removed = registry.unregister("custom");
    assert!(removed);
    assert!(!registry.contains("custom"));
    assert_eq!(registry.count(), 0);
}
