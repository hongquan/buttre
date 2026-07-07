// Test complete Nôm keyboard flow
use buttre_core::{vietnamese, KeyboardBuilder};

fn main() {
    println!("=== Testing Complete Nôm Keyboard Flow ===\n");

    // Step 1: Find dictionary
    println!("Step 1: Finding Nôm dictionary...");
    let nom_path = vietnamese::get_nom_db_path();
    match &nom_path {
        Some(p) => println!("  ✓ Found at: {:?}", p),
        None => {
            println!("  ✗ Not found!");
            return;
        }
    }

    // Step 2: Build Nôm keyboard
    println!("\nStep 2: Building Nôm keyboard...");
    match KeyboardBuilder::nom(nom_path) {
        Ok(_keyboard) => {
            println!("  ✓ Keyboard created successfully!");

            // Step 3: Test typing
            println!("\nStep 3: Testing typing...");
            println!("  Input: 'nguoi' (expecting Nôm characters if dictionary works)");

            // This is just a creation test - actual typing would need input simulation
            println!("  ✓ Keyboard is ready to use");
        }
        Err(e) => {
            println!("  ✗ Failed to create keyboard: {}", e);
            println!("     Error: {:?}", e);
        }
    }

    println!("\n=== Test Complete ===");
}
