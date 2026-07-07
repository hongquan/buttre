//! Tone character table: single authoritative mapping between base vowels and toned vowels.
//!
//! ## Design
//!
//! One `[12][6]` array for lowercase, one for uppercase.
//! All callers use `apply` / `strip` — no other file should define this mapping.
//!
//! ## Vowel order (row index)
//! 0=a  1=ă  2=â  3=e  4=ê  5=i  6=o  7=ô  8=ơ  9=u  10=ư  11=y
//!
//! ## Tone order (column index)
//! 0=None  1=Acute  2=Grave  3=Hook  4=Tilde  5=Dot

use crate::pipeline::config::ToneMark;

// ── Tables ───────────────────────────────────────────────────────────────────

static TONE_TABLE: [[char; 6]; 12] = [
    ['a', 'á', 'à', 'ả', 'ã', 'ạ'], // base 'a'
    ['ă', 'ắ', 'ằ', 'ẳ', 'ẵ', 'ặ'], // base 'ă'
    ['â', 'ấ', 'ầ', 'ẩ', 'ẫ', 'ậ'], // base 'â'
    ['e', 'é', 'è', 'ẻ', 'ẽ', 'ẹ'], // base 'e'
    ['ê', 'ế', 'ề', 'ể', 'ễ', 'ệ'], // base 'ê'
    ['i', 'í', 'ì', 'ỉ', 'ĩ', 'ị'], // base 'i'
    ['o', 'ó', 'ò', 'ỏ', 'õ', 'ọ'], // base 'o'
    ['ô', 'ố', 'ồ', 'ổ', 'ỗ', 'ộ'], // base 'ô'
    ['ơ', 'ớ', 'ờ', 'ở', 'ỡ', 'ợ'], // base 'ơ'
    ['u', 'ú', 'ù', 'ủ', 'ũ', 'ụ'], // base 'u'
    ['ư', 'ứ', 'ừ', 'ử', 'ữ', 'ự'], // base 'ư'
    ['y', 'ý', 'ỳ', 'ỷ', 'ỹ', 'ỵ'], // base 'y'
];

static TONE_TABLE_UPPER: [[char; 6]; 12] = [
    ['A', 'Á', 'À', 'Ả', 'Ã', 'Ạ'], // base 'A'
    ['Ă', 'Ắ', 'Ằ', 'Ẳ', 'Ẵ', 'Ặ'], // base 'Ă'
    ['Â', 'Ấ', 'Ầ', 'Ẩ', 'Ẫ', 'Ậ'], // base 'Â'
    ['E', 'É', 'È', 'Ẻ', 'Ẽ', 'Ẹ'], // base 'E'
    ['Ê', 'Ế', 'Ề', 'Ể', 'Ễ', 'Ệ'], // base 'Ê'
    ['I', 'Í', 'Ì', 'Ỉ', 'Ĩ', 'Ị'], // base 'I'
    ['O', 'Ó', 'Ò', 'Ỏ', 'Õ', 'Ọ'], // base 'O'
    ['Ô', 'Ố', 'Ồ', 'Ổ', 'Ỗ', 'Ộ'], // base 'Ô'
    ['Ơ', 'Ớ', 'Ờ', 'Ở', 'Ỡ', 'Ợ'], // base 'Ơ'
    ['U', 'Ú', 'Ù', 'Ủ', 'Ũ', 'Ụ'], // base 'U'
    ['Ư', 'Ứ', 'Ừ', 'Ử', 'Ữ', 'Ự'], // base 'Ư'
    ['Y', 'Ý', 'Ỳ', 'Ỷ', 'Ỹ', 'Ỵ'], // base 'Y'
];

// ── Row index lookup ─────────────────────────────────────────────────────────

/// Map a bare base vowel (lowercase, no tone) to a row index in TONE_TABLE.
/// Returns `None` for non-vowels.
#[inline]
fn vowel_row(base: char) -> Option<usize> {
    match base {
        'a' => Some(0),
        'ă' => Some(1),
        'â' => Some(2),
        'e' => Some(3),
        'ê' => Some(4),
        'i' => Some(5),
        'o' => Some(6),
        'ô' => Some(7),
        'ơ' => Some(8),
        'u' => Some(9),
        'ư' => Some(10),
        'y' => Some(11),
        _ => None,
    }
}

/// Map a `ToneMark` to a column index in TONE_TABLE.
#[inline]
fn tone_col(tone: ToneMark) -> usize {
    match tone {
        ToneMark::None => 0,
        ToneMark::Acute => 1,
        ToneMark::Grave => 2,
        ToneMark::Hook => 3,
        ToneMark::Tilde => 4,
        ToneMark::Dot => 5,
    }
}

