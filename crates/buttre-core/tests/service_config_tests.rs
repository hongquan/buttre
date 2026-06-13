use buttre_core::services::ConfigService;

#[test]
fn test_load_builtin_telex() {
    let service = ConfigService::new().unwrap();
    let config = service.load("telex");
    assert!(config.is_ok());
    
    let config = config.unwrap();
    assert_eq!(config.metadata.id, "telex");
}

#[test]
fn test_load_builtin_vni() {
    let service = ConfigService::new().unwrap();
    let config = service.load("vni");
    assert!(config.is_ok());
    
    let config = config.unwrap();
    assert_eq!(config.metadata.id, "vni");
}

#[test]
fn test_load_nonexistent() {
    let service = ConfigService::new().unwrap();
    let config = service.load("nonexistent");
    assert!(config.is_err());
}

#[test]
fn test_list_configs() {
    let service = ConfigService::new().unwrap();
    let configs = service.list().unwrap();
    
    // Should have at least telex and vni
    assert!(configs.len() >= 2);
    
    let ids: Vec<_> = configs.iter().map(|c| c.id.as_str()).collect();
    assert!(ids.contains(&"telex"));
    assert!(ids.contains(&"vni"));
}

#[test]
fn test_exists() {
    let service = ConfigService::new().unwrap();
    
    assert!(service.exists("telex"));
    assert!(service.exists("vni"));
    assert!(!service.exists("nonexistent"));
}
