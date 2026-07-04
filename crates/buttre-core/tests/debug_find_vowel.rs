//! Debug find_main_vowel logic

use buttre_core::keyboard::telex;
use buttre_engine::pipeline::presets::telex_config;
use buttre_engine::pipeline::PipelineExecutor;

#[test]
fn debug_huyeenf_with_presets_telex() {
    let config = telex_config();
    let mut executor = PipelineExecutor::new(config);

    let input = "huyeenf";
    println!(
        "\n=== Processing '{}' with presets::telex_config() ===",
        input
    );

    for ch in input.chars() {
        let actions = executor.process(ch);
        println!(
            "Input '{}': syllable='{}', actions={:?}",
            ch,
            executor.syllable(),
            actions
        );
    }

    println!("Final syllable: '{}'", executor.syllable());
    assert_eq!(
        executor.syllable(),
        "huyền",
        "Should be 'huyền' not 'huỳên'"
    );
}

#[test]
fn debug_huyeenf_with_core_telex() {
    // This is what buttre-core Keyboard uses
    let config = telex::build_config();
    let mut executor = PipelineExecutor::new(config);

    let input = "huyeenf";
    println!(
        "\n=== Processing '{}' with telex::build_config() ===",
        input
    );

    for ch in input.chars() {
        let actions = executor.process(ch);
        println!(
            "Input '{}': syllable='{}', actions={:?}",
            ch,
            executor.syllable(),
            actions
        );
    }

    println!("Final syllable: '{}'", executor.syllable());
    assert_eq!(
        executor.syllable(),
        "huyền",
        "Should be 'huyền' not 'huỳên'"
    );
}

#[test]
fn debug_thuowr_with_core_telex() {
    // This is what buttre-core Keyboard uses
    let config = telex::build_config();
    let mut executor = PipelineExecutor::new(config);

    let input = "thuowr";
    println!(
        "\n=== Processing '{}' with telex::build_config() ===",
        input
    );

    for ch in input.chars() {
        let actions = executor.process(ch);
        println!(
            "Input '{}': syllable='{}', actions={:?}",
            ch,
            executor.syllable(),
            actions
        );
    }

    println!("Final syllable: '{}'", executor.syllable());
    assert_eq!(executor.syllable(), "thuở", "Should be 'thuở' not 'thửơ'");
}

#[test]
fn debug_huyeenf_with_composition() {
    // This is exactly what TSF uses
    let mut config = telex::build_config();
    config.pipeline.use_composition = true;

    let mut executor = PipelineExecutor::new(config);

    let input = "huyeenf";
    println!(
        "\n=== Processing '{}' with telex::build_config() + composition ===",
        input
    );

    for ch in input.chars() {
        let actions = executor.process(ch);
        println!(
            "Input '{}': syllable='{}', actions={:?}",
            ch,
            executor.syllable(),
            actions
        );
    }

    println!("Final syllable: '{}'", executor.syllable());
    assert_eq!(
        executor.syllable(),
        "huyền",
        "Should be 'huyền' not 'huỳên'"
    );
}

#[test]
fn debug_thuowr_with_composition() {
    // This is exactly what TSF uses
    let mut config = telex::build_config();
    config.pipeline.use_composition = true;

    let mut executor = PipelineExecutor::new(config);

    let input = "thuowr";
    println!(
        "\n=== Processing '{}' with telex::build_config() + composition ===",
        input
    );

    for ch in input.chars() {
        let actions = executor.process(ch);
        println!(
            "Input '{}': syllable='{}', actions={:?}",
            ch,
            executor.syllable(),
            actions
        );
    }

    println!("Final syllable: '{}'", executor.syllable());
    assert_eq!(executor.syllable(), "thuở", "Should be 'thuở' not 'thửơ'");
}
