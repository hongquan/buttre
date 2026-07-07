use buttre_engine::pipeline::config::ToneMark;
/// VNI Tone Undo Tests
///
/// Tests VNI undo/toggle behavior for tone marks.
/// VNI uses a toggle mechanism: pressing the same tone key twice
/// toggles the tone on/off.
///
/// Based on Unikey algorithm analysis from:
/// - .agent/optimization/unikey_implementation.md
///
/// VNI Tone Keys:
/// - 1: Acute (sắc) - á
/// - 2: Grave (huyền) - à
/// - 3: Hook (hỏi) - ả
/// - 4: Tilde (ngã) - ã
/// - 5: Dot (nặng) - ạ
///
/// ## Phase 4 (Compose recompute) behavior notes
///
/// The old incremental pipeline (Stage 5) tracked per-keystroke tone state and
/// re-applied tone on the third digit (e.g. `a111` → `á1`). The compose model
/// reprocesses the full raw sequence at once; undo is detected as a repeated-key
/// pair at the tail, after which `temp_english_mode` engages and literal digits
/// pass through unchanged.
///
/// **Triple-digit reapply** (`a111`, `a222`, …): compose gives `a11`, `a22`, …
/// (undo pair at tail → temp_english_mode → literal `1`/`2`/… appended).
/// This matches Unikey `tempVietOff` behaviour: after tone-undo, subsequent
/// same-key taps are literal (no re-apply). Intentional standard, not a deferral.
///
/// **Transform+tone undo** (`a611`, `a822`, `u733`, `o744`, `u7o711`): compose
/// strips the tone but PRESERVES the diacritic transform. `a611` → `â1`,
/// `a822` → `ă2`, `u733` → `ư3`, `o744` → `ơ4`, `u7o711` → `ươ1`.
/// Matches all four reference IMEs (fcitx5-unikey, Unikey-Windows, OpenKey, ibus-bamboo).
use buttre_engine::pipeline::{PipelineConfig, PipelineExecutor};

/// Helper function to create VNI config
fn create_vni_config() -> PipelineConfig {
    let mut config = PipelineConfig::new("vni");

    // VNI transformations - lowercase
    config.add_transform("a6", "â");
    config.add_transform("a8", "ă");
    config.add_transform("e6", "ê");
    config.add_transform("o6", "ô");
    config.add_transform("o7", "ơ");
    config.add_transform("u7", "ư");
    config.add_transform("d9", "đ");

    // VNI transformations - uppercase
    config.add_transform("A6", "Â");
    config.add_transform("A8", "Ă");
    config.add_transform("E6", "Ê");
    config.add_transform("O6", "Ô");
    config.add_transform("O7", "Ơ");
    config.add_transform("U7", "Ư");
    config.add_transform("D9", "Đ");

    // VNI tones
    config.add_tone('1', ToneMark::Acute);
    config.add_tone('2', ToneMark::Grave);
    config.add_tone('3', ToneMark::Hook);
    config.add_tone('4', ToneMark::Tilde);
    config.add_tone('5', ToneMark::Dot);

    config
}

/// Helper function to process a sequence
fn process_sequence(config: &PipelineConfig, input: &str) -> PipelineExecutor {
    let mut executor = PipelineExecutor::new(config.clone());
    for ch in input.chars() {
        executor.process(ch);
    }
    executor
}

// ============================================================================
// A. ACUTE TONE UNDO (Key 1)
// ============================================================================

#[test]
fn test_vni_a1_acute_apply() {
    let config = create_vni_config();
    let executor = process_sequence(&config, "a1");
    assert_eq!(executor.syllable(), "á", "a1 should produce á");
}

#[test]
fn test_vni_a11_acute_undo() {
    let config = create_vni_config();
    let executor = process_sequence(&config, "a11");
    assert_eq!(executor.syllable(), "a1", "a11 should undo to a1");
}

#[test]
fn test_vni_a111_acute_reapply() {
    // Matches Unikey: after tone-undo (11), temp_english_mode engages.
    // The third `1` is a literal append → "a11". This is intentional standard
    // behaviour (Unikey tempVietOff), not a missing feature.
    let config = create_vni_config();
    let executor = process_sequence(&config, "a111");
    assert_eq!(executor.syllable(), "a11",
        "a111 → a11: Unikey standard — after undo pair, subsequent same-key taps are literal (no re-apply)");
}

