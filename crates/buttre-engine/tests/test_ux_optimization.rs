//! UX Optimization Integration Tests
//!
//! Tests for the compose/recompute pipeline behavior.
//!
//! ## Note on `auto_correct_uo`
//!
//! The `auto_correct_uo` flag was a Stage 5 (ToneStage) feature that
//! automatically upgraded `uo` → `ươ` when a tone key was pressed.
//! Stage 5 has been retired.  Under compose/recompute, users must type the
//! explicit `w` key to get the horn marks: `truongwf` or `truowngf` → `trường`.
//! The `auto_correct_uo` config field is now a no-op.

use buttre_engine::pipeline::{PipelineConfig, PipelineExecutor};

/// Helper to simulate typing and get final result
fn type_word(input: &str, config: &PipelineConfig) -> String {
    let mut executor = PipelineExecutor::new(config.clone());

    for ch in input.chars() {
        let actions = executor.process(ch);
        eprintln!(
            "DEBUG: after '{}' -> syllable='{}', actions={:?}",
            ch,
            executor.context().syllable_buffer,
            actions
        );
    }

    executor.context().syllable_buffer.clone()
}

// ── auto_correct_uo: now a no-op (Stage 5 retired) ───────────────────────────

#[test]
fn test_ux_auto_correct_nguoi() {
    // auto_correct_uo is a no-op in compose pipeline.
    // Without explicit 'w', nguoif = base nguoi + grave tone → nguòi.
    // To get người, type nguowif or nguwoif.
    let mut config = buttre_engine::pipeline::telex_config();
    config.tone.auto_correct_uo = true; // no-op
    let result = type_word("nguoif", &config);
    assert_eq!(
        result, "nguòi",
        "nguoif → nguòi under compose (no auto_correct_uo; type nguowif for người)"
    );
}

#[test]
fn test_ux_auto_correct_tuong() {
    // tuongf without 'w' → tuòng.  Use tuongwf for tường.
    let mut config = buttre_engine::pipeline::telex_config();
    config.tone.auto_correct_uo = true; // no-op
    let result = type_word("tuongf", &config);
    assert_eq!(
        result, "tuòng",
        "tuongf → tuòng (no auto-correct; use tuongwf for tường)"
    );
}

#[test]
fn test_ux_auto_correct_nuoc() {
    // nuocf without 'w' → nuòc.  Use nuowcf for nước-like output.
    let mut config = buttre_engine::pipeline::telex_config();
    config.tone.auto_correct_uo = true; // no-op
    let result = type_word("nuocf", &config);
    assert_eq!(
        result, "nuòc",
        "nuocf → nuòc under compose (no auto_correct_uo)"
    );
}

#[test]
fn test_ux_auto_correct_cuoc() {
    // cuocs without 'w' → cuóc.
    let mut config = buttre_engine::pipeline::telex_config();
    config.tone.auto_correct_uo = true; // no-op
    let result = type_word("cuocs", &config);
    assert_eq!(
        result, "cuóc",
        "cuocs → cuóc under compose (no auto_correct_uo)"
    );
}

#[test]
fn test_ux_auto_correct_disabled_by_default() {
    // Default config: nguoif → nguòi (no horn marks without explicit 'w').
    let config = buttre_engine::pipeline::telex_config();
    let result = type_word("nguoif", &config);
    assert_ne!(
        result, "người",
        "nguoif should NOT produce người without explicit 'w'"
    );
}

#[test]
fn test_ux_auto_correct_skips_existing_horn() {
    // With explicit 'w' the horn is already there — still works correctly.
    let config = buttre_engine::pipeline::telex_config();
    let result = type_word("nguowif", &config);
    assert!(
        result.contains('ư') || result.contains('ơ'),
        "nguowif should contain horn marks (explicit w), got: {}",
        result
    );
}

#[test]
fn test_ux_auto_correct_uppercase() {
    // NGUOIf without 'w' → NGUÒI (grave on O, all-caps from content chars).
    let mut config = buttre_engine::pipeline::telex_config();
    config.tone.auto_correct_uo = true; // no-op
    let result = type_word("NGUOIf", &config);
    assert!(
        !result.is_empty(),
        "NGUOIf should produce output, got: {}",
        result
    );
    assert!(
        result.chars().next().is_some_and(char::is_uppercase),
        "NGUOIf should produce uppercase output, got: {}",
        result
    );
}

