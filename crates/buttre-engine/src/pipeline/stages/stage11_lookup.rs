//! Stage 11: Dictionary Lookup
//!
//! **Tests**: Integration tests for this stage are located in `crates/buttre-engine/tests/stage11_lookup_tests.rs`.
//!
//! This stage performs dictionary lookup and generates candidates.
//!
//! ## Algorithm
//!
//! 1. Check if dictionary lookup is enabled
//! 2. Query the dictionary for matching entries
//! 3. Store candidates in TypingContext
//! 4. Continue to next stage
//!
//! ## Rationale
//!
//! Dictionary lookup provides:
//! - Word suggestions
//! - Nôm character candidates
//! - Auto-completion
//! - Spelling correction

use crate::pipeline::dictionary::DictionaryProvider;
use crate::pipeline::{Candidate, PipelineConfig, PipelineStage, StageResult, TypingContext};
use crate::types::Action;
use std::sync::Arc;

/// Stage 11: Dictionary Lookup
///
/// Performs dictionary lookup and generates candidates.
///
/// ## Algorithm
///
/// This stage implements dictionary lookup:
///
/// 1. **Check if Enabled**:
///    - If `enable_lookup` is false, skip lookup
///    - Continue to next stage immediately
///
/// 2. **Query Dictionary**:
///    - Look up current syllable in dictionary
///    - Find matching words/characters
///    - Get ranked candidate list
///
/// 3. **Store Candidates**:
///    - Store candidates in TypingContext
///    - Set showing_candidates flag if candidates found
///    - UI can display candidate window
///
/// 4. **Return Result**:
///    - Always continue to next stage
///    - Candidate display is handled by UI layer
///
/// ## Example
///
/// ```text
/// Syllable: "nguoi"
/// Lookup: Find "người", "𠊛" (Nôm)
/// Candidates: [Candidate { text: "người", score: 1.0 }, ...]
/// ```
#[derive(Clone)]
pub struct LookupStage {
    /// Whether dictionary lookup is enabled
    pub enabled: bool,

    /// Dictionary provider (optional)
    /// If None, lookup is disabled
    pub dictionary: Option<Arc<dyn DictionaryProvider>>,

    /// Auto replace buffer with top candidate
    auto_replace: bool,

    /// Space key behavior
    space_behavior: SpaceBehavior,

    /// Enter key behavior
    enter_behavior: EnterBehavior,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SpaceBehavior {
    /// Auto-select if exactly 1 candidate, otherwise add space to search
    AutoSelectSingle,
    /// Always select first candidate (if available)
    AlwaysSelect,
    /// Always add space to search keywords
    AlwaysSearch,
    /// Let space pass through normally
    PassThrough,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EnterBehavior {
    /// Select first candidate
    SelectFirst,
    /// Select currently highlighted candidate
    SelectCurrent,
    /// Let enter pass through normally
    PassThrough,
}

impl LookupStage {
    /// Create a new lookup stage from config
    pub fn from_config(config: &PipelineConfig) -> Self {
        let (auto_replace, space_behavior, enter_behavior) = if let Some(ref lookup) = config.lookup
        {
            let space = match lookup.space_behavior.as_str() {
                "auto_select_single" => SpaceBehavior::AutoSelectSingle,
                "always_select" => SpaceBehavior::AlwaysSelect,
                "always_search" => SpaceBehavior::AlwaysSearch,
                _ => SpaceBehavior::PassThrough,
            };
            let enter = match lookup.enter_behavior.as_str() {
                "select_first" => EnterBehavior::SelectFirst,
                "select_current" => EnterBehavior::SelectCurrent,
                _ => EnterBehavior::PassThrough,
            };
            (lookup.auto_replace, space, enter)
        } else {
            (
                false,
                SpaceBehavior::PassThrough,
                EnterBehavior::PassThrough,
            )
        };

        Self {
            enabled: config.enable_lookup || config.lookup.is_some(),
            dictionary: config.dictionary.clone(),
            auto_replace,
            space_behavior,
            enter_behavior,
        }
    }

    /// Create a new lookup stage with custom settings
    pub fn new(enabled: bool) -> Self {
        Self {
            enabled,
            dictionary: None,
            auto_replace: false,
            space_behavior: SpaceBehavior::PassThrough,
            enter_behavior: EnterBehavior::PassThrough,
        }
    }

    /// Create a lookup stage with dictionary provider
    pub fn with_dictionary(dictionary: Arc<dyn DictionaryProvider>) -> Self {
        Self {
            enabled: true,
            dictionary: Some(dictionary),
            auto_replace: false,
            space_behavior: SpaceBehavior::PassThrough,
            enter_behavior: EnterBehavior::PassThrough,
        }
    }

    /// Set auto-replace mode
    pub fn with_auto_replace(mut self, auto_replace: bool) -> Self {
        self.auto_replace = auto_replace;
        self
    }

    /// Perform dictionary lookup
    ///
    /// ## Algorithm
    ///
    /// Queries the dictionary provider for matching candidates.
    /// Returns empty vector if dictionary is not available.
    pub fn lookup(&self, syllable: &str) -> Vec<Candidate> {
        if let Some(ref dict) = self.dictionary {
            dict.lookup(syllable)
        } else {
            Vec::new()
        }
    }

    /// Public method for direct dictionary lookup
    /// Used by PipelineExecutor for multi-keyword search
    pub fn lookup_query(&self, query: &str) -> Vec<Candidate> {
        self.lookup(query)
    }
}

// Manual Debug implementation since Arc<dyn DictionaryProvider> doesn't implement Debug
impl std::fmt::Debug for LookupStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LookupStage")
            .field("enabled", &self.enabled)
            .field("has_dictionary", &self.dictionary.is_some())
            .finish()
    }
}

