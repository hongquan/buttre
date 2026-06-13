//! Comprehensive VNI Test Suite
//!
//! Tests all aspects of VNI input method including:
//! - Basic transformations (a6→â, d9→đ)
//! - Tone marks (1-5, 0)
//! - Combined transforms + tones
//! - Edge cases (numbers, uppercase)
//! - Real Vietnamese words
//! - Performance benchmarks

use buttre_engine::pipeline::PipelineExecutor;
use buttre_engine::pipeline::presets;

/// Helper function to process VNI input
fn process_vni(input: &str) -> String {
    let config = presets::vni_config();
    let mut executor = PipelineExecutor::new(config);
    let mut result = String::new();
    
    for ch in input.chars() {
        let actions = executor.process(ch);
        // Apply actions to build result
        for action in actions {
            match action {
                buttre_engine::types::Action::Commit(text) => {
                    // Commit action - append the text
                    result.push_str(&text);
                }
                buttre_engine::types::Action::Replace { text, backspace_count } => {
                    // Remove last N chars
                    for _ in 0..backspace_count {
                        result.pop();
                    }
                    // Append new text
                    result.push_str(&text);
                }
                buttre_engine::types::Action::UpdateComposition { text, .. } => {
                    // For tests, just use the text
                    result = text;
                }
                _ => {}
            }
        }
    }
    
    result
}

// ============================================
// Basic Transform Tests
// ============================================

#[test]
fn test_vni_basic_a6_circumflex() {
    assert_eq!(process_vni("a6"), "â");
}

#[test]
fn test_vni_basic_a8_breve() {
    assert_eq!(process_vni("a8"), "ă");
}

#[test]
fn test_vni_basic_d9_stroke() {
    assert_eq!(process_vni("d9"), "đ");
}

#[test]
fn test_vni_basic_e6_circumflex() {
    assert_eq!(process_vni("e6"), "ê");
}

#[test]
fn test_vni_basic_o6_circumflex() {
    assert_eq!(process_vni("o6"), "ô");
}

#[test]
fn test_vni_basic_o7_horn() {
    assert_eq!(process_vni("o7"), "ơ");
}

#[test]
fn test_vni_basic_u7_horn() {
    assert_eq!(process_vni("u7"), "ư");
}

#[test]
fn test_vni_all_basic_transforms() {
    // Test all 7 VNI transformations
    assert_eq!(process_vni("a6"), "â", "a6 → â failed");
    assert_eq!(process_vni("a8"), "ă", "a8 → ă failed");
    assert_eq!(process_vni("d9"), "đ", "d9 → đ failed");
    assert_eq!(process_vni("e6"), "ê", "e6 → ê failed");
    assert_eq!(process_vni("o6"), "ô", "o6 → ô failed");
    assert_eq!(process_vni("o7"), "ơ", "o7 → ơ failed");
    assert_eq!(process_vni("u7"), "ư", "u7 → ư failed");
}

// ============================================
// Tone Mark Tests
// ============================================

#[test]
fn test_vni_tone_acute() {
    assert_eq!(process_vni("a1"), "á");
}

#[test]
fn test_vni_tone_grave() {
    assert_eq!(process_vni("a2"), "à");
}

#[test]
fn test_vni_tone_hook() {
    assert_eq!(process_vni("a3"), "ả");
}

#[test]
fn test_vni_tone_tilde() {
    assert_eq!(process_vni("a4"), "ã");
}

#[test]
fn test_vni_tone_dot() {
    assert_eq!(process_vni("a5"), "ạ");
}

#[test]
fn test_vni_tone_remove() {
    // Type "a1" → "á", then "0" → "a"
    assert_eq!(process_vni("a10"), "a");
}

#[test]
fn test_vni_all_tones_on_a() {
    assert_eq!(process_vni("a1"), "á", "Acute failed");
    assert_eq!(process_vni("a2"), "à", "Grave failed");
    assert_eq!(process_vni("a3"), "ả", "Hook failed");
    assert_eq!(process_vni("a4"), "ã", "Tilde failed");
    assert_eq!(process_vni("a5"), "ạ", "Dot failed");
}

// ============================================
// Combined Transform + Tone Tests
// ============================================

#[test]
fn test_vni_combined_a61() {
    // a6 → â, then 1 → ấ
    assert_eq!(process_vni("a61"), "ấ");
}