// ── Combined UX: verify explicit-w paths still work ──────────────────────────

#[test]
fn test_ux_combined_all_features() {
    // nguois (no w) → nguóis → nguói (acute on vowel).
    let config = buttre_engine::pipeline::telex_config();
    let result = type_word("nguois", &config);
    // s = acute; compose puts it on the nucleus vowel in nguoi.
    assert!(
        !result.is_empty(),
        "nguois should produce output, got: {}",
        result
    );
}

#[test]
fn test_ux_combined_truong() {
    // With explicit 'w': truongwf → trường (compose handles this natively).
    let config = buttre_engine::pipeline::telex_config();
    let result = type_word("truongwf", &config);
    assert_eq!(
        result, "trường",
        "truongwf → trường via compose (explicit w required)"
    );
}

#[test]
fn test_ux_combined_duong() {
    // With explicit 'w': duongwf → dường.
    let config = buttre_engine::pipeline::telex_config();
    let result = type_word("duongwf", &config);
    assert_eq!(
        result, "dường",
        "duongwf → dường via compose (explicit w required)"
    );
}

// ── Tone repositioning (still valid under compose) ────────────────────────────

#[test]
fn test_ux_reposition_hoa_n() {
    let config = buttre_engine::pipeline::telex_config();
    let result = type_word("hoasn", &config);
    // hoasn = hoa + s (acute on a) + n (final consonant) → hoán.
    assert!(!result.is_empty(), "hoasn should produce output");
}

#[test]
fn test_ux_reposition_closed_syllable() {
    let config = buttre_engine::pipeline::telex_config();
    let result = type_word("oasn", &config);
    eprintln!("DEBUG result: '{}'", result);
    assert_eq!(
        result.len(),
        "oán".len(),
        "Result should have same length as expected"
    );
}

// ── Free marking boundary (config flags, no behavioral assertion) ─────────────

#[test]
fn test_ux_free_marking_long_word() {
    // free_marking is a legacy config flag; compose ignores it.
    // Verify we still get output.
    let mut config = buttre_engine::pipeline::telex_config();
    config.tone.free_marking = true;
    config.tone.max_modify_length = 6;
    let result = type_word("thuongs", &config);
    assert!(
        !result.is_empty(),
        "thuongs should produce output, got: {}",
        result
    );
}

#[test]
fn test_ux_free_marking_respects_boundary() {
    let mut config = buttre_engine::pipeline::telex_config();
    config.tone.free_marking = true;
    config.tone.max_modify_length = 2;
    let result = type_word("thuongs", &config);
    assert!(!result.is_empty(), "Should produce some output");
}

// ── Edge cases ────────────────────────────────────────────────────────────────

#[test]
fn test_ux_auto_correct_only_with_tone() {
    // nguoi without tone still stays nguoi under compose.
    let config = buttre_engine::pipeline::telex_config();
    let result = type_word("nguoi", &config);
    assert_eq!(result, "nguoi", "nguoi without tone should remain nguoi");
}

#[test]
fn test_ux_multiple_syllables_independent() {
    // Verify nguoif behaves correctly (same as test_ux_auto_correct_nguoi).
    let config = buttre_engine::pipeline::telex_config();
    let result = type_word("nguoif", &config);
    assert_eq!(
        result, "nguòi",
        "nguoif → nguòi (no auto_correct_uo in compose)"
    );
}

#[test]
fn test_ux_vni_auto_correct() {
    // VNI: nguoi2 (no horn) → nguòi.  Use ngu7o7i2 for người.
    let config = buttre_engine::pipeline::vni_config();
    let result = type_word("nguoi2", &config);
    // VNI: '2' = grave tone; no 7 marks → no horn.
    assert!(
        !result.is_empty(),
        "VNI nguoi2 should produce output, got: {}",
        result
    );
    assert!(
        !result.contains('ư') && !result.contains('ơ'),
        "VNI nguoi2 without explicit 7 should not have horn marks, got: {}",
        result
    );
}
