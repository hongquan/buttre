//! Telex Special Rules
//!
//! **Tests**: Integration tests for this module are located in `crates/buttre-core/tests/keyboard_telex_tests.rs`.
//!
//! This module defines Telex-specific context rules for complex cases.

use buttre_engine::pipeline::rules::{ContextRule, RuleAction, RuleMatcher};

/// Get all Telex special context rules
///
/// ## Returns
/// Vector of ContextRules for Telex special cases
pub fn get_rules() -> Vec<ContextRule> {
    vec![w_after_u_horn(), oeo_block(), ua_tone_position()]
}

/// Block 'w' transformation after 'ư'
///
/// ## Purpose
/// In Telex, "uw" → "ư", but "ư" + "w" should NOT transform again
///
/// ## Example
/// - "uw" → "ư" ✅
/// - "ư" + "w" → "ưw" (no transformation) ✅
fn w_after_u_horn() -> ContextRule {
    ContextRule::new(
        "telex_w_after_ư",
        RuleMatcher::And(vec![
            RuleMatcher::LastTransformKey('w'),
            RuleMatcher::EndsWith("ư".to_string()),
            RuleMatcher::LastChar('w'),
        ]),
        RuleAction::Skip,
    )
}

/// Block 'o' transformation in "oeo" pattern
///
/// ## Purpose
/// "oeo" pattern should NOT transform middle 'o' to 'ô'
///
/// ## Example
/// - "oeo" → "oeo" (no transformation) ✅
fn oeo_block() -> ContextRule {
    ContextRule::new(
        "telex_oeo_block",
        RuleMatcher::Pattern("oeo".to_string()),
        RuleAction::Skip,
    )
}

/// Set tone position for "ua" combination
///
/// ## Purpose
/// "ua" combination needs special tone positioning
///
/// ## Example
/// - "qua" → tone on 'a'
/// - "lua" → tone on 'u'
fn ua_tone_position() -> ContextRule {
    ContextRule::new(
        "telex_ua_tone",
        RuleMatcher::Pattern("ua".to_string()),
        RuleAction::Custom(Box::new(|ctx| {
            // Find position of 'u' in "ua"
            if let Some(pos) = ctx.syllable_buffer.find('u') {
                ctx.tone_position = Some(pos);
            }
        })),
    )
}
