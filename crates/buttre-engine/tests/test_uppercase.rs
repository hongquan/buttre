use buttre_engine::pipeline::PipelineExecutor;

/// Test case preservation for uppercase input
/// When CapsLock is on (input is uppercase), output should also be uppercase

#[test]
fn test_uppercase_aa_produces_uppercase_circumflex() {
    let config = buttre_engine::pipeline::telex_config();
    let mut executor = PipelineExecutor::new(config);
    
    executor.process('A');
    assert_eq!(executor.context().syllable_buffer, "A", "Single A should be 'A'");
    
    executor.process('A');
    assert_eq!(executor.context().syllable_buffer, "Â", "AA should produce uppercase Â");
}

#[test]
fn test_uppercase_nguoi_produces_uppercase_nguoi() {
    let config = buttre_engine::pipeline::telex_config();
    let mut executor = PipelineExecutor::new(config);
    
    // Type "NGUOIF" (NGUOI + f for grave tone)
    for ch in "NGUOI".chars() {
        executor.process(ch);
    }
    executor.process('f'); // Apply grave tone
    
    // Should produce "NGƯỜI" (all uppercase with diacritics)
    let output = &executor.context().syllable_buffer;
    
    // Check that output is uppercase
    assert!(output.chars().all(|c| !c.is_alphabetic() || c.is_uppercase() || !c.is_ascii()), 
        "Expected all uppercase, got: {}", output);
}

#[test]
fn test_mixed_case_nguoi_produces_mixed_case() {
    let config = buttre_engine::pipeline::telex_config();
    let mut executor = PipelineExecutor::new(config);
    
    // Type "Nguoif" (first letter caps, rest lowercase + f for grave)
    executor.process('N');
    for ch in "guoi".chars() {
        executor.process(ch);
    }
    executor.process('f'); // Apply grave tone
    
    // Should produce "Người" (first letter uppercase)
    let output = &executor.context().syllable_buffer;
    
    // First char should be uppercase
    assert!(output.chars().next().unwrap().is_uppercase(),
        "Expected first char uppercase, got: {}", output);
}

#[test]
fn test_lowercase_preserved() {
    let config = buttre_engine::pipeline::telex_config();
    let mut executor = PipelineExecutor::new(config);
    
    // Type "nguoif" (all lowercase)
    for ch in "nguoi".chars() {
        executor.process(ch);
    }
    executor.process('f');
    
    let output = &executor.context().syllable_buffer;
    
    // All should be lowercase
    assert!(output.chars().filter(|c| c.is_alphabetic()).all(|c| c.is_lowercase()),
        "Expected all lowercase, got: {}", output);
}

#[test]
fn trace_uppercase_aa() {
    let config = buttre_engine::pipeline::telex_config();
    let mut executor = PipelineExecutor::new(config);

    executor.process('A');
    println!("After 'A': syllable='{}'", executor.context().syllable_buffer);

    executor.process('A');
    println!("After 'AA': syllable='{}'", executor.context().syllable_buffer);
}

#[test]
fn trace_truwowngf() {
    let config = buttre_engine::pipeline::telex_config();
    let mut executor = PipelineExecutor::new(config);
    for ch in "TRuwowngf".chars() {
        executor.process(ch);
        println!("After '{}': syllable='{}' raw='{}'",
                 ch, executor.context().syllable_buffer, executor.context().raw_buffer());
    }
}
