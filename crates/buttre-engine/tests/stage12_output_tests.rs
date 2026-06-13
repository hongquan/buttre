use buttre_engine::pipeline::{PipelineStage, StageResult, TypingContext};
use buttre_engine::pipeline::stages::stage12_output::OutputStage;
use buttre_engine::types::Action;

#[test]
fn test_no_change() {
    let stage = OutputStage::new(false);
    let mut ctx = TypingContext::new();
    ctx.syllable_buffer = "test".to_string();
    ctx.last_output = "test".to_string();

    let result = stage.process(&mut ctx, 'a');

    match result {
        StageResult::Output(actions) => {
            assert_eq!(actions.len(), 1);
            assert_eq!(actions[0], Action::DoNothing);
        }
        _ => panic!("Expected Output result"),
    }
}

#[test]
fn test_only_additions() {
    let stage = OutputStage::new(false);
    let mut ctx = TypingContext::new();
    ctx.syllable_buffer = "test".to_string();
    ctx.last_output = "te".to_string();

    let result = stage.process(&mut ctx, 't');

    match result {
        StageResult::Output(actions) => {
            assert_eq!(actions.len(), 1);
            assert_eq!(actions[0], Action::Commit("st".to_string()));
        }
        _ => panic!("Expected Output result"),
    }
}

#[test]
fn test_replacement() {
    let stage = OutputStage::new(false);
    let mut ctx = TypingContext::new();
    ctx.syllable_buffer = "thú".to_string();
    ctx.last_output = "thu".to_string();

    let result = stage.process(&mut ctx, 's');

    match result {
        StageResult::Output(actions) => {
            assert_eq!(actions.len(), 1);
            match &actions[0] {
                Action::Replace { backspace_count, text } => {
                    assert_eq!(*backspace_count, 1);
                    assert_eq!(text, "ú");
                }
                _ => panic!("Expected Replace action"),
            }
        }
        _ => panic!("Expected Output result"),
    }
}

#[test]
fn test_find_diff_position() {
    assert_eq!(OutputStage::find_diff_position("abc", "abc"), 3);
    assert_eq!(OutputStage::find_diff_position("abc", "abd"), 2);
    assert_eq!(OutputStage::find_diff_position("abc", "xyz"), 0);
    assert_eq!(OutputStage::find_diff_position("abc", "ab"), 2);
    assert_eq!(OutputStage::find_diff_position("ab", "abc"), 2);
    assert_eq!(OutputStage::find_diff_position("", "abc"), 0);
    assert_eq!(OutputStage::find_diff_position("abc", ""), 0);
}

#[test]
fn test_calculate_backspace_count() {
    assert_eq!(OutputStage::calculate_backspace_count("abc", 0), 3);
    assert_eq!(OutputStage::calculate_backspace_count("abc", 1), 2);
    assert_eq!(OutputStage::calculate_backspace_count("abc", 2), 1);
    assert_eq!(OutputStage::calculate_backspace_count("abc", 3), 0);
    assert_eq!(OutputStage::calculate_backspace_count("abc", 4), 0);
}

#[test]
fn test_get_changed_text() {
    assert_eq!(OutputStage::get_changed_text("abc", 0), "abc");
    assert_eq!(OutputStage::get_changed_text("abc", 1), "bc");
    assert_eq!(OutputStage::get_changed_text("abc", 2), "c");
    assert_eq!(OutputStage::get_changed_text("abc", 3), "");
    assert_eq!(OutputStage::get_changed_text("abc", 4), "");
}

#[test]
fn test_generate_action_no_change() {
    let stage = OutputStage::new(false);
    let action = stage.generate_action("test", "test");
    assert_eq!(action, Action::DoNothing);
}

#[test]
fn test_generate_action_commit() {
    let stage = OutputStage::new(false);
    let action = stage.generate_action("te", "test");
    assert_eq!(action, Action::Commit("st".to_string()));
}

#[test]
fn test_generate_action_replace() {
    let stage = OutputStage::new(false);
    let action = stage.generate_action("thu", "thú");
    match action {
        Action::Replace { backspace_count, text } => {
            assert_eq!(backspace_count, 1);
            assert_eq!(text, "ú");
        }
        _ => panic!("Expected Replace action"),
    }
}