impl PipelineStage for LookupStage {
    fn process(&self, ctx: &mut TypingContext, input: char) -> StageResult {
        // Algorithm Step 1: Check if lookup is enabled
        if !self.enabled || self.dictionary.is_none() {
            return StageResult::Continue;
        }

        // Algorithm Step 2: Handle special keys when candidates are showing
        if ctx.showing_candidates && !ctx.candidates.is_empty() {
            // Handle Space key
            if input == ' ' {
                match self.space_behavior {
                    SpaceBehavior::AutoSelectSingle => {
                        if ctx.candidates.len() == 1 {
                            // Exactly 1 candidate - auto-select it
                            let selected = ctx.candidates[0].get_value().to_string();
                            let backspace_count = ctx.syllable_buffer.chars().count();

                            // Hide candidates and reset
                            ctx.candidates.clear();
                            ctx.showing_candidates = false;
                            ctx.clear();

                            // Return Replace action
                            return StageResult::Output(vec![
                                Action::HideCandidates,
                                Action::Replace {
                                    backspace_count,
                                    text: selected,
                                },
                            ]);
                        } else {
                            // Multiple candidates - add space to search keyword
                            // Continue processing to update candidates
                        }
                    }
                    SpaceBehavior::AlwaysSelect => {
                        // Always select first candidate
                        let selected = ctx.candidates[0].get_value().to_string();
                        let backspace_count = ctx.syllable_buffer.chars().count();

                        ctx.candidates.clear();
                        ctx.showing_candidates = false;
                        ctx.clear();

                        return StageResult::Output(vec![
                            Action::HideCandidates,
                            Action::Replace {
                                backspace_count,
                                text: selected,
                            },
                        ]);
                    }
                    SpaceBehavior::AlwaysSearch => {
                        // Continue to add space to search
                    }
                    SpaceBehavior::PassThrough => {
                        // Let space pass through normally
                        return StageResult::Continue;
                    }
                }
            }

            // Handle Enter key
            if input == '\n' || input == '\r' {
                match self.enter_behavior {
                    EnterBehavior::SelectFirst => {
                        // Select first candidate
                        let selected = ctx.candidates[0].get_value().to_string();
                        let backspace_count = ctx.syllable_buffer.chars().count();

                        ctx.candidates.clear();
                        ctx.showing_candidates = false;
                        ctx.clear();

                        return StageResult::Output(vec![
                            Action::HideCandidates,
                            Action::Replace {
                                backspace_count,
                                text: selected,
                            },
                        ]);
                    }
                    EnterBehavior::SelectCurrent => {
                        // Select currently highlighted candidate
                        let index = ctx.selected_candidate.unwrap_or(0);
                        if let Some(candidate) = ctx.candidates.get(index) {
                            let selected = candidate.get_value().to_string();
                            let backspace_count = ctx.syllable_buffer.chars().count();

                            ctx.candidates.clear();
                            ctx.showing_candidates = false;
                            ctx.clear();

                            return StageResult::Output(vec![
                                Action::HideCandidates,
                                Action::Replace {
                                    backspace_count,
                                    text: selected,
                                },
                            ]);
                        }
                    }
                    EnterBehavior::PassThrough => {
                        // Let enter pass through normally
                        return StageResult::Continue;
                    }
                }
            }
        }

        // Algorithm Step 3: Perform lookup
        let candidates = self.lookup(&ctx.syllable_buffer);

        // Algorithm Step 4: Store candidates in context
        if !candidates.is_empty() {
            if self.auto_replace {
                // If auto-replace enabled, replace buffer with top candidate
                // This is useful for Nôm input without UI
                if let Some(top) = candidates.first() {
                    ctx.syllable_buffer = top.get_value().to_string();
                    // We don't show candidates if auto-replaced
                    ctx.showing_candidates = false;
                }
            } else {
                ctx.candidates = candidates;
                ctx.showing_candidates = true;
            }
        } else {
            // No candidates found for current query
            ctx.candidates.clear();
            ctx.showing_candidates = false;
        }

        // Algorithm Step 5: Return result
        StageResult::Continue
    }

    fn name(&self) -> &'static str {
        "LookupStage"
    }

    fn reset(&mut self) {
        // No internal state to reset
    }
}
