use buttre_engine::pipeline::config::ToneMark;
use buttre_engine::pipeline::{PipelineConfig, PipelineExecutor};

fn main() {
    // Create VNI config manually
    let mut config = PipelineConfig::new("vni");

    // Add VNI transformations
    config.add_transform("a6", "â");
    config.add_transform("a7", "ă");
    config.add_transform("e6", "ê");
    config.add_transform("o6", "ô");
    config.add_transform("o7", "ơ");
    config.add_transform("u7", "ư");
    config.add_transform("dd", "đ");

    // Add VNI tones
    config.add_tone('1', ToneMark::Acute);
    config.add_tone('2', ToneMark::Grave);
    config.add_tone('3', ToneMark::Hook);
    config.add_tone('4', ToneMark::Tilde);
    config.add_tone('5', ToneMark::Dot);

    println!("=== Config Debug ===");
    println!("Transform rules: {:?}", config.transform_rules);
    println!("Tone mappings: {:?}", config.tone_map);

    let mut executor = PipelineExecutor::new(config);

    println!("\n=== VNI Test ===");

    // Test 'a'
    println!("\nProcessing 'a'...");
    let actions = executor.process('a');
    println!("  Actions: {:?}", actions);
    println!("  syllable: '{}'", executor.syllable());
    println!("  raw: '{}'", executor.raw_buffer());

    // Test '6'
    println!("\nProcessing '6'...");
    let actions = executor.process('6');
    println!("  Actions: {:?}", actions);
    println!("  syllable: '{}'", executor.syllable());
    println!("  raw: '{}'", executor.raw_buffer());

    // Reset and test VNI tones
    executor.reset();
    println!("\n=== VNI Tones Test (a1 -> á) ===");

    println!("\nProcessing 'a'...");
    let actions = executor.process('a');
    println!("  Actions: {:?}", actions);
    println!("  syllable: '{}'", executor.syllable());

    println!("\nProcessing '1' (acute tone)...");
    let actions = executor.process('1');
    println!("  Actions: {:?}", actions);
    println!("  syllable: '{}'", executor.syllable());

    // Test full word: "Việt" = vie65t1 in VNI
    executor.reset();
    println!("\n=== Full word test: 'Viet' with tones ===");

    for ch in ['v', 'i', 'e', '6', 't', '1'] {
        println!("\nProcessing '{}'...", ch);
        let actions = executor.process(ch);
        println!("  Actions: {:?}", actions);
        println!("  syllable: '{}'", executor.syllable());
    }
}
