use buttre_core::keyboard::nom::{special, transforms};

#[test]
fn test_get_rules() {
    let rules = special::get_rules();
    // For now, should be empty
    assert_eq!(rules.len(), 0);
}

#[test]
fn test_get_transforms() {
    let rules = transforms::get_rules();
    assert!(!rules.is_empty());
}
