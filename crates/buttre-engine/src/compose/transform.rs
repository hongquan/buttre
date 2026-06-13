//! Transform step — apply diacritic marks to the base string.
//!
//! ## Position-aware transform
//!
//! Each `TransformMark` carries `base_len_at_typing` — the number of base chars
//! that existed when the mark key was pressed.  The mark applies to the
//! RIGHT-MOST vowel in `base[..base_len_at_typing]`, matching incremental
//! behaviour (the mark was typed immediately after that vowel).
//!
//! Example: "ngoaw" → base="ngoat"(sic "ngoa"), mark='w' with base_len=4.
//! Search `base[..4]="ngoa"` right-to-left → 'a' matches "aw"→"ă". ✓
//!
//! ## UO compound rule (port of stage6 lines 204-240)
//!
//! `uo + w/7`:
//! - When "uo" is followed by more chars (has a coda) → ươ (both transformed).
//! - When "uo" is at word end → uơ (only o transformed).
//! - SUPPRESSED when another compound-trigger mark follows: "uwow" → first mark
//!   applies to 'u' individually, second mark applies to 'o' individually.

use crate::vowel::cluster::normalize_vowel;
use super::{ComposeOpts, segment::TransformMark};

// ── Public API ────────────────────────────────────────────────────────────────

/// Apply all `transforms` to `base` in order.
///
/// Returns the transformed string; unapplied marks are appended literally.
pub fn apply_transforms(base: &str, transforms: &[TransformMark], opts: &ComposeOpts) -> String {
    let mut result = base.to_string();
    for (idx, tm) in transforms.iter().enumerate() {
        // Build the remaining mark keys slice for compound-suppression check.
        let remaining_keys: Vec<char> = transforms[idx + 1..].iter().map(|t| t.key).collect();
        let new = apply_one_transform(&result, tm.key, tm.base_len_at_typing, &remaining_keys, opts);
        if let Some(new_result) = new {
            result = new_result;
        } else {
            result.push(tm.key);
        }
    }
    result
}

// ── Internal ──────────────────────────────────────────────────────────────────

/// Apply a single transform mark to `result`.
///
/// `base_len_at_typing`: char count of the base when the mark was typed.
/// The mark targets the rightmost matching vowel in `result[..base_len_at_typing]`.
///
/// `remaining_keys`: later marks (for compound-suppression decision).
fn apply_one_transform(
    result: &str,
    mark: char,
    base_len_at_typing: usize,
    remaining_keys: &[char],
    opts: &ComposeOpts,
) -> Option<String> {
    let mark_lc = mark.to_ascii_lowercase();
    let mut chars: Vec<char> = result.chars().collect();

    // Cap the search range to base_len_at_typing (right-most vowel before mark was typed).
    // If the result has grown beyond the original base (e.g. after a previous transform
    // that expanded the string), we use the full result length to avoid out-of-bounds.
    let search_end = base_len_at_typing.min(chars.len());

    // ── UO compound rule ──────────────────────────────────────────────────────
    // Only within the base slice that was present when this mark was typed.
    let triggers_compound = is_compound_trigger(mark, opts);
    let has_later_compound = remaining_keys.iter()
        .any(|&m| m.to_ascii_lowercase() == mark_lc && is_compound_trigger(m, opts));

    if triggers_compound && !has_later_compound {
        let base_slice: String = chars[..search_end].iter().collect();
        let base_slice_lower = base_slice.to_lowercase();
        if let Some(pos) = find_uo_pos(&base_slice_lower) {
            let u_ch = chars[pos];
            let o_ch = chars[pos + 1];
            let u_is_base = normalize_vowel(u_ch) == 'u' && !is_u_horn(u_ch);
            let o_is_base = normalize_vowel(o_ch) == 'o'
                && !matches!(normalize_vowel(o_ch), 'ơ' | 'ô');
            if u_is_base && o_is_base {
                let has_following = pos + 2 < chars.len();
                if has_following {
                    chars[pos]     = preserve_case(u_ch, 'ư');
                    chars[pos + 1] = preserve_case(o_ch, 'ơ');
                } else {
                    chars[pos + 1] = preserve_case(o_ch, 'ơ');
                }
                return Some(chars.into_iter().collect());
            }
        }
    }

    // ── Data-driven single-vowel transform ────────────────────────────────────
    // Normal case: scan RIGHT-TO-LEFT within base[..search_end] to find the
    // rightmost vowel that has a matching rule.  This honours the "mark applies
    // to the vowel typed immediately before it" contract.
    //
    // Prefix-transform case (base_len_at_typing == 0): the mark key was the
    // FIRST key typed, so the entire base was typed AFTER it.  Two sub-cases:
    //
    // 1. The mark has a 1-char rule for itself (e.g. Telex "w"→"ư"): prepend
    //    the result to the base.  This handles "win" → base="in", mark='w' at
    //    pos 0 → single-char rule "w"→"ư" → prepend → "ưin".
    //
    // 2. No 1-char rule: scan left-to-right for the first vowel that has a
    //    2-char rule with the mark (e.g. a future config where "iw"→"ị").
    if search_end == 0 && !chars.is_empty() {
        // Sub-case 1: 1-char rule for the mark key itself (standalone prefix).
        let single_key = mark_lc.to_string();
        if let Some(result_str) = opts.transform_rules.get(&single_key) {
            let prefix: String = result_str.chars().collect();
            let rest: String = chars.into_iter().collect();
            return Some(format!("{prefix}{rest}"));
        }

        // Sub-case 2: Forward scan for a vowel + mark 2-char rule.
        for i in 0..chars.len() {
            let ch    = chars[i];
            let ch_lc = normalize_vowel(ch);
            let lookup_key = format!("{ch_lc}{mark_lc}");
            if let Some(result_str) = opts.transform_rules.get(&lookup_key) {
                if let Some(new_char) = result_str.chars().next() {
                    chars[i] = preserve_case(ch, new_char);
                    return Some(chars.into_iter().collect());
                }
            }
        }
        return None;
    }

    for i in (0..search_end).rev() {
        let ch    = chars[i];
        let ch_lc = normalize_vowel(ch); // base vowel (strips tone diacritics)

        let lookup_key = format!("{ch_lc}{mark_lc}");

        if let Some(result_str) = opts.transform_rules.get(&lookup_key) {
            if let Some(new_char) = result_str.chars().next() {
                chars[i] = preserve_case(ch, new_char);
                return Some(chars.into_iter().collect());
            }
        }
    }

    None
}

