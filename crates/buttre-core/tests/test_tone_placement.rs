//! Test tone placement using buttre-core Keyboard (same as TSF uses)
//! Bug 1: huyeenf -> huỳên (should be huyền)
//! Bug 2: thuowr -> thửơ (should be thuở)

use buttre_core::keyboard::KeyboardBuilder;

fn test_telex_keyboard(input: &str) -> String {
    let mut keyboard = KeyboardBuilder::telex().expect("Failed to create telex keyboard");

    for ch in input.chars() {
        let _ = keyboard.process(ch);
    }

    keyboard.buffer().to_string()
}

#[test]
fn test_huyeenf_should_be_huyen() {
    // huyeenf: h-u-y-e-e-n-f
    // ee -> ê, so huyên + f (grave tone)
    // Expected: huyền (tone on ê because ê is super vowel)
    let result = test_telex_keyboard("huyeenf");
    println!("huyeenf -> '{}'", result);
    assert_eq!(result, "huyền", "Tone should be on ê (super vowel), not y");
}

#[test]
fn test_thuowr_should_be_thuo() {
    // thuowr: th-u-o-w-r
    // ow -> ơ, then uo + w makes ươ
    // Expected: thuở (tone on ơ because ơ is super vowel)
    let result = test_telex_keyboard("thuowr");
    println!("thuowr -> '{}'", result);
    assert_eq!(result, "thuở", "Tone should be on ơ (super vowel), not ư");
}

#[test]
fn test_simple_ee_tone() {
    // heenf: ee -> ê, f is grave
    let result = test_telex_keyboard("heenf");
    println!("heenf -> '{}'", result);
    assert_eq!(result, "hền", "Tone should be on ê");
}

#[test]
fn test_simple_ow_tone() {
    // thowr: ow -> ơ, r is hook
    let result = test_telex_keyboard("thowr");
    println!("thowr -> '{}'", result);
    assert_eq!(result, "thở", "Tone should be on ơ");
}

#[test]
fn test_uyen_with_tone() {
    // uyeenf: uy + ee -> uyê + f
    let result = test_telex_keyboard("uyeenf");
    println!("uyeenf -> '{}'", result);
    assert_eq!(result, "uyền", "Tone should be on ê (super vowel)");
}

#[test]
fn test_nguoif_should_be_nguoi_grave() {
    // nguoif: ng-u-o-i-f
    // Vowels: u-o-i (3 vowels)
    // By VIETNAMESE_ACCENT.md Priority 2: With 3 vowels, tone goes on MIDDLE vowel
    // Middle vowel is 'o', so result should have tone on 'o' → 'ò'
    let result = test_telex_keyboard("nguoif");
    println!("nguoif -> '{}'", result);
    // Priority 2: 3 vowels → middle vowel gets tone → nguòi
    assert_eq!(
        result, "nguòi",
        "Tone should be on middle vowel 'o' (Priority 2)"
    );
}
