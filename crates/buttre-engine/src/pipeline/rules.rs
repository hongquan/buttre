//! # Enhanced Pipeline Rules
//!
//! ## Purpose:
//! Provides advanced rule types for encoding complex input method logic
//! within the config-driven pipeline architecture.
//!
//! ## Architecture Context:
//! - Layer: Engine (buttre-engine/src/pipeline/rules.rs)
//! - Used by: PipelineConfig, all 9 pipeline stages
//! - Dependencies: TypingContext
//!
//! ## Key Concepts:
//! - **ContextRule**: Custom closures that can read/modify context based on complex conditions
//! - **ConditionalRule**: Pattern-based rules with matchers and actions
//! - **RuleMatcher**: Flexible pattern matching for context state
//! - **RuleAction**: Actions to execute when rules match
//!
//! ## Note:
//! Input method specific logic (Telex, VNI, Nôm) has been moved to buttre-core.
//! See: buttre-core/src/keyboard/{telex,vni,nom}/special.rs
//!
//! ## Example:
//! ```rust
//! use buttre_engine::pipeline::rules::{ContextRule, RuleMatcher, RuleAction};
//!
//! // Custom rule example
//! let rule = ContextRule {
//!     name: "my_rule".to_string(),
//!     matcher: RuleMatcher::Custom(Box::new(|ctx| {
//!         ctx.syllable_buffer.len() > 3
//!     })),
//!     action: RuleAction::Skip,
//! };
//! ```

use crate::pipeline::context::TypingContext;
use std::fmt;

// ========================================
// ARCHITECTURE DECISION RECORD (ADR)
// ========================================
// Decision: Use trait objects (Box<dyn Fn>) for custom closures
// Date: 2025-12-23
// Context: Need to store complex logic in config without separate engines
// Options Considered:
//   1. Enum with predefined cases (limited flexibility)
//   2. Trait objects with closures (maximum flexibility)
//   3. Separate engine structs (breaks unified architecture)
// Decision: Option 2 (Trait objects)
// Rationale:
//   - Allows encoding ANY logic in config
//   - Maintains unified pipeline (no special cases)
//   - Config functions can be arbitrarily complex
//   - Type-safe at compile time
// Consequences:
//   - Cannot derive Debug/Clone automatically (need manual impl)
//   - Slightly more complex API
//   - But: maximum flexibility, consistent architecture
// ========================================

/// Rule Matcher - Determines when a rule should apply
///
/// ## Flow:
/// 1. Matcher is evaluated against current TypingContext
/// 2. If matcher returns true, the associated action is executed
/// 3. Multiple matchers can be combined with And/Or logic
pub enum RuleMatcher {
    /// Always match
    Always,

    /// Never match (disabled rule)
    Never,

    /// Match if syllable buffer matches pattern
    /// Example: Pattern("ươ") matches if buffer contains "ươ"
    Pattern(String),

    /// Match if syllable buffer ends with pattern
    /// Example: EndsWith("ư") matches "trư", "cư", etc.
    EndsWith(String),

    /// Match if syllable buffer starts with pattern
    /// Example: StartsWith("qu") matches "qua", "quê", etc.
    StartsWith(String),

    /// Match if last input character equals this
    /// Example: LastChar('w') matches if user just typed 'w'
    LastChar(char),

    /// Match if last transform key equals this
    /// Example: LastTransformKey('w') matches if last transform used 'w'
    LastTransformKey(char),

    /// Match if buffer length equals this
    /// Example: Length(3) matches "abc" but not "ab"
    Length(usize),

    /// Match if buffer length is in range (min, max)
    /// Example: LengthRange(2, 5) matches "ab", "abc", "abcd", "abcde"
    LengthRange(usize, usize),

    /// Combine matchers with AND logic
    /// Example: And(vec![EndsWith("ư"), LastChar('w')])
    And(Vec<RuleMatcher>),

    /// Combine matchers with OR logic
    /// Example: Or(vec![Pattern("oa"), Pattern("oe")])
    Or(Vec<RuleMatcher>),

