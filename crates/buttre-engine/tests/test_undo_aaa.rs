use buttre_engine::pipeline::PipelineExecutor;

#[test]
fn test_undo_aaa() {
    let config = buttre_engine::pipeline::telex_config();
    let mut executor = PipelineExecutor::new(config);
    
    println!("\n=== Typing 'a' (1st) ===");
    executor.process('a');
    println!("syllable: '{}', last_output: '{}', history: {}", 
             executor.context().syllable_buffer,
             executor.context().last_output,
             executor.context().transform_history.len());
    
    println!("\n=== Typing 'a' (2nd) ===");
    executor.process('a');
    println!("syllable: '{}', last_output: '{}', history: {}", 
             executor.context().syllable_buffer,
             executor.context().last_output,
             executor.context().transform_history.len());
    
    println!("\n=== Before typing 'a' (3rd) ===");
    println!("syllable: '{}', last_output: '{}'", 
             executor.context().syllable_buffer,
             executor.context().last_output);
    println!("syllable == last_output? {}", 
             executor.context().syllable_buffer == executor.context().last_output);
    
    println!("\n=== Typing 'a' (3rd) - should UNDO ===");
    executor.process('a');
    println!("syllable: '{}', last_output: '{}', history: {}", 
             executor.context().syllable_buffer,
             executor.context().last_output,
             executor.context().transform_history.len());
    println!("temp_english_mode: {}", executor.context().temp_english_mode);
    println!("last_was_undo: {}", executor.context().last_was_undo);
    
    if executor.context().syllable_buffer == "aa" {
        println!("\n✓ UNDO WORKED!");
    } else {
        println!("\n✗ UNDO FAILED! Expected 'aa', got '{}'", 
                 executor.context().syllable_buffer);
    }
}