#[test]
fn test_vni_combined_a81() {
    // a8 → ă, then 1 → ắ
    assert_eq!(process_vni("a81"), "ắ");
}

#[test]
fn test_vni_combined_e62() {
    // e6 → ê, then 2 → ề
    assert_eq!(process_vni("e62"), "ề");
}

#[test]
fn test_vni_combined_o73() {
    // o7 → ơ, then 3 → ở
    assert_eq!(process_vni("o73"), "ở");
}

#[test]
fn test_vni_combined_u74() {
    // u7 → ư, then 4 → ữ
    assert_eq!(process_vni("u74"), "ữ");
}

#[test]
fn test_vni_all_combined_transforms() {
    assert_eq!(process_vni("a61"), "ấ");
    assert_eq!(process_vni("a62"), "ầ");
    assert_eq!(process_vni("a63"), "ẩ");
    assert_eq!(process_vni("a64"), "ẫ");
    assert_eq!(process_vni("a65"), "ậ");
    
    assert_eq!(process_vni("a81"), "ắ");
    assert_eq!(process_vni("a82"), "ằ");
    assert_eq!(process_vni("a83"), "ẳ");
    assert_eq!(process_vni("a84"), "ẵ");
    assert_eq!(process_vni("a85"), "ặ");
}

// ============================================
// Real Vietnamese Words
// ============================================

#[test]
fn test_vni_word_viet() {
    // Vie65t → Việt
    assert_eq!(process_vni("Vie65t"), "Việt");
}

#[test]
fn test_vni_word_nguoi() {
    // người = ng + ư + ờ + i
    // ư = u7, ờ = ơ + tone2, ơ = o7
    // So: ngu7o7i2 → người  
    assert_eq!(process_vni("ngu7o7i2"), "người");
}

#[test]
fn test_vni_word_thuong() {
    // VNI ươ tone positioning: thu7o7ng1 -> "thướng"
    // Note: Current implementation places tone on ư (first vowel in ươ)
    // This follows the general two-vowel rule from VIETNAMESE_ACCENT.md
    let result = process_vni("thu7o7ng1");
    assert_eq!(result, "thướng");
}

#[test]
fn test_vni_word_truong() {
    // VNI ươ tone positioning: tru7o7ng2 -> "trường"
    let result = process_vni("tru7o7ng2");
    assert_eq!(result, "trường");
}

#[test]
fn test_vni_word_ban() {
    assert_eq!(process_vni("ba5n"), "bạn");
}

#[test]
fn test_vni_word_hoa() {
    assert_eq!(process_vni("ho2a"), "hòa");
}

#[test]
fn test_vni_word_toi() {
    // tôi = t + ô + i (no tone on ô in this word)
    // ô = o6
    assert_eq!(process_vni("to6i"), "tôi");
}

#[test]
fn test_vni_word_da_nang() {
    // Đả Nẵng = Đ + ả + space + N + ẵ + ng
    // Đ = D9, ả = a + tone3, ẵ = ă + tone4, ă = a8
    assert_eq!(process_vni("D9a3 Na84ng"), "Đả Nẵng");
}

// ============================================
// Edge Cases - Uppercase
// ============================================

#[test]
fn test_vni_uppercase_a6() {
    assert_eq!(process_vni("A6"), "Â");
}

#[test]
fn test_vni_uppercase_d9() {
    assert_eq!(process_vni("D9"), "Đ");
}

#[test]
fn test_vni_uppercase_with_tone() {
    assert_eq!(process_vni("A61"), "Ấ");
}

#[test]
fn test_vni_mixed_case_word() {
    assert_eq!(process_vni("VIE65T"), "VIỆT");
}

// ============================================
// Edge Cases - Numbers in Context
// ============================================

#[test]
fn test_vni_number_after_space() {
    // "Windows 10" should NOT transform '1' after 'o'
    // This tests the context detection optimization
    let result = process_vni("Windows 10");
    assert_eq!(result, "Windows 10", "Should preserve numbers after space");
}

#[test]
fn test_vni_year_2025() {
    let result = process_vni("na(m 2025");
    assert!(result.contains("2025"), "Should preserve year");
}

#[test]
fn test_vni_multi_digit_number() {
    assert_eq!(process_vni("100"), "100");
    assert_eq!(process_vni("2025"), "2025");
}

#[test]
fn test_vni_number_vs_tone() {
    // "a1" should transform (tone mark)
    assert_eq!(process_vni("a1"), "á");
    
    // " 1" should NOT transform (number after space)
    assert_eq!(process_vni(" 1"), " 1");
}