// ── Strip (toned char → base + ToneMark) ─────────────────────────────────────

/// Strip a tone mark from any Vietnamese vowel.
///
/// Returns `(base_vowel, tone_mark)` where `base_vowel` preserves the original
/// case and has no tone diacritic.  Non-vowels are returned as-is with `ToneMark::None`.
///
/// ## Examples
///
/// ```
/// use buttre_engine::tone::strip;
/// use buttre_engine::pipeline::config::ToneMark;
///
/// let (base, tone) = strip('á');
/// assert_eq!(base, 'a');
/// assert_eq!(tone, ToneMark::Acute);
///
/// let (base, tone) = strip('Ấ');
/// assert_eq!(base, 'Â');
/// assert_eq!(tone, ToneMark::Acute);
/// ```
pub fn strip(toned: char) -> (char, ToneMark) {
    let lower = toned.to_lowercase().next().unwrap_or(toned);

    let (base_lower, tone) = match lower {
        // Acute
        'á' => ('a', ToneMark::Acute),
        'ắ' => ('ă', ToneMark::Acute),
        'ấ' => ('â', ToneMark::Acute),
        'é' => ('e', ToneMark::Acute),
        'ế' => ('ê', ToneMark::Acute),
        'í' => ('i', ToneMark::Acute),
        'ó' => ('o', ToneMark::Acute),
        'ố' => ('ô', ToneMark::Acute),
        'ớ' => ('ơ', ToneMark::Acute),
        'ú' => ('u', ToneMark::Acute),
        'ứ' => ('ư', ToneMark::Acute),
        'ý' => ('y', ToneMark::Acute),
        // Grave
        'à' => ('a', ToneMark::Grave),
        'ằ' => ('ă', ToneMark::Grave),
        'ầ' => ('â', ToneMark::Grave),
        'è' => ('e', ToneMark::Grave),
        'ề' => ('ê', ToneMark::Grave),
        'ì' => ('i', ToneMark::Grave),
        'ò' => ('o', ToneMark::Grave),
        'ồ' => ('ô', ToneMark::Grave),
        'ờ' => ('ơ', ToneMark::Grave),
        'ù' => ('u', ToneMark::Grave),
        'ừ' => ('ư', ToneMark::Grave),
        'ỳ' => ('y', ToneMark::Grave),
        // Hook
        'ả' => ('a', ToneMark::Hook),
        'ẳ' => ('ă', ToneMark::Hook),
        'ẩ' => ('â', ToneMark::Hook),
        'ẻ' => ('e', ToneMark::Hook),
        'ể' => ('ê', ToneMark::Hook),
        'ỉ' => ('i', ToneMark::Hook),
        'ỏ' => ('o', ToneMark::Hook),
        'ổ' => ('ô', ToneMark::Hook),
        'ở' => ('ơ', ToneMark::Hook),
        'ủ' => ('u', ToneMark::Hook),
        'ử' => ('ư', ToneMark::Hook),
        'ỷ' => ('y', ToneMark::Hook),
        // Tilde
        'ã' => ('a', ToneMark::Tilde),
        'ẵ' => ('ă', ToneMark::Tilde),
        'ẫ' => ('â', ToneMark::Tilde),
        'ẽ' => ('e', ToneMark::Tilde),
        'ễ' => ('ê', ToneMark::Tilde),
        'ĩ' => ('i', ToneMark::Tilde),
        'õ' => ('o', ToneMark::Tilde),
        'ỗ' => ('ô', ToneMark::Tilde),
        'ỡ' => ('ơ', ToneMark::Tilde),
        'ũ' => ('u', ToneMark::Tilde),
        'ữ' => ('ư', ToneMark::Tilde),
        'ỹ' => ('y', ToneMark::Tilde),
        // Dot
        'ạ' => ('a', ToneMark::Dot),
        'ặ' => ('ă', ToneMark::Dot),
        'ậ' => ('â', ToneMark::Dot),
        'ẹ' => ('e', ToneMark::Dot),
        'ệ' => ('ê', ToneMark::Dot),
        'ị' => ('i', ToneMark::Dot),
        'ọ' => ('o', ToneMark::Dot),
        'ộ' => ('ô', ToneMark::Dot),
        'ợ' => ('ơ', ToneMark::Dot),
        'ụ' => ('u', ToneMark::Dot),
        'ự' => ('ư', ToneMark::Dot),
        'ỵ' => ('y', ToneMark::Dot),
        // No tone / pass-through
        other => (other, ToneMark::None),
    };

    let base = if toned.is_uppercase() {
        base_lower.to_uppercase().next().unwrap_or(base_lower)
    } else {
        base_lower
    };

    (base, tone)
}

