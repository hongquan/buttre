use buttre_engine::unicode::normalization::{
    normalize_nfc, normalize_nfd, sanitize_filename, str_eq_normalized,
};

#[test]
fn test_nfc_normalization() {
    // NFD: e + combining acute
    let nfd = "e\u{0301}";
    // NFC: é (single codepoint)
    let nfc = "\u{00e9}";

    assert_eq!(normalize_nfc(nfd), nfc);
    assert_eq!(normalize_nfc(nfc), nfc);
}

#[test]
fn test_string_comparison() {
    let nfd = "café"; // might be NFD on macOS
    let nfc = "café"; // NFC on Windows/Linux

    assert!(str_eq_normalized(nfd, nfc));
}

#[test]
fn test_filename_sanitization() {
    assert_eq!(
        sanitize_filename("file:name*test?.txt"),
        "file_name_test_.txt"
    );

    assert_eq!(
        sanitize_filename("café.txt"),
        "café.txt" // Normalized to NFC
    );
}

// ==================== Additional Comprehensive Tests ====================

// === NFC Normalization Tests ===

#[test]
fn test_nfc_vietnamese_chars() {
    // Test Vietnamese characters with combining marks
    // NFD: a + combining hook above
    let nfd = "a\u{0309}"; // ả in NFD
    let nfc = "\u{1EA3}"; // ả in NFC

    assert_eq!(normalize_nfc(nfd), nfc);
    assert_eq!(normalize_nfc(nfc), nfc);
}

#[test]
fn test_nfc_all_vietnamese_vowels() {
    // Test all Vietnamese base vowels
    let vowels = vec![
        ("ă", "\u{0103}"),
        ("â", "\u{00E2}"),
        ("đ", "\u{0111}"),
        ("ê", "\u{00EA}"),
        ("ô", "\u{00F4}"),
        ("ơ", "\u{01A1}"),
        ("ư", "\u{01B0}"),
    ];

    for (input, expected) in vowels {
        assert_eq!(normalize_nfc(input), expected);
    }
}

#[test]
fn test_nfc_vietnamese_tones() {
    // Test all 5 tones on 'a'
    let tones = vec![
        ("á", "\u{00E1}"), // Acute
        ("à", "\u{00E0}"), // Grave
        ("ả", "\u{1EA3}"), // Hook above
        ("ã", "\u{00E3}"), // Tilde
        ("ạ", "\u{1EA1}"), // Dot below
    ];

    for (input, expected) in tones {
        assert_eq!(normalize_nfc(input), expected);
    }
}

#[test]
fn test_nfc_complex_vietnamese_word() {
    // Test complex Vietnamese word
    let word = "Việt Nam";
    let normalized = normalize_nfc(word);

    // Should remain the same if already NFC
    assert_eq!(normalized, word);
    assert_eq!(normalized.chars().count(), 8); // V i ệ t [space] N a m
}

#[test]
fn test_nfc_empty_string() {
    assert_eq!(normalize_nfc(""), "");
}

#[test]
fn test_nfc_ascii_unchanged() {
    let ascii = "hello world 123";
    assert_eq!(normalize_nfc(ascii), ascii);
}

#[test]
fn test_nfc_mixed_content() {
    let mixed = "Hello café, xin chào!";
    let normalized = normalize_nfc(mixed);

    // Should normalize all characters
    assert!(normalized.contains("café"));
    assert!(normalized.contains("chào"));
}

// === NFD Normalization Tests ===

#[test]
fn test_nfd_normalization() {
    // NFC: é (single codepoint)
    let nfc = "\u{00e9}";
    // NFD: e + combining acute
    let nfd = "e\u{0301}";

    assert_eq!(normalize_nfd(nfc), nfd);
    assert_eq!(normalize_nfd(nfd), nfd);
}

#[test]
fn test_nfd_vietnamese_chars() {
    // NFC: ả (single codepoint)
    let nfc = "\u{1EA3}";
    // NFD: a + combining hook above
    let nfd = "a\u{0309}";

    assert_eq!(normalize_nfd(nfc), nfd);
}

