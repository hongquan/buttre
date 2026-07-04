//! VNI Transformation Rules
//!
//! **Tests**: Integration tests for this module are located in `crates/buttre-core/tests/keyboard_vni_tests.rs`.
//!
//! This module defines all VNI character transformations.

use std::collections::HashMap;

/// Get all VNI transformation rules
///
/// ## Returns
/// HashMap mapping input sequences to output characters
///
/// ## Examples
/// - "a6" → "â"
/// - "a8" → "ă"
/// - "d9" → "đ"
pub fn get_rules() -> HashMap<String, String> {
    let mut rules = HashMap::new();

    // Basic transformations
    rules.insert("a6".to_string(), "â".to_string());
    rules.insert("a8".to_string(), "ă".to_string());
    rules.insert("d9".to_string(), "đ".to_string());
    rules.insert("e6".to_string(), "ê".to_string());
    rules.insert("o6".to_string(), "ô".to_string());
    rules.insert("o7".to_string(), "ơ".to_string());
    rules.insert("u7".to_string(), "ư".to_string());

    // Uppercase variants
    rules.insert("A6".to_string(), "Â".to_string());
    rules.insert("A8".to_string(), "Ă".to_string());
    rules.insert("D9".to_string(), "Đ".to_string());
    rules.insert("E6".to_string(), "Ê".to_string());
    rules.insert("O6".to_string(), "Ô".to_string());
    rules.insert("O7".to_string(), "Ơ".to_string());
    rules.insert("U7".to_string(), "Ư".to_string());

    rules
}
