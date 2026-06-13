use buttre_engine::pipeline::PipelineExecutor;
use buttre_engine::pipeline::config::{PipelineConfig, ToneMark};

fn create_telex_config() -> PipelineConfig {
    let mut config = PipelineConfig::new("telex");
    
    // Add transformations
    config.add_transform("aa", "â");
    config.add_transform("aw", "ă");
    config.add_transform("ee", "ê");
    config.add_transform("oo", "ô");
    config.add_transform("ow", "ơ");
    config.add_transform("uw", "ư");
    config.add_transform("dd", "đ");
    
    // Add tones
    config.add_tone('s', ToneMark::Acute);
    config.add_tone('f', ToneMark::Grave);
    config.add_tone('r', ToneMark::Hook);
    config.add_tone('x', ToneMark::Tilde);
    config.add_tone('j', ToneMark::Dot);
    
    config
}

#[test]
fn test_tone_positioning_oa() {
    // Updated per VIETNAMESE_ACCENT.md Priority 3.2.B
    // Test "hoas" -> "hóa" (tone on 'o' in 'oa' without final consonant - CLASSIC style)
    let config = create_telex_config();
    let mut executor = PipelineExecutor::new(config);
    
    // Process "hoas"
    executor.process('h');
    executor.process('o');
    executor.process('a');
    executor.process('s'); // tone key
    
    // Get the current syllable
    let syllable = executor.syllable();
    println!("Input: hoas -> Result: {}", syllable);
    
    assert_eq!(syllable, "hóa", "Expected 'hóa' (CLASSIC style: tone on first vowel), got '{}'", syllable);
}

#[test]
fn test_tone_positioning_oe() {
    // Updated per VIETNAMESE_ACCENT.md Priority 3.2.B
    // Test "hoes" -> "hóe" (tone on 'o' in 'oe' without final consonant - CLASSIC style)
    let config = create_telex_config();
    let mut executor = PipelineExecutor::new(config);
    
    executor.process('h');
    executor.process('o');
    executor.process('e');
    executor.process('s');
    
    let syllable = executor.syllable();
    println!("Input: hoes -> Result: {}", syllable);
    
    assert_eq!(syllable, "hóe", "Expected 'hóe' (CLASSIC style: tone on first vowel), got '{}'", syllable);
}

#[test]
fn test_tone_positioning_ua() {
    // Test "quas" -> "quá" (tone on 'a' in 'ua')
    let config = create_telex_config();
    let mut executor = PipelineExecutor::new(config);
    
    executor.process('q');
    executor.process('u');
    executor.process('a');
    executor.process('s');
    
    let syllable = executor.syllable();
    println!("Input: quas -> Result: {}", syllable);
    
    assert_eq!(syllable, "quá", "Expected 'quá', got '{}'", syllable);
}

#[test]
fn test_tone_positioning_oan() {
    // Updated: Correct input sequence
    // Test "hoanf" -> "hoàn" (tone on 'a' in 'oa' WITH final consonant)
    // When tone is applied AFTER final consonant, tone goes on second vowel
    let config = create_telex_config();
    let mut executor = PipelineExecutor::new(config);
    
    executor.process('h');
    executor.process('o');
    executor.process('a');
    executor.process('n'); // final consonant first
    executor.process('f'); // then grave tone
    
    let syllable = executor.syllable();
    println!("Input: hoanf -> Result: {}", syllable);
    
    assert_eq!(syllable, "hoàn", "Expected 'hoàn' (tone on 'a' because 'n' exists), got '{}'", syllable);
}

