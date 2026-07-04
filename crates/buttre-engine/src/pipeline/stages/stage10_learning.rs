//! Stage 10: Learning (Future Enhancement)
//!
//! **Tests**: Integration tests for this stage are located in `crates/buttre-engine/tests/stage10_learning_tests.rs`.
//!
//! This stage tracks user typing patterns to improve future predictions.
//! Currently a stub/placeholder for future implementation.
//!
//! ## Planned Features
//!
//! 1. **Frequency Tracking**:
//!    - Record each completed syllable
//!    - Count occurrences per syllable
//!    - Persist to user profile
//!
//! 2. **Context Awareness**:
//!    - Track word pairs (bigrams)
//!    - Use for prediction and ranking
//!
//! 3. **Custom Dictionary**:
//!    - User-added words
//!    - Technical terms
//!    - Names and places
//!
//! ## Usage
//!
//! Currently disabled by default. Enable with:
//! ```rust,ignore
//! ctx.learning_enabled = true;
//! ```

use crate::pipeline::{PipelineStage, StageResult, TypingContext};
use tracing::trace;

/// Stage 10: Learning
///
/// Tracks user typing patterns for future optimization.
/// Currently a stub that passes through all input.
#[derive(Clone, Default)]
pub struct LearningStage {
    /// Whether learning is enabled
    pub enabled: bool,

    /// Maximum number of syllables to track per session
    pub max_history: usize,
}

impl LearningStage {
    /// Create a new learning stage (disabled by default)
    pub fn new() -> Self {
        Self {
            enabled: false,
            max_history: 1000,
        }
    }

    /// Create a learning stage with specific settings
    pub fn with_settings(enabled: bool, max_history: usize) -> Self {
        Self {
            enabled,
            max_history,
        }
    }

    /// Enable or disable learning
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Check if learning is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Record a completed syllable (future implementation)
    #[allow(dead_code)]
    pub fn record_syllable(&self, ctx: &mut TypingContext, syllable: &str) {
        if !self.enabled || !ctx.learning_enabled {
            return;
        }

        // Add to session history
        if ctx.completed_syllables.len() < self.max_history {
            ctx.completed_syllables.push(syllable.to_string());
        }

        // Future: Persist to disk
        // Future: Update frequency counters
        // Future: Track bigrams
    }

    /// Get frequency of a syllable (future implementation)
    #[allow(dead_code)]
    fn get_frequency(&self, _syllable: &str) -> Option<f32> {
        // Future: Look up in frequency database
        None
    }
}

impl PipelineStage for LearningStage {
    fn process(&self, ctx: &mut TypingContext, _input: char) -> StageResult {
        // Skip if learning is disabled
        if !self.enabled && !ctx.learning_enabled {
            trace!("Learning stage: disabled");
            return StageResult::Continue;
        }

        // Future: Track typing patterns here
        // For now, just pass through
        trace!("Learning stage: enabled but not yet implemented");

        StageResult::Continue
    }

    fn name(&self) -> &'static str {
        "LearningStage"
    }

    fn reset(&mut self) {
        // Don't reset enabled flag - it's a configuration setting
        // Session history is in TypingContext and managed there
    }
}