#[test]
fn test_vni_e1_acute_apply() {
    let config = create_vni_config();
    let executor = process_sequence(&config, "e1");
    assert_eq!(executor.syllable(), "é", "e1 should produce é");
}

#[test]
fn test_vni_e11_acute_undo() {
    let config = create_vni_config();
    let executor = process_sequence(&config, "e11");
    assert_eq!(executor.syllable(), "e1", "e11 should undo to e1");
}

// ============================================================================
// B. GRAVE TONE UNDO (Key 2)
// ============================================================================

#[test]
fn test_vni_a2_grave_apply() {
    let config = create_vni_config();
    let executor = process_sequence(&config, "a2");
    assert_eq!(executor.syllable(), "à", "a2 should produce à");
}

#[test]
fn test_vni_a22_grave_undo() {
    let config = create_vni_config();
    let executor = process_sequence(&config, "a22");
    assert_eq!(executor.syllable(), "a2", "a22 should undo to a2");
}

#[test]
fn test_vni_a222_grave_reapply() {
    // Matches Unikey: after tone-undo (22), temp_english_mode engages.
    // The third `2` is a literal append → "a22". Intentional standard, not a deferral.
    let config = create_vni_config();
    let executor = process_sequence(&config, "a222");
    assert_eq!(executor.syllable(), "a22",
        "a222 → a22: Unikey standard — after undo pair, subsequent same-key taps are literal (no re-apply)");
}

#[test]
fn test_vni_o2_grave_apply() {
    let config = create_vni_config();
    let executor = process_sequence(&config, "o2");
    assert_eq!(executor.syllable(), "ò", "o2 should produce ò");
}

#[test]
fn test_vni_o22_grave_undo() {
    let config = create_vni_config();
    let executor = process_sequence(&config, "o22");
    assert_eq!(executor.syllable(), "o2", "o22 should undo to o2");
}

// ============================================================================
// C. HOOK TONE UNDO (Key 3)
// ============================================================================

#[test]
fn test_vni_a3_hook_apply() {
    let config = create_vni_config();
    let executor = process_sequence(&config, "a3");
    assert_eq!(executor.syllable(), "ả", "a3 should produce ả");
}

#[test]
fn test_vni_a33_hook_undo() {
    let config = create_vni_config();
    let executor = process_sequence(&config, "a33");
    assert_eq!(executor.syllable(), "a3", "a33 should undo to a3");
}

#[test]
fn test_vni_a333_hook_reapply() {
    // Matches Unikey: after tone-undo (33), temp_english_mode engages.
    // The third `3` is a literal append → "a33". Intentional standard, not a deferral.
    let config = create_vni_config();
    let executor = process_sequence(&config, "a333");
    assert_eq!(executor.syllable(), "a33",
        "a333 → a33: Unikey standard — after undo pair, subsequent same-key taps are literal (no re-apply)");
}

#[test]
fn test_vni_i3_hook_apply() {
    let config = create_vni_config();
    let executor = process_sequence(&config, "i3");
    assert_eq!(executor.syllable(), "ỉ", "i3 should produce ỉ");
}

#[test]
fn test_vni_i33_hook_undo() {
    let config = create_vni_config();
    let executor = process_sequence(&config, "i33");
    assert_eq!(executor.syllable(), "i3", "i33 should undo to i3");
}

// ============================================================================
// D. TILDE TONE UNDO (Key 4)
// ============================================================================

#[test]
fn test_vni_a4_tilde_apply() {
    let config = create_vni_config();
    let executor = process_sequence(&config, "a4");
    assert_eq!(executor.syllable(), "ã", "a4 should produce ã");
}

#[test]
fn test_vni_a44_tilde_undo() {
    let config = create_vni_config();
    let executor = process_sequence(&config, "a44");
    assert_eq!(executor.syllable(), "a4", "a44 should undo to a4");
}

