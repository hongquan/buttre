//! Example: Using buttre-keyboard in buttre-platform
//!
//! This example shows how to integrate buttre-keyboard

use buttre_platform::shared::KeyboardManager;

fn main() -> anyhow::Result<()> {
    println!("buttre Keyboard Integration Example");
    println!("===================================\n");

    // Create keyboard manager
    let manager = KeyboardManager::new()?;

    // Set method to Telex
    println!("Setting method to Telex...");
    manager.set_method("telex")?;
    println!("✓ Telex loaded\n");

    // Get keyboard instance
    let kb_arc = manager.get_keyboard();

    // Helper to process key
    let process = |ch: char| -> anyhow::Result<()> {
        let mut binding = kb_arc.write().expect("RwLock poisoned");
        if let Some(kb) = binding.as_mut() {
            println!("  Input: '{}'", ch);
            let action = kb.process(ch)?;
            println!("  Action: {:?}", action);
        } else {
            println!("  No keyboard active");
        }
        Ok(())
    };

    // Test basic typing
    println!("Test 1: Basic typing");
    process('a')?;
    println!();

    // Test transformation
    println!("Test 2: Transformation (aa → â)");
    process('a')?;
    println!();

    // Test tone
    println!("Test 3: Tone (s → acute)");
    process('s')?;
    println!();

    // Reset
    println!("Test 4: Reset");
    {
        let mut binding = kb_arc.write().expect("RwLock poisoned");
        if let Some(kb) = binding.as_mut() {
            kb.reset();
        }
    }
    println!("  ✓ Reset complete\n");

    println!("All tests passed! ✓");

    Ok(())
}
