use buttre_engine::pipeline::stages::stage2_gatekeeper::GatekeeperStage;
use buttre_engine::pipeline::{PipelineStage, StageResult, TypingContext};

#[test]
fn test_normal_alphabetic_input() {
    let stage = GatekeeperStage::new();
    let mut ctx = TypingContext::new();

    let result = stage.process(&mut ctx, 'a');

    assert_eq!(result, StageResult::Continue);
    assert!(!ctx.temp_english_mode);
}

#[test]
fn test_non_alphabetic_passthrough() {
    let stage = GatekeeperStage::new();
    let mut ctx = TypingContext::new();

    assert_eq!(stage.process(&mut ctx, '1'), StageResult::PassThrough);
    assert_eq!(stage.process(&mut ctx, ' '), StageResult::PassThrough);
    assert_eq!(stage.process(&mut ctx, '.'), StageResult::PassThrough);
    assert_eq!(stage.process(&mut ctx, '!'), StageResult::PassThrough);
}

#[test]
fn test_temp_english_mode_active() {
    let stage = GatekeeperStage::new();
    let mut ctx = TypingContext::new();
    ctx.temp_english_mode = true;

    // Alphabetic input should Continue in temp English mode (Stage 4 will append)
    // PassThrough would trigger executor.reset() which clears buffer
    let result = stage.process(&mut ctx, 'f');
    assert_eq!(result, StageResult::Continue);
    assert!(ctx.temp_english_mode); // Mode still active

    let result = stage.process(&mut ctx, 'i');
    assert_eq!(result, StageResult::Continue);
    assert!(ctx.temp_english_mode);
}

#[test]
fn test_temp_english_mode_reset_on_space() {
    let stage = GatekeeperStage::new();
    let mut ctx = TypingContext::new();
    ctx.temp_english_mode = true;

    // Space should reset temp English mode
    let result = stage.process(&mut ctx, ' ');
    assert_eq!(result, StageResult::PassThrough);
    assert!(!ctx.temp_english_mode); // Mode reset
}

#[test]
fn test_temp_english_mode_reset_on_punctuation() {
    let stage = GatekeeperStage::new();
    let mut ctx = TypingContext::new();
    ctx.temp_english_mode = true;

    // Punctuation should reset temp English mode
    let result = stage.process(&mut ctx, '.');
    assert_eq!(result, StageResult::PassThrough);
    assert!(!ctx.temp_english_mode);
}

#[test]
fn test_temp_english_mode_number_continues() {
    let stage = GatekeeperStage::new();
    let mut ctx = TypingContext::new();
    ctx.temp_english_mode = true;

    // Numbers do NOT reset temp English mode — after a VNI undo (e.g. a111 → á1),
    // the repeated tone-key digit must reach Stage 4 to be appended literally.
    // Only separators and non-alphanumeric symbols reset the mode.
    let result = stage.process(&mut ctx, '1');
    assert_eq!(result, StageResult::Continue);
    assert!(ctx.temp_english_mode);
}

#[test]
fn test_is_separator() {
    let stage = GatekeeperStage::new();

    assert!(stage.is_separator(' '));
    assert!(stage.is_separator('\n'));
    assert!(stage.is_separator('.'));
    assert!(stage.is_separator(','));
    assert!(stage.is_separator('!'));
    assert!(stage.is_separator('?'));
    assert!(stage.is_separator('-'));

    assert!(!stage.is_separator('a'));
    assert!(!stage.is_separator('1'));
}

#[test]
fn test_stage_name() {
    let stage = GatekeeperStage::new();
    assert_eq!(stage.name(), "GatekeeperStage");
}

#[test]
fn test_default() {
    let _stage = GatekeeperStage::default();
    // Just verify it compiles and constructs
}

#[test]
fn test_english_word_scenario() {
    // Simulate typing "file" after an undo
    let stage = GatekeeperStage::new();
    let mut ctx = TypingContext::new();
    ctx.temp_english_mode = true;

    // Type "file" - now returns Continue (not PassThrough) to allow buffer accumulation
    // Stage 4 will append these chars since no transformation matches
    assert_eq!(stage.process(&mut ctx, 'f'), StageResult::Continue);
    assert_eq!(stage.process(&mut ctx, 'i'), StageResult::Continue);
    assert_eq!(stage.process(&mut ctx, 'l'), StageResult::Continue);
    assert_eq!(stage.process(&mut ctx, 'e'), StageResult::Continue);

    // Space ends the word - returns PassThrough which resets context in executor
    assert_eq!(stage.process(&mut ctx, ' '), StageResult::PassThrough);
    assert!(!ctx.temp_english_mode);

    // Next word should be processed as Vietnamese
    assert_eq!(stage.process(&mut ctx, 'v'), StageResult::Continue);
}