/// True when the mark key is the compound-trigger in the transform table.
fn is_compound_trigger(mark: char, opts: &ComposeOpts) -> bool {
    let ml = mark.to_ascii_lowercase();
    opts.transform_rules.contains_key(&format!("o{ml}"))
        && opts.transform_rules.contains_key(&format!("u{ml}"))
}

/// Find the char index of "uo" in a lowercase string.
fn find_uo_pos(lower: &str) -> Option<usize> {
    let chars: Vec<char> = lower.chars().collect();
    for i in 0..chars.len().saturating_sub(1) {
        if normalize_vowel(chars[i]) == 'u' && normalize_vowel(chars[i + 1]) == 'o' {
            return Some(i);
        }
    }
    None
}

/// True when `ch` is 'ư' (already has the u-horn diacritic).
fn is_u_horn(ch: char) -> bool {
    matches!(ch.to_lowercase().next(), Some('ư'))
}

/// Preserve the case of `original` on `new_char`.
#[inline]
fn preserve_case(original: char, new_char: char) -> char {
    if original.is_uppercase() {
        new_char.to_uppercase().next().unwrap_or(new_char)
    } else {
        new_char
    }
}

// ── Unit tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compose::{ComposeOpts, segment::TransformMark};
    use crate::pipeline::config::{PipelineConfig, ToneMark};

    fn telex_opts() -> ComposeOpts {
        let mut cfg = PipelineConfig::new("telex");
        cfg.add_transform("aw", "ă");
        cfg.add_transform("aa", "â");
        cfg.add_transform("ee", "ê");
        cfg.add_transform("oo", "ô");
        cfg.add_transform("ow", "ơ");
        cfg.add_transform("uw", "ư");
        cfg.add_transform("dd", "đ");
        cfg.add_tone('s', ToneMark::Acute);
        ComposeOpts::from_config(&cfg)
    }

    fn tm(key: char, base_len: usize) -> TransformMark {
        TransformMark { key, base_len_at_typing: base_len }
    }

    #[test]
    fn a_plus_w_yields_breve() {
        let opts = telex_opts();
        // base="a", mark='w', base_len=1 (base had 1 char when 'w' was typed)
        let result = apply_transforms("a", &[tm('w', 1)], &opts);
        assert_eq!(result, "ă");
    }

    #[test]
    fn a_plus_a_yields_circumflex() {
        let opts = telex_opts();
        let result = apply_transforms("a", &[tm('a', 1)], &opts);
        assert_eq!(result, "â");
    }

    #[test]
    fn uo_w_has_coda_yields_uhorn_ohorn() {
        // base="tuong", mark='w' typed after full base → base_len=5
        let opts = telex_opts();
        let result = apply_transforms("tuong", &[tm('w', 5)], &opts);
        assert_eq!(result, "tương");
    }

    #[test]
    fn uo_w_no_coda_yields_only_ohorn() {
        let opts = telex_opts();
        let result = apply_transforms("thuo", &[tm('w', 4)], &opts);
        assert_eq!(result, "thuơ");
    }

    #[test]
    fn ngoa_w_transforms_a_not_o() {
        // "ngoaw" → 'w' typed after 'a' (base_len=4 for "ngoa") → 'a'→'ă'
        let opts = telex_opts();
        let result = apply_transforms("ngoa", &[tm('w', 4)], &opts);
        assert_eq!(result, "ngoă");
    }

    #[test]
    fn uwa_w_transforms_u_not_a() {
        // "uwa": base="ua", 'w' typed after 'u' (base_len=1) → 'u'→'ư', 'a' untouched
        let opts = telex_opts();
        let result = apply_transforms("ua", &[tm('w', 1)], &opts);
        assert_eq!(result, "ưa");
    }

    #[test]
    fn dd_transform() {
        let opts = telex_opts();
        let result = apply_transforms("d", &[tm('d', 1)], &opts);
        assert_eq!(result, "đ");
    }

    #[test]
    fn unknown_mark_appended_literally() {
        let opts = telex_opts();
        let result = apply_transforms("ban", &[tm('z', 3)], &opts);
        assert_eq!(result, "banz");
    }
}
