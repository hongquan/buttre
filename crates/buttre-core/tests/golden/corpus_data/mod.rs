//! Corpus builders: Telex, VNI, and Nôm case generators.
//!
//! **Single source of truth.** This module and its sub-modules own the only
//! copy of the syllable list, decompose tables, and Tag enum.  Both
//! `examples/gen_golden.rs` (which writes the .snap files) and
//! `tests/golden_regression.rs` (which reads them) reference this module,
//! so there is no risk of the two sides diverging.
//!
//! Data tables live in sibling modules:
//! - `char_tables`    — Vietnamese char → key decomposition functions
//! - `syllable_list`  — SYLLABLES constant, ENGLISH_WORDS, undo/toggle sets

use char_tables::{decompose_telex, decompose_vni, telex_tone_key, vni_tone_key};
use syllable_list::{ENGLISH_WORDS, SYLLABLES, TELEX_UNDO_TOGGLE, VNI_UNDO_TOGGLE};

mod char_tables;
mod syllable_list;

// ============================================================================
// Tag
// ============================================================================

/// Tag classifying a test case's expected mutability during the refactor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Tag {
    /// Valid Vietnamese syllable — output MUST NOT change across phases.
    VietnameseValid,
    /// Flexible typing order (tone before final consonant, etc.) — MUST NOT change.
    FlexibleTyping,
    /// Pure ASCII English word — output MAY change in Phase 4.
    EnglishWord,
    /// Undo / toggle sequence (`aaa`, `aww`, `a11`, …) — MUST NOT change.
    UndoToggle,
}

impl Tag {
    /// Canonical string representation written into .snap files.
    pub fn as_str(self) -> &'static str {
        match self {
            Tag::VietnameseValid => "VietnameseValid",
            Tag::FlexibleTyping  => "FlexibleTyping",
            Tag::EnglishWord     => "EnglishWord",
            Tag::UndoToggle      => "UndoToggle",
        }
    }
}

// ============================================================================
// Key converters
// ============================================================================

/// Convert a Vietnamese syllable into its Telex keystroke sequence.
///
/// - Onset consonants emitted as-is.
/// - Diacritics: â→aa, ă→aw, ê→ee, ô→oo, ơ→ow, ư→uw, đ→dd.
/// - Tone key appended at end: á→…s, à→…f, ả→…r, ã→…x, ạ→…j.
pub fn vn_to_telex_keys(syllable: &str) -> String {
    let mut out = String::new();
    let mut tone_key: Option<char> = None;
    for ch in syllable.chars() {
        let (base, extra, tone) = decompose_telex(ch);
        if let Some(t) = tone { tone_key = Some(telex_tone_key(t)); }
        out.push(base);
        if let Some(e) = extra { out.push(e); }
    }
    if let Some(t) = tone_key { out.push(t); }
    out
}

/// Convert a Vietnamese syllable into its VNI keystroke sequence.
///
/// - Diacritics: â→a6, ă→a8, ê→e6, ô→o6, ơ→o7, ư→u7, đ→d9.
/// - Tone key appended at end: á→…1, à→…2, ả→…3, ã→…4, ạ→…5.
pub fn vn_to_vni_keys(syllable: &str) -> String {
    let mut out = String::new();
    let mut tone_key: Option<char> = None;
    for ch in syllable.chars() {
        let (base, extra, tone) = decompose_vni(ch);
        if let Some(t) = tone { tone_key = Some(vni_tone_key(t)); }
        out.push(base);
        if let Some(e) = extra { out.push(e); }
    }
    if let Some(t) = tone_key { out.push(t); }
    out
}

// ============================================================================
// Flexible-typing permuters
// ============================================================================

/// Telex tone keys (sfrxj).
const TELEX_TONE_KEYS: &[char] = &['s', 'f', 'r', 'x', 'j'];
/// VNI tone keys (12345).
const VNI_TONE_KEYS: &[char] = &['1', '2', '3', '4', '5'];
/// Letters that can be coda consonants (used for permutation detection).
const CODA_LETTERS: &[char] = &['n', 'g', 'h', 'm', 'c', 't', 'p', 'k'];

/// Insert Telex tone key before the final consonant run.
///
/// Returns `None` if the key string ends without a tone key, or has no
/// trailing consonant coda to insert before.
fn telex_permute_tone_before_coda(keys: &str) -> Option<String> {
    permute_tone(keys, TELEX_TONE_KEYS)
}

/// Insert VNI tone digit before the final consonant run.
fn vni_permute_tone_before_coda(keys: &str) -> Option<String> {
    permute_tone(keys, VNI_TONE_KEYS)
}

fn permute_tone(keys: &str, tone_keys: &[char]) -> Option<String> {
    let chars: Vec<char> = keys.chars().collect();
    let last = *chars.last()?;
    if !tone_keys.contains(&last) { return None; }
    let body = &chars[..chars.len() - 1];
    let coda_start = body
        .iter()
        .rposition(|c| !CODA_LETTERS.contains(c))
        .map(|i| i + 1)
        .unwrap_or(0);
    if coda_start >= body.len() { return None; }
    let mut result: Vec<char> = body[..coda_start].to_vec();
    result.push(last);
    result.extend_from_slice(&body[coda_start..]);
    Some(result.into_iter().collect())
}

// ============================================================================
// Public corpus builders
// ============================================================================

