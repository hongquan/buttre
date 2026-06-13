//! Pipeline Stage - Trait and result types for pipeline stages
//!
//! Each stage in the 7-stage pipeline implements the PipelineStage trait,
//! which defines a single `process` method that takes a context and input character.

use crate::types::Action;
use super::context::TypingContext;

/// Result of processing through a pipeline stage
///
/// ## Algorithm
///
/// Each stage returns one of these results to control pipeline flow:
/// - **Continue**: Processing should continue to the next stage
/// - **PassThrough**: Stop processing and pass input through as-is (English mode)
/// - **Output**: Stop processing and return these actions to the application
#[derive(Debug, Clone, PartialEq)]
pub enum StageResult {
    /// Continue to the next stage
    Continue,

    /// Pass through the input without transformation (English mode)
    /// The input character should be sent as-is
    PassThrough,

    /// Stop processing and return these actions
    /// This is typically returned by the final Output stage
    Output(Vec<Action>),
}

/// Pipeline Stage trait
///
/// Each of the 7 stages implements this trait to process input.
///
/// ## Algorithm
///
/// The `process` method:
/// 1. Reads the current context state
/// 2. Processes the input character
/// 3. Modifies the context as needed
/// 4. Returns a StageResult to control pipeline flow
///
/// ## Example
///
/// ```rust,ignore
/// impl PipelineStage for NormalizationStage {
///     fn process(&self, ctx: &mut TypingContext, input: char) -> StageResult {
///         // Normalize case
///         let normalized = input.to_lowercase().next().unwrap_or(input);
///         
///         // Update raw buffer
///         ctx.push_raw(normalized);
///         
///         // Continue to next stage
///         StageResult::Continue
///     }
/// }
/// ```
pub trait PipelineStage: Send + Sync {
    /// Process an input character through this stage
    ///
    /// ## Parameters
    ///
    /// - `ctx`: Mutable reference to the typing context
    /// - `input`: The input character to process
    ///
    /// ## Returns
    ///
    /// A StageResult indicating how the pipeline should proceed
    fn process(&self, ctx: &mut TypingContext, input: char) -> StageResult;

    /// Get the name of this stage (for debugging)
    fn name(&self) -> &'static str {
        "UnnamedStage"
    }

    /// Reset any internal state (called when context is cleared)
    fn reset(&mut self) {
        // Default: no-op
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mock stage for testing
    struct MockStage {
        should_continue: bool,
    }

    impl PipelineStage for MockStage {
        fn process(&self, ctx: &mut TypingContext, input: char) -> StageResult {
            ctx.push_raw(input);
            if self.should_continue {
                StageResult::Continue
            } else {
                StageResult::PassThrough
            }
        }

        fn name(&self) -> &'static str {
            "MockStage"
        }
    }

    #[test]
    fn test_stage_continue() {
        let stage = MockStage { should_continue: true };
        let mut ctx = TypingContext::new();
        
        let result = stage.process(&mut ctx, 'a');
        
        assert_eq!(result, StageResult::Continue);
        assert_eq!(ctx.raw_buffer(), "a");
    }

    #[test]
    fn test_stage_passthrough() {
        let stage = MockStage { should_continue: false };
        let mut ctx = TypingContext::new();
        
        let result = stage.process(&mut ctx, 'a');
        
        assert_eq!(result, StageResult::PassThrough);
    }

    #[test]
    fn test_stage_name() {
        let stage = MockStage { should_continue: true };
        assert_eq!(stage.name(), "MockStage");
    }
}
