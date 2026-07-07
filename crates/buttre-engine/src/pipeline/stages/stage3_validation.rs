//! Stage 3: Structure Validation
//!
//! **Tests**: Integration tests for this stage are located in `crates/buttre-engine/tests/stage3_validation_tests.rs`.
//!
//! This stage validates Vietnamese syllable structure.
//!
//! ## Algorithm
//!
//! 1. Check if the current buffer forms a valid Vietnamese syllable structure
//! 2. Parse syllable into Onset-Nucleus-Coda components
//! 3. Validate using Vietnamese phonology rules
//! 4. Support both permissive and strict modes
//!
//! ## Rationale
//!
//! Vietnamese syllables follow a C-V-C (Consonant-Vowel-Consonant) structure:
//! - Optional initial consonant (c, ch, tr, ng, ngh, gi, qu, etc.)
//! - Required vowel nucleus (a, e, i, o, u, y, and combinations)
//! - Optional final consonant (c, ch, m, n, ng, nh, p, t)
//!
//! buttre follows a permissive philosophy by default: we don't block user input
//! during typing. Strict validation can be enabled for spell-checking features.

use crate::pipeline::validation::SyllableStructure;
use crate::pipeline::{PipelineStage, StageResult, TypingContext};
use tracing::{debug, instrument, trace, warn};

/// Stage 3: Structure Validation
///
/// Validates Vietnamese syllable structure using phonology rules.
///
/// ## Algorithm
///
/// This stage implements syllable structure validation:
///
/// 1. **Check Buffer State**:
///    - If buffer is empty → Continue (start new syllable)
///    - If buffer has content → Validate structure
///
/// 2. **Parse Syllable** (New Implementation):
///    - Parse buffer into Onset-Nucleus-Coda components
///    - Use Vietnamese phonology rules
///    - Check valid consonant clusters
///    - Validate vowel sequences
///    - Check valid V-C combinations
///
/// 3. **Validation Mode**:
///    - **Permissive (default)**: Allow all alphabetic input
///    - **Strict**: Only allow valid Vietnamese syllables
///
/// ## Example Scenarios
///
/// ### Scenario 1: Valid Syllable (Both Modes)
/// ```text
/// Buffer: "thu"
/// Structure: C(th) + V(u)
/// Result: Continue (valid)
/// ```
///
/// ### Scenario 2: Complex Syllable (Both Modes)
/// ```text
/// Buffer: "thường"
/// Structure: C(th) + V(ươ) + C(ng)
/// Result: Continue (valid)
/// ```
///
/// ### Scenario 3: Invalid Syllable
/// ```text
/// Buffer: "xyz"
/// Permissive: Continue (don't block)
/// Strict: PassThrough (invalid structure)
/// ```
#[derive(Debug, Clone)]
pub struct ValidationStage {
    /// Whether to use strict validation
    /// - false (default): Permissive - allow all alphabetic input
    /// - true: Strict - only allow valid Vietnamese syllables
    pub strict_mode: bool,
}

impl ValidationStage {
    /// Create a new validation stage (permissive mode)
    pub fn new() -> Self {
        Self { strict_mode: false }
    }

    /// Create a validation stage from config
    pub fn from_config(config: &crate::pipeline::PipelineConfig) -> Self {
        if let Some(ref validation_settings) = config.validation {
            // If allow_invalid is true → use permissive mode (strict_mode = false)
            // If allow_invalid is false → use strict mode (strict_mode = true)
            Self {
                strict_mode: !validation_settings.allow_invalid,
            }
        } else {
            // No validation config → default to permissive mode
            Self::new()
        }
    }

    /// Create a validation stage with strict mode
    pub fn with_strict_mode(strict: bool) -> Self {
        Self {
            strict_mode: strict,
        }
    }

    /// Check if a character is alphabetic (basic validation)
    ///
    /// ## Algorithm
    ///
    /// Checks if character is alphabetic (Latin or Vietnamese).
    /// Non-alphabetic characters should be handled by Gatekeeper.
    pub fn is_valid_char(&self, ch: char) -> bool {
        ch.is_alphabetic()
    }

    /// Validate syllable structure
    ///
    /// ## Algorithm
    ///
    /// 1. Parse syllable into components
    /// 2. Check if structure is valid Vietnamese
    /// 3. Return validation result
    pub fn is_valid_syllable(&self, syllable: &str) -> bool {
        if syllable.is_empty() {
            return true; // Empty is valid (start of new syllable)
        }

        // Parse syllable structure
        let structure = SyllableStructure::parse(syllable);

        // Check if valid Vietnamese syllable
        structure.is_valid()
    }
}

impl Default for ValidationStage {
    fn default() -> Self {
        Self::new()
    }
}

impl PipelineStage for ValidationStage {
    #[instrument(skip(self, ctx), fields(stage = "validation", input, syllable = %ctx.syllable_buffer, strict = self.strict_mode))]
    fn process(&self, ctx: &mut TypingContext, input: char) -> StageResult {
        // Algorithm Step 1: Check if input is alphabetic OR numeric (for VNI support)
        // VNI uses numbers for transformations (a6 → â, a1 → á)
        // If Gatekeeper allowed a digit through, we should continue processing it
        if !self.is_valid_char(input) && !input.is_numeric() {
            // SPECIAL CASE: Space with candidates (multi-keyword Nôm search)
            // Gatekeeper already checked and allowed space through for multi-keyword search
            if input == ' ' && ctx.showing_candidates {
                return StageResult::Continue;
            }

            // Non-alphabetic, non-numeric should have been handled by Gatekeeper
            warn!(
                "Non-alphabetic, non-numeric character '{}' passed through gatekeeper",
                input
            );
            return StageResult::PassThrough;
        }

        // Algorithm Step 2: Validate based on mode
        if self.strict_mode {
            // Strict mode: Validate syllable structure
            // Build hypothetical syllable with new input
            let mut test_syllable = ctx.syllable_buffer.clone();
            test_syllable.push(input);

            if !self.is_valid_syllable(&test_syllable) {
                // Invalid syllable in strict mode
                debug!(
                    "Invalid syllable '{}' in strict mode, passing through",
                    test_syllable
                );
                return StageResult::PassThrough;
            }
            trace!("Valid syllable '{}' in strict mode", test_syllable);
        }

        // Algorithm Step 3: Continue (permissive or valid in strict)
        StageResult::Continue
    }

    fn name(&self) -> &'static str {
        "ValidationStage"
    }

    fn reset(&mut self) {
        // No internal state to reset
    }
}