#[test]
fn test_stage_name() {
    let stage = OutputStage::new(false);
    assert_eq!(stage.name(), "OutputStage");
}

#[test]
fn test_commit_output() {
    let stage = OutputStage::new(false);
    let mut ctx = TypingContext::new();
    ctx.syllable_buffer = "test".to_string();
    ctx.last_output = "".to_string();

    stage.process(&mut ctx, 't');

    // After processing, last_output should be updated
    assert_eq!(ctx.last_output, "test");
}

#[test]
fn test_empty_buffers() {
    let stage = OutputStage::new(false);
    let mut ctx = TypingContext::new();

    let result = stage.process(&mut ctx, 'a');

    match result {
        StageResult::Output(actions) => {
            assert_eq!(actions.len(), 1);
            assert_eq!(actions[0], Action::DoNothing);
        }
        _ => panic!("Expected Output result"),
    }
}

// ==================== Final Tests to Reach 250 Goal ====================

#[test]
fn test_empty_syllable_buffer() {
    let stage = OutputStage::new(false);
    let mut ctx = TypingContext::new();
    // Empty syllable buffer

    let result = stage.process(&mut ctx, 'a');

    match result {
        StageResult::Output(actions) => {
            assert_eq!(actions.len(), 1);
            // Empty buffer returns DoNothing
            assert_eq!(actions[0], Action::DoNothing);
        }
        _ => panic!("Expected Output result"),
    }
}

#[test]
fn test_composition_mode_empty() {
    let stage = OutputStage::new(true);
    let mut ctx = TypingContext::new();

    let result = stage.process(&mut ctx, 'x');

    match result {
        StageResult::Output(actions) => {
            assert_eq!(actions.len(), 1);
            // Empty buffer in composition mode also returns DoNothing
            assert_eq!(actions[0], Action::DoNothing);
        }
        _ => panic!("Expected Output result"),
    }
}

#[test]
fn test_long_syllable_buffer() {
    let stage = OutputStage::new(false);
    let mut ctx = TypingContext::new();
    ctx.syllable_buffer = "abcdefghijklmnop".to_string();
    ctx.last_output = "".to_string();

    let result = stage.process(&mut ctx, 'q');

    match result {
        StageResult::Output(actions) => {
            assert_eq!(actions.len(), 1);
            assert_eq!(actions[0], Action::Commit("abcdefghijklmnop".to_string()));
        }
        _ => panic!("Expected Output result"),
    }
}

#[test]
fn test_vietnamese_complete_word() {
    let stage = OutputStage::new(false);
    let mut ctx = TypingContext::new();
    ctx.syllable_buffer = "việt".to_string();
    ctx.last_output = "".to_string();

    let result = stage.process(&mut ctx, 't');

    match result {
        StageResult::Output(actions) => {
            assert_eq!(actions.len(), 1);
            assert_eq!(actions[0], Action::Commit("việt".to_string()));
        }
        _ => panic!("Expected Output result"),
    }
}

#[test]
fn test_composition_vietnamese() {
    let stage = OutputStage::new(true);
    let mut ctx = TypingContext::new();
    ctx.syllable_buffer = "thương".to_string();
    ctx.last_output = "".to_string();

    let result = stage.process(&mut ctx, 'g');

    match result {
        StageResult::Output(actions) => {
            assert_eq!(actions.len(), 1);
            match &actions[0] {
                Action::UpdateComposition { text, .. } => {
                    assert_eq!(text, "thương");
                }
                _ => panic!("Expected UpdateComposition"),
            }
        }
        _ => panic!("Expected Output result"),
    }
}

#[test]
fn test_multiple_char_replacement() {
    let stage = OutputStage::new(false);
    let mut ctx = TypingContext::new();
    ctx.syllable_buffer = "â".to_string();
    ctx.last_output = "aa".to_string();

    let result = stage.process(&mut ctx, 'a');

    match result {
        StageResult::Output(actions) => {
            assert_eq!(actions.len(), 1);
            match &actions[0] {
                Action::Replace { backspace_count, text } => {
                    assert_eq!(*backspace_count, 2);
                    assert_eq!(text, "â");
                }
                _ => panic!("Expected Replace action"),
            }
        }
        _ => panic!("Expected Output result"),
    }
}
