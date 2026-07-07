use buttre_engine::pipeline::config::ToneMark;
/// VNI Transformation Undo Tests
///
/// Tests VNI undo/toggle behavior for transformations.
///
/// ## Phase 4 (Compose recompute) behavior notes
///
/// The old incremental pipeline (Stage 5) re-applied transforms on the third
/// consecutive same-trigger digit (e.g. `a666` → `â6`). The compose model
/// detects the repeated-trigger pair as an undo and enters temp_english_mode;
/// the third digit is then a literal append → `a66`.
///
/// This matches Unikey's `tempVietOff` behaviour: after a transform-undo,
/// subsequent same-key taps are literal (no re-apply). This is the correct
/// standard; it is not a missing feature.
use buttre_engine::pipeline::{PipelineConfig, PipelineExecutor};

fn create_vni_config() -> PipelineConfig {
    let mut config = PipelineConfig::new("vni");
    // Lowercase transforms
    config.add_transform("a6", "â");
    config.add_transform("a8", "ă");
    config.add_transform("e6", "ê");
    config.add_transform("o6", "ô");
    config.add_transform("o7", "ơ");
    config.add_transform("u7", "ư");
    config.add_transform("d9", "đ");

    // Uppercase transforms
    config.add_transform("A6", "Â");
    config.add_transform("A8", "Ă");
    config.add_transform("E6", "Ê");
    config.add_transform("O6", "Ô");
    config.add_transform("O7", "Ơ");
    config.add_transform("U7", "Ư");
    config.add_transform("D9", "Đ");

    // Tones
    config.add_tone('1', ToneMark::Acute);
    config.add_tone('2', ToneMark::Grave);
    config.add_tone('3', ToneMark::Hook);
    config.add_tone('4', ToneMark::Tilde);
    config.add_tone('5', ToneMark::Dot);
    config
}

fn process_sequence(config: &PipelineConfig, input: &str) -> PipelineExecutor {
    let mut executor = PipelineExecutor::new(config.clone());
    for ch in input.chars() {
        executor.process(ch);
    }
    executor
}

// Circumflex tests
#[test]
fn test_vni_a6_circumflex_apply() {
    let config = create_vni_config();
    let executor = process_sequence(&config, "a6");
    assert_eq!(executor.syllable(), "â");
}

#[test]
fn test_vni_a66_circumflex_undo() {
    let config = create_vni_config();
    let executor = process_sequence(&config, "a66");
    assert_eq!(executor.syllable(), "a6");
}

#[test]
fn test_vni_a666_circumflex_reapply() {
    // a66 undo pair → temp_english_mode; literal `6` appended → "a66".
    // Matches Unikey: after transform-undo, subsequent same-key taps are literal
    // (no re-apply). This is intentional standard behaviour, not a missing feature.
    let config = create_vni_config();
    let executor = process_sequence(&config, "a666");
    assert_eq!(
        executor.syllable(),
        "a66",
        "a666 → a66: Unikey standard — after undo pair, subsequent taps are literal (no re-apply)"
    );
}

// Horn tests
#[test]
fn test_vni_o7_horn_apply() {
    let config = create_vni_config();
    let executor = process_sequence(&config, "o7");
    assert_eq!(executor.syllable(), "ơ");
}

#[test]
fn test_vni_o77_horn_undo() {
    let config = create_vni_config();
    let executor = process_sequence(&config, "o77");
    assert_eq!(executor.syllable(), "o7");
}

#[test]
fn test_vni_u7_horn_apply() {
    let config = create_vni_config();
    let executor = process_sequence(&config, "u7");
    assert_eq!(executor.syllable(), "ư");
}

#[test]
fn test_vni_u77_horn_undo() {
    let config = create_vni_config();
    let executor = process_sequence(&config, "u77");
    assert_eq!(executor.syllable(), "u7");
}

// Breve tests
#[test]
fn test_vni_a8_breve_apply() {
    let config = create_vni_config();
    let executor = process_sequence(&config, "a8");
    assert_eq!(executor.syllable(), "ă");
}

#[test]
fn test_vni_a88_breve_undo() {
    let config = create_vni_config();
    let executor = process_sequence(&config, "a88");
    assert_eq!(executor.syllable(), "a8");
}

// D-stroke tests
#[test]
fn test_vni_d9_dstroke_apply() {
    let config = create_vni_config();
    let executor = process_sequence(&config, "d9");
    assert_eq!(executor.syllable(), "đ");
}

#[test]
fn test_vni_d99_dstroke_undo() {
    let config = create_vni_config();
    let executor = process_sequence(&config, "d99");
    assert_eq!(executor.syllable(), "d9");
}

// Uppercase tests
#[test]
fn test_vni_uppercase_a6() {
    let config = create_vni_config();
    let executor = process_sequence(&config, "A6");
    assert_eq!(executor.syllable(), "Â");
}

#[test]
fn test_vni_uppercase_a66_undo() {
    let config = create_vni_config();
    let executor = process_sequence(&config, "A66");
    assert_eq!(executor.syllable(), "A6");
}
