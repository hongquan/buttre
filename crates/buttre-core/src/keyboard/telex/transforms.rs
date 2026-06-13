//! Telex Transformation Rules
//!
//! **Tests**: Integration tests for this module are located in `crates/buttre-core/tests/keyboard_telex_tests.rs`.
//!
//! This module defines all Telex character transformations.

use std::collections::HashMap;

/// Get all Telex transformation rules
///
/// ## Returns
/// HashMap mapping input sequences to output characters
///
/// ## Examples
/// - "aa" → "â"
/// - "aw" → "ă"
/// - "dd" → "đ"
pub fn get_rules() -> HashMap<String, String> {
    let mut rules = HashMap::new();
    
    // Basic transformations
    rules.insert("aa".to_string(), "â".to_string());
    rules.insert("aw".to_string(), "ă".to_string());
    rules.insert("dd".to_string(), "đ".to_string());
    rules.insert("ee".to_string(), "ê".to_string());
    rules.insert("oo".to_string(), "ô".to_string());
    rules.insert("ow".to_string(), "ơ".to_string());
    rules.insert("uw".to_string(), "ư".to_string());

    // Standalone single-char transforms.
    // 'w' alone (as first char of a syllable) maps to 'ư' — e.g. "win"→"ưin",
    // "w" prefix before non-aw/ow/uw vowels.  This matches stage4 hardcoded
    // behaviour ('w' → "ư") and is needed by the compose engine which only
    // uses the rules table (no hardcoded stage4 fallback).
    rules.insert("w".to_string(), "ư".to_string());
    rules.insert("W".to_string(), "Ư".to_string());
    
    // Uppercase variants
    rules.insert("AA".to_string(), "Â".to_string());
    rules.insert("AW".to_string(), "Ă".to_string());
    rules.insert("Aw".to_string(), "Ă".to_string());
    rules.insert("DD".to_string(), "Đ".to_string());
    rules.insert("Dd".to_string(), "Đ".to_string());
    rules.insert("EE".to_string(), "Ê".to_string());
    rules.insert("OO".to_string(), "Ô".to_string());
    rules.insert("OW".to_string(), "Ơ".to_string());
    rules.insert("Ow".to_string(), "Ơ".to_string());
    rules.insert("UW".to_string(), "Ư".to_string());
    rules.insert("Uw".to_string(), "Ư".to_string());
    
    // NOTE: "uow" → "ươ" rules are intentionally REMOVED
    // Stage 6 handles uo+w contextually:
    // - thuowr → thuở (uơ, only hook o when at end of word)
    // - tuowng → tương (ươ, hook both when followed by consonant)
    // Keeping this HashMap rule would override the Stage 4 skip logic.
    
    rules
}

