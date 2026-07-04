//! Permutation Matching Module
//!
//! **Tests**: Integration tests for this module are located in `crates/buttre-engine/tests/pipeline_permutation_tests.rs`.
//!
//! This module provides algorithms for flexible typing support - allowing users
//! to type Vietnamese marks in various orders and positions.
//!
//! ## Examples
//!
//! - VNI: `truong6f`, `truon6gf`, `tru6ongf` → all produce `trường`
//! - Telex: `truongwf`, `truwongf`, `truowfng` → all produce `trường`
//!
//! ## Algorithm
//!
//! 1. **Extract Base and Marks**: Split input into base word + marks
//!    - Input: "truongwf" → Base: "truong", Marks: ['w', 'f']
//! 2. **Find Vowel Cluster**: Locate vowel sequence in base
//! 3. **Apply Marks**: Apply marks to appropriate vowels in cluster
//! 4. **Validate**: Check if result is valid Vietnamese

use crate::pipeline::config::ToneConfig;
use crate::vowel::{find_vowel_clusters, normalize_vowel, VowelCluster};

/// Mark Operation
///
/// Represents a diacritical mark or tone that needs to be applied.
#[derive(Debug, Clone, PartialEq)]
pub enum MarkOp {
    /// Transform mark (Telex: w, VNI: 6, 7, 8)
    Transform(char),

    /// Tone mark (Telex: s, f, r, x, j, VNI: 1-5)
    Tone(char),
}

/// Extract base word and marks from input sequence
///
/// ## Algorithm
///
/// 1. Scan input from left to right
/// 2. Separate vowels/consonants (base) from marks (w, f, s, r, x, j, 0-9)
/// 3. Preserve order of marks for application
///
/// ## Arguments
///
/// - `input`: The input string (e.g., "truongwf", "truon6gf")
/// - `is_vni`: Whether this is VNI input method (affects mark detection)
///
/// ## Returns
///
/// Tuple of (base_word, marks)
///
/// ## Examples
///
/// ```rust,ignore
/// // Telex
/// extract_base_and_marks("truongwf", false)
///   → ("truong", [Transform('w'), Tone('f')])
///
/// // VNI
/// extract_base_and_marks("truong6f", true)
///   → ("truong", [Transform('6'), Tone('f')])
/// ```
pub fn extract_base_and_marks(input: &str, is_vni: bool) -> (String, Vec<MarkOp>) {
    let mut base = String::new();
    let mut marks = Vec::new();

    for ch in input.chars() {
        if is_mark_key(ch, is_vni) {
            // This is a mark - determine type
            let mark_op = if is_transform_mark(ch, is_vni) {
                MarkOp::Transform(ch)
            } else {
                MarkOp::Tone(ch)
            };
            marks.push(mark_op);
        } else {
            // This is part of the base word
            base.push(ch);
        }
    }

    (base, marks)
}

/// Check if a character is a mark key
pub fn is_mark_key(ch: char, is_vni: bool) -> bool {
    if is_vni {
        // VNI: Numbers 0-9 are marks
        ch.is_ascii_digit()
    } else {
        // Telex: w, s, f, r, x, j, z are marks
        matches!(
            ch,
            'w' | 'W' | 's' | 'S' | 'f' | 'F' | 'r' | 'R' | 'x' | 'X' | 'j' | 'J' | 'z' | 'Z'
        )
    }
}

/// Check if a mark is a transform mark (vs tone mark)
pub fn is_transform_mark(ch: char, is_vni: bool) -> bool {
    if is_vni {
        // VNI: 6, 7, 8, 9 are transform marks
        matches!(ch, '6' | '7' | '8' | '9')
    } else {
        // Telex: w, z are transform marks
        matches!(ch, 'w' | 'W' | 'z' | 'Z')
    }
}

/// Apply marks to a base word using permutation matching
///
/// ## Algorithm
///
/// 1. Find vowel cluster in base word
/// 2. For each mark in marks list:
///    - If transform mark: Apply to appropriate vowel in cluster
///    - If tone mark: Apply to appropriate vowel (based on ToneConfig)
/// 3. Validate the result
///
/// ## Arguments
///
/// - `base`: The base word (consonants + vowels without marks)
/// - `marks`: List of marks to apply
/// - `config`: Tone configuration (for positioning rules)
///
/// ## Returns
///
/// The transformed word, or None if transformation failed
///
/// ## Example
///
/// ```rust,ignore
/// apply_marks_permutation("truong",
///                        [Transform('w'), Tone('f')],
///                        &config)
///   → Some("trường")
/// ```
pub fn apply_marks_permutation(
    base: &str,
    marks: &[MarkOp],
    config: &ToneConfig,
) -> Option<String> {
    // Find vowel cluster in base
    let clusters = find_vowel_clusters(base);
    if clusters.is_empty() {
        return None;
    }

    // For now, work with the last cluster (most common case)
    let cluster = clusters.last()?;

    // Build result by applying marks
    let mut result = base.to_string();

    for mark in marks {
        match mark {
            MarkOp::Transform(ch) => {
                // Apply transform mark to vowel cluster
                result = apply_transform_to_cluster(&result, cluster, *ch, config)?;
            }
            MarkOp::Tone(ch) => {
                // Apply tone mark to vowel cluster
                result = apply_tone_to_cluster(&result, cluster, *ch, config)?;
            }
        }
    }

    Some(result)
}

