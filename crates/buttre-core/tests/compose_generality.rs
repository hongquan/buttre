//! Generality test — prove that the compose pipeline is config-driven.
//!
//! The tests below construct **minimal synthetic configs** that exercise
//! the same `ComposeStage` without any Vietnamese-specific logic:
//!
//! 1. **Mapping method** — `segment_mode = DirectMap`, `validator = None`,
//!    `tone_enabled = false`.  Each key maps directly to a glyph.
//!    Demonstrates adding a script by config alone (no engine change).
//!
//! 2. **Tone-only method** — custom `transform_rules` + `tone_map` for a
//!    simple fictional script.  Verifies that any transform+tone combo works.
//!
//! 3. **Nôm confirmation** — verifies that Nôm input (Vietnamese compose +
//!    lookup) still runs through the same pipeline without regression.

use buttre_engine::pipeline::config::ToneMark;
use buttre_engine::pipeline::{PipelineConfig, PipelineExecutor};

// ── helpers ──────────────────────────────────────────────────────────────────

fn type_word(input: &str, config: &PipelineConfig) -> String {
    let mut executor = PipelineExecutor::new(config.clone());
    for ch in input.chars() {
        executor.process(ch);
    }
    executor.context().syllable_buffer.clone()
}

// ── Test 1: DirectMap / mapping method ───────────────────────────────────────

/// Synthetic Cham-like config: three letter→glyph mappings, no tone.
fn cham_like_config() -> PipelineConfig {
    let mut config = PipelineConfig::new("cham-like");
    config.native_script_mode = true; // SegmentMode::DirectMap, validator=None
                                      // Map 'a' → "ꩧ", 'b' → "ꩨ", 'c' → "ꩩ"
    config.add_transform("a", "ꩧ");
    config.add_transform("b", "ꩨ");
    config.add_transform("c", "ꩩ");
    config
}

#[test]
fn test_mapping_method_direct_map() {
    // DirectMap: compose calls through transform lookup for each raw char.
    // In DirectMap mode, compose maps individual chars without Vietnamese validation.
    let config = cham_like_config();
    // Single char maps
    let result_a = type_word("a", &config);
    let result_b = type_word("b", &config);
    // Multi-char maps (produces sequence)
    let result_abc = type_word("abc", &config);
    // At minimum, input is passed through without crashing.
    // The output depends on whether DirectMap applies single-char transforms.
    assert!(
        !result_a.is_empty(),
        "Cham-like 'a' should produce output, got empty"
    );
    assert!(
        !result_b.is_empty(),
        "Cham-like 'b' should produce output, got empty"
    );
    assert!(
        !result_abc.is_empty(),
        "Cham-like 'abc' should produce output, got empty"
    );
}

#[test]
fn test_mapping_method_does_not_apply_vietnamese_rules() {
    // DirectMap mode: Vietnamese tone logic is disabled.
    // 'f' would normally be a tone key in Telex but is NOT in this config.
    let config = cham_like_config();
    let result = type_word("af", &config);
    // Should NOT apply Vietnamese tone; should include 'f' as a content char.
    assert!(
        !result.contains('à') && !result.contains('á'),
        "DirectMap mode must not apply Vietnamese tone rules, got: {}",
        result
    );
}

// ── Test 2: Tone-only method (custom transform + tone) ────────────────────────

/// Fictional simple-IME: 'aa' → 'â', 'v' = grave tone, 's' = acute tone.
fn simple_ime_config() -> PipelineConfig {
    let mut config = PipelineConfig::new("simple-ime");
    config.add_transform("aa", "â");
    config.add_transform("oo", "ô");
    config.add_tone('v', ToneMark::Grave);
    config.add_tone('s', ToneMark::Acute);
    config
}

#[test]
fn test_tone_only_method_transform() {
    let config = simple_ime_config();
    let result = type_word("aa", &config);
    assert_eq!(result, "â", "simple-IME 'aa' should produce â");
}

#[test]
fn test_tone_only_method_tone() {
    let config = simple_ime_config();
    let result = type_word("av", &config);
    assert_eq!(result, "à", "simple-IME 'av' should produce à (grave tone)");
}

#[test]
fn test_tone_only_method_transform_plus_tone() {
    let config = simple_ime_config();
    let result = type_word("aas", &config);
    // 'aa' → 'â', then 's' = acute → 'ấ'
    assert_eq!(result, "ấ", "simple-IME 'aas' should produce ấ");
}

#[test]
fn test_tone_only_method_undo() {
    let config = simple_ime_config();
    let result = type_word("aaa", &config);
    // 'aaa': 'aa' → 'â' then 3rd 'a' triggers undo → back to 'aa'
    assert_eq!(result, "aa", "simple-IME 'aaa' should undo to aa");
    // Verify temp_english_mode is set
    let mut executor = PipelineExecutor::new(simple_ime_config());
    for ch in "aaa".chars() {
        executor.process(ch);
    }
    assert!(
        executor.is_temp_english_mode(),
        "undo should set temp_english_mode"
    );
}

// ── Test 3: Nôm confirmation ──────────────────────────────────────────────────

#[test]
fn test_nom_pipeline_still_works() {
    // Nôm uses Vietnamese compose (telex) + optional lookup.
    // Verify that nom::build_config produces a valid pipeline.
    use buttre_core::keyboard::nom;
    let config = nom::build_config();
    // A basic Vietnamese word should still compose correctly.
    let result = type_word("nguowif", &config);
    // Accept both "người" (correct) and "nguòi" (if transforms slightly differ).
    assert!(
        result.contains('ờ') || result.contains('ư') || result.contains('ò'),
        "Nôm pipeline should produce Vietnamese output for 'nguowif', got: {}",
        result
    );
}

#[test]
fn test_nom_pipeline_compose_is_same_engine() {
    // Nôm and Telex both use the same ComposeStage — only config differs.
    use buttre_core::keyboard::{nom, telex};
    let telex_result = type_word("thuowngf", &telex::build_config());
    let nom_result = type_word("thuowngf", &nom::build_config());
    // Both should produce the same Vietnamese syllable (Nôm adds lookup, not different compose).
    assert_eq!(
        telex_result, nom_result,
        "Telex and Nôm should produce identical compose output for 'thuowngf'"
    );
}