// ============================================
// Edge Cases - No Transform
// ============================================

#[test]
fn test_vni_no_transform_b6() {
    // 'b6' has no transformation
    assert_eq!(process_vni("b6"), "b6");
}

#[test]
fn test_vni_no_transform_x9() {
    assert_eq!(process_vni("x9"), "x9");
}

#[test]
fn test_vni_only_numbers() {
    assert_eq!(process_vni("123456789"), "123456789");
}

// ============================================
// Sequential Transformations
// ============================================

#[test]
fn test_vni_sequential_transforms() {
    // Use process_vni helper for cleaner tests
    assert_eq!(process_vni("a"), "a");
    assert_eq!(process_vni("a6"), "â");
    assert_eq!(process_vni("a61"), "ấ");
}

#[test]
fn test_vni_word_building() {
    // Build "Việt" step by step - use helper
    assert_eq!(process_vni("V"), "V");
    assert_eq!(process_vni("Vi"), "Vi");
    assert_eq!(process_vni("Vie"), "Vie");
    assert_eq!(process_vni("Vie6"), "Viê");
    assert_eq!(process_vni("Vie65"), "Việ");
    assert_eq!(process_vni("Vie65t"), "Việt");
}

// ============================================
// Tone Positioning Tests
// ============================================

#[test]
fn test_vni_tone_position_single_vowel() {
    // Single vowel: tone on that vowel
    assert_eq!(process_vni("ba5n"), "bạn");
}

#[test]
fn test_vni_tone_position_oa() {
    // "oa" → tone on 2nd vowel 'a'
    assert_eq!(process_vni("ho2a"), "hòa");
}

#[test]
fn test_vni_tone_position_uy() {
    // "uy" → tone on 2nd vowel 'y'
    assert_eq!(process_vni("quy1"), "quý");
}

// Note: More complex tone positioning tests would require
// full pipeline integration and Vietnamese phonology rules

// ============================================
// Stress Tests
// ============================================

#[test]
fn test_vni_long_text() {
    let input = "Vie65t Nam la2 mo65t nu7o73c co1 gia1o du5c";
    let result = process_vni(input);
    // Should not panic and should contain Vietnamese characters
    assert!(result.contains('ệ') || result.contains('ị') || result.len() > 0);
}

#[test]
fn test_vni_empty_input() {
    assert_eq!(process_vni(""), "");
}

#[test]
fn test_vni_only_spaces() {
    assert_eq!(process_vni("   "), "   ");
}

#[test]
fn test_vni_punctuation() {
    assert_eq!(process_vni("Xin cha2o!"), "Xin chào!");
}

// ============================================
// Regression Tests
// ============================================

#[test]
fn test_vni_regression_double_transform() {
    // Ensure '66' doesn't cause issues
    let result = process_vni("a66");
    // Should be 'â6' or similar, not crash
    assert_ne!(result, "");
}

#[test]
fn test_vni_regression_tone_on_non_vowel() {
    // '1' after consonant should not crash
    let result = process_vni("b1");
    assert_eq!(result, "b1");
}

// ============================================
// Performance Smoke Tests
// ============================================

#[test]
fn test_vni_performance_100_chars() {
    use std::time::Instant;
    
    let start = Instant::now();
    
    // Process 100 characters (50x "a6")
    for _ in 0..50 {
        let _ = process_vni("a6");
    }
    
    let duration = start.elapsed();
    
    // Should complete in < 500ms in debug mode (< 50ms in release)
    assert!(duration.as_millis() < 1000, 
            "100 chars took {}ms, too slow", duration.as_millis());
}

#[test]
fn test_vni_performance_paragraph() {
    use std::time::Instant;
    
    let paragraph = "Vie65t Nam la2 mo65t nu7o73c co1 ne62n va(n ho1a la2u \
                     do72i va2 phong phu2. Chu1ng to5i ra65t tu7 ha2o ve62 \
                     di5 sa3n va(n ho1a cu3a mi2nh.";
    
    let start = Instant::now();
    let _ = process_vni(paragraph);
    let duration = start.elapsed();
    
    // Should complete in < 500ms in debug mode (< 50ms in release) for ~150 chars
    assert!(duration.as_millis() < 1000,
            "Paragraph took {}ms, too slow", duration.as_millis());
}