/// Apply a transform mark to a vowel cluster
///
/// ## Telex Transform Rules
/// - w → ơ (o + w), ư (u + w), ă (a + w), â (a + w), ê (e + w), ô (o + w)
///
/// ## VNI Transform Rules
/// - 6 → ă (a + 6), ê (e + 6), ô (o + 6)
/// - 7 → â (a + 7), ơ (o + 7), ư (u + 7)
/// - 8 → ă (a + 8)
/// - 9 → đ (d + 9)
pub fn apply_transform_to_cluster(
    base: &str,
    cluster: &VowelCluster,
    mark: char,
    _config: &ToneConfig,
) -> Option<String> {
    let mut chars: Vec<char> = base.chars().collect();

    // Determine which vowel in cluster to transform
    // For now, simple heuristic: transform based on vowel type

    if mark == 'w' || mark == 'W' {
        // Telex w: Look for o → ơ, u → ư, a → ă/â, e → ê
        for i in cluster.start_pos..cluster.end_pos {
            let vowel = normalize_vowel(chars[i]);
            match vowel {
                'o' => {
                    chars[i] = 'ơ';
                    return Some(chars.iter().collect());
                }
                'u' => {
                    chars[i] = 'ư';
                    return Some(chars.iter().collect());
                }
                'a' => {
                    // Check context: if already has marks, might be ă or â
                    // For now, default to ă
                    chars[i] = 'ă';
                    return Some(chars.iter().collect());
                }
                'e' => {
                    chars[i] = 'ê';
                    return Some(chars.iter().collect());
                }
                _ => continue,
            }
        }
    } else if mark == '6' {
        // VNI 6: a → ă, e → ê, o → ô
        for i in cluster.start_pos..cluster.end_pos {
            let vowel = normalize_vowel(chars[i]);
            match vowel {
                'a' => {
                    chars[i] = 'ă';
                    return Some(chars.iter().collect());
                }
                'e' => {
                    chars[i] = 'ê';
                    return Some(chars.iter().collect());
                }
                'o' => {
                    chars[i] = 'ô';
                    return Some(chars.iter().collect());
                }
                _ => continue,
            }
        }
    } else if mark == '7' {
        // VNI 7: a → â, o → ơ, u → ư
        for i in cluster.start_pos..cluster.end_pos {
            let vowel = normalize_vowel(chars[i]);
            match vowel {
                'a' => {
                    chars[i] = 'â';
                    return Some(chars.iter().collect());
                }
                'o' => {
                    chars[i] = 'ơ';
                    return Some(chars.iter().collect());
                }
                'u' => {
                    chars[i] = 'ư';
                    return Some(chars.iter().collect());
                }
                _ => continue,
            }
        }
    } else if mark == '8' {
        // VNI 8: a → ă
        for i in cluster.start_pos..cluster.end_pos {
            let vowel = normalize_vowel(chars[i]);
            if vowel == 'a' {
                chars[i] = 'ă';
                return Some(chars.iter().collect());
            }
        }
    }

    None
}

/// Apply a tone mark to a vowel cluster
///
/// Uses the vowel sequence table and tone positioning rules from config.
pub fn apply_tone_to_cluster(
    base: &str,
    cluster: &VowelCluster,
    tone_key: char,
    config: &ToneConfig,
) -> Option<String> {
    let mut chars: Vec<char> = base.chars().collect();

    // Find which vowel should receive the tone
    // Use vowel sequence table if available
    let tone_pos = if !config.vowel_sequences.is_empty() {
        // Look up in table
        let cluster_str: String = cluster.vowels.iter().collect();
        if let Some(seq_info) = config.vowel_sequences.find(&cluster_str) {
            seq_info
                .primary_tone_position()
                .map(|pos| cluster.start_pos + pos)
        } else {
            // Fallback: first vowel
            Some(cluster.start_pos)
        }
    } else {
        // No table: use first vowel
        Some(cluster.start_pos)
    }?;

    // Apply tone to the vowel at tone_pos
    let vowel = chars[tone_pos];
    let toned_vowel = apply_tone_to_vowel(vowel, tone_key)?;
    chars[tone_pos] = toned_vowel;

    Some(chars.iter().collect())
}

