//! Test to verify tone table optimization is working

use buttre_engine::pipeline::presets;
use buttre_engine::pipeline::PipelineExecutor;

#[test]
fn test_tone_table_is_used() {
    // This test verifies that tone marks work correctly with optimized table
    let config = presets::telex_config();
    let mut executor = PipelineExecutor::new(config);
    
    // Test basic tone application
    let mut result = String::new();
    
    // Type "as" → "á"
    for ch in "as".chars() {
        let actions = executor.process(ch);
        for action in actions {
            if let buttre_engine::types::Action::Commit(text) = action {
                result.push_str(&text);
            } else if let buttre_engine::types::Action::Replace { text, backspace_count } = action {
                for _ in 0..backspace_count {
                    result.pop();
                }
                result.push_str(&text);
            }
        }
    }
    
    assert_eq!(result, "á", "Tone table should produce correct result");
}

#[test]
fn test_all_vietnamese_vowels_with_tones() {
    let config = presets::vni_config();
    let mut executor = PipelineExecutor::new(config);
    
    let test_cases = vec![
        ("a1", "á"), // a + acute
        ("a2", "à"), // a + grave  
        ("a3", "ả"), // a + hook
        ("a4", "ã"), // a + tilde
        ("a5", "ạ"), // a + dot
        ("e1", "é"),
        ("o1", "ó"),
        ("u1", "ú"),
    ];
    
    for (input, expected) in test_cases {
        executor.reset();
        let mut result = String::new();
        
        for ch in input.chars() {
            let actions = executor.process(ch);
            for action in actions {
                if let buttre_engine::types::Action::Commit(text) = action {
                    result.push_str(&text);
                } else if let buttre_engine::types::Action::Replace { text, backspace_count } = action {
                    for _ in 0..backspace_count {
                        result.pop();
                    }
                    result.push_str(&text);
                }
            }
        }
        
        assert_eq!(result, expected, "Failed for input: {}", input);
    }
}

#[test]
fn test_tone_on_transformed_vowels() {
    let config = presets::vni_config();
    let mut executor = PipelineExecutor::new(config);
    
    // Test tone on ă (a8 + tone)
    let test_cases = vec![
        ("a81", "ắ"), // ă + acute
        ("a82", "ằ"), // ă + grave
        ("e61", "ế"), // ê + acute
        ("o71", "ớ"), // ơ + acute
    ];
    
    for (input, expected) in test_cases {
        executor.reset();
        let mut result = String::new();
        
        for ch in input.chars() {
            let actions = executor.process(ch);
            for action in actions {
                if let buttre_engine::types::Action::Commit(text) = action {
                    result.push_str(&text);
                } else if let buttre_engine::types::Action::Replace { text, backspace_count } = action {
                    for _ in 0..backspace_count {
                        result.pop();
                    }
                    result.push_str(&text);
                }
            }
        }
        
        assert_eq!(result, expected, "Failed for input: {}", input);
    }
}