#[test]
fn test_nfd_empty_string() {
    assert_eq!(normalize_nfd(""), "");
}

#[test]
fn test_nfd_ascii_unchanged() {
    let ascii = "hello world";
    assert_eq!(normalize_nfd(ascii), ascii);
}

#[test]
fn test_nfd_complex_vietnamese() {
    let word = "Việt";
    let nfd = normalize_nfd(word);

    // NFD should have more codepoints due to combining marks
    assert!(nfd.len() >= word.len());
}

// === Normalized String Comparison Tests ===

#[test]
fn test_str_eq_normalized_identical() {
    assert!(str_eq_normalized("hello", "hello"));
    assert!(str_eq_normalized("", ""));
    assert!(str_eq_normalized("café", "café"));
}

#[test]
fn test_str_eq_normalized_different() {
    assert!(!str_eq_normalized("hello", "world"));
    assert!(!str_eq_normalized("café", "cafe"));
    assert!(!str_eq_normalized("a", "b"));
}

#[test]
fn test_str_eq_normalized_nfc_vs_nfd() {
    // NFD vs NFC should be equal
    let nfd = "e\u{0301}"; // é in NFD
    let nfc = "\u{00e9}"; // é in NFC

    assert!(str_eq_normalized(nfd, nfc));
    assert!(str_eq_normalized(nfc, nfd));
}

#[test]
fn test_str_eq_normalized_vietnamese() {
    // Test Vietnamese characters with different normalizations
    let word1 = "Việt Nam";
    let word2 = "Việt Nam"; // Could be NFD on macOS

    assert!(str_eq_normalized(word1, word2));
}

#[test]
fn test_str_eq_normalized_empty() {
    assert!(str_eq_normalized("", ""));
    assert!(!str_eq_normalized("", "a"));
    assert!(!str_eq_normalized("a", ""));
}

#[test]
fn test_str_eq_normalized_whitespace() {
    assert!(str_eq_normalized("hello world", "hello world"));
    assert!(!str_eq_normalized("hello world", "helloworld"));
    assert!(!str_eq_normalized("hello  world", "hello world"));
}

// === Filename Sanitization Tests ===

#[test]
fn test_sanitize_all_windows_forbidden() {
    // Test all Windows forbidden characters
    assert_eq!(sanitize_filename("a\\b"), "a_b");
    assert_eq!(sanitize_filename("a/b"), "a_b");
    assert_eq!(sanitize_filename("a:b"), "a_b");
    assert_eq!(sanitize_filename("a*b"), "a_b");
    assert_eq!(sanitize_filename("a?b"), "a_b");
    assert_eq!(sanitize_filename("a\"b"), "a_b");
    assert_eq!(sanitize_filename("a<b"), "a_b");
    assert_eq!(sanitize_filename("a>b"), "a_b");
    assert_eq!(sanitize_filename("a|b"), "a_b");
}

#[test]
fn test_sanitize_multiple_forbidden() {
    assert_eq!(
        sanitize_filename("file://path/to/file*.txt"),
        "file___path_to_file_.txt"
    );
}

#[test]
fn test_sanitize_control_characters() {
    // Test control characters (null, tab, newline, etc.)
    assert_eq!(sanitize_filename("file\0name"), "file_name");
    assert_eq!(sanitize_filename("file\tname"), "file_name");
    assert_eq!(sanitize_filename("file\nname"), "file_name");
    assert_eq!(sanitize_filename("file\rname"), "file_name");
}

#[test]
fn test_sanitize_empty() {
    assert_eq!(sanitize_filename(""), "");
}

#[test]
fn test_sanitize_ascii_safe() {
    let safe = "simple-filename_123.txt";
    assert_eq!(sanitize_filename(safe), safe);
}

#[test]
fn test_sanitize_vietnamese_filename() {
    assert_eq!(
        sanitize_filename("Tài liệu Việt Nam.txt"),
        "Tài liệu Việt Nam.txt"
    );
}