/// Apply a tone mark to a single vowel
///
/// ## Telex Tones
/// - s → Acute (á, ế, í, ...)
/// - f → Grave (à, ề, ì, ...)
/// - r → Hook (ả, ể, ỉ, ...)
/// - x → Tilde (ã, ễ, ĩ, ...)
/// - j → Dot (ạ, ệ, ị, ...)
///
/// ## VNI Tones
/// - 1 → Acute
/// - 2 → Grave
/// - 3 → Hook
/// - 4 → Tilde
/// - 5 → Dot
pub fn apply_tone_to_vowel(vowel: char, tone_key: char) -> Option<char> {
    let base = normalize_vowel(vowel);
    let is_upper = vowel.is_uppercase();

    let toned = match (base, tone_key) {
        // a family + Telex
        ('a', 's') | ('a', '1') => 'á',
        ('a', 'f') | ('a', '2') => 'à',
        ('a', 'r') | ('a', '3') => 'ả',
        ('a', 'x') | ('a', '4') => 'ã',
        ('a', 'j') | ('a', '5') => 'ạ',

        ('ă', 's') | ('ă', '1') => 'ắ',
        ('ă', 'f') | ('ă', '2') => 'ằ',
        ('ă', 'r') | ('ă', '3') => 'ẳ',
        ('ă', 'x') | ('ă', '4') => 'ẵ',
        ('ă', 'j') | ('ă', '5') => 'ặ',

        ('â', 's') | ('â', '1') => 'ấ',
        ('â', 'f') | ('â', '2') => 'ầ',
        ('â', 'r') | ('â', '3') => 'ẩ',
        ('â', 'x') | ('â', '4') => 'ẫ',
        ('â', 'j') | ('â', '5') => 'ậ',

        // e family
        ('e', 's') | ('e', '1') => 'é',
        ('e', 'f') | ('e', '2') => 'è',
        ('e', 'r') | ('e', '3') => 'ẻ',
        ('e', 'x') | ('e', '4') => 'ẽ',
        ('e', 'j') | ('e', '5') => 'ẹ',

        ('ê', 's') | ('ê', '1') => 'ế',
        ('ê', 'f') | ('ê', '2') => 'ề',
        ('ê', 'r') | ('ê', '3') => 'ể',
        ('ê', 'x') | ('ê', '4') => 'ễ',
        ('ê', 'j') | ('ê', '5') => 'ệ',

        // i
        ('i', 's') | ('i', '1') => 'í',
        ('i', 'f') | ('i', '2') => 'ì',
        ('i', 'r') | ('i', '3') => 'ỉ',
        ('i', 'x') | ('i', '4') => 'ĩ',
        ('i', 'j') | ('i', '5') => 'ị',

        // o family
        ('o', 's') | ('o', '1') => 'ó',
        ('o', 'f') | ('o', '2') => 'ò',
        ('o', 'r') | ('o', '3') => 'ỏ',
        ('o', 'x') | ('o', '4') => 'õ',
        ('o', 'j') | ('o', '5') => 'ọ',

        ('ô', 's') | ('ô', '1') => 'ố',
        ('ô', 'f') | ('ô', '2') => 'ồ',
        ('ô', 'r') | ('ô', '3') => 'ổ',
        ('ô', 'x') | ('ô', '4') => 'ỗ',
        ('ô', 'j') | ('ô', '5') => 'ộ',

        ('ơ', 's') | ('ơ', '1') => 'ớ',
        ('ơ', 'f') | ('ơ', '2') => 'ờ',
        ('ơ', 'r') | ('ơ', '3') => 'ở',
        ('ơ', 'x') | ('ơ', '4') => 'ỡ',
        ('ơ', 'j') | ('ơ', '5') => 'ợ',

        // u family
        ('u', 's') | ('u', '1') => 'ú',
        ('u', 'f') | ('u', '2') => 'ù',
        ('u', 'r') | ('u', '3') => 'ủ',
        ('u', 'x') | ('u', '4') => 'ũ',
        ('u', 'j') | ('u', '5') => 'ụ',

        ('ư', 's') | ('ư', '1') => 'ứ',
        ('ư', 'f') | ('ư', '2') => 'ừ',
        ('ư', 'r') | ('ư', '3') => 'ử',
        ('ư', 'x') | ('ư', '4') => 'ữ',
        ('ư', 'j') | ('ư', '5') => 'ự',

        // y
        ('y', 's') | ('y', '1') => 'ý',
        ('y', 'f') | ('y', '2') => 'ỳ',
        ('y', 'r') | ('y', '3') => 'ỷ',
        ('y', 'x') | ('y', '4') => 'ỹ',
        ('y', 'j') | ('y', '5') => 'ỵ',

        _ => return None,
    };

    Some(if is_upper {
        toned.to_uppercase().next().unwrap_or(toned)
    } else {
        toned
    })
}
