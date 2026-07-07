//! Stage 9: Orthography
//!
//! **Tests**: Integration tests for this stage are located in `crates/buttre-engine/tests/stage9_orthography_tests.rs`.
//!
//! This stage normalizes tone position and Unicode form.
//!
//! ## Algorithm
//!
//! 1. Normalize tone position (old vs new orthography)
//! 2. Normalize Unicode form (NFC vs NFD)
//! 3. Ensure consistent output format
//!
//! ## Rationale
//!
//! Vietnamese orthography has two styles:
//! - Old style: "hoà", "toà" (tone on 'o')
//! - New style: "hòa", "tòa" (tone on 'a')
//!
//! Unicode has two normalization forms:
//! - NFC (Canonical Composition): "â" as single codepoint
//! - NFD (Canonical Decomposition): "a" + "^" as separate codepoints

use crate::pipeline::config::{ToneStyle, UnicodeForm};
use crate::pipeline::{PipelineConfig, PipelineStage, StageResult, TypingContext};

/// Stage 9: Orthography
///
/// Normalizes tone position and Unicode form.
///
/// ## Algorithm
///
/// This stage implements orthography normalization:
///
/// 1. **Tone Position Normalization** (Future):
///    - Check if using old or new orthography style
///    - Adjust tone position for special cases (hoa, toa, etc.)
///    - Example: "hoà" (old) → "hòa" (new)
///
/// 2. **Unicode Normalization**:
///    - Convert to NFC (single codepoint) or NFD (decomposed)
///    - Most systems prefer NFC for Vietnamese
///    - Example: "â" (NFC) vs "a" + "^" (NFD)
///
/// 3. **Consistency Check**:
///    - Ensure all characters use the same normalization
///    - Prevent mixing NFC and NFD in the same text
///
/// ## Example
///
/// ```text
/// Input: "hoà" (old style, NFD)
/// Output: "hòa" (new style, NFC)
/// ```
///
/// ## Future Enhancements
///
/// - Integrate with `buttre-vietnamese::rules::place_tone_with_context`
/// - Implement full tone position rules
/// - Support user preference for old/new style
#[derive(Debug, Clone)]
pub struct OrthographyStage {
    /// Tone style (old vs new)
    pub tone_style: ToneStyle,

    /// Unicode normalization form
    pub unicode_form: UnicodeForm,
}

impl OrthographyStage {
    /// Create a new orthography stage from config
    pub fn from_config(config: &PipelineConfig) -> Self {
        Self {
            tone_style: config.tone_style,
            unicode_form: config.unicode_form,
        }
    }

    /// Create a new orthography stage with custom settings
    pub fn new(tone_style: ToneStyle, unicode_form: UnicodeForm) -> Self {
        Self {
            tone_style,
            unicode_form,
        }
    }

    /// Normalize Unicode form
    ///
    /// ## Algorithm
    ///
    /// Converts text to the specified Unicode normalization form (NFC or NFD).
    ///
    /// - **NFC (Canonical Composition)**: Combines characters into single codepoints
    ///   - Example: "a" + "^" + "`" → "ầ" (single codepoint U+1EA7)
    ///   - Preferred for most systems and applications
    ///
    /// - **NFD (Canonical Decomposition)**: Separates into base + combining marks
    ///   - Example: "ầ" → "a" + "^" + "`" (3 codepoints)
    ///   - Useful for text processing and analysis
    pub fn normalize_unicode(&self, text: &str) -> String {
        use unicode_normalization::UnicodeNormalization;

        match self.unicode_form {
            UnicodeForm::NFC => {
                // Convert to NFC (Canonical Composition)
                text.nfc().collect::<String>()
            }
            UnicodeForm::NFD => {
                // Convert to NFD (Canonical Decomposition)
                text.nfd().collect::<String>()
            }
        }
    }

    /// Normalize tone position
    ///
    /// ## Algorithm
    ///
    /// Applies Vietnamese orthography rules for tone placement:
    ///
    /// **Old Style (Traditional)**:
    /// - "hoà" (tone on 'o')
    /// - "toà" (tone on 'o')
    /// - Generally places tone on first vowel in diphthongs
    ///
    /// **New Style (Modern)**:
    /// - "hòa" (tone on 'a')
    /// - "tòa" (tone on 'a')
    /// - Follows modern Vietnamese orthography rules
    /// - Places tone on main vowel nucleus
    ///
    /// For now, we don't modify tone position as the input from Stage 5
    /// already has correct tone placement. This stage is reserved for
    /// future enhancements like old/new style conversion.
    pub fn normalize_tone_position(&self, text: &str) -> String {
        // Algorithm:
        // The tone position is already correct from Stage 5 (Tone Processing)
        // which uses the same tone placement rules.
        //
        // Future enhancement: Implement old/new style conversion
        // by parsing the syllable, extracting tone, and re-applying
        // with different style settings.

        match self.tone_style {
            ToneStyle::New => {
                // Modern orthography is already applied in Stage 5
                text.to_string()
            }
            ToneStyle::Old => {
                // Old orthography would require re-parsing and re-applying tone
                // For now, just return as-is
                text.to_string()
            }
        }
    }