    /// Negate a matcher
    /// Example: Not(Box::new(Pattern("ươ")))
    Not(Box<RuleMatcher>),

    /// Custom closure for complex logic
    /// Example: Custom(Box::new(|ctx| ctx.syllable_buffer.chars().count() > 3))
    Custom(Box<dyn Fn(&TypingContext) -> bool + Send + Sync>),
}

impl RuleMatcher {
    /// Evaluate this matcher against the current context
    ///
    /// ## Flow:
    /// 1. Match on matcher type
    /// 2. Check condition against context
    /// 3. Return true if rule should apply
    pub fn matches(&self, ctx: &TypingContext) -> bool {
        match self {
            RuleMatcher::Always => true,
            RuleMatcher::Never => false,
            RuleMatcher::Pattern(pattern) => ctx.syllable_buffer.contains(pattern),
            RuleMatcher::EndsWith(pattern) => ctx.syllable_buffer.ends_with(pattern),
            RuleMatcher::StartsWith(pattern) => ctx.syllable_buffer.starts_with(pattern),
            RuleMatcher::LastChar(ch) => ctx.last_char == Some(*ch),
            RuleMatcher::LastTransformKey(ch) => ctx.last_transform_key == Some(*ch),
            RuleMatcher::Length(len) => ctx.syllable_buffer.chars().count() == *len,
            RuleMatcher::LengthRange(min, max) => {
                let len = ctx.syllable_buffer.chars().count();
                len >= *min && len <= *max
            }
            RuleMatcher::And(matchers) => matchers.iter().all(|m| m.matches(ctx)),
            RuleMatcher::Or(matchers) => matchers.iter().any(|m| m.matches(ctx)),
            RuleMatcher::Not(matcher) => !matcher.matches(ctx),
            RuleMatcher::Custom(f) => f(ctx),
        }
    }
}

// Manual Debug implementation (closures don't derive Debug)
impl fmt::Debug for RuleMatcher {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuleMatcher::Always => write!(f, "Always"),
            RuleMatcher::Never => write!(f, "Never"),
            RuleMatcher::Pattern(p) => write!(f, "Pattern({:?})", p),
            RuleMatcher::EndsWith(p) => write!(f, "EndsWith({:?})", p),
            RuleMatcher::StartsWith(p) => write!(f, "StartsWith({:?})", p),
            RuleMatcher::LastChar(c) => write!(f, "LastChar({:?})", c),
            RuleMatcher::LastTransformKey(c) => write!(f, "LastTransformKey({:?})", c),
            RuleMatcher::Length(l) => write!(f, "Length({})", l),
            RuleMatcher::LengthRange(min, max) => write!(f, "LengthRange({}, {})", min, max),
            RuleMatcher::And(matchers) => write!(f, "And({:?})", matchers),
            RuleMatcher::Or(matchers) => write!(f, "Or({:?})", matchers),
            RuleMatcher::Not(m) => write!(f, "Not({:?})", m),
            RuleMatcher::Custom(_) => write!(f, "Custom(<closure>)"),
        }
    }
}

/// Rule Action - What to do when a rule matches
///
/// ## Flow:
/// 1. Action is executed when associated matcher returns true
/// 2. Action can modify context, skip processing, or transform buffer
pub enum RuleAction {
    /// Skip this transformation (no-op)
    /// Example: Used for blocking unwanted transforms
    Skip,

    /// Replace syllable buffer with this string
    /// Example: Replace("ươ") sets buffer to "ươ"
    Replace(String),

    /// Append to syllable buffer
    /// Example: Append("ng") adds "ng" to end
    Append(String),

    /// Set a context flag
    /// Example: SetFlag("w_converted", true)
    SetFlag(String, bool),

    /// Set last transform key
    /// Example: SetTransformKey('w')
    SetTransformKey(char),

    /// Clear syllable buffer
    Clear,

    /// Custom closure for complex actions
    /// Example: Custom(Box::new(|ctx| { ctx.tone_position = Some(2); }))
    Custom(Box<dyn Fn(&mut TypingContext) + Send + Sync>),
}