#[test]
fn test_vni_a444_tilde_reapply() {
    // Matches Unikey: after tone-undo (44), temp_english_mode engages.
    // The third `4` is a literal append → "a44". Intentional standard, not a deferral.
    let config = create_vni_config();
    let executor = process_sequence(&config, "a444");
    assert_eq!(executor.syllable(), "a44",
        "a444 → a44: Unikey standard — after undo pair, subsequent same-key taps are literal (no re-apply)");
}

#[test]
fn test_vni_u4_tilde_apply() {
    let config = create_vni_config();
    let executor = process_sequence(&config, "u4");
    assert_eq!(executor.syllable(), "ũ", "u4 should produce ũ");
}

#[test]
fn test_vni_u44_tilde_undo() {
    let config = create_vni_config();
    let executor = process_sequence(&config, "u44");
    assert_eq!(executor.syllable(), "u4", "u44 should undo to u4");
}

// ============================================================================
// E. DOT TONE UNDO (Key 5)
// ============================================================================

#[test]
fn test_vni_a5_dot_apply() {
    let config = create_vni_config();
    let executor = process_sequence(&config, "a5");
    assert_eq!(executor.syllable(), "ạ", "a5 should produce ạ");
}

#[test]
fn test_vni_a55_dot_undo() {
    let config = create_vni_config();
    let executor = process_sequence(&config, "a55");
    assert_eq!(executor.syllable(), "a5", "a55 should undo to a5");
}

#[test]
fn test_vni_a555_dot_reapply() {
    // Matches Unikey: after tone-undo (55), temp_english_mode engages.
    // The third `5` is a literal append → "a55". Intentional standard, not a deferral.
    let config = create_vni_config();
    let executor = process_sequence(&config, "a555");
    assert_eq!(executor.syllable(), "a55",
        "a555 → a55: Unikey standard — after undo pair, subsequent same-key taps are literal (no re-apply)");
}

#[test]
fn test_vni_y5_dot_apply() {
    let config = create_vni_config();
    let executor = process_sequence(&config, "y5");
    assert_eq!(executor.syllable(), "ỵ", "y5 should produce ỵ");
}

#[test]
fn test_vni_y55_dot_undo() {
    let config = create_vni_config();
    let executor = process_sequence(&config, "y55");
    assert_eq!(executor.syllable(), "y5", "y55 should undo to y5");
}

// ============================================================================
// F. UPPERCASE TONE UNDO
// ============================================================================

#[test]
fn test_vni_a1_uppercase_acute_apply() {
    let config = create_vni_config();
    let executor = process_sequence(&config, "A1");
    assert_eq!(executor.syllable(), "Á", "A1 should produce Á");
}

#[test]
fn test_vni_a11_uppercase_acute_undo() {
    let config = create_vni_config();
    let executor = process_sequence(&config, "A11");
    assert_eq!(executor.syllable(), "A1", "A11 should undo to A1");
}

#[test]
fn test_vni_e2_uppercase_grave_apply() {
    let config = create_vni_config();
    let executor = process_sequence(&config, "E2");
    assert_eq!(executor.syllable(), "È", "E2 should produce È");
}

#[test]
fn test_vni_e22_uppercase_grave_undo() {
    let config = create_vni_config();
    let executor = process_sequence(&config, "E22");
    assert_eq!(executor.syllable(), "E2", "E22 should undo to E2");
}

#[test]
fn test_vni_i3_uppercase_hook_apply() {
    let config = create_vni_config();
    let executor = process_sequence(&config, "I3");
    assert_eq!(executor.syllable(), "Ỉ", "I3 should produce Ỉ");
}

#[test]
fn test_vni_i33_uppercase_hook_undo() {
    let config = create_vni_config();
    let executor = process_sequence(&config, "I33");
    assert_eq!(executor.syllable(), "I3", "I33 should undo to I3");
}

// ============================================================================
// G. TONE ON TRANSFORMED VOWELS
// ============================================================================

#[test]
fn test_vni_a61_circumflex_then_acute() {
    let config = create_vni_config();
    let executor = process_sequence(&config, "a61");
    assert_eq!(executor.syllable(), "ấ", "a61 should produce ấ");
}