/// Build the full Telex corpus.
///
/// - VietnameseValid: one per syllable from SYLLABLES
/// - FlexibleTyping: tone-before-final-coda permutation for toned CVC syllables
/// - EnglishWord: ~40 ASCII words
/// - UndoToggle: ~30 undo/double-key sequences
pub fn telex_corpus() -> Vec<(String, Tag)> {
    let mut v = Vec::new();
    for &s in SYLLABLES {
        let k = vn_to_telex_keys(s);
        if !k.is_empty() { v.push((k, Tag::VietnameseValid)); }
    }
    for &s in SYLLABLES {
        let k = vn_to_telex_keys(s);
        if let Some(p) = telex_permute_tone_before_coda(&k) {
            if p != k { v.push((p, Tag::FlexibleTyping)); }
        }
    }
    for &w in ENGLISH_WORDS { v.push((w.to_string(), Tag::EnglishWord)); }
    for &u in TELEX_UNDO_TOGGLE { v.push((u.to_string(), Tag::UndoToggle)); }
    v
}

/// Build the full VNI corpus.
pub fn vni_corpus() -> Vec<(String, Tag)> {
    let mut v = Vec::new();
    for &s in SYLLABLES {
        let k = vn_to_vni_keys(s);
        if !k.is_empty() { v.push((k, Tag::VietnameseValid)); }
    }
    for &s in SYLLABLES {
        let k = vn_to_vni_keys(s);
        if let Some(p) = vni_permute_tone_before_coda(&k) {
            if p != k { v.push((p, Tag::FlexibleTyping)); }
        }
    }
    for &w in ENGLISH_WORDS { v.push((w.to_string(), Tag::EnglishWord)); }
    for &u in VNI_UNDO_TOGGLE { v.push((u.to_string(), Tag::UndoToggle)); }
    v
}

/// Build the Nôm corpus (uses Telex key sequences as input).
/// Used by `examples/gen_golden.rs`; not called from the test binary.
#[allow(dead_code)]
pub fn nom_corpus() -> Vec<(String, Tag)> {
    let mut v = Vec::new();
    for &s in SYLLABLES {
        let k = vn_to_telex_keys(s);
        if !k.is_empty() { v.push((k, Tag::VietnameseValid)); }
    }
    v
}

// ============================================================================
// Unit tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn telex_a_to_as() {
        assert_eq!(vn_to_telex_keys("á"), "as");
    }

    #[test]
    fn telex_a_circumflex_acute() {
        assert_eq!(vn_to_telex_keys("ấ"), "aas");
    }

    #[test]
    fn telex_dd() {
        assert_eq!(vn_to_telex_keys("đ"), "dd");
    }

    #[test]
    fn vni_a_circumflex_acute() {
        assert_eq!(vn_to_vni_keys("ấ"), "a61");
    }

    #[test]
    fn vni_dd() {
        assert_eq!(vn_to_vni_keys("đ"), "d9");
    }

    #[test]
    fn telex_corpus_has_enough() {
        let cases = telex_corpus();
        assert!(cases.len() >= 800, "Telex corpus too small: {} cases", cases.len());
    }

    #[test]
    fn vni_corpus_has_enough() {
        let cases = vni_corpus();
        assert!(cases.len() >= 800, "VNI corpus too small: {} cases", cases.len());
    }

    #[test]
    fn telex_permute_produces_different_order() {
        // "banf" = bàn; coda is 'n', tone 'f' at end → permuted "bafn"
        let perm = telex_permute_tone_before_coda("banf");
        assert!(perm.is_some(), "should find a permutation for 'banf'");
        let p = perm.unwrap();
        assert_ne!(p, "banf");
        let pos_f = p.chars().position(|c| c == 'f').unwrap();
        let pos_n = p.chars().position(|c| c == 'n').unwrap();
        assert!(pos_f < pos_n, "tone key should precede coda: got '{}'", p);
    }

    #[test]
    fn syllables_no_duplicates() {
        use std::collections::HashSet;
        let mut seen = HashSet::new();
        for &s in super::SYLLABLES {
            assert!(seen.insert(s), "duplicate syllable in SYLLABLES: {:?}", s);
        }
    }

    #[test]
    fn syllables_include_uppercase_coverage() {
        // Verify a representative sample of required uppercase entries.
        let required = ["Â", "Đ", "Việt", "Đúng", "Người", "NGƯỜI"];
        for r in required {
            assert!(
                super::SYLLABLES.contains(&r),
                "SYLLABLES missing required uppercase entry: {:?}",
                r
            );
        }
    }

    #[test]
    fn syllables_include_uo_cluster() {
        // Verify the 8 previously-missing uo/quô entries are present.
        let required = ["ngườ", "quo", "quô", "quố", "quồ", "quổ", "quỗ", "quộ"];
        for r in required {
            assert!(
                super::SYLLABLES.contains(&r),
                "SYLLABLES missing uo-cluster entry: {:?}",
                r
            );
        }
    }

    #[test]
    fn tag_as_str_roundtrip() {
        assert_eq!(Tag::VietnameseValid.as_str(), "VietnameseValid");
        assert_eq!(Tag::FlexibleTyping.as_str(), "FlexibleTyping");
        assert_eq!(Tag::EnglishWord.as_str(), "EnglishWord");
        assert_eq!(Tag::UndoToggle.as_str(), "UndoToggle");
    }
}