#[test]
fn test_sanitize_vietnamese_with_forbidden() {
    assert_eq!(
        sanitize_filename("Tài/liệu:Việt*Nam.txt"),
        "Tài_liệu_Việt_Nam.txt"
    );
}

#[test]
fn test_sanitize_normalizes_to_nfc() {
    // Input with NFD should be normalized to NFC
    let nfd_input = "cafe\u{0301}.txt"; // café in NFD
    let result = sanitize_filename(nfd_input);

    // Should be NFC
    assert_eq!(result, "café.txt");
}

#[test]
fn test_sanitize_unicode_safe_chars() {
    // Test that safe Unicode characters are preserved
    assert_eq!(sanitize_filename("文件.txt"), "文件.txt");
    assert_eq!(sanitize_filename("файл.txt"), "файл.txt");
    assert_eq!(sanitize_filename("αρχείο.txt"), "αρχείο.txt");
}

#[test]
fn test_sanitize_only_forbidden() {
    // All forbidden characters
    assert_eq!(sanitize_filename("\\/:*?\"<>|"), "_________");
}

// === Vietnamese Character Edge Cases ===

#[test]
fn test_vietnamese_double_diacritics() {
    // Characters with both circumflex/horn and tone
    let chars = vec![
        "ấ", "ầ", "ẩ", "ẫ", "ậ", // â + tones
        "ế", "ề", "ể", "ễ", "ệ", // ê + tones
        "ố", "ồ", "ổ", "ỗ", "ộ", // ô + tones
        "ứ", "ừ", "ử", "ữ", "ự", // ư + tones
        "ớ", "ờ", "ở", "ỡ", "ợ", // ơ + tones
    ];

    for ch in chars {
        let normalized = normalize_nfc(ch);
        assert!(!normalized.is_empty());
        // Should remain as single character after NFC normalization
        assert_eq!(normalized.chars().count(), 1);
    }
}

#[test]
fn test_vietnamese_word_normalization() {
    let words = vec!["thường", "trường", "người", "được", "không", "những"];

    for word in words {
        let nfc = normalize_nfc(word);
        let nfd = normalize_nfd(word);

        // NFC and NFD should be equivalent when compared
        assert!(str_eq_normalized(&nfc, &nfd));
    }
}

#[test]
fn test_roundtrip_nfc_nfd() {
    let original = "Việt Nam";

    // NFC -> NFD -> NFC should equal NFC
    let nfc1 = normalize_nfc(original);
    let nfd = normalize_nfd(&nfc1);
    let nfc2 = normalize_nfc(&nfd);

    assert_eq!(nfc1, nfc2);
}

#[test]
fn test_normalization_idempotent() {
    let text = "café Việt Nam";

    // Normalizing multiple times should give same result
    let nfc1 = normalize_nfc(text);
    let nfc2 = normalize_nfc(&nfc1);
    let nfc3 = normalize_nfc(&nfc2);

    assert_eq!(nfc1, nfc2);
    assert_eq!(nfc2, nfc3);
}

// === Cross-Platform Compatibility Tests ===

#[test]
fn test_macos_filesystem_compat() {
    // macOS uses NFD, but we should normalize to NFC
    let macos_nfd = "cafe\u{0301}"; // café in NFD (macOS)
    let normalized = normalize_nfc(macos_nfd);

    assert_eq!(normalized, "café"); // NFC
}

#[test]
fn test_windows_linux_compat() {
    // Windows/Linux use NFC
    let windows_nfc = "café";
    let normalized = normalize_nfc(windows_nfc);

    assert_eq!(normalized, windows_nfc);
}

#[test]
fn test_cross_platform_comparison() {
    // File from macOS (NFD) vs Windows (NFC)
    let macos_nfd = "cafe\u{0301}"; // café in NFD (macOS)
    let windows_nfc = "café"; // café in NFC (Windows)

    // Both should normalize to same NFC form
    let result1 = normalize_nfc(macos_nfd);
    let result2 = normalize_nfc(windows_nfc);

    // After normalization, should be identical
    assert_eq!(result1, result2);
    assert!(str_eq_normalized(macos_nfd, windows_nfc));
}
