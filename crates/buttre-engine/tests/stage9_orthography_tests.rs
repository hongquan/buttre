use buttre_engine::pipeline::config::{ToneStyle, UnicodeForm};
use buttre_engine::pipeline::stages::stage9_orthography::OrthographyStage;
use buttre_engine::pipeline::{PipelineConfig, PipelineStage, StageResult, TypingContext};
use unicode_normalization::UnicodeNormalization;

#[test]
fn test_new_style() {
    let stage = OrthographyStage::new(ToneStyle::New, UnicodeForm::NFC);
    let mut ctx = TypingContext::new();
    ctx.syllable_buffer = "hòa".to_string();

    let result = stage.process(&mut ctx, 'a');

    assert_eq!(result, StageResult::Continue);
    // For now, output should be unchanged (placeholder implementation)
    assert_eq!(ctx.syllable_buffer, "hòa");
}

#[test]
fn test_old_style() {
    let stage = OrthographyStage::new(ToneStyle::Old, UnicodeForm::NFC);
    let mut ctx = TypingContext::new();
    ctx.syllable_buffer = "hoà".to_string();

    stage.process(&mut ctx, 'a');

    // For now, output should be unchanged (placeholder implementation)
    assert_eq!(ctx.syllable_buffer, "hoà");
}

#[test]
fn test_nfc_form() {
    let stage = OrthographyStage::new(ToneStyle::New, UnicodeForm::NFC);
    let mut ctx = TypingContext::new();
    // Input: NFD form (decomposed)
    ctx.syllable_buffer = "a\u{0302}".to_string(); // a + combining circumflex

    stage.process(&mut ctx, 'a');

    // Output: NFC form (composed)
    assert_eq!(ctx.syllable_buffer, "â");
    // Verify it's actually NFC
    assert_eq!(
        ctx.syllable_buffer,
        ctx.syllable_buffer.nfc().collect::<String>()
    );
}

#[test]
fn test_nfd_form() {
    let stage = OrthographyStage::new(ToneStyle::New, UnicodeForm::NFD);
    let mut ctx = TypingContext::new();
    // Input: NFC form (composed)
    ctx.syllable_buffer = "â".to_string();

    stage.process(&mut ctx, 'a');

    // Output: NFD form (decomposed)
    // Should be "a" + combining circumflex
    assert_eq!(ctx.syllable_buffer, "â".nfd().collect::<String>());
    // Verify it's actually NFD
    assert_eq!(
        ctx.syllable_buffer,
        ctx.syllable_buffer.nfd().collect::<String>()
    );
}

#[test]
fn test_stage_name() {
    let stage = OrthographyStage::new(ToneStyle::New, UnicodeForm::NFC);
    assert_eq!(stage.name(), "OrthographyStage");
}

#[test]
fn test_from_config() {
    let mut config = PipelineConfig::new("test");
    config.tone_style = ToneStyle::New;
    config.unicode_form = UnicodeForm::NFC;

    let stage = OrthographyStage::from_config(&config);

    assert_eq!(stage.tone_style, ToneStyle::New);
    assert_eq!(stage.unicode_form, UnicodeForm::NFC);
}

#[test]
fn test_normalize_unicode() {
    let stage = OrthographyStage::new(ToneStyle::New, UnicodeForm::NFC);

    let result = stage.normalize_unicode("test");
    assert_eq!(result, "test");

    let result = stage.normalize_unicode("thường");
    assert_eq!(result, "thường");
}

#[test]
fn test_normalize_tone_position() {
    let stage = OrthographyStage::new(ToneStyle::New, UnicodeForm::NFC);

    let result = stage.normalize_tone_position("hòa");
    assert_eq!(result, "hòa");
}

#[test]
fn test_empty_buffer() {
    let stage = OrthographyStage::new(ToneStyle::New, UnicodeForm::NFC);
    let mut ctx = TypingContext::new();

    stage.process(&mut ctx, 'a');

    assert_eq!(ctx.syllable_buffer, "");
}

#[test]
fn test_multiple_process_calls() {
    let stage = OrthographyStage::new(ToneStyle::New, UnicodeForm::NFC);
    let mut ctx = TypingContext::new();
    ctx.syllable_buffer = "test".to_string();

    assert_eq!(stage.process(&mut ctx, 'a'), StageResult::Continue);
    assert_eq!(stage.process(&mut ctx, 'b'), StageResult::Continue);
    assert_eq!(ctx.syllable_buffer, "test");
}

#[test]
fn test_nfc_vietnamese_text() {
    let stage = OrthographyStage::new(ToneStyle::New, UnicodeForm::NFC);
    let mut ctx = TypingContext::new();
    // NFD: "hòa" as decomposed characters
    ctx.syllable_buffer = "ho\u{0300}a".to_string(); // h + o + grave + a

    stage.process(&mut ctx, 'a');

    // Should be NFC: "hòa" as composed
    let expected = "hòa".nfc().collect::<String>();
    assert_eq!(ctx.syllable_buffer, expected);
}

#[test]
fn test_nfd_vietnamese_text() {
    let stage = OrthographyStage::new(ToneStyle::New, UnicodeForm::NFD);
    let mut ctx = TypingContext::new();
    // NFC: "hòa" as composed
    ctx.syllable_buffer = "hòa".to_string();

    stage.process(&mut ctx, 'a');

    // Should be NFD: decomposed form
    let expected = "hòa".nfd().collect::<String>();
    assert_eq!(ctx.syllable_buffer, expected);
}

