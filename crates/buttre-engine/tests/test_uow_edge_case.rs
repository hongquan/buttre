//! Test for ươ tone positioning edge case (Task 1.3)
//!
//! Rule:
//! - ươ with final consonant: tone on ơ (e.g., "trường")
//! - ươ without final consonant: tone on ư (e.g., "cười")

use buttre_engine::pipeline::{PipelineConfig, PipelineExecutor};

fn create_telex_config() -> PipelineConfig {
    buttre_engine::pipeline::telex_config()
}

#[test]
fn test_uow_with_final_consonant() {
    // "truwowngf" → "trường"
    // ươ + ng (final consonant) → tone on ơ
    let config = create_telex_config();
    let mut executor = PipelineExecutor::new(config);

    for ch in "truwowngf".chars() {
        executor.process(ch);
    }

    assert_eq!(
        executor.context().syllable_buffer,
        "trường",
        "Expected 'trường' but got '{}'",
        executor.context().syllable_buffer
    );
}

#[test]
fn test_uow_without_final_consonant() {
    // "cuwowif" → "cười"
    // ươ without final consonant → tone on ư
    let config = create_telex_config();
    let mut executor = PipelineExecutor::new(config);

    for ch in "cuwowif".chars() {
        executor.process(ch);
    }

    assert_eq!(
        executor.context().syllable_buffer,
        "cười",
        "Expected 'cười' but got '{}'",
        executor.context().syllable_buffer
    );
}

#[test]
fn test_giuowng_with_final_consonant() {
    // "giuwowngf" → "giường"
    // gi + ươ + ng → tone on ơ
    let config = create_telex_config();
    let mut executor = PipelineExecutor::new(config);

    for ch in "giuwowngf".chars() {
        executor.process(ch);
    }

    assert_eq!(
        executor.context().syllable_buffer,
        "giường",
        "Expected 'giường' but got '{}'",
        executor.context().syllable_buffer
    );
}

#[test]
fn test_uow_variations() {
    let config = create_telex_config();

    // With final consonant
    let test_cases_with_consonant = vec![
        ("thuwowngf", "thường"),
        ("luwowngj", "lượng"),
        ("cuwowngf", "cường"),
    ];

    for (input, expected) in test_cases_with_consonant {
        let mut executor = PipelineExecutor::new(config.clone());
        for ch in input.chars() {
            executor.process(ch);
        }
        assert_eq!(
            executor.context().syllable_buffer,
            expected,
            "Input '{}': expected '{}' but got '{}'",
            input,
            expected,
            executor.context().syllable_buffer
        );
    }

    // Without final consonant - tone follows triple vowel rule (on middle vowel)
    let test_cases_without_consonant = vec![
        ("cuwowif", "cười"), // ươi + huyền → cười
        ("tuwowis", "tưới"), // ươi + sắc → tưới
    ];

    for (input, expected) in test_cases_without_consonant {
        let mut executor = PipelineExecutor::new(config.clone());
        for ch in input.chars() {
            executor.process(ch);
        }
        assert_eq!(
            executor.context().syllable_buffer,
            expected,
            "Input '{}': expected '{}' but got '{}'",
            input,
            expected,
            executor.context().syllable_buffer
        );
    }
}
