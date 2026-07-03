use buttre_engine::pipeline::{PipelineStage, StageResult, TypingContext};
use buttre_engine::pipeline::stages::stage1_normalization::NormalizationStage;

#[test]
fn test_normalize_lowercase() {
    let stage = NormalizationStage::new();
    let mut ctx = TypingContext::new();

    let result = stage.process(&mut ctx, 'a');

    assert_eq!(result, StageResult::Continue);
    assert_eq!(ctx.raw_buffer(), "a");
}

#[test]
fn test_normalize_uppercase() {
    let stage = NormalizationStage::new();
    let mut ctx = TypingContext::new();

    let result = stage.process(&mut ctx, 'A');

    assert_eq!(result, StageResult::Continue);
    assert_eq!(ctx.raw_buffer(), "a");
}

#[test]
fn test_normalize_multiple_chars() {
    let stage = NormalizationStage::new();
    let mut ctx = TypingContext::new();

    stage.process(&mut ctx, 'T');
    stage.process(&mut ctx, 'h');
    stage.process(&mut ctx, 'U');

    assert_eq!(ctx.raw_buffer(), "thu");
}

#[test]
fn test_normalize_non_alphabetic() {
    let stage = NormalizationStage::new();
    let mut ctx = TypingContext::new();

    stage.process(&mut ctx, '1');
    stage.process(&mut ctx, ' ');
    stage.process(&mut ctx, '!');

    assert_eq!(ctx.raw_buffer(), "1 !");
}

#[test]
fn test_normalize_vietnamese_chars() {
    let stage = NormalizationStage::new();
    let mut ctx = TypingContext::new();

    stage.process(&mut ctx, 'â');
    stage.process(&mut ctx, 'Ă');
    stage.process(&mut ctx, 'Đ');

    // Vietnamese characters should be normalized to lowercase
    assert_eq!(ctx.raw_buffer(), "âăđ");
}

#[test]
fn test_stage_name() {
    let stage = NormalizationStage::new();
    assert_eq!(stage.name(), "NormalizationStage");
}

#[test]
fn test_default() {
    let stage = NormalizationStage::default();
    assert!(!stage.preserve_case);
}

#[test]
fn test_normalize_char_method() {
    let stage = NormalizationStage::new();

    assert_eq!(stage.normalize_char('A'), 'a');
    assert_eq!(stage.normalize_char('z'), 'z');
    assert_eq!(stage.normalize_char('1'), '1');
    assert_eq!(stage.normalize_char(' '), ' ');
}

// ==================== Additional Comprehensive Tests ====================

// === Character Normalization Tests ===

#[test]
fn test_normalize_all_uppercase_letters() {
    let stage = NormalizationStage::new();
    
    for ch in 'A'..='Z' {
        let normalized = stage.normalize_char(ch);
        assert!(normalized.is_lowercase(), "Failed for '{}'", ch);
    }
}

#[test]
fn test_normalize_all_lowercase_unchanged() {
    let stage = NormalizationStage::new();
    
    for ch in 'a'..='z' {
        let normalized = stage.normalize_char(ch);
        assert_eq!(normalized, ch);
    }
}

#[test]
fn test_normalize_digits_unchanged() {
    let stage = NormalizationStage::new();
    
    for ch in '0'..='9' {
        let normalized = stage.normalize_char(ch);
        assert_eq!(normalized, ch);
    }
}

#[test]
fn test_normalize_punctuation_unchanged() {
    let stage = NormalizationStage::new();
    
    let punctuation = vec!['.', ',', '!', '?', ';', ':', '-', '_', '(', ')'];
    for ch in punctuation {
        let normalized = stage.normalize_char(ch);
        assert_eq!(normalized, ch);
    }
}

#[test]
fn test_normalize_vietnamese_uppercase() {
    let stage = NormalizationStage::new();
    
    let test_cases = vec![
        ('Ă', 'ă'),
        ('Â', 'â'),
        ('Đ', 'đ'),
        ('Ê', 'ê'),
        ('Ô', 'ô'),
        ('Ơ', 'ơ'),
        ('Ư', 'ư'),
    ];
    
    for (upper, lower) in test_cases {
        assert_eq!(stage.normalize_char(upper), lower);
    }
}

