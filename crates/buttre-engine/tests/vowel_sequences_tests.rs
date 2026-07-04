use buttre_engine::vowel::{VowelSeq, VowelSeqInfo, VowelSeqTable};

#[test]
fn test_vowel_seq_info_creation() {
    let info = VowelSeqInfo {
        sequence: "ươ".to_string(),
        len: 2,
        complete: true,
        vowels: vec!['ư', 'ơ'],
        tone_positions: vec![1, 0],
        roof_pos: None,
        hook_pos: None,
        with_roof: Some(VowelSeq::UHO),
        with_hook: None,
    };

    assert_eq!(info.sequence, "ươ");
    assert_eq!(info.len, 2);
    assert!(info.complete);
    assert_eq!(info.vowels, vec!['ư', 'ơ']);
}

#[test]
fn test_can_receive_tone() {
    let info = VowelSeqInfo {
        sequence: "oa".to_string(),
        len: 2,
        complete: true,
        vowels: vec!['o', 'a'],
        tone_positions: vec![0, 1], // Can tone both positions
        roof_pos: None,
        hook_pos: None,
        with_roof: None,
        with_hook: None,
    };

    assert!(info.can_receive_tone(0));
    assert!(info.can_receive_tone(1));
    assert!(!info.can_receive_tone(2));
}

#[test]
fn test_primary_tone_position() {
    let info = VowelSeqInfo {
        sequence: "ươi".to_string(),
        len: 3,
        complete: true,
        vowels: vec!['ư', 'ơ', 'i'],
        tone_positions: vec![1, 0, 2], // Prefer 'ơ'
        roof_pos: None,
        hook_pos: None,
        with_roof: None,
        with_hook: None,
    };

    assert_eq!(info.primary_tone_position(), Some(1));
}

#[test]
fn test_vowel_seq_table_find() {
    let table = VowelSeqTable::new(vec![
        VowelSeqInfo {
            sequence: "a".to_string(),
            len: 1,
            complete: true,
            vowels: vec!['a'],
            tone_positions: vec![0],
            roof_pos: Some(0),
            hook_pos: Some(0),
            with_roof: Some(VowelSeq::AR),
            with_hook: Some(VowelSeq::AB),
        },
        VowelSeqInfo {
            sequence: "ươ".to_string(),
            len: 2,
            complete: true,
            vowels: vec!['ư', 'ơ'],
            tone_positions: vec![1, 0],
            roof_pos: None,
            hook_pos: None,
            with_roof: None,
            with_hook: None,
        },
    ]);

    assert!(table.find("a").is_some());
    assert!(table.find("ươ").is_some());
    assert!(table.find("xyz").is_none());
}

#[test]
fn test_vowel_seq_table_find_by_vowels() {
    let table = VowelSeqTable::new(vec![VowelSeqInfo {
        sequence: "ươ".to_string(),
        len: 2,
        complete: true,
        vowels: vec!['ư', 'ơ'],
        tone_positions: vec![1, 0],
        roof_pos: None,
        hook_pos: None,
        with_roof: None,
        with_hook: None,
    }]);

    assert!(table.find_by_vowels(&['ư', 'ơ']).is_some());
    assert!(table.find_by_vowels(&['a', 'b']).is_none());
}

#[test]
fn test_empty_table() {
    let table = VowelSeqTable::empty();
    assert!(table.is_empty());
    assert_eq!(table.len(), 0);
}
