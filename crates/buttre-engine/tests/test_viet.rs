#[test]
fn test_what_viet_produces() {
    let config = buttre_engine::pipeline::telex_config();
    let mut executor = buttre_engine::pipeline::PipelineExecutor::new(config);

    for ch in "viet".chars() {
        executor.process(ch);
    }

    println!(
        "After 'viet': syllable = '{}'",
        executor.context().syllable_buffer
    );
    println!("Expected by test: 'viêt'");

    // The test expects "viet" → "viêt" but "viet" has no "ee" so it should be "viet"
    assert_eq!(executor.context().syllable_buffer, "viet");
}
