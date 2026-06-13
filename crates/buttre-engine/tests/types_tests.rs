use buttre_engine::types::{Action, Config};

#[test]
fn test_action() {
    let action = Action::Commit("test".to_string());
    assert!(matches!(action, Action::Commit(_)));
}

#[test]
fn test_config_default() {
    let config = Config::default();
    assert!(config.enabled);
    assert!(config.auto_commit);
}
