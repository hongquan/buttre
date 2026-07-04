//! Integration tests for Stages 1-3
//!
//! These tests verify that the first three stages work correctly together.
//! `#[cfg(test)]`-gated at the `mod integration_tests;` declaration site
//! (`pipeline::stages::mod`), not here.

use crate::pipeline::stages::{GatekeeperStage, NormalizationStage, ValidationStage};
use crate::pipeline::{PipelineStage, StageResult, TypingContext};

#[test]
fn test_stages_1_3_normal_flow() {
    // Create stages
    let stage1 = NormalizationStage::new();
    let stage2 = GatekeeperStage::new();
    let stage3 = ValidationStage::new();

    let mut ctx = TypingContext::new();

    // Process 'A' through all three stages
    let result1 = stage1.process(&mut ctx, 'A');
    assert_eq!(result1, StageResult::Continue);
    assert_eq!(ctx.raw_buffer(), "a"); // Normalized to lowercase

    let result2 = stage2.process(&mut ctx, 'a');
    assert_eq!(result2, StageResult::Continue);

    let result3 = stage3.process(&mut ctx, 'a');
    assert_eq!(result3, StageResult::Continue);
}

#[test]
fn test_stages_1_3_non_alphabetic() {
    let stage1 = NormalizationStage::new();
    let stage2 = GatekeeperStage::new();
    let _stage3 = ValidationStage::new();

    let mut ctx = TypingContext::new();

    // Process '1' through stages
    stage1.process(&mut ctx, '1');
    assert_eq!(ctx.raw_buffer(), "1");

    // Gatekeeper should Continue for digits (VNI support)
    let result2 = stage2.process(&mut ctx, '1');
    assert_eq!(result2, StageResult::Continue);

    // Process '!' - should PassThrough
    // Reset context or just process next char

    // Gatekeeper should pass through punctuation
    let result3 = stage2.process(&mut ctx, '!');
    assert_eq!(result3, StageResult::PassThrough);
}

#[test]
fn test_stages_1_3_temp_english_mode() {
    let stage1 = NormalizationStage::new();
    let stage2 = GatekeeperStage::new();
    let _stage3 = ValidationStage::new();

    let mut ctx = TypingContext::new();
    ctx.temp_english_mode = true;

    // Process 'f' through stages
    stage1.process(&mut ctx, 'f');
    assert_eq!(ctx.raw_buffer(), "f");

    // Gatekeeper should Continue in temp English mode (Stage 4 will append)
    // PassThrough would trigger executor.reset() which clears buffer
    let result2 = stage2.process(&mut ctx, 'f');
    assert_eq!(result2, StageResult::Continue);
    assert!(ctx.temp_english_mode); // Still active

    // Process space to reset mode
    stage1.process(&mut ctx, ' ');
    let result2 = stage2.process(&mut ctx, ' ');
    assert_eq!(result2, StageResult::PassThrough);
    assert!(!ctx.temp_english_mode); // Reset
}

#[test]
fn test_stages_1_3_vietnamese_word() {
    // Simulate typing "thu" through stages 1-3
    let stage1 = NormalizationStage::new();
    let stage2 = GatekeeperStage::new();
    let stage3 = ValidationStage::new();

    let mut ctx = TypingContext::new();

    // Process 't'
    assert_eq!(stage1.process(&mut ctx, 't'), StageResult::Continue);
    assert_eq!(stage2.process(&mut ctx, 't'), StageResult::Continue);
    assert_eq!(stage3.process(&mut ctx, 't'), StageResult::Continue);
    assert_eq!(ctx.raw_buffer(), "t");

    // Process 'h'
    assert_eq!(stage1.process(&mut ctx, 'h'), StageResult::Continue);
    assert_eq!(stage2.process(&mut ctx, 'h'), StageResult::Continue);
    assert_eq!(stage3.process(&mut ctx, 'h'), StageResult::Continue);
    assert_eq!(ctx.raw_buffer(), "th");

    // Process 'u'
    assert_eq!(stage1.process(&mut ctx, 'u'), StageResult::Continue);
    assert_eq!(stage2.process(&mut ctx, 'u'), StageResult::Continue);
    assert_eq!(stage3.process(&mut ctx, 'u'), StageResult::Continue);
    assert_eq!(ctx.raw_buffer(), "thu");
}

#[test]
fn test_stages_1_3_mixed_case() {
    // Test uppercase and lowercase mixing
    let stage1 = NormalizationStage::new();
    let stage2 = GatekeeperStage::new();
    let stage3 = ValidationStage::new();

    let mut ctx = TypingContext::new();

    // Process 'T', 'H', 'U' (all uppercase)
    stage1.process(&mut ctx, 'T');
    stage2.process(&mut ctx, 't');
    stage3.process(&mut ctx, 't');

    stage1.process(&mut ctx, 'H');
    stage2.process(&mut ctx, 'h');
    stage3.process(&mut ctx, 'h');

    stage1.process(&mut ctx, 'U');
    stage2.process(&mut ctx, 'u');
    stage3.process(&mut ctx, 'u');

    // All should be normalized to lowercase
    assert_eq!(ctx.raw_buffer(), "thu");
}
