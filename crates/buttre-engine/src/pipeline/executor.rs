//! Pipeline Executor — orchestrates the 7-stage processing pipeline (Phase 4+).
//!
//! ## 7-Stage Architecture
//!
//! | # | Stage | Responsibility |
//! |---|-------|----------------|
//! | 1 | Normalization | Normalize input, populate char_buffer |
//! | 2 | Gatekeeper | Route non-Vietnamese / temp-English passthrough |
//! | 3 | Compose | Recompute-from-raw: segment → transform → tone → fallback |
//! | 4 | Orthography | Normalize Unicode form |
//! | 5 | Learning | Track patterns (no-op until Phase 5) |
//! | 6 | Lookup | Dictionary lookup (Nôm candidates) |
//! | 7 | Output | Diff last_output → syllable_buffer → emit actions |
//!
//! The former dual-engine (Transform + Tone + Permutation + Reconciliation + Retrofix)
//! has been retired.  All composition logic lives in `crates/buttre-engine/src/compose/`
//! and is invoked by `ComposeStage` as a single recompute-from-raw step.

use crate::pipeline::{PipelineStage, StageResult, TypingContext, PipelineConfig};
use crate::pipeline::stages::*;
use crate::types::Action;
use tracing::{instrument, debug, trace, warn};

/// Pipeline Executor
///
/// Orchestrates the 7-stage pipeline from Phase 4 onward.
///
/// ## Flow control
///
/// Each stage returns a [`StageResult`]:
/// - `Continue` — proceed to the next stage.
/// - `PassThrough` — confirm any in-progress composition, commit the raw char,
///   reset context.
/// - `Output(actions)` — short-circuit; return these actions directly.
pub struct PipelineExecutor {
    /// All stages in processing order.
    stages: Vec<Box<dyn PipelineStage>>,

    /// Shared mutable state threaded through every stage.
    context: TypingContext,

    /// Whether to emit TSF composition actions (vs. simple commit/replace).
    use_composition: bool,
}

impl PipelineExecutor {
    /// Construct a new `PipelineExecutor` from a [`PipelineConfig`].
    ///
    /// Stages 1 (Normalization) and 2 (Gatekeeper) are always first.
    /// Stage 7 (Output) is always last.
    /// The middle stages are controlled by `config.pipeline.enabled`:
    /// - Empty → default order: `["compose", "orthography", "learning", "lookup"]`
    /// - Non-empty → honour that list (allows non-default configs to opt in/out of steps).
    ///
    /// # Arguments
    ///
    /// * `config` — Pipeline configuration (transform tables, tone map, etc.)
    pub fn new(config: PipelineConfig) -> Self {
        let mut stages: Vec<Box<dyn PipelineStage>> = Vec::new();
        let use_composition = config.pipeline.use_composition;

        // Stage 1: Normalization — ALWAYS FIRST
        stages.push(Box::new(NormalizationStage::new()));

        // Stage 2: Gatekeeper — ALWAYS SECOND
        stages.push(Box::new(GatekeeperStage::from_config(&config)));

        // Middle stages: driven by config.pipeline.enabled or default order.
        let stage_order: Vec<String> = if config.pipeline.enabled.is_empty() {
            // Default 5-stage middle pipeline (total = 7 with fixed 1 + 7).
            vec![
                "compose".to_string(),
                "orthography".to_string(),
                "learning".to_string(),
                "lookup".to_string(),
            ]
        } else {
            config.pipeline.enabled.clone()
        };

        for stage_name in &stage_order {
            match stage_name.as_str() {
                "compose" => {
                    stages.push(Box::new(ComposeStage::from_config(&config)));
                }
                "orthography" => {
                    stages.push(Box::new(OrthographyStage::from_config(&config)));
                }
                "learning" => {
                    stages.push(Box::new(LearningStage::new()));
                }
                "lookup" => {
                    stages.push(Box::new(LookupStage::from_config(&config)));
                }
                // Tolerate old stage names in non-default configs so that
                // integration tests that pass custom stage lists don't hard-break.
                "validation" => {
                    stages.push(Box::new(ValidationStage::from_config(&config)));
                }
                other => {
                    warn!("Unknown or retired stage name: '{}' — skipped", other);
                }
            }
        }

        // Stage 7 (last): Output — ALWAYS LAST
        stages.push(Box::new(OutputStage::new(use_composition)));

        Self {
            stages,
            context: TypingContext::new(),
            use_composition,
        }
    }

    /// Process one input character through the pipeline.
    ///
    /// ## Returns
    ///
    /// A `Vec<Action>` describing what the IME should do (backspace, send text, etc.).
    #[instrument(skip(self), fields(input, syllable = %self.context.syllable_buffer))]
    pub fn process(&mut self, input: char) -> Vec<Action> {
        trace!("Processing input character: '{}'", input);

        for stage in &self.stages {
            let result = stage.process(&mut self.context, input);

            match result {
                StageResult::Continue => {}
                StageResult::PassThrough => {
                    debug!("Stage '{}' returned PassThrough — confirming and resetting", stage.name());

                    let mut actions = Vec::new();

                    // If TSF composition is active, confirm any pending composition first.
                    if self.use_composition && !self.context.syllable_buffer.is_empty() {
                        debug!("Confirming composition: {}", self.context.syllable_buffer);
                        actions.push(Action::ConfirmComposition(
                            self.context.syllable_buffer.clone(),
                        ));
                    }

                    trace!("Committing pass-through character: '{}'", input);
                    actions.push(Action::Commit(input.to_string()));

                    self.reset();
                    return actions;
                }
                StageResult::Output(actions) => {
                    debug!(
                        "Stage '{}' returned Output with {} action(s)",
                        stage.name(),
                        actions.len()
                    );
                    return actions;
                }
            }
        }

        // All stages returned Continue — should not happen because OutputStage always emits.
        warn!("All stages returned Continue — OutputStage should have terminated processing");
        vec![Action::DoNothing]
    }

    /// Reset the pipeline to initial state (clears char_buffer, syllable_buffer, etc.).
    pub fn reset(&mut self) {
        self.context.clear();
        for stage in &mut self.stages {
            stage.reset();
        }
    }

    /// Current composed syllable (after the latest `process` call).
    pub fn syllable(&self) -> &str {
        &self.context.syllable_buffer
    }

    /// Current composed syllable (alias for backward compatibility).
    pub fn get_buffer(&self) -> &str {
        &self.context.syllable_buffer
    }

    /// Raw key buffer (lowercase normalized keystrokes).
    pub fn raw_buffer(&self) -> String {
        self.context.raw_buffer()
    }

    /// Whether the current buffer is in temporary-English passthrough mode.
    ///
    /// This flag is set by `ComposeStage` when the key sequence looks like an
    /// English word (validation-first fallback).  The Gatekeeper reads it on
    /// the *next* keystroke.
    pub fn is_temp_english_mode(&self) -> bool {
        self.context.temp_english_mode
    }

    /// Read-only access to the typing context (used in tests).
    pub fn context(&self) -> &TypingContext {
        &self.context
    }

    /// Total number of stages (used in tests to assert pipeline depth).
    pub fn stage_count(&self) -> usize {
        self.stages.len()
    }
}
