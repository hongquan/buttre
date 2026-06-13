/// PGO Workload Runner
/// 
/// This program processes the Vietnamese typing corpus to generate
/// profile data for Profile-Guided Optimization (PGO).
/// 
/// Usage:
/// 1. Build with instrumentation: RUSTFLAGS="-Cprofile-generate=/tmp/pgo-data" cargo build --release --example pgo_workload
/// 2. Run workload: ./target/release/examples/pgo_workload
/// 3. Rebuild with profile: RUSTFLAGS="-Cprofile-use=/tmp/pgo-data" cargo build --release

use std::fs;
use buttre_engine::pipeline::PipelineExecutor;
use buttre_engine::pipeline::presets;
use buttre_engine::types::Action;

fn main() {
    println!("Starting PGO workload...");
    
    // Load the corpus
    let corpus = fs::read_to_string("crates/buttre-engine/benches/pgo_workload.txt")
        .or_else(|_| fs::read_to_string("benches/pgo_workload.txt"))
        .expect("Failed to read pgo_workload.txt. Make sure to run from project root or crates/buttre-engine");
    
    let config = presets::vni_config();
    
    // Process the corpus multiple times to ensure hot paths are profiled
    let iterations = 100;
    
    for i in 0..iterations {
        if i % 10 == 0 {
            println!("Iteration {}/{}", i + 1, iterations);
        }
        
        for line in corpus.lines() {
            // Skip comments and empty lines
            if line.trim().is_empty() || line.starts_with('#') {
                continue;
            }
            
            process_line(line, &config);
        }
    }
    
    println!("PGO workload completed successfully!");
    println!("Profile data has been generated.");
}

fn process_line(line: &str, config: &buttre_engine::pipeline::PipelineConfig) {
    let mut executor = PipelineExecutor::new(config.clone());
    let mut output = String::new();
    
    for ch in line.chars() {
        let actions = executor.process(ch);
        
        for action in actions {
            match action {
                Action::Commit(text) => {
                    output.push_str(&text);
                }
                Action::Replace { text, backspace_count } => {
                    for _ in 0..backspace_count {
                        output.pop();
                    }
                    output.push_str(&text);
                }
                _ => {}
            }
        }
    }
    
    // Force usage of output to prevent dead code elimination
    if output.len() > 1000000 {
        println!("Unexpected output length: {}", output.len());
    }
}