// ── Apply (base + ToneMark → toned char) ─────────────────────────────────────

/// Apply a tone mark to a vowel character, preserving case.
///
/// `base` may already carry a tone; it is stripped first, then the new tone is
/// applied.  `ToneMark::None` therefore returns the bare base vowel (no diacritic).
///
/// Non-vowel characters are returned unchanged.
///
/// ## Examples
///
/// ```
/// use buttre_engine::tone::apply;
/// use buttre_engine::pipeline::config::ToneMark;
///
/// assert_eq!(apply('a', ToneMark::Acute), 'á');
/// assert_eq!(apply('Ă', ToneMark::Grave), 'Ằ');
/// assert_eq!(apply('á', ToneMark::None),  'a');  // strips existing tone
/// assert_eq!(apply('x', ToneMark::Acute), 'x');  // non-vowel unchanged
/// ```
pub fn apply(base: char, tone: ToneMark) -> char {
    let is_upper = base.is_uppercase();
    let lower = base.to_lowercase().next().unwrap_or(base);

    // Strip any existing tone diacritic to get a clean base.
    let (bare_lower, _) = strip(lower);

    let row = match vowel_row(bare_lower) {
        Some(r) => r,
        None => return base, // not a vowel — leave unchanged
    };
    let col = tone_col(tone);

    if is_upper {
        TONE_TABLE_UPPER[row][col]
    } else {
        TONE_TABLE[row][col]
    }
}

// ── Unit tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pipeline::config::ToneMark;

    /// All 12 base vowels × 5 non-None tones × 2 cases must round-trip.
    #[test]
    fn round_trip_all_vowels_and_tones() {
        let bases_lower = ['a', 'ă', 'â', 'e', 'ê', 'i', 'o', 'ô', 'ơ', 'u', 'ư', 'y'];
        let tones = [
            ToneMark::Acute,
            ToneMark::Grave,
            ToneMark::Hook,
            ToneMark::Tilde,
            ToneMark::Dot,
        ];

        for base in bases_lower {
            for &tone in &tones {
                // Lowercase round-trip
                let toned = apply(base, tone);
                let (recovered_base, recovered_tone) = strip(toned);
                assert_eq!(
                    recovered_base, base,
                    "strip(apply('{base}', {tone:?})) base mismatch: got '{recovered_base}'"
                );
                assert_eq!(
                    recovered_tone, tone,
                    "strip(apply('{base}', {tone:?})) tone mismatch: got {recovered_tone:?}"
                );

                // Uppercase round-trip
                let base_upper = base.to_uppercase().next().unwrap();
                let toned_upper = apply(base_upper, tone);
                assert!(
                    toned_upper.is_uppercase(),
                    "apply('{base_upper}', {tone:?}) should be uppercase, got '{toned_upper}'"
                );
                let (recovered_base_upper, recovered_tone_upper) = strip(toned_upper);
                assert_eq!(
                    recovered_base_upper, base_upper,
                    "strip(apply('{base_upper}', {tone:?})) base mismatch"
                );
                assert_eq!(
                    recovered_tone_upper, tone,
                    "strip(apply('{base_upper}', {tone:?})) tone mismatch"
                );
            }
        }
    }

    #[test]
    fn apply_none_strips_existing_tone() {
        // apply with None should remove any existing tone
        assert_eq!(apply('á', ToneMark::None), 'a');
        assert_eq!(apply('ấ', ToneMark::None), 'â');
        assert_eq!(apply('ợ', ToneMark::None), 'ơ');
        assert_eq!(apply('Ắ', ToneMark::None), 'Ă');
    }

    #[test]
    fn apply_non_vowel_unchanged() {
        assert_eq!(apply('x', ToneMark::Acute), 'x');
        assert_eq!(apply('đ', ToneMark::Grave), 'đ');
        assert_eq!(apply(' ', ToneMark::Dot), ' ');
    }

    #[test]
    fn strip_already_bare_vowel() {
        let (base, tone) = strip('a');
        assert_eq!(base, 'a');
        assert_eq!(tone, ToneMark::None);
    }

    #[test]
    fn strip_uppercase_toned() {
        let (base, tone) = strip('Ấ');
        assert_eq!(base, 'Â');
        assert_eq!(tone, ToneMark::Acute);
    }

    #[test]
    fn apply_already_toned_replaces_tone() {
        // apply to an already-toned char should replace, not stack
        let result = apply('á', ToneMark::Grave);
        assert_eq!(result, 'à');
    }
}
