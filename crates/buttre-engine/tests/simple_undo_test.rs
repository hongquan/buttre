//! Simple undo test — verifies recompute-from-raw undo behavior.
//!
//! Note: `transform_history` and `last_was_undo` were Stage 4 / Stage 8
//! artifacts that no longer exist in the compose pipeline.  Behavior is
//! verified via syllable output and `temp_english_mode`.

use buttre_engine::pipeline::{PipelineExecutor, telex_config};

#[test]
fn test_basic_transform() {
    let config = telex_config();
    let mut executor = PipelineExecutor::new(config);

    let actions1 = executor.process('a');
    println!("After 'a': syllable='{}', raw='{}', actions={:?}",
             executor.syllable(), executor.raw_buffer(), actions1);

    let actions2 = executor.process('a');
    println!("After 'aa': syllable='{}', raw='{}', actions={:?}",
             executor.syllable(), executor.raw_buffer(), actions2);

    assert_eq!(executor.syllable(), "â", "aa should transform to â");
    assert!(!executor.is_temp_english_mode(), "transform should not set temp_english");
}

#[test]
fn test_basic_undo() {
    let config = telex_config();
    let mut executor = PipelineExecutor::new(config);

    executor.process('a');
    executor.process('a');
    println!("After 'aa': syllable='{}', temp_english={}",
             executor.syllable(), executor.is_temp_english_mode());
    assert_eq!(executor.syllable(), "â");

    // Third 'a' → compose detects `aaa` as undo → outputs "aa", temp_english=true.
    let actions3 = executor.process('a');
    println!("After 'aaa': syllable='{}', temp_english={}, actions={:?}",
             executor.syllable(), executor.is_temp_english_mode(), actions3);

    assert_eq!(executor.syllable(), "aa", "aaa should undo to aa");
    assert!(executor.is_temp_english_mode(), "undo should enable temp_english_mode");
}