// ==================== Additional Tests to Reach Goal ====================

#[test]
fn test_vietnamese_chars_continue() {
    let stage = GatekeeperStage::new();
    let mut ctx = TypingContext::new();

    // Vietnamese characters should continue
    assert_eq!(stage.process(&mut ctx, 'â'), StageResult::Continue);
    assert_eq!(stage.process(&mut ctx, 'đ'), StageResult::Continue);
    assert_eq!(stage.process(&mut ctx, 'ư'), StageResult::Continue);
}

#[test]
fn test_uppercase_alphabetic() {
    let stage = GatekeeperStage::new();
    let mut ctx = TypingContext::new();

    // Uppercase should continue
    assert_eq!(stage.process(&mut ctx, 'A'), StageResult::Continue);
    assert_eq!(stage.process(&mut ctx, 'Z'), StageResult::Continue);
}

#[test]
fn test_multiple_separators() {
    let stage = GatekeeperStage::new();
    let mut ctx = TypingContext::new();
    ctx.temp_english_mode = true;

    // All separators should reset mode
    for sep in [' ', '.', ',', ';', ':', '!', '?', '-', '_'].iter() {
        ctx.temp_english_mode = true;
        stage.process(&mut ctx, *sep);
        assert!(!ctx.temp_english_mode, "Failed for '{}'", sep);
    }
}

#[test]
fn test_reset_no_effect() {
    let mut stage = GatekeeperStage::new();

    stage.reset();

    // Should still work
    let mut ctx = TypingContext::new();
    assert_eq!(stage.process(&mut ctx, 'a'), StageResult::Continue);
}

#[test]
fn test_passthrough_preserves_char() {
    let stage = GatekeeperStage::new();
    let mut ctx = TypingContext::new();

    // PassThrough should not modify the character
    let result = stage.process(&mut ctx, '5');
    assert_eq!(result, StageResult::PassThrough);
}

#[test]
fn test_temp_english_sequence() {
    let stage = GatekeeperStage::new();
    let mut ctx = TypingContext::new();

    // Enable temp English mode
    ctx.temp_english_mode = true;

    // Type "file" - now returns Continue (not PassThrough) to allow buffer accumulation
    // PassThrough would trigger executor.reset() which clears buffer
    assert_eq!(stage.process(&mut ctx, 'f'), StageResult::Continue);
    assert_eq!(stage.process(&mut ctx, 'i'), StageResult::Continue);
    assert_eq!(stage.process(&mut ctx, 'l'), StageResult::Continue);
    assert_eq!(stage.process(&mut ctx, 'e'), StageResult::Continue);

    // All should continue in temp English mode (Stage 4 will append)
}

#[test]
fn test_normal_mode_sequence() {
    let stage = GatekeeperStage::new();
    let mut ctx = TypingContext::new();

    // Type Vietnamese
    assert_eq!(stage.process(&mut ctx, 'v'), StageResult::Continue);
    assert_eq!(stage.process(&mut ctx, 'i'), StageResult::Continue);
    assert_eq!(stage.process(&mut ctx, 'e'), StageResult::Continue);
    assert_eq!(stage.process(&mut ctx, 't'), StageResult::Continue);
}

#[test]
fn test_is_separator_newline() {
    let stage = GatekeeperStage::new();

    assert!(stage.is_separator('\n'));
    assert!(stage.is_separator('\r'));
    assert!(stage.is_separator('\t'));
}

#[test]
fn test_mixed_input_sequence() {
    let stage = GatekeeperStage::new();
    let mut ctx = TypingContext::new();

    // Vietnamese text
    assert_eq!(stage.process(&mut ctx, 'a'), StageResult::Continue);

    // Number (passthrough, resets if temp mode was on)
    assert_eq!(stage.process(&mut ctx, '1'), StageResult::PassThrough);

    // Space (passthrough)
    assert_eq!(stage.process(&mut ctx, ' '), StageResult::PassThrough);

    // Back to Vietnamese
    assert_eq!(stage.process(&mut ctx, 'b'), StageResult::Continue);
}

#[test]
fn test_special_chars_passthrough() {
    let stage = GatekeeperStage::new();
    let mut ctx = TypingContext::new();

    // Special characters should pass through
    assert_eq!(stage.process(&mut ctx, '@'), StageResult::PassThrough);
    assert_eq!(stage.process(&mut ctx, '#'), StageResult::PassThrough);
    assert_eq!(stage.process(&mut ctx, '$'), StageResult::PassThrough);
}
