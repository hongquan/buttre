//! Stage 12: Output Generation
//!
//! **Tests**: Integration tests for this stage are located in `crates/buttre-engine/tests/stage12_output_tests.rs`.
//!
//! This stage generates the final output actions for the application.
//!
//! ## Algorithm
//!
//! 1. Compare last_output with current syllable_buffer
//! 2. Calculate how many characters to backspace
//! 3. Generate the text to send
//! 4. Return Output action with backspace count and new text
//!
//! ## Rationale
//!
//! The output stage is responsible for:
//! - Calculating the diff between old and new output
//! - Generating efficient backspace + send sequences
//! - Updating last_output for next iteration

use crate::pipeline::{PipelineStage, StageResult, TypingContext};
use crate::types::Action;

/// Stage 12: Output Generation
///
/// Generates output actions by comparing old and new syllable.
///
/// ## Algorithm
///
/// This stage implements the output generation logic:
///
/// 1. **Compare Outputs**:
///    - Get last_output (what's currently on screen)
///    - Get syllable_buffer (what should be on screen)
///    - Find the first position where they differ
///
/// 2. **Calculate Backspace Count**:
///    - Count characters from diff position to end of last_output
///    - This is how many characters to delete
///
/// 3. **Generate New Text**:
///    - Extract characters from diff position to end of syllable_buffer
///    - This is the new text to send
///
/// 4. **Create Action**:
///    - If no change: Return DoNothing
///    - If only additions: Return Commit(new_text)
///    - If replacements: Return Replace { backspace_count, text }
///
/// 5. **Update Context**:
///    - Set last_output = syllable_buffer
///    - This prepares for the next iteration
///
/// ## Example
///
/// ```text
/// last_output: "thu"
/// syllable_buffer: "thú"
/// Diff at position: 2
/// Backspace: 1 (delete "u")
/// New text: "ú"
/// Action: Replace { backspace_count: 1, text: "ú" }
/// ```
#[derive(Debug, Clone)]
pub struct OutputStage {
    pub use_composition: bool,
}

impl OutputStage {
    /// Create a new output stage
    pub fn new(use_composition: bool) -> Self {
        Self { use_composition }
    }

    /// Find the first position where two strings differ
    ///
    /// ## Algorithm
    ///
    /// Compare character by character until we find a difference.
    /// Returns the position of the first difference, or the length of the shorter string.
    pub fn find_diff_position(old: &str, new: &str) -> usize {
        // Zip the two char streams directly (perf: this ran on every
        // keystroke and used to collect BOTH strings into fresh Vec<char>s).
        // Returns the index of the first mismatch, or the length of the
        // shorter string when one is a prefix of the other.
        old.chars()
            .zip(new.chars())
            .take_while(|(o, n)| o == n)
            .count()
    }

    /// Calculate backspace count
    ///
    /// ## Algorithm
    ///
    /// Count characters from diff position to end of old string.
    pub fn calculate_backspace_count(old: &str, diff_pos: usize) -> usize {
        old.chars().count().saturating_sub(diff_pos)
    }

    /// Extract changed portion
    ///
    /// ## Algorithm
    ///
    /// Get substring from diff position to end of new string.
    pub fn get_changed_text(new: &str, diff_pos: usize) -> String {
        new.chars().skip(diff_pos).collect()
    }

    /// Generate output action
    ///
    /// ## Algorithm
    ///
    /// Based on the diff, create the appropriate Action.
    pub fn generate_action(&self, old: &str, new: &str) -> Action {
        // TSF Mode: Return full composition state
        if self.use_composition {
            // Optimization: If no change, do nothing
            // Careful: TSF might need refresh? Assume DoNothing is fine if identical.
            if old == new {
                return Action::DoNothing;
            }
            
            return Action::UpdateComposition {
                text: new.to_string(),
                cursor: new.chars().count(), // Cursor at end for now
            };
        }

        // Standard Mode: Generate Diff (Replace/Commit)
        
        // Find where they differ
        let diff_pos = Self::find_diff_position(old, new);
        
        // Calculate what needs to change
        let backspace_count = Self::calculate_backspace_count(old, diff_pos);
        let changed_text = Self::get_changed_text(new, diff_pos);
        
        // Generate appropriate action
        if backspace_count == 0 && changed_text.is_empty() {
            // No change
            Action::DoNothing
        } else if backspace_count == 0 {
            // Only additions
            Action::Commit(changed_text)
        } else {
            // Replacements needed
            Action::Replace {
                backspace_count,
                text: changed_text,
            }
        }
    }
}

impl Default for OutputStage {
    fn default() -> Self {
        Self::new(false)
    }
}

impl PipelineStage for OutputStage {
    fn process(&self, ctx: &mut TypingContext, input: char) -> StageResult {
        let mut actions = Vec::new();
        
        // SPECIAL CASE: Space with candidates (multi-keyword Nôm search)
        // Don't output space to screen, only update candidates
        if input == ' ' && ctx.showing_candidates {
            // Skip main action - don't output space
            // Only show updated candidates
            if !ctx.candidates.is_empty() {
                actions.push(Action::ShowCandidates {
                    candidates: ctx.candidates.clone(),
                    input: ctx.syllable_buffer.clone(),
                });
            }
            return StageResult::Output(actions);
        }
        
        // Algorithm Step 1: Generate main action (composition/replace/commit)
        let main_action = self.generate_action(&ctx.last_output, &ctx.syllable_buffer);
        actions.push(main_action);
        
        // Algorithm Step 2: Generate candidate UI action if candidates are available
        if ctx.showing_candidates && !ctx.candidates.is_empty() {
            // Pass full Candidate objects (includes display text and actual value)
            actions.push(Action::ShowCandidates {
                candidates: ctx.candidates.clone(),
                input: ctx.syllable_buffer.clone(),
            });
        } else {
            // Hide candidates if not showing OR if candidates list is empty
            if !ctx.candidates.is_empty() || ctx.showing_candidates {
                actions.push(Action::HideCandidates);
            }
        }
        
        // Algorithm Step 3: Update last_output
        ctx.commit_output();
        
        // Algorithm Step 4: Return output actions
        StageResult::Output(actions)
    }

    fn name(&self) -> &'static str {
        "OutputStage"
    }

    fn reset(&mut self) {
        // No internal state to reset
    }
}

