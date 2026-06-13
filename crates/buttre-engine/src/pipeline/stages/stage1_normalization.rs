//! Stage 1: Input Normalization
//!
//! **Tests**: Integration tests for this stage are located in `crates/buttre-engine/tests/stage1_normalization_tests.rs`.
//!
//! This stage normalizes the input character and updates the raw buffer.
//!
//! ## Algorithm
//!
//! 1. Normalize the input character (lowercase for Vietnamese input)
//! 2. Add the normalized character to the raw buffer
//! 3. Always return Continue to proceed to the next stage
//!
//! ## Rationale
//!
//! Vietnamese input methods are case-insensitive for transformation keys.
//! For example, both 'A' and 'a' should trigger the same transformations.
//! However, we preserve the original case for the final output.

use crate::pipeline::{PipelineStage, StageResult, TypingContext};

/// Stage 1: Input Normalization
///
/// Normalizes input characters and updates the raw buffer.
///
/// ## Algorithm
///
/// This stage performs the following operations:
/// 1. **Case Normalization**: Convert uppercase to lowercase for processing
///    - Vietnamese transformation keys are case-insensitive
///    - Example: 'A' and 'a' both contribute to "aa" → "â"
/// 2. **Raw Buffer Update**: Append the normalized character to raw_buffer
///    - The raw_buffer tracks actual keystrokes for undo/history
/// 3. **Flow Control**: Always return Continue
///    - This stage never blocks input, it just normalizes
///
/// ## Example
///
/// ```text
/// Input: 'A'
/// Action: Normalize to 'a', append to raw_buffer
/// Output: StageResult::Continue
/// Context: raw_buffer = "a"
/// ```
#[derive(Debug, Clone)]
pub struct NormalizationStage {
    /// Whether to preserve original case (future feature)
    #[allow(dead_code)]
    pub preserve_case: bool,
}

impl NormalizationStage {
    /// Create a new normalization stage
    pub fn new() -> Self {
        Self {
            preserve_case: false,
        }
    }

    /// Normalize a character for processing
    ///
    /// ## Algorithm
    ///
    /// - For alphabetic characters: Convert to lowercase
    /// - For non-alphabetic: Return as-is
    ///
    /// This ensures consistent processing regardless of input case.
    pub fn normalize_char(&self, ch: char) -> char {
        if ch.is_alphabetic() {
            ch.to_lowercase().next().unwrap_or(ch)
        } else {
            ch
        }
    }
}

impl Default for NormalizationStage {
    fn default() -> Self {
        Self::new()
    }
}

impl PipelineStage for NormalizationStage {
    #[tracing::instrument(skip(self, ctx), fields(stage = "normalization", input))]
    fn process(&self, ctx: &mut TypingContext, input: char) -> StageResult {
        // Algorithm:
        // 1. Normalize the input character (push_raw handles case tracking)
        let _normalized = self.normalize_char(input);

        // 2. Update the char buffer (automatically tracks case)
        ctx.push_raw(input);

        // 3. Continue to next stage
        // This stage never blocks, it just normalizes and records input
        StageResult::Continue
    }

    fn name(&self) -> &'static str {
        "NormalizationStage"
    }

    fn reset(&mut self) {
        // No internal state to reset
    }
}
