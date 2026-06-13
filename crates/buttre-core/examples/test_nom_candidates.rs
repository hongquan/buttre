// Test Nôm keyboard with candidates
use buttre_core::{KeyboardBuilder, vietnamese, Action};

fn main() {
    println!("=== Testing Nôm Keyboard with Candidates ===\n");
    
    // Step 1: Find dictionary
    println!("Step 1: Finding Nôm dictionary...");
    let nom_path = vietnamese::get_nom_db_path();
    match &nom_path {
        Some(p) => println!("  ✓ Found at: {:?}", p),
        None => {
            println!("  ✗ Not found! Make sure buttre_nom.db exists.");
            return;
        }
    }
    
    // Step 2: Build Nôm keyboard
    println!("\nStep 2: Building Nôm keyboard...");
    let mut keyboard = match KeyboardBuilder::nom(nom_path) {
        Ok(kb) => {
            println!("  ✓ Keyboard created successfully!");
            kb
        }
        Err(e) => {
            println!("  ✗ Failed: {}", e);
            return;
        }
    };
    
    // Step 3: Test typing "troi" to see if we get candidates
    println!("\nStep 3: Testing typing 'troi'...");
    
    for ch in "troi".chars() {
        let actions = keyboard.process(ch).expect("Process failed");
        println!("  Input '{}' -> {} action(s)", ch, actions.len());
        
        for (i, action) in actions.iter().enumerate() {
            match action {
                Action::ShowCandidates { candidates, input } => {
                    println!("    [{}] ShowCandidates for '{}': {} candidates", i, input, candidates.len());
                    for (j, candidate) in candidates.iter().take(5).enumerate() {
                        println!("      {}: {} (score: {})", j + 1, candidate.text, candidate.score);
                    }
                }
                Action::Replace { backspace_count, text } => {
                    println!("    [{}] Replace: backspace={}, text='{}'", i, backspace_count, text);
                }
                Action::Commit(text) => {
                    println!("    [{}] Commit: '{}'", i, text);
                }
                Action::HideCandidates => {
                    println!("    [{}] HideCandidates", i);
                }
                Action::DoNothing => {
                    println!("    [{}] DoNothing", i);
                }
                _ => {
                    println!("    [{}] {:?}", i, action);
                }
            }
        }
    }
    
    println!("\n=== Test Complete ===");
    println!("\nNote: If you see ShowCandidates with multiple Nôm characters,");
    println!("the candidate UI system is working correctly!");
}
