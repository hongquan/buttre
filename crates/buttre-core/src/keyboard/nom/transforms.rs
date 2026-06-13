//! Nôm Transformation Rules
//!
//! **Tests**: Integration tests for this module are located in `crates/buttre-core/tests/keyboard_nom_tests.rs`.
//!
//! This module defines all Nôm character transformations.
//! 
//! TODO: Add actual Nôm-specific transforms
//! For now, this is a placeholder with basic Vietnamese transforms.

use std::collections::HashMap;

/// Get all Nôm transformation rules
///
/// ## Returns
/// HashMap mapping input sequences to output characters
///
/// ## Note
/// This is a placeholder implementation.
/// Actual Nôm transforms need to be researched and added.
pub fn get_rules() -> HashMap<String, String> {
    let mut rules = HashMap::new();
    
    // TODO: Add Nôm-specific transformations
    // For now, using basic Vietnamese transforms as placeholder
    
    // Basic transformations (placeholder)
    rules.insert("aa".to_string(), "â".to_string());
    rules.insert("aw".to_string(), "ă".to_string());
    rules.insert("dd".to_string(), "đ".to_string());
    rules.insert("ee".to_string(), "ê".to_string());
    rules.insert("oo".to_string(), "ô".to_string());
    rules.insert("ow".to_string(), "ơ".to_string());
    rules.insert("uw".to_string(), "ư".to_string());
    
    rules
}

