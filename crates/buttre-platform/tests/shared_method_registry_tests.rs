use buttre_platform::shared::input::{MethodRegistry, MethodSource};

#[test]
fn test_registry_creation() {
    let registry = MethodRegistry::new();
    
    // Should have at least 3 built-in methods
    assert!(registry.get_all().len() >= 3);
    
    // Check built-in methods
    assert!(registry.get("telex").is_some());
    assert!(registry.get("vni").is_some());
    assert!(registry.get("nom").is_some());
}

#[test]
fn test_get_builtin() {
    let registry = MethodRegistry::new();
    let builtin = registry.get_builtin();
    
    assert_eq!(builtin.len(), 3);
    assert!(builtin.iter().all(|m| m.source == MethodSource::BuiltIn));
}
