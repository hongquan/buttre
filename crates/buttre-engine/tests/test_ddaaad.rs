/// Regression test: ddaaad sequence
///
/// Correct transform-preserving undo behavior:
/// - "dd" → đ (d-stroke transform)
/// - "aa" → â (circumflex transform)
/// - third "a" → undo circumflex: prefix "dd" re-composed to "đ", literal "aa" appended → "đaa"
///   then temp_english_mode engages
/// - "d" → literal append in temp_english_mode → "đaad"
///
/// This confirms that undoing ONE transform (â→aa) does NOT revert the earlier
/// unrelated transform (dd→đ).  Matches all four reference IMEs.
#[test]
fn test_ddaaad_sequence() {
    let config = buttre_engine::pipeline::telex_config();
    let mut executor = buttre_engine::pipeline::PipelineExecutor::new(config);

    // Build up step by step so we can verify intermediate states.
    executor.process('d');
    executor.process('d');
    assert_eq!(executor.context().syllable_buffer, "đ",
               "dd → đ");

    executor.process('a');
    executor.process('a');
    assert_eq!(executor.context().syllable_buffer, "đâ",
               "ddaa → đâ");

    executor.process('a');
    assert_eq!(executor.context().syllable_buffer, "đaa",
               "ddaaa → đaa: undo of â reverts to aa; đ (from dd) is preserved");
    assert!(executor.context().temp_english_mode,
            "temp_english_mode must be set after transform undo");

    executor.process('d');
    assert_eq!(executor.context().syllable_buffer, "đaad",
               "ddaaad → đaad: d is literal in temp_english_mode");
}
