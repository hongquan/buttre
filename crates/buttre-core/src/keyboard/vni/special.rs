//! VNI Special Rules
//!
//! **Tests**: Integration tests for this module are located in `crates/buttre-core/tests/keyboard_vni_tests.rs`.
//!
//! This module defines VNI-specific context rules for complex cases.

use buttre_engine::pipeline::rules::{ContextRule, RuleMatcher, RuleAction};

/// Get all VNI special context rules
///
/// ## Returns
/// Vector of ContextRules for VNI special cases
pub fn get_rules() -> Vec<ContextRule> {
    vec![
        double_char_block(),
        uo_7_block(),
        uu_7_block(),
    ]
}

/// Block '7' Stage 4 transformation for "uo" at end of syllable
pub fn uo_7_block() -> ContextRule {
    ContextRule::new(
        "vni_uo_7",
        RuleMatcher::And(vec![
            RuleMatcher::EndsWith("uo".to_string()),
            RuleMatcher::LastChar('7'),
        ]),
        RuleAction::Skip,
    )
}

/// Block '7' Stage 4 transformation for "uu"
pub fn uu_7_block() -> ContextRule {
    ContextRule::new(
        "vni_uu_7",
        RuleMatcher::And(vec![
            RuleMatcher::EndsWith("uu".to_string()),
            RuleMatcher::LastChar('7'),
        ]),
        RuleAction::Skip,
    )
}

/// Block double digit transformation
///
/// ## Purpose
/// VNI uses digits (6,7,8,9) for transformations.
/// Typing the same digit twice should NOT transform twice.
///
/// ## Example
/// - "a6" → "â" ✅
/// - "a66" → "â6" (not "â" + transform again) ✅
pub fn double_char_block() -> ContextRule {
    ContextRule::new(
        "vni_double_char",
        RuleMatcher::Custom(Box::new(|ctx| {
            // Check if current input is a VNI digit
            if let Some(last_char) = ctx.last_char {
                if "6789".contains(last_char) {
                    // Check if last transform was also this digit
                    if ctx.last_transform_key == Some(last_char) {
                        return true; // Block double transform
                    }
                }
            }
            false
        })),
        RuleAction::Skip,
    )
}