    /// Restore original case from case_mask
    ///
    /// ## Algorithm
    ///
    /// Applies uppercase from case_mask to the output text.
    ///
    /// **Case Mapping Strategy**:
    /// - If case_mask.len() == output.len(): direct 1:1 mapping
    /// - If case_mask.len() > output.len(): chars merged (e.g., aa→â)
    ///   → follow case of first char in merged pattern (UniKey behavior)
    /// - If case_mask.len() < output.len(): chars expanded (e.g., special cases)
    ///   → use last known case, default to lowercase
    ///
    /// ## Examples
    ///
    /// - Input: "NGUOI", mask: [T,T,T,T,T] → Output: "NGƯỜI"
    /// - Input: "Nguoi", mask: [T,F,F,F,F] → Output: "Người"
    /// - Input: "Aa" (→"â"), mask: [T,F] → Output: "Â" (follow first char case)
    pub fn restore_case(&self, text: &str, case_mask: &[bool]) -> String {
        if case_mask.is_empty() {
            return text.to_string();
        }

        let output_len = text.chars().count();
        let mask_len = case_mask.len();

        let chars: Vec<char> = text.chars().collect();
        let mut result = String::with_capacity(text.len());

        for (i, ch) in chars.iter().enumerate() {
            // Map output index to mask index
            //
            // KEY FIX: When mask_len > output_len (chars were merged, e.g. "DD" → "Đ")
            //
            // OLD (buggy): proportional mapping (i * mask_len) / output_len
            //   mask=[T,T,F] output="Đa" → i=0 maps to 0, i=1 maps to 1 → 'a' gets mask[1]=T (wrong!)
            //
            // NEW: Use first char's case for the merged portion, then direct mapping for rest
            //   - First output char gets mask[0] (for merged chars, follow first)
            //   - Remaining output chars get their corresponding mask entries from the END
            //
            // Example: "DDa" → "Đa", mask=[T,T,F]
            //   - i=0 ('Đ'): merged from DD, use mask[0] = T → 'Đ' ✓
            //   - i=1 ('a'): not merged, use mask[mask_len - (output_len - i)] = mask[3-1] = mask[2] = F → 'a' ✓
            let mask_idx = if mask_len > output_len && output_len > 0 {
                if i == 0 {
                    // First char in output - use first case from mask (for merged chars)
                    0
                } else {
                    // Remaining chars - map to trailing entries in mask
                    // i chars from end of output → i entries from end of mask
                    mask_len - (output_len - i)
                }
            } else {
                i
            };

            let is_upper = case_mask.get(mask_idx).copied().unwrap_or(false);

            if is_upper {
                // Convert to uppercase
                for upper_ch in ch.to_uppercase() {
                    result.push(upper_ch);
                }
            } else {
                result.push(*ch);
            }
        }

        result
    }
}

impl PipelineStage for OrthographyStage {
    fn process(&self, ctx: &mut TypingContext, _input: char) -> StageResult {
        use unicode_normalization::{is_nfc_quick, is_nfd_quick, IsNormalized};

        // Fast path (perf: this stage used to allocate two fresh Strings on
        // EVERY keystroke): ComposeStage builds its output from precomposed
        // table entries, so the buffer is virtually always already in the
        // target form — verify with the O(n) quick check and skip the
        // normalization allocations entirely when it is.
        //
        // COUPLING GUARD: skipping is valid ONLY while
        // `normalize_tone_position` remains an identity function (it is — see
        // its body: both styles return the input unchanged). If Old-style
        // tone conversion is ever implemented there, it is orthogonal to
        // Unicode form and must run REGARDLESS of this quick check — move it
        // out of the `!already_normalized` branch at that point.
        let already_normalized = match self.unicode_form {
            UnicodeForm::NFC => {
                matches!(is_nfc_quick(ctx.syllable_buffer.chars()), IsNormalized::Yes)
            }
            UnicodeForm::NFD => {
                matches!(is_nfd_quick(ctx.syllable_buffer.chars()), IsNormalized::Yes)
            }
        };

        if !already_normalized {
            // Step 1: Normalize tone position (if needed)
            let normalized_tone = self.normalize_tone_position(&ctx.syllable_buffer);

            // Step 2: Normalize Unicode form (if needed)
            let normalized_unicode = self.normalize_unicode(&normalized_tone);

            // Step 3: Update syllable buffer.
            // Note: Case restoration was previously done here via restore_case().
            // Since Phase 4, ComposeStage applies the correct per-char case mask before
            // this stage runs, so restore_case() must NOT be called here — it would
            // overwrite the already-cased output with incorrect mask-based mapping.
            ctx.syllable_buffer = normalized_unicode;
        }

        // Always continue to next stage
        StageResult::Continue
    }

    fn name(&self) -> &'static str {
        "OrthographyStage"
    }

    fn reset(&mut self) {
        // No internal state to reset
    }
}