#[test]
fn test_normalize_vietnamese_tones_uppercase() {
    let stage = NormalizationStage::new();
    
    // Uppercase toned characters
    assert_eq!(stage.normalize_char('Á'), 'á');
    assert_eq!(stage.normalize_char('À'), 'à');
    assert_eq!(stage.normalize_char('Ả'), 'ả');
    assert_eq!(stage.normalize_char('Ã'), 'ã');
    assert_eq!(stage.normalize_char('Ạ'), 'ạ');
}

// === Raw Buffer Tests ===

#[test]
fn test_raw_buffer_single_char() {
    let stage = NormalizationStage::new();
    let mut ctx = TypingContext::new();

    stage.process(&mut ctx, 'x');

    assert_eq!(ctx.raw_buffer(), "x");
}

#[test]
fn test_raw_buffer_accumulation() {
    let stage = NormalizationStage::new();
    let mut ctx = TypingContext::new();

    stage.process(&mut ctx, 'a');
    stage.process(&mut ctx, 'b');
    stage.process(&mut ctx, 'c');

    assert_eq!(ctx.raw_buffer(), "abc");
}

#[test]
fn test_raw_buffer_mixed_case() {
    let stage = NormalizationStage::new();
    let mut ctx = TypingContext::new();

    stage.process(&mut ctx, 'A');
    stage.process(&mut ctx, 'b');
    stage.process(&mut ctx, 'C');

    // All normalized to lowercase
    assert_eq!(ctx.raw_buffer(), "abc");
}

#[test]
fn test_raw_buffer_with_numbers() {
    let stage = NormalizationStage::new();
    let mut ctx = TypingContext::new();

    stage.process(&mut ctx, 'a');
    stage.process(&mut ctx, '1');
    stage.process(&mut ctx, 'b');

    assert_eq!(ctx.raw_buffer(), "a1b");
}

#[test]
fn test_raw_buffer_with_special_chars() {
    let stage = NormalizationStage::new();
    let mut ctx = TypingContext::new();

    stage.process(&mut ctx, 'h');
    stage.process(&mut ctx, 'i');
    stage.process(&mut ctx, '!');

    assert_eq!(ctx.raw_buffer(), "hi!");
}

// === Stage Result Tests ===

#[test]
fn test_always_returns_continue() {
    let stage = NormalizationStage::new();
    let mut ctx = TypingContext::new();

    // Should always return Continue regardless of input
    assert_eq!(stage.process(&mut ctx, 'a'), StageResult::Continue);
    assert_eq!(stage.process(&mut ctx, 'Z'), StageResult::Continue);
    assert_eq!(stage.process(&mut ctx, '1'), StageResult::Continue);
    assert_eq!(stage.process(&mut ctx, ' '), StageResult::Continue);
    assert_eq!(stage.process(&mut ctx, '!'), StageResult::Continue);
}

#[test]
fn test_never_blocks_input() {
    let stage = NormalizationStage::new();
    let mut ctx = TypingContext::new();

    // Try various problematic inputs
    let inputs = vec!['@', '#', '$', '%', '&', '*', '\t', '\n'];
    
    for ch in inputs {
        let result = stage.process(&mut ctx, ch);
        assert_eq!(result, StageResult::Continue, "Failed for '{:?}'", ch);
    }
}

// === Reset Tests ===

#[test]
fn test_reset_no_state() {
    let mut stage = NormalizationStage::new();
    
    // Should not crash
    stage.reset();
    
    // Should still work
    let mut ctx = TypingContext::new();
    assert_eq!(stage.process(&mut ctx, 'a'), StageResult::Continue);
}

#[test]
fn test_reset_multiple_times() {
    let mut stage = NormalizationStage::new();
    
    stage.reset();
    stage.reset();
    stage.reset();
    
    // Still functional
    let mut ctx = TypingContext::new();
    stage.process(&mut ctx, 'x');
    assert_eq!(ctx.raw_buffer(), "x");
}

// === Context Preservation Tests ===

