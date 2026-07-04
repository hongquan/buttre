use buttre_engine::unicode::vowel_sequences::{
    char_to_vnlexi, get_tone_position, lookup_vowel_seq, lookup_vowel_seq_str, VnLexiName,
    VSEQ_LIST,
};

// ==================== char_to_vnlexi Tests ====================

#[test]
fn test_char_to_vnlexi_vowels() {
    assert_eq!(char_to_vnlexi('a'), VnLexiName::A);
    assert_eq!(char_to_vnlexi('e'), VnLexiName::E);
    assert_eq!(char_to_vnlexi('i'), VnLexiName::I);
    assert_eq!(char_to_vnlexi('o'), VnLexiName::O);
    assert_eq!(char_to_vnlexi('u'), VnLexiName::U);
    assert_eq!(char_to_vnlexi('y'), VnLexiName::Y);
}

#[test]
fn test_char_to_vnlexi_uppercase() {
    assert_eq!(char_to_vnlexi('A'), VnLexiName::A);
    assert_eq!(char_to_vnlexi('E'), VnLexiName::E);
    assert_eq!(char_to_vnlexi('I'), VnLexiName::I);
}

#[test]
fn test_char_to_vnlexi_consonants() {
    assert_eq!(char_to_vnlexi('b'), VnLexiName::B);
    assert_eq!(char_to_vnlexi('c'), VnLexiName::C);
    assert_eq!(char_to_vnlexi('d'), VnLexiName::D);
    assert_eq!(char_to_vnlexi('g'), VnLexiName::G);
}

#[test]
fn test_char_to_vnlexi_nonvietamese() {
    assert_eq!(char_to_vnlexi('z'), VnLexiName::NonVnChar);
    assert_eq!(char_to_vnlexi('w'), VnLexiName::NonVnChar);
    assert_eq!(char_to_vnlexi('1'), VnLexiName::NonVnChar);
    assert_eq!(char_to_vnlexi(' '), VnLexiName::NonVnChar);
}

// ==================== lookup_vowel_seq Tests ====================

#[test]
fn test_lookup_vowel_seq_single_vowel() {
    let vowels = [VnLexiName::A, VnLexiName::NonVnChar, VnLexiName::NonVnChar];
    let info = lookup_vowel_seq(&vowels, 1).unwrap();
    assert_eq!(info.len, 1);
    assert_eq!(info.vowels[0], VnLexiName::A);
}

#[test]
fn test_lookup_vowel_seq_two_vowels() {
    let vowels = [VnLexiName::A, VnLexiName::I, VnLexiName::NonVnChar];
    let info = lookup_vowel_seq(&vowels, 2).unwrap();
    assert_eq!(info.len, 2);
    assert_eq!(info.vowels[0], VnLexiName::A);
    assert_eq!(info.vowels[1], VnLexiName::I);
}

#[test]
fn test_lookup_vowel_seq_three_vowels() {
    let vowels = [VnLexiName::I, VnLexiName::Er, VnLexiName::U];
    let info = lookup_vowel_seq(&vowels, 3).unwrap();
    assert_eq!(info.len, 3);
}

#[test]
fn test_lookup_vowel_seq_not_found() {
    let vowels = [
        VnLexiName::NonVnChar,
        VnLexiName::NonVnChar,
        VnLexiName::NonVnChar,
    ];
    let info = lookup_vowel_seq(&vowels, 1);
    assert!(info.is_none());
}

#[test]
fn test_lookup_vowel_seq_invalid_length() {
    let vowels = [VnLexiName::A, VnLexiName::NonVnChar, VnLexiName::NonVnChar];
    assert!(lookup_vowel_seq(&vowels, 0).is_none());
    assert!(lookup_vowel_seq(&vowels, 4).is_none());
}

// ==================== lookup_vowel_seq_str Tests ====================

#[test]
fn test_lookup_vowel_seq_str_single() {
    let info = lookup_vowel_seq_str("a").unwrap();
    assert_eq!(info.len, 1);
}

