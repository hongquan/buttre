//! Minimal test to isolate the issue

use buttre_engine::pipeline::{PipelineExecutor, telex_config};

#[test]
fn test_executor_basic() {
    let config = telex_config();
    
    println!("Config has {} transform rules", config.transform_rules.len());
    println!("Config has {} tone mappings", config.tone_map.len());
    
    let mut executor = PipelineExecutor::new(config);
    
    // Process first 'a'
    let _actions = executor.process('a');
    
    let ctx = executor.context();
    println!("After 'a':");
    println!("  raw_buffer: '{}'", ctx.raw_buffer());
    println!("  syllable_buffer: '{}'", ctx.syllable_buffer);
    println!("  temp_english_mode: {}", ctx.temp_english_mode);
    
    // Process second 'a'
    let _actions = executor.process('a');
    
    let ctx2 = executor.context();
    println!("After 'aa':");
    println!("  raw_buffer: '{}'", ctx2.raw_buffer());
    println!("  syllable_buffer: '{}'", ctx2.syllable_buffer);
    println!("  temp_english_mode: {}", ctx2.temp_english_mode);
    println!("  transform_history: {} records", ctx2.transform_history.len());
    
    // Check what we got
    if ctx2.syllable_buffer == "â" {
        println!("✓ Transform WORKED!");
    } else {
        println!("✗ Transform FAILED! Expected 'â', got '{}'", ctx2.syllable_buffer);
    }
}
