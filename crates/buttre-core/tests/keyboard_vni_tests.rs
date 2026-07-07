use buttre_core::keyboard::vni::{special, tones, transforms, vowel_sequences};
use buttre_engine::pipeline::config::ToneMark;
use buttre_engine::pipeline::context::TypingContext;

// ========================================================================
// Special Rules Tests
// ========================================================================

#[test]
fn test_double_char_block() {
    let mut ctx = TypingContext::new();
    ctx.syllable_buffer = "â".to_string();
    ctx.last_char = Some('6');
    ctx.last_transform_key = Some('6');

    let rule = special::double_char_block();

    // Should block second '6'
    assert!(rule.matches(&ctx));
}

#[test]
fn test_double_char_no_block_different_digit() {
    let mut ctx = TypingContext::new();
    ctx.syllable_buffer = "â".to_string();
    ctx.last_char = Some('7');
    ctx.last_transform_key = Some('6'); // Different digit

    let rule = special::double_char_block();

    // Should NOT block different digit
    assert!(!rule.matches(&ctx));
}

// ========================================================================
// Tones Tests
// ========================================================================

#[test]
fn test_tone_mappings() {
    let map = tones::get_map();
    assert_eq!(map.get(&'1'), Some(&ToneMark::Acute));
    assert_eq!(map.get(&'2'), Some(&ToneMark::Grave));
    assert_eq!(map.get(&'3'), Some(&ToneMark::Hook));
    assert_eq!(map.get(&'4'), Some(&ToneMark::Tilde));
    assert_eq!(map.get(&'5'), Some(&ToneMark::Dot));
    assert_eq!(map.get(&'0'), Some(&ToneMark::None));
}

// ========================================================================
// Transforms Tests
// ========================================================================

#[test]
fn test_basic_transforms() {
    let rules = transforms::get_rules();
    assert_eq!(rules.get("a6"), Some(&"â".to_string()));
    assert_eq!(rules.get("a8"), Some(&"ă".to_string()));
    assert_eq!(rules.get("d9"), Some(&"đ".to_string()));
}

#[test]
fn test_uppercase_transforms() {
    let rules = transforms::get_rules();
    assert_eq!(rules.get("A6"), Some(&"Â".to_string()));
    assert_eq!(rules.get("D9"), Some(&"Đ".to_string()));
}

// ========================================================================
// Vowel Sequence Tests
// ========================================================================

#[test]
fn test_table_not_empty() {
    let table = vowel_sequences::get_table();
    assert!(!table.is_empty());
    assert!(table.len() >= 30); // At least 30 sequences
}

#[test]
fn test_find_single_vowel() {
    let table = vowel_sequences::get_table();
    let info = table.find("a").expect("Should find 'a'");
    assert_eq!(info.len, 1);
    assert_eq!(info.vowels, vec!['a']);
    assert!(info.can_receive_roof());
    assert!(info.can_receive_hook());
}

#[test]
fn test_find_compound_uo() {
    let table = vowel_sequences::get_table();
    let info = table.find("ươ").expect("Should find 'ươ'");
    assert_eq!(info.len, 2);
    assert_eq!(info.vowels, vec!['ư', 'ơ']);
    assert_eq!(info.primary_tone_position(), Some(1)); // Prefer 'ơ'
}

#[test]
fn test_find_triple() {
    let table = vowel_sequences::get_table();
    let info = table.find("oai").expect("Should find 'oai'");
    assert_eq!(info.len, 3);
    assert_eq!(info.vowels, vec!['o', 'a', 'i']);
    assert_eq!(info.primary_tone_position(), Some(1)); // Tone on 'a'
}

#[test]
fn test_oa_modern_style() {
    let table = vowel_sequences::get_table();
    let info = table.find("oa").expect("Should find 'oa'");
    // Modern style: tone on 'a' (position 1)
    assert_eq!(info.primary_tone_position(), Some(1));
}
