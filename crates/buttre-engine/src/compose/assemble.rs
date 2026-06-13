//! Assemble step — place + apply tone mark onto the syllable nucleus.
//!
//! ## Design
//!
//! Delegates to the canonical P2 `crate::tone::{place, apply}` module.
//! No tone tables are defined here — they live in `crate::tone::tables`.
//!
//! ## Tone placement algorithm
//!
//! 1. Parse the word into consonant onset + vowel nucleus + consonant coda
//!    using `crate::pipeline::validation::extract_coda` + `extract_onset`.
//! 2. Call `crate::tone::place` to find the nucleus index that receives the tone.
//! 3. Call `crate::tone::apply` to get the toned character.
//!
//! ## Multi-tone input
//!
//! Vietnamese allows exactly one tone. The caller (compose) passes only the
//! **last** tone key typed (`stage6:392` semantics). This function does not
//! receive the full list.

use crate::pipeline::config::ToneMark;
use crate::pipeline::validation::{extract_onset, extract_coda, normalize_vietnamese};
use crate::tone;
use crate::vowel::cluster::{is_vowel, normalize_vowel};
use super::ComposeOpts;

// ── Public API ────────────────────────────────────────────────────────────────

/// Apply `tone_key` to `word`, return `Some(toned_word)` or `None` if the key
/// is not in `opts.tone_map` or no vowel could receive the tone.
///
/// Placement follows Vietnamese orthography via `crate::tone::place`.
pub fn apply_tone(word: &str, tone_key: char, opts: &ComposeOpts) -> Option<String> {
    let tone_mark = opts.tone_map.get(&tone_key.to_ascii_lowercase()).copied()?;
    if tone_mark == ToneMark::None {
        return None;
    }

    let mut chars: Vec<char> = word.chars().collect();

    // ── Find vowel nucleus ────────────────────────────────────────────────────
    let word_lower = normalize_vietnamese(word);
    let onset = extract_onset(&word_lower);
    let after_onset = &word_lower[onset.len()..];
    let coda  = extract_coda(after_onset);
    let nucleus_end = after_onset.len() - coda.len();
    let nucleus_str = &after_onset[..nucleus_end];

    // Collect indices of vowel chars in the ORIGINAL word (not lowercase).
    // Align with the nucleus slice from the lowercase analysis.
    let onset_char_count = onset.chars().count();
    let nucleus_chars_count = nucleus_str.chars().count();
    let coda_char_count   = coda.chars().count();

    // Vowels in the word (in order), restricted to the nucleus range.
    let vowel_positions: Vec<usize> = (onset_char_count
        ..onset_char_count + nucleus_chars_count)
        .filter(|&i| i < chars.len() && is_vowel(normalize_vowel(chars[i])))
        .collect();

    if vowel_positions.is_empty() {
        // Fallback: scan all vowels in the word.
        let all_vowels: Vec<usize> = chars.iter().enumerate()
            .filter(|(_, &c)| is_vowel(normalize_vowel(c)))
            .map(|(i, _)| i)
            .collect();
        if all_vowels.is_empty() {
            return None;
        }
        return apply_at_position(&mut chars, all_vowels[0], tone_mark);
    }

    // Build the nucleus character slice for the placement algorithm.
    let nucleus_vowels: Vec<char> = vowel_positions.iter()
        .map(|&i| normalize_vowel(chars[i]))
        .collect();

    let has_final = coda_char_count > 0;
    let place_idx = tone::place(&nucleus_vowels, opts.tone_style, has_final)?;

    // `place_idx` is an index into `nucleus_vowels` (vowels only).
    // Map back to the original `chars` index.
    let char_idx = vowel_positions[place_idx];
    apply_at_position(&mut chars, char_idx, tone_mark)
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn apply_at_position(chars: &mut Vec<char>, idx: usize, tone: ToneMark) -> Option<String> {
    let original = chars[idx];
    let toned = tone::apply(original, tone);
    if toned == original {
        return None; // apply() found no mapping (shouldn't happen for valid vowels).
    }
    chars[idx] = toned;
    Some(chars.iter().collect())
}

// ── Unit tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compose::ComposeOpts;
    use crate::pipeline::config::{PipelineConfig, ToneMark, ToneStyle};

    fn telex_opts() -> ComposeOpts {
        let mut cfg = PipelineConfig::new("telex");
        cfg.add_tone('s', ToneMark::Acute);
        cfg.add_tone('f', ToneMark::Grave);
        cfg.add_tone('r', ToneMark::Hook);
        cfg.add_tone('x', ToneMark::Tilde);
        cfg.add_tone('j', ToneMark::Dot);
        ComposeOpts::from_config(&cfg)
    }

    #[test]
    fn a_acute() {
        let opts = telex_opts();
        assert_eq!(apply_tone("a", 's', &opts), Some("á".to_string()));
    }

    #[test]
    fn ba_grave() {
        let opts = telex_opts();
        assert_eq!(apply_tone("ba", 'f', &opts), Some("bà".to_string()));
    }

    #[test]
    fn tuong_hook() {
        let opts = telex_opts();
        // "tường" base is "tương", hook on ơ
        assert_eq!(apply_tone("tương", 'f', &opts), Some("tường".to_string()));
    }

    #[test]
    fn unknown_tone_key_returns_none() {
        let opts = telex_opts();
        assert_eq!(apply_tone("a", 'z', &opts), None);
    }

    #[test]
    fn multi_vowel_nucleus_open_ia() {
        // "ia" open syllable: tone on 'i' (index 0)
        let opts = telex_opts();
        let result = apply_tone("ia", 's', &opts);
        assert_eq!(result, Some("ía".to_string()));
    }
}