// Manual Debug implementation
impl fmt::Debug for RuleAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuleAction::Skip => write!(f, "Skip"),
            RuleAction::Replace(s) => write!(f, "Replace({:?})", s),
            RuleAction::Append(s) => write!(f, "Append({:?})", s),
            RuleAction::SetFlag(name, val) => write!(f, "SetFlag({:?}, {})", name, val),
            RuleAction::SetTransformKey(c) => write!(f, "SetTransformKey({:?})", c),
            RuleAction::Clear => write!(f, "Clear"),
            RuleAction::Custom(_) => write!(f, "Custom(<closure>)"),
        }
    }
}

impl RuleAction {
    /// Execute this action on the context
    ///
    /// ## Flow:
    /// 1. Match on action type
    /// 2. Modify context accordingly
    /// 3. Return whether to skip further processing
    ///
    /// ## Returns:
    /// - true: Skip further processing (action handled it)
    /// - false: Continue processing
    pub fn execute(&self, ctx: &mut TypingContext) -> bool {
        match self {
            RuleAction::Skip => true, // Signal to skip
            RuleAction::Replace(s) => {
                ctx.set_syllable(s.clone());
                false
            }
            RuleAction::Append(s) => {
                let mut buf = ctx.syllable_buffer.clone();
                buf.push_str(s);
                ctx.set_syllable(buf);
                false
            }
            RuleAction::SetFlag(name, value) => {
                ctx.set_flag(name, *value);
                false
            }
            RuleAction::SetTransformKey(ch) => {
                ctx.last_transform_key = Some(*ch);
                false
            }
            RuleAction::Clear => {
                ctx.clear();
                false
            }
            RuleAction::Custom(f) => {
                f(ctx);
                false
            }
        }
    }
}

/// Context Rule - Conditional rule with matcher and action
///
/// ## Purpose:
/// Encodes complex input method logic as config data.
/// When matcher returns true, action is executed.
///
/// ## Example:
/// ```rust
/// use buttre_engine::pipeline::{ContextRule, RuleMatcher, RuleAction};
///
/// // Block transformation in specific pattern
/// let rule = ContextRule {
///     name: "my_block_rule".to_string(),
///     matcher: RuleMatcher::And(vec![
///         RuleMatcher::Pattern("xyz".to_string()),
///         RuleMatcher::LastChar('z'),
///     ]),
///     action: RuleAction::Skip,
/// };
/// ```
#[derive(Debug)]
pub struct ContextRule {
    /// Rule name (for debugging)
    pub name: String,

    /// When to apply this rule
    pub matcher: RuleMatcher,

    /// What to do when rule matches
    pub action: RuleAction,
}

impl ContextRule {
    /// Create a new context rule
    pub fn new(name: impl Into<String>, matcher: RuleMatcher, action: RuleAction) -> Self {
        Self {
            name: name.into(),
            matcher,
            action,
        }
    }

    /// Check if this rule applies to the current context
    pub fn matches(&self, ctx: &TypingContext) -> bool {
        self.matcher.matches(ctx)
    }

    /// Execute this rule's action
    ///
    /// ## Returns:
    /// - true: Skip further processing
    /// - false: Continue processing
    pub fn execute(&self, ctx: &mut TypingContext) -> bool {
        self.action.execute(ctx)
    }
}

/// Conditional Rule - Transform rule with conditions
///
/// ## Purpose:
/// Extends basic transform rules with conditional logic.
/// Example: "aa → â" but only if not after "q"
///
/// ## Example:
/// ```rust
/// use buttre_engine::pipeline::{ConditionalRule, RuleMatcher};
///
/// let rule = ConditionalRule {
///     from: "aa".to_string(),
///     to: "â".to_string(),
///     condition: Some(RuleMatcher::Not(Box::new(
///         RuleMatcher::StartsWith("q".to_string())
///     ))),
/// };
/// ```
#[derive(Debug)]
pub struct ConditionalRule {
    /// Source pattern
    pub from: String,

