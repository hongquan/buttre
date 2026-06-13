use buttre_engine::pipeline::{PipelineExecutor, PipelineConfig};
use buttre_engine::pipeline::config::ToneMark;

#[test]
fn test_vni_truong() {
    let mut config = PipelineConfig::new("vni");
    
    // VNI transformations
    config.add_transform("a6", "â");
    config.add_transform("a7", "ă");
    config.add_transform("e6", "ê");
    config.add_transform("o6", "ô");
    config.add_transform("o7", "ơ");
    config.add_transform("u7", "ư");
    config.add_transform("dd", "đ");
    
    // VNI tones
    config.add_tone('1', ToneMark::Acute);
    config.add_tone('2', ToneMark::Grave);
    config.add_tone('3', ToneMark::Hook);
    config.add_tone('4', ToneMark::Tilde);
    config.add_tone('5', ToneMark::Dot);
    
    let mut executor = PipelineExecutor::new(config);
    
    // Test: "trường" = tru7o7ng2 in VNI
    // t -> t
    // r -> tr
    // u -> tru
    // 7 -> trư
    // o -> trưo
    // 7 -> trươ
    // n -> trươn
    // g -> trương
    // 2 -> trường
    
    println!("\n=== Testing 'trường' (tru7o7ng2) ===");
    
    for ch in "tru7o7ng2".chars() {
        println!("\nInput: '{}'", ch);
        let actions = executor.process(ch);
        println!("  Actions: {:?}", actions);
        println!("  Syllable: '{}'", executor.syllable());
        println!("  Raw: '{}'", executor.raw_buffer());
    }
    
    let result = executor.syllable();
    println!("\nFinal result: '{}'", result);
    assert_eq!(result, "trường", "Expected 'trường', got '{}'", result);
}

#[test]
fn test_vni_truong_acute() {
    let mut config = PipelineConfig::new("vni");
    
    // VNI transformations
    config.add_transform("a6", "â");
    config.add_transform("a7", "ă");
    config.add_transform("e6", "ê");
    config.add_transform("o6", "ô");
    config.add_transform("o7", "ơ");
    config.add_transform("u7", "ư");
    config.add_transform("dd", "đ");
    
    // VNI tones
    config.add_tone('1', ToneMark::Acute);
    config.add_tone('2', ToneMark::Grave);
    config.add_tone('3', ToneMark::Hook);
    config.add_tone('4', ToneMark::Tilde);
    config.add_tone('5', ToneMark::Dot);
    
    let mut executor = PipelineExecutor::new(config);
    
    // Test: "trưởng" = tru7o7ng3 in VNI
    
    println!("\n=== Testing 'trưởng' (tru7o7ng3) ===");
    
    for ch in "tru7o7ng3".chars() {
        println!("\nInput: '{}'", ch);
        let actions = executor.process(ch);
        println!("  Actions: {:?}", actions);
        println!("  Syllable: '{}'", executor.syllable());
        println!("  Raw: '{}'", executor.raw_buffer());
    }
    
    let result = executor.syllable();
    println!("\nFinal result: '{}'", result);
    assert_eq!(result, "trưởng", "Expected 'trưởng', got '{}'", result);
}

#[test]
fn test_vni_truoang_tilde() {
    let mut config = PipelineConfig::new("vni");
    
    // VNI transformations
    config.add_transform("a6", "â");
    config.add_transform("a7", "ă");
    config.add_transform("e6", "ê");
    config.add_transform("o6", "ô");
    config.add_transform("o7", "ơ");
    config.add_transform("u7", "ư");
    config.add_transform("dd", "đ");
    
    // VNI tones
    config.add_tone('1', ToneMark::Acute);
    config.add_tone('2', ToneMark::Grave);
    config.add_tone('3', ToneMark::Hook);
    config.add_tone('4', ToneMark::Tilde);
    config.add_tone('5', ToneMark::Dot);
    
    let mut executor = PipelineExecutor::new(config);
    
    // Test: "trượng" = tru7o7ng5 in VNI
    
    println!("\n=== Testing 'trượng' (tru7o7ng5) ===");
    
    for ch in "tru7o7ng5".chars() {
        println!("\nInput: '{}'", ch);
        let actions = executor.process(ch);
        println!("  Actions: {:?}", actions);
        println!("  Syllable: '{}'", executor.syllable());
        println!("  Raw: '{}'", executor.raw_buffer());
    }
    
    let result = executor.syllable();
    println!("\nFinal result: '{}'", result);
    assert_eq!(result, "trượng", "Expected 'trượng', got '{}'", result);
}

#[test]
fn test_vni_uow_all_variants() {
    let mut config = PipelineConfig::new("vni");
    
    // VNI transformations
    config.add_transform("a6", "â");
    config.add_transform("a7", "ă");
    config.add_transform("e6", "ê");
    config.add_transform("o6", "ô");
    config.add_transform("o7", "ơ");
    config.add_transform("u7", "ư");
    config.add_transform("dd", "đ");
    
    // VNI tones
    config.add_tone('1', ToneMark::Acute);
    config.add_tone('2', ToneMark::Grave);
    config.add_tone('3', ToneMark::Hook);
    config.add_tone('4', ToneMark::Tilde);
    config.add_tone('5', ToneMark::Dot);
    
    // Test all tones on ươ (just the vowel cluster, no consonants)
    let test_cases = [
        ("u7o7", "ươ"),      // no tone
        ("u7o71", "ướ"),     // acute - tone on ơ
        ("u7o72", "ườ"),     // grave - tone on ơ
        ("u7o73", "ưở"),     // hook - tone on ơ
        ("u7o74", "ưỡ"),     // tilde - tone on ơ
        ("u7o75", "ượ"),     // dot - tone on ơ
    ];
    
    for (input, expected) in test_cases {
        let mut executor = PipelineExecutor::new(config.clone());
        
        println!("\n=== Testing '{}' -> '{}' ===", input, expected);
        
        for ch in input.chars() {
            let actions = executor.process(ch);
            println!("  Input '{}': {:?} -> '{}'", ch, actions, executor.syllable());
        }
        
        let result = executor.syllable();
        assert_eq!(result, expected, "Input '{}': expected '{}', got '{}'", input, expected, result);
    }
}