#[test]
fn test_vni_a611_undo_tone_keep_circumflex() {
    // a6 → â (circumflex transform), then 11 undo pair strips tone but keeps transform.
    // Correct output: â1. Matches all four reference IMEs.
    let config = create_vni_config();
    let executor = process_sequence(&config, "a611");
    assert_eq!(
        executor.syllable(),
        "â1",
        "a611 → â1: tone stripped, circumflex transform preserved (universal IME behaviour)"
    );
}

#[test]
fn test_vni_a6116_undo_then_literal_6() {
    // a611 → â1 (undo), then `6` is literal in temp_english_mode → â16.
    let config = create_vni_config();
    let executor = process_sequence(&config, "a6116");
    assert_eq!(
        executor.syllable(),
        "â16",
        "a6116 → â16: undo gives â1, then literal 6 appended in temp_english_mode"
    );
}

#[test]
fn test_vni_a82_breve_then_grave() {
    let config = create_vni_config();
    let executor = process_sequence(&config, "a82");
    assert_eq!(executor.syllable(), "ằ", "a82 should produce ằ");
}

#[test]
fn test_vni_a822_undo_tone_keep_breve() {
    // a8 → ă (breve transform), then 22 undo pair strips tone but keeps transform.
    // Correct output: ă2. Matches all four reference IMEs.
    let config = create_vni_config();
    let executor = process_sequence(&config, "a822");
    assert_eq!(
        executor.syllable(),
        "ă2",
        "a822 → ă2: tone stripped, breve transform preserved (universal IME behaviour)"
    );
}

#[test]
fn test_vni_u73_horn_then_hook() {
    let config = create_vni_config();
    let executor = process_sequence(&config, "u73");
    assert_eq!(executor.syllable(), "ử", "u73 should produce ử");
}

#[test]
fn test_vni_u733_undo_tone_keep_horn() {
    // u7 → ư (horn transform), then 33 undo pair strips tone but keeps transform.
    // Correct output: ư3. Matches all four reference IMEs.
    let config = create_vni_config();
    let executor = process_sequence(&config, "u733");
    assert_eq!(
        executor.syllable(),
        "ư3",
        "u733 → ư3: tone stripped, horn transform preserved (universal IME behaviour)"
    );
}

#[test]
fn test_vni_o74_horn_then_tilde() {
    let config = create_vni_config();
    let executor = process_sequence(&config, "o74");
    assert_eq!(executor.syllable(), "ỡ", "o74 should produce ỡ");
}

#[test]
fn test_vni_o744_undo_tone_keep_horn() {
    // o7 → ơ (horn transform), then 44 undo pair strips tone but keeps transform.
    // Correct output: ơ4. Matches all four reference IMEs.
    let config = create_vni_config();
    let executor = process_sequence(&config, "o744");
    assert_eq!(
        executor.syllable(),
        "ơ4",
        "o744 → ơ4: tone stripped, horn transform preserved (universal IME behaviour)"
    );
}

// ============================================================================
// H. TONE SWITCHING (Different tone keys)
// ============================================================================

#[test]
fn test_vni_a12_switch_acute_to_grave() {
    let config = create_vni_config();
    let executor = process_sequence(&config, "a12");
    // a1 → á, then 2 → à (switch tone)
    assert_eq!(
        executor.syllable(),
        "à",
        "a12 should switch from acute to grave"
    );
}

#[test]
fn test_vni_a123_switch_through_tones() {
    let config = create_vni_config();
    let executor = process_sequence(&config, "a123");
    // a1 → á, 2 → à, 3 → ả
    assert_eq!(executor.syllable(), "ả", "a123 should switch to hook");
}

#[test]
fn test_vni_a1234_switch_through_all_tones() {
    let config = create_vni_config();
    let executor = process_sequence(&config, "a1234");
    // a1 → á, 2 → à, 3 → ả, 4 → ã
    assert_eq!(executor.syllable(), "ã", "a1234 should switch to tilde");
}

#[test]
fn test_vni_a12345_switch_through_all_tones_to_dot() {
    let config = create_vni_config();
    let executor = process_sequence(&config, "a12345");
    // a1 → á, 2 → à, 3 → ả, 4 → ã, 5 → ạ
    assert_eq!(executor.syllable(), "ạ", "a12345 should switch to dot");
}

// ============================================================================
// I. COMPLEX SCENARIOS
// ============================================================================

