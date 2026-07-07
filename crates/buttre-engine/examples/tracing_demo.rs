//! Tracing Demo - Demonstrates structured logging with the buttre-engine
//!
//! This example shows how to use tracing to observe the pipeline execution.
//!
//! ## Usage
//!
//! Run with different log levels:
//! ```bash
//! # Show all debug logs
//! RUST_LOG=debug cargo run --example tracing_demo
//!
//! # Show only info and warnings
//! RUST_LOG=info cargo run --example tracing_demo
//!
//! # Show only engine logs
//! RUST_LOG=buttre_engine=trace cargo run --example tracing_demo
//!
//! # Show specific stages
//! RUST_LOG=buttre_engine::pipeline::stages::stage4_transform=trace cargo run --example tracing_demo
//! ```

use buttre_engine::pipeline::config::ToneMark;
use buttre_engine::pipeline::{PipelineConfig, PipelineExecutor};

fn main() {
    // Initialize tracing subscriber
    buttre_engine::init_tracing();

    println!("=== buttre Engine Tracing Demo ===\n");
    println!("This demo shows structured logging during pipeline execution.");
    println!("Set RUST_LOG environment variable to control verbosity.\n");

    // Create Telex config
    let mut config = PipelineConfig::new("telex");

    // Add Telex transformations
    config.add_transform("aa", "â");
    config.add_transform("aw", "ă");
    config.add_transform("dd", "đ");
    config.add_transform("ee", "ê");
    config.add_transform("oo", "ô");
    config.add_transform("ow", "ơ");
    config.add_transform("uw", "ư");

    // Add Telex tones
    config.add_tone('s', ToneMark::Acute);
    config.add_tone('f', ToneMark::Grave);
    config.add_tone('r', ToneMark::Hook);
    config.add_tone('x', ToneMark::Tilde);
    config.add_tone('j', ToneMark::Dot);

    let mut executor = PipelineExecutor::new(config);

    // Example 1: Simple transformation (aa → â)
    println!("\n--- Example 1: Transformation (aa → â) ---");
    for ch in ['a', 'a'] {
        println!("Input: '{}'", ch);
        let actions = executor.process(ch);
        println!("Output: {:?}", actions);
        println!("Syllable: '{}'\n", executor.syllable());
    }

    // Example 2: Tone application (thu + s → thú)
    executor.reset();
    println!("\n--- Example 2: Tone Application (thu + s → thú) ---");
    for ch in ['t', 'h', 'u', 's'] {
        println!("Input: '{}'", ch);
        let actions = executor.process(ch);
        println!("Output: {:?}", actions);
        println!("Syllable: '{}'\n", executor.syllable());
    }

    // Example 3: Complex word (việt)
    executor.reset();
    println!("\n--- Example 3: Complex Word (việt) ---");
    for ch in ['v', 'i', 'e', 'e', 't', 's'] {
        println!("Input: '{}'", ch);
        let actions = executor.process(ch);
        println!("Output: {:?}", actions);
        println!("Syllable: '{}'\n", executor.syllable());
    }

    // Example 4: Word with multiple transformations (trường)
    executor.reset();
    println!("\n--- Example 4: Multiple Transforms (trường) ---");
    for ch in ['t', 'r', 'u', 'o', 'w', 'n', 'g', 'f'] {
        println!("Input: '{}'", ch);
        let actions = executor.process(ch);
        println!("Output: {:?}", actions);
        println!("Syllable: '{}'\n", executor.syllable());
    }

    println!("\n=== Demo Complete ===");
    println!("\nTip: Run with RUST_LOG=trace to see detailed pipeline execution logs!");
}
