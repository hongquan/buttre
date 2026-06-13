use buttre_core::keyboard::telex::{special, tones, transforms, vowel_sequences};
use buttre_engine::pipeline::context::TypingContext;
use buttre_engine::pipeline::config::ToneMark;

// ========================================================================
// Special Rules Tests
// ========================================================================

#[test]
fn test_w_after_u_horn() {
    let mut ctx = TypingContext::new();
    ctx.syllable_buffer = "trư".to_string();
    ctx.last_char = Some('w');
    ctx.last_transform_key = Some('w');
    
    let rules = special::get_rules();
    let rule = rules.iter().find(|r| r.name == "telex_w_after_ư").expect("Should find rule");
    assert!(rule.matches(&ctx));
}

#[test]
fn test_oeo_block() {
    let mut ctx = TypingContext::new();
    ctx.syllable_buffer = "oeo".to_string();
    
    let rules = special::get_rules();
    let rule = rules.iter().find(|r| r.name == "telex_oeo_block").expect("Should find rule");
    assert!(rule.matches(&ctx));
}

#[test]
fn test_ua_tone_position() {
    let mut ctx = TypingContext::new();
    ctx.syllable_buffer = "qua".to_string();
    
    let rules = special::get_rules();
    let rule = rules.iter().find(|r| r.name == "telex_ua_tone").expect("Should find rule");
    assert!(rule.matches(&ctx));
    
    rule.execute(&mut ctx);
    assert_eq!(ctx.tone_position, Some(1)); // 'u' at index 1
}

// ========================================================================
// Tones Tests
// ========================================================================

#[test]
fn test_tone_mappings() {
    let map = tones::get_map();
    assert_eq!(map.get(&'s'), Some(&ToneMark::Acute));
    assert_eq!(map.get(&'f'), Some(&ToneMark::Grave));
    assert_eq!(map.get(&'r'), Some(&ToneMark::Hook));
    assert_eq!(map.get(&'x'), Some(&ToneMark::Tilde));
    assert_eq!(map.get(&'j'), Some(&ToneMark::Dot));
}

#[test]
fn test_uppercase_tones() {
    let map = tones::get_map();
    assert_eq!(map.get(&'S'), Some(&ToneMark::Acute));
    assert_eq!(map.get(&'J'), Some(&ToneMark::Dot));
}

// ========================================================================
// Transforms Tests
// ========================================================================

#[test]
fn test_basic_transforms() {
    let rules = transforms::get_rules();
    assert_eq!(rules.get("aa"), Some(&"â".to_string()));
    assert_eq!(rules.get("aw"), Some(&"ă".to_string()));
    assert_eq!(rules.get("dd"), Some(&"đ".to_string()));
}

#[test]
fn test_uppercase_transforms() {
    let rules = transforms::get_rules();
    assert_eq!(rules.get("AA"), Some(&"Â".to_string()));
    assert_eq!(rules.get("DD"), Some(&"Đ".to_string()));
}

// ========================================================================
// Vowel Sequence Tests
// ========================================================================

#[test]
fn test_table_not_empty() {
    let table = vowel_sequences::get_table();
    assert!(!table.is_empty());
    assert!(table.len() >= 30);  // At least 30 sequences
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
    assert_eq!(info.primary_tone_position(), Some(1));  // Prefer 'ơ'
}

#[test]
fn test_find_triple() {
    let table = vowel_sequences::get_table();
    let info = table.find("oai").expect("Should find 'oai'");
    assert_eq!(info.len, 3);
    assert_eq!(info.vowels, vec!['o', 'a', 'i']);
    assert_eq!(info.primary_tone_position(), Some(1));  // Tone on 'a'
}

#[test]
fn test_oa_modern_style() {
    let table = vowel_sequences::get_table();
    let info = table.find("oa").expect("Should find 'oa'");
    // Modern style: tone on 'a' (position 1)
    assert_eq!(info.primary_tone_position(), Some(1));
}