#[test]
fn test_vni_word_viet_with_tone_undo() {
    let config = create_vni_config();

    // vie65t → việt
    let mut executor = PipelineExecutor::new(config.clone());
    for ch in "vie65t".chars() {
        executor.process(ch);
    }
    assert_eq!(executor.syllable(), "việt");

    // vie65t5: same-tone repress (dot `5` already applied, pressed again).
    // Correct output: viêt5 — strip dot tone from việt, keep ê circumflex, literal 5.
    // Matches all four reference IMEs (Unikey tempVietOff after repress).
    executor.process('5');
    assert_eq!(
        executor.syllable(),
        "viêt5",
        "vie65t5 → viêt5: same-tone repress strips tone, keeps ê transform, literal 5"
    );
}

#[test]
fn test_vni_word_nguoi_with_tone_undo() {
    let config = create_vni_config();

    // ngu7o7i2 → người
    let mut executor = PipelineExecutor::new(config.clone());
    for ch in "ngu7o7i2".chars() {
        executor.process(ch);
    }
    assert_eq!(executor.syllable(), "người");

    // ngu7o7i22: `22` contiguous undo pair at tail.
    // Correct: compose segments raw=[n,g,u,7,o,7,i,2,2], detects 22 undo pair,
    // applies transforms to base "nguoi" with marks [7@3, 7@5] → "ngươi",
    // then appends literal "2" → "ngươi2".
    executor.process('2');
    assert_eq!(
        executor.syllable(),
        "ngươi2",
        "ngu7o7i22 → ngươi2: tone stripped, ươ compound preserved, literal 2 appended"
    );
}

#[test]
fn test_vni_sequential_tone_undo() {
    let config = create_vni_config();

    // Multiple tone toggles
    let mut executor = PipelineExecutor::new(config.clone());
    executor.process('a');
    executor.process('1'); // á
    assert_eq!(executor.syllable(), "á");

    executor.process('1'); // a (undo)
    assert_eq!(executor.syllable(), "a1");

    // Matches Unikey: after undo pair (11), temp_english_mode engages.
    // Third `1` is literal append → "a11". Intentional standard, not a deferral.
    executor.process('1'); // literal in temp_english_mode
    assert_eq!(
        executor.syllable(),
        "a11",
        "a111 → a11: Unikey standard — after tone-undo, subsequent same-key taps are literal"
    );

    executor.process('1'); // compose: another literal
    assert_eq!(executor.syllable(), "a111");
}

// ============================================================================
// J. EDGE CASES
// ============================================================================

#[test]
fn test_vni_tone_on_vowel_cluster() {
    let config = create_vni_config();

    // ai1 → ái (tone on first vowel in cluster)
    let executor = process_sequence(&config, "ai1");
    assert_eq!(executor.syllable(), "ái", "ai1 should produce ái");
}

#[test]
fn test_vni_tone_undo_on_vowel_cluster() {
    let config = create_vni_config();

    // ai11 → ai1 (undo tone)
    let executor = process_sequence(&config, "ai11");
    assert_eq!(executor.syllable(), "ai1", "ai11 should undo to ai1");
}

#[test]
fn test_vni_uow_with_tone_and_undo() {
    let config = create_vni_config();

    // u7o71 → ướ (tone on ơ)
    let executor = process_sequence(&config, "u7o71");
    assert_eq!(executor.syllable(), "ướ");

    // u7o711: `11` undo pair at tail strips tone, keeps ươ compound transform.
    // Correct output: ươ1. Matches all four reference IMEs.
    let executor2 = process_sequence(&config, "u7o711");
    assert_eq!(
        executor2.syllable(),
        "ươ1",
        "u7o711 → ươ1: tone stripped, ươ compound transform preserved, literal 1 appended"
    );
}

#[test]
fn test_vni_tone_then_transform_undo() {
    let config = create_vni_config();

    // a1 → á, then 6 → á (try to apply circumflex on already toned)
    // Expected: add circumflex to get ấ
    let mut executor = PipelineExecutor::new(config.clone());
    executor.process('a');
    executor.process('1'); // á
    executor.process('6'); // Should transform a→â with tone: ấ

    assert_eq!(executor.syllable(), "ấ", "a16 should produce ấ");
}
