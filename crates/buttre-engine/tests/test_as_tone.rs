#[test]
fn test_as_creates_history() {
    let config = buttre_engine::pipeline::telex_config();
    let mut executor = buttre_engine::pipeline::PipelineExecutor::new(config);

    executor.process('a');
    println!(
        "After 'a': syllable='{}', history={}",
        executor.context().syllable_buffer,
        executor.context().transform_history.len()
    );

    executor.process('s');
    println!(
        "After 'as': syllable='{}', history={}",
        executor.context().syllable_buffer,
        executor.context().transform_history.len()
    );

    if !executor.context().transform_history.is_empty() {
        let record = &executor.context().transform_history[0];
        println!(
            "Record: input='{}', before='{}', after='{}', type={:?}",
            record.input_char, record.before, record.after, record.transform_type
        );
    }

    // Now try 'ass'
    executor.process('s');
    println!(
        "After 'ass': syllable='{}', history={}, temp_english={}",
        executor.context().syllable_buffer,
        executor.context().transform_history.len(),
        executor.context().temp_english_mode
    );
}