#[test]
fn test_lookup_vowel_seq_str_two() {
    let info = lookup_vowel_seq_str("ai").unwrap();
    assert_eq!(info.len, 2);
}

#[test]
fn test_lookup_vowel_seq_str_three() {
    let info = lookup_vowel_seq_str("ieu").unwrap();
    assert_eq!(info.len, 3);
}

#[test]
fn test_lookup_vowel_seq_str_not_found() {
    assert!(lookup_vowel_seq_str("zz").is_none());
}

#[test]
fn test_lookup_vowel_seq_str_empty() {
    assert!(lookup_vowel_seq_str("").is_none());
}

#[test]
fn test_lookup_vowel_seq_str_too_long() {
    assert!(lookup_vowel_seq_str("abcd").is_none());
}

// ==================== get_tone_position Tests ====================

#[test]
fn test_get_tone_position_single_vowel() {
    let info = lookup_vowel_seq_str("a").unwrap();
    assert_eq!(get_tone_position(info, false), 0);
    assert_eq!(get_tone_position(info, true), 0);
}

#[test]
fn test_get_tone_position_ai() {
    let info = lookup_vowel_seq_str("ai").unwrap();
    // "ai" pattern: tone on first vowel
    assert_eq!(get_tone_position(info, false), 0);
}

#[test]
fn test_get_tone_position_ua_no_final() {
    let info = lookup_vowel_seq_str("ua").unwrap();
    // "ua" without final consonant: tone on first vowel
    assert_eq!(get_tone_position(info, false), 0);
}

#[test]
fn test_get_tone_position_ua_with_final() {
    let info = lookup_vowel_seq_str("ua").unwrap();
    // "ua" with final consonant: tone on second vowel
    assert_eq!(get_tone_position(info, true), 1);
}

#[test]
fn test_get_tone_position_ue_no_final() {
    let info = lookup_vowel_seq_str("ue").unwrap();
    // "ue" without final consonant: tone on first vowel
    assert_eq!(get_tone_position(info, false), 0);
}

#[test]
fn test_get_tone_position_ue_with_final() {
    let info = lookup_vowel_seq_str("ue").unwrap();
    // "ue" with final consonant: tone on second vowel
    assert_eq!(get_tone_position(info, true), 1);
}

#[test]
fn test_get_tone_position_uoi() {
    let info = lookup_vowel_seq_str("uoi").unwrap();
    // "uoi" pattern: tone on second vowel
    assert_eq!(get_tone_position(info, false), 1);
    assert_eq!(get_tone_position(info, true), 1);
}

// ==================== VSEQ_LIST Integrity Tests ====================

#[test]
fn test_vseq_list_length() {
    assert_eq!(VSEQ_LIST.len(), 70);
}

#[test]
fn test_vseq_list_single_vowels() {
    // First 12 entries should be single vowels
    for (i, entry) in VSEQ_LIST.iter().enumerate().take(12) {
        assert_eq!(entry.len, 1, "Entry {} should be single vowel", i);
    }
}

#[test]
fn test_vseq_list_two_vowels() {
    // Find the range of two-vowel entries
    let mut two_vowel_count = 0;
    for entry in VSEQ_LIST.iter() {
        if entry.len == 2 {
            two_vowel_count += 1;
        }
    }
    // Should have 36 two-vowel entries
    assert_eq!(two_vowel_count, 36);
}

#[test]
fn test_vseq_list_three_vowels() {
    // Find the range of three-vowel entries
    let mut three_vowel_count = 0;
    for entry in VSEQ_LIST.iter() {
        if entry.len == 3 {
            three_vowel_count += 1;
        }
    }
    // Should have 22 three-vowel entries
    assert_eq!(three_vowel_count, 22);
}

#[test]
fn test_vseq_list_no_duplicates() {
    // Each vowel combination should be unique
    let mut seen = std::collections::HashSet::new();
    for entry in VSEQ_LIST {
        let key = (
            entry.vowels[0] as u8,
            entry.vowels[1] as u8,
            entry.vowels[2] as u8,
        );
        assert!(
            seen.insert(key),
            "Duplicate vowel sequence found: {:?}",
            key
        );
    }
}