#[test]
fn test_nfc_complex_vietnamese() {
    let stage = OrthographyStage::new(ToneStyle::New, UnicodeForm::NFC);
    let mut ctx = TypingContext::new();
    // Complex Vietnamese syllable with multiple diacritics
    // NFD: "ướ" = u + horn + acute + o
    ctx.syllable_buffer = "u\u{031B}\u{0301}".to_string();

    stage.process(&mut ctx, 'o');

    // Should be NFC
    assert_eq!(
        ctx.syllable_buffer,
        ctx.syllable_buffer.nfc().collect::<String>()
    );
}

#[test]
fn test_unicode_normalization_idempotent() {
    // Test that normalizing already normalized text is idempotent
    let stage_nfc = OrthographyStage::new(ToneStyle::New, UnicodeForm::NFC);
    let stage_nfd = OrthographyStage::new(ToneStyle::New, UnicodeForm::NFD);

    let mut ctx_nfc = TypingContext::new();
    ctx_nfc.syllable_buffer = "hòa".to_string();
    stage_nfc.process(&mut ctx_nfc, 'a');
    let first_nfc = ctx_nfc.syllable_buffer.clone();
    stage_nfc.process(&mut ctx_nfc, 'a');
    assert_eq!(ctx_nfc.syllable_buffer, first_nfc);

    let mut ctx_nfd = TypingContext::new();
    ctx_nfd.syllable_buffer = "hòa".to_string();
    stage_nfd.process(&mut ctx_nfd, 'a');
    let first_nfd = ctx_nfd.syllable_buffer.clone();
    stage_nfd.process(&mut ctx_nfd, 'a');
    assert_eq!(ctx_nfd.syllable_buffer, first_nfd);
}

// ==================== Case Restoration Tests ====================

// ── restore_case unit tests (utility function, not called from process) ────────
//
// NOTE (Phase 4): OrthographyStage.process no longer calls restore_case().
// Case restoration is now ComposeStage's responsibility (apply_case_mask).
// These tests verify restore_case() as a utility; they call it directly.

#[test]
fn test_restore_case_all_uppercase() {
    let stage = OrthographyStage::new(ToneStyle::New, UnicodeForm::NFC);
    // Direct call to restore_case (not via process).
    let result = stage.restore_case("người", &[true, true, true, true, true]);
    assert_eq!(result, "NGƯỜI");
}

#[test]
fn test_restore_case_first_capital() {
    let stage = OrthographyStage::new(ToneStyle::New, UnicodeForm::NFC);
    let result = stage.restore_case("người", &[true, false, false, false, false]);
    assert_eq!(result, "Người");
}

#[test]
fn test_restore_case_mixed() {
    let stage = OrthographyStage::new(ToneStyle::New, UnicodeForm::NFC);
    let result = stage.restore_case("người", &[false, true, true, true, true]);
    assert_eq!(result, "nGƯỜI");
}

#[test]
fn test_restore_case_all_lowercase() {
    let stage = OrthographyStage::new(ToneStyle::New, UnicodeForm::NFC);
    let result = stage.restore_case("người", &[false, false, false, false, false]);
    assert_eq!(result, "người");
}

#[test]
fn test_restore_case_empty_mask() {
    let stage = OrthographyStage::new(ToneStyle::New, UnicodeForm::NFC);
    let result = stage.restore_case("người", &[]);
    assert_eq!(result, "người");
}

#[test]
fn test_restore_case_merged_chars() {
    let stage = OrthographyStage::new(ToneStyle::New, UnicodeForm::NFC);
    // "Aa" → "â": mask [T,F], output 1 char. restore_case uses first char's case.
    let result = stage.restore_case("â", &[true, false]);
    assert_eq!(result, "Â");
}

#[test]
fn test_restore_case_merged_chars_lowercase_first() {
    let stage = OrthographyStage::new(ToneStyle::New, UnicodeForm::NFC);
    let result = stage.restore_case("â", &[false, true]);
    assert_eq!(result, "â"); // first char's case = lowercase
}

#[test]
fn test_restore_case_thuong() {
    let stage = OrthographyStage::new(ToneStyle::New, UnicodeForm::NFC);
    let result = stage.restore_case("thường", &[true, true, true, true, true, true]);
    assert_eq!(result, "THƯỜNG");
}

// ── process() no longer does case restoration (since Phase 4) ────────────────

#[test]
fn test_process_does_not_change_case() {
    // ComposeStage sets case; OrthographyStage.process must preserve it unchanged.
    let stage = OrthographyStage::new(ToneStyle::New, UnicodeForm::NFC);
    let mut ctx = TypingContext::new();
    ctx.set_raw_buffer("nguoi");
    ctx.set_case_mask(vec![true, true, true, true, true]);
    ctx.syllable_buffer = "NGƯỜI".to_string(); // already cased by ComposeStage
    stage.process(&mut ctx, 'i');
    assert_eq!(
        ctx.syllable_buffer, "NGƯỜI",
        "process must not alter case — ComposeStage owns case restoration"
    );
}
