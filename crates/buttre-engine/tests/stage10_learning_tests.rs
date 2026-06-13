use buttre_engine::pipeline::{PipelineStage, StageResult, TypingContext};
use buttre_engine::pipeline::stages::stage10_learning::LearningStage;

#[test]
fn test_new() {
    let stage = LearningStage::new();
    assert!(!stage.is_enabled());
}

#[test]
fn test_with_settings() {
    let stage = LearningStage::with_settings(true, 500);
    assert!(stage.is_enabled());
    assert_eq!(stage.max_history, 500);
}

#[test]
fn test_set_enabled() {
    let mut stage = LearningStage::new();
    assert!(!stage.is_enabled());
    
    stage.set_enabled(true);
    assert!(stage.is_enabled());
    
    stage.set_enabled(false);
    assert!(!stage.is_enabled());
}

#[test]
fn test_process_disabled() {
    let stage = LearningStage::new();
    let mut ctx = TypingContext::new();
    ctx.learning_enabled = false;
    
    let result = stage.process(&mut ctx, 'a');
    
    assert_eq!(result, StageResult::Continue);
}

#[test]
fn test_process_enabled() {
    let stage = LearningStage::with_settings(true, 100);
    let mut ctx = TypingContext::new();
    ctx.learning_enabled = true;
    
    let result = stage.process(&mut ctx, 'a');
    
    assert_eq!(result, StageResult::Continue);
}

#[test]
fn test_stage_name() {
    let stage = LearningStage::new();
    assert_eq!(stage.name(), "LearningStage");
}

#[test]
fn test_reset_preserves_enabled() {
    let mut stage = LearningStage::with_settings(true, 100);
    assert!(stage.is_enabled());
    
    stage.reset();
    
    // enabled should be preserved after reset
    assert!(stage.is_enabled());
}

#[test]
fn test_record_syllable_disabled() {
    let stage = LearningStage::new();
    let mut ctx = TypingContext::new();
    ctx.learning_enabled = false;
    
    stage.record_syllable(&mut ctx, "test");
    
    assert!(ctx.completed_syllables.is_empty());
}

#[test]
fn test_record_syllable_enabled() {
    let stage = LearningStage::with_settings(true, 100);
    let mut ctx = TypingContext::new();
    ctx.learning_enabled = true;
    
    stage.record_syllable(&mut ctx, "người");
    stage.record_syllable(&mut ctx, "việt");
    
    assert_eq!(ctx.completed_syllables.len(), 2);
    assert_eq!(ctx.completed_syllables[0], "người");
    assert_eq!(ctx.completed_syllables[1], "việt");
}

#[test]
fn test_record_syllable_max_history() {
    let stage = LearningStage::with_settings(true, 2);
    let mut ctx = TypingContext::new();
    ctx.learning_enabled = true;
    
    stage.record_syllable(&mut ctx, "một");
    stage.record_syllable(&mut ctx, "hai");
    stage.record_syllable(&mut ctx, "ba"); // Should not be added
    
    assert_eq!(ctx.completed_syllables.len(), 2);
    assert_eq!(ctx.completed_syllables[1], "hai");
}