    /// Target result
    pub to: String,

    /// Optional condition (None = always apply)
    pub condition: Option<RuleMatcher>,
}

impl ConditionalRule {
    /// Create a new conditional rule
    pub fn new(from: impl Into<String>, to: impl Into<String>) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
            condition: None,
        }
    }

    /// Create a conditional rule with a condition
    pub fn with_condition(
        from: impl Into<String>,
        to: impl Into<String>,
        condition: RuleMatcher,
    ) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
            condition: Some(condition),
        }
    }

    /// Check if this rule should apply
    pub fn should_apply(&self, ctx: &TypingContext) -> bool {
        if let Some(ref condition) = self.condition {
            condition.matches(ctx)
        } else {
            true // No condition = always apply
        }
    }
}

// ========================================
// NOTE: Input Method Specific Logic Moved
// ========================================
// SpecialHandler enum and implementations have been moved to buttre-core:
// - buttre-core/src/keyboard/telex/special.rs (Telex rules)
// - buttre-core/src/keyboard/vni/special.rs (VNI rules)
// - buttre-core/src/keyboard/nom/special.rs (Nôm rules)
//
// This keeps buttre-engine generic and focused on pipeline infrastructure.
// ========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matcher_always() {
        let ctx = TypingContext::new();
        let matcher = RuleMatcher::Always;
        assert!(matcher.matches(&ctx));
    }

    #[test]
    fn test_matcher_pattern() {
        let mut ctx = TypingContext::new();
        ctx.set_syllable("hello".to_string());

        let matcher = RuleMatcher::Pattern("ell".to_string());
        assert!(matcher.matches(&ctx));

        let matcher = RuleMatcher::Pattern("xyz".to_string());
        assert!(!matcher.matches(&ctx));
    }

    #[test]
    fn test_matcher_ends_with() {
        let mut ctx = TypingContext::new();
        ctx.set_syllable("trư".to_string());

        let matcher = RuleMatcher::EndsWith("ư".to_string());
        assert!(matcher.matches(&ctx));
    }

    #[test]
    fn test_matcher_and() {
        let mut ctx = TypingContext::new();
        ctx.set_syllable("trư".to_string());
        ctx.last_char = Some('w');

        let matcher = RuleMatcher::And(vec![
            RuleMatcher::EndsWith("ư".to_string()),
            RuleMatcher::LastChar('w'),
        ]);
        assert!(matcher.matches(&ctx));
    }

    #[test]
    fn test_action_skip() {
        let mut ctx = TypingContext::new();
        let action = RuleAction::Skip;
        assert!(action.execute(&mut ctx)); // Should return true (skip)
    }

    #[test]
    fn test_action_replace() {
        let mut ctx = TypingContext::new();
        let action = RuleAction::Replace("test".to_string());
        assert!(!action.execute(&mut ctx)); // Should return false (continue)
        assert_eq!(ctx.syllable_buffer, "test");
    }

    #[test]
    fn test_context_rule() {
        let mut ctx = TypingContext::new();
        ctx.set_syllable("trư".to_string());
        ctx.last_char = Some('w');
        ctx.last_transform_key = Some('w');

        let rule = ContextRule::new(
            "test_rule",
            RuleMatcher::And(vec![
                RuleMatcher::EndsWith("ư".to_string()),
                RuleMatcher::LastChar('w'),
            ]),
            RuleAction::Skip,
        );

        assert!(rule.matches(&ctx));
        assert!(rule.execute(&mut ctx)); // Should skip
    }

    #[test]
    fn test_conditional_rule() {
        let mut ctx = TypingContext::new();
        ctx.set_syllable("aa".to_string());

        let rule = ConditionalRule::with_condition(
            "aa",
            "â",
            RuleMatcher::Not(Box::new(RuleMatcher::StartsWith("q".to_string()))),
        );

        assert!(rule.should_apply(&ctx));

        ctx.set_syllable("qaa".to_string());
        assert!(!rule.should_apply(&ctx));
    }
}
