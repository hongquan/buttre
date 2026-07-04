//! Debug keyboard processing

use buttre_core::keyboard::KeyboardBuilder;

#[test]
fn debug_huyeenf_no_composition() {
    let mut keyboard = KeyboardBuilder::telex().expect("Failed to create telex keyboard");

    let input = "huyeenf";
    println!("\n=== Processing '{}' (no composition) ===", input);

    for ch in input.chars() {
        let actions = keyboard.process(ch).unwrap();
        println!(
            "Input '{}': buffer='{}', actions={:?}",
            ch,
            keyboard.buffer(),
            actions
        );
    }

    println!("Final buffer: '{}'", keyboard.buffer());
}

#[test]
fn debug_huyeenf_with_composition() {
    // TSF uses composition mode
    let mut keyboard =
        KeyboardBuilder::telex_with_composition(true).expect("Failed to create telex keyboard");

    let input = "huyeenf";
    println!("\n=== Processing '{}' (with composition) ===", input);

    for ch in input.chars() {
        let actions = keyboard.process(ch).unwrap();
        println!(
            "Input '{}': buffer='{}', actions={:?}",
            ch,
            keyboard.buffer(),
            actions
        );
    }

    println!("Final buffer: '{}'", keyboard.buffer());
}

#[test]
fn debug_thuowr_with_composition() {
    // TSF uses composition mode
    let mut keyboard =
        KeyboardBuilder::telex_with_composition(true).expect("Failed to create telex keyboard");

    let input = "thuowr";
    println!("\n=== Processing '{}' (with composition) ===", input);

    for ch in input.chars() {
        let actions = keyboard.process(ch).unwrap();
        println!(
            "Input '{}': buffer='{}', actions={:?}",
            ch,
            keyboard.buffer(),
            actions
        );
    }

    println!("Final buffer: '{}'", keyboard.buffer());
}