#[test]
fn test_preserves_syllable_buffer() {
    let stage = NormalizationStage::new();
    let mut ctx = TypingContext::new();
    ctx.syllable_buffer = "existing".to_string();

    stage.process(&mut ctx, 'a');

    // Syllable buffer should not be modified by this stage
    assert_eq!(ctx.syllable_buffer, "existing");
    // But raw buffer should be updated
    assert_eq!(ctx.raw_buffer(), "a");
}

#[test]
fn test_preserves_context_flags() {
    let stage = NormalizationStage::new();
    let mut ctx = TypingContext::new();
    ctx.temp_english_mode = true;

    stage.process(&mut ctx, 'a');

    // Flags should not be modified
    assert!(ctx.temp_english_mode);
}

// === Vietnamese Input Tests ===

#[test]
fn test_vietnamese_word_input() {
    let stage = NormalizationStage::new();
    let mut ctx = TypingContext::new();

    // Type "Việt Nam" with mixed case
    for ch in "VieeTNam".chars() {
        stage.process(&mut ctx, ch);
    }

    assert_eq!(ctx.raw_buffer(), "vieetnam");
}

#[test]
fn test_vietnamese_characters_direct() {
    let stage = NormalizationStage::new();
    let mut ctx = TypingContext::new();

    stage.process(&mut ctx, 'â');
    stage.process(&mut ctx, 'ư');
    stage.process(&mut ctx, 'ơ');

    assert_eq!(ctx.raw_buffer(), "âươ");
}

#[test]
fn test_vietnamese_toned_characters() {
    let stage = NormalizationStage::new();
    let mut ctx = TypingContext::new();

    stage.process(&mut ctx, 'á');
    stage.process(&mut ctx, 'à');
    stage.process(&mut ctx, 'ả');

    assert_eq!(ctx.raw_buffer(), "áàả");
}

// === Edge Cases ===

#[test]
fn test_empty_sequence() {
    let _stage = NormalizationStage::new();
    let ctx = TypingContext::new();

    // No processing, buffer should be empty
    assert_eq!(ctx.raw_buffer(), "");
}

#[test]
fn test_long_sequence() {
    let stage = NormalizationStage::new();
    let mut ctx = TypingContext::new();

    // Type a long sequence
    for ch in "abcdefghijklmnopqrstuvwxyz".chars() {
        stage.process(&mut ctx, ch);
    }

    assert_eq!(ctx.raw_buffer(), "abcdefghijklmnopqrstuvwxyz");
}

#[test]
fn test_repeated_characters() {
    let stage = NormalizationStage::new();
    let mut ctx = TypingContext::new();

    // Type same character multiple times
    for _ in 0..5 {
        stage.process(&mut ctx, 'a');
    }

    assert_eq!(ctx.raw_buffer(), "aaaaa");
}

#[test]
fn test_whitespace_characters() {
    let stage = NormalizationStage::new();
    let mut ctx = TypingContext::new();

    stage.process(&mut ctx, ' ');
    stage.process(&mut ctx, '\t');
    
    // Whitespace preserved as-is
    assert_eq!(ctx.raw_buffer(), " \t");
}

// === Integration Scenarios ===

#[test]
fn test_scenario_typing_english_word() {
    let stage = NormalizationStage::new();
    let mut ctx = TypingContext::new();

    // Type "Hello" with capital H
    for ch in "Hello".chars() {
        stage.process(&mut ctx, ch);
    }

    assert_eq!(ctx.raw_buffer(), "hello");
}

#[test]
fn test_scenario_typing_sentence() {
    let stage = NormalizationStage::new();
    let mut ctx = TypingContext::new();

    // Type "Hi! How are you?"
    for ch in "Hi! How are you?".chars() {
        stage.process(&mut ctx, ch);
    }

    assert_eq!(ctx.raw_buffer(), "hi! how are you?");
}

#[test]
fn test_scenario_mixed_vietnamese_english() {
    let stage = NormalizationStage::new();
    let mut ctx = TypingContext::new();

    // Type "Xin chào"
    for ch in "Xin chao".chars() {
        stage.process(&mut ctx, ch);
    }

    assert_eq!(ctx.raw_buffer(), "xin chao");
}
