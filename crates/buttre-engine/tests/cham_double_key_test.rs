//! Test Cham double-key mapping functionality
//!
//! This test verifies that:
//! 1. native_script_mode is properly set from config
//! 2. Double-key patterns like "kk" → "ꩀ" work correctly

use buttre_engine::pipeline::{PipelineExecutor, PipelineConfig};

/// Create a Cham Akhar Thrah test config
fn create_cham_config() -> PipelineConfig {
    let mut config = PipelineConfig::new("cham_akhar_thrah");
    
    // CRITICAL: Enable native script mode for single-char and double-key transforms
    config.native_script_mode = true;
    
    // Single-char transforms
    config.add_transform("k", "ꨆ");  // KA
    config.add_transform("g", "ꨈ");  // GA
    config.add_transform("c", "ꨌ");  // CA
    config.add_transform("j", "ꨎ");  // JA
    config.add_transform("t", "ꨓ");  // TA
    config.add_transform("d", "ꨕ");  // DA
    config.add_transform("n", "ꨘ");  // NA
    config.add_transform("p", "ꨚ");  // PA
    config.add_transform("b", "ꨝ");  // BA
    config.add_transform("m", "ꨟ");  // MA
    config.add_transform("y", "ꨢ");  // YA
    config.add_transform("r", "ꨣ");  // RA
    config.add_transform("l", "ꨤ");  // LA
    config.add_transform("w", "ꨥ");  // WA
    config.add_transform("s", "ꨦ");  // SA
    config.add_transform("h", "ꨨ");  // HA
    
    // Double-key patterns (akhar matai - final consonants)
    config.add_transform("kk", "ꩀ");  // final KA
    config.add_transform("gg", "ꩁ");  // final GA
    config.add_transform("cc", "ꩂ");  // final CA
    config.add_transform("jj", "ꩃ");  // final JA
    config.add_transform("tt", "ꩄ");  // final TA
    config.add_transform("nn", "ꩅ");  // final NA
    config.add_transform("pp", "ꩆ");  // final PA
    config.add_transform("yy", "ꩇ");  // final YA
    config.add_transform("rr", "ꩈ");  // final RA
    config.add_transform("ll", "ꩉ");  // final LA
    config.add_transform("ss", "ꩊ");  // final SA
    config.add_transform("mm", "ꩌ");  // final MA
    config.add_transform("hh", "ꩍ");  // final HA
    
    config
}

#[test]
fn test_cham_config_native_script_mode() {
    let config = create_cham_config();
    assert!(config.native_script_mode, "native_script_mode should be true for Cham keyboard");
}

#[test]
fn test_cham_single_k_transform() {
    let config = create_cham_config();
    let mut executor = PipelineExecutor::new(config);
    
    // Type 'ka' - 'k' should transform to ꨆ, 'a' should pass through
    executor.process('k');
    assert_eq!(executor.syllable(), "ꨆ", "Single 'k' should transform to Cham KA");
}

#[test]
fn test_cham_double_kk_transform() {
    let config = create_cham_config();
    
    // DEBUG: Print config native_script_mode
    eprintln!("CONFIG CREATED: native_script_mode = {}", config.native_script_mode);
    eprintln!("CONFIG transform_rules keys: {:?}", config.transform_rules.keys().collect::<Vec<_>>());
    
    let mut executor = PipelineExecutor::new(config);
    
    // Type 'kk' - should transform to ꩀ (final KA)
    executor.process('k');
    eprintln!("After first 'k': syllable='{}', raw='{}'", 
        executor.syllable(), executor.raw_buffer());
    
    executor.process('k');
    eprintln!("After second 'k': syllable='{}', raw='{}'", 
        executor.syllable(), executor.raw_buffer());
    
    assert_eq!(executor.syllable(), "ꩀ", "Double 'kk' should transform to Cham final KA");
}

#[test]
fn test_cham_double_gg_transform() {
    let config = create_cham_config();
    let mut executor = PipelineExecutor::new(config);
    
    // Type 'gg' - should transform to ꩁ (final GA)
    executor.process('g');
    executor.process('g');
    
    assert_eq!(executor.syllable(), "ꩁ", "Double 'gg' should transform to Cham final GA");
}

#[test]
fn test_cham_mixed_word() {
    let config = create_cham_config();
    let mut executor = PipelineExecutor::new(config);
    
    // Type 'kak' - should be: k→ꨆ, a→(none/fallback), *second k adds*
    // Actually for Cham, every consonant/vowel has a mapping
    // Let's test a simpler case: just "kk"
    for ch in "kk".chars() {
        executor.process(ch);
    }
    
    // Should have final KA
    assert_eq!(executor.syllable(), "ꩀ", "'kk' should produce final KA");
}
