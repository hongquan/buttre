use buttre_engine::pipeline::{PipelineExecutor, PipelineConfig};

/// Helper to simulate typing and get final result
fn type_word(input: &str, config: &PipelineConfig) -> String {
    let mut executor = PipelineExecutor::new(config.clone());
    
    for ch in input.chars() {
        executor.process(ch);
    }
    
    executor.context().syllable_buffer.clone()
}

#[test]
fn test_tone_positioning_fixes() {
    println!("\nTesting tone positioning fixes:");
    println!("================================");
    
    let config = buttre_engine::pipeline::telex_config();
    
    let test_cases = vec![
        ("mowis", "mới"),
        ("lais", "lái"),      // s = acute tone → lái (not lại which would be laij)
        ("hoaif", "hoài"),
        ("thoaix", "thoãi"),  // x = tilde (ngã) tone → thoãi (not thoải which would be thoair)
        ("thuoor", "thuổ"),   // oo → ô, r = hook → thuổ (not thuở which would be thuowr)
        ("huowu", "hươu"),    // compose: uow segment → ươ compound (correct; old engine gave huơu as artifact of incremental ordering)
        ("luw", "lư"),        // uw → ư (simpler case)
        ("moww", "mow"),
    ];
    
    for (input, expected) in test_cases {
        let result = type_word(input, &config);
        
        let status = if result == expected { "✓" } else { "✗" };
        println!("{} Input: {:10} => Expected: {:10} Got: {}", 
                 status, input, expected, result);
        
        assert_eq!(result, expected, "Failed for input: {}", input);
    }
}

#[test]
fn test_flexible_typing() {
    println!("\nTesting flexible typing:");
    println!("========================");
    
    // Flexible typing requires permutation to be enabled
    let mut config = buttre_engine::pipeline::telex_config();
    config.tone.allow_permutation = true;
    
    // These test flexible typing where marks are typed out of normal order
    // Note: Some edge cases produce different results based on when w is typed
    let test_cases = vec![
        ("truongwf", "trường"),  // w after 'ong' triggers compound uo→ươ + f tone
        ("truowngf", "trường"),  // w after 'uo' triggers compound uo→ươ + f tone
        // ("trwowngf", "trường"),  // TODO: w before 'o' is an edge case - currently produces trwờng
    ];
    
    for (input, expected) in test_cases {
        let result = type_word(input, &config);
        
        let status = if result == expected { "✓" } else { "✗" };
        println!("{} Input: {:10} => Expected: {:10} Got: {}", 
                 status, input, expected, result);
        
        assert_eq!(result, expected, "Failed for input: {}", input);
    }
}
