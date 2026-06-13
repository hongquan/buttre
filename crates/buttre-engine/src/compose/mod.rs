//! Compose engine — pure recompute-from-raw core (Phase 3).
//!
//! ## Design
//!
//! One call to [`compose`] rebuilds the entire syllable from the raw key buffer.
//! No hidden state, no history flags. Each sub-module owns one pipeline step:
//!
//! | Step | Module | What it does |
//! |------|--------|-------------|
//! | Segment | [`segment`] | Raw keys → (base, transform marks, tone marks) |
//! | Transform | [`transform`] | Apply diacritic marks to base, validation-gated |
//! | Assemble | [`assemble`] | Place + apply tone mark onto nucleus |
//! | Fallback | [`fallback`] | Undo / toggle / English-fallback from key counts |
//!
//! ## Phase scope
//!
//! This module is a **pure library** — it does NOT touch `PipelineExecutor`
//! or any existing stage.  Wiring happens in Phase 4.

mod segment;
mod transform;
mod assemble;
mod fallback;

#[cfg(test)]
mod tests;

use std::collections::{HashMap, HashSet};
use crate::pipeline::config::{PipelineConfig, ToneMark, ToneStyle};

// Re-export public types only.
pub use segment::SegmentMode;

// ── Configuration ─────────────────────────────────────────────────────────────

/// Validator strategy for transformation gating.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Validator {
    /// Full Vietnamese syllable structure check via `SyllableStructure`.
    Vietnamese,
    /// Hmong syllable rules (placeholder — currently same gate as Vietnamese).
    Hmong,
    /// Custom validator registered in config — treated as no-gate for now.
    Custom,
    /// No validation gate (native scripts: Cham, Khmer, …).
    None,
}

/// All configuration needed for one `compose()` call.
///
/// Derived from `PipelineConfig` via `ComposeOpts::from_config`.
/// Does not borrow from config — owned copies of the tables.
#[derive(Debug, Clone)]
pub struct ComposeOpts {
    /// Raw key → result string (e.g. "aa" → "â").
    /// Keys are lowercase; lookup is done case-insensitively.
    pub transform_rules: HashMap<String, String>,

    /// Tone key → `ToneMark` (e.g. 's' → Acute for Telex, '1' → Acute for VNI).
    pub tone_map: HashMap<char, ToneMark>,

    /// Old vs. New tone positioning style.
    pub tone_style: ToneStyle,

    /// How raw keys are segmented into base + marks.
    pub segment_mode: SegmentMode,

    /// Validation strategy used when gating transforms.
    pub validator: Validator,

    /// `true` when `tone_map` is non-empty.  When `false`, the tone step is
    /// skipped entirely (DirectMap / native scripts).
    pub tone_enabled: bool,

    /// Non-alphabetic chars that act as transform triggers (e.g. VNI digits 6–9).
    ///
    /// Used by `apply_case_mask` in `ComposeStage` to exclude these chars from
    /// the "content chars" count, so `VIE65T` produces `VIỆT` (not `Việt`).
    /// Only non-alphabetic trigger chars are included; letter triggers (like Telex
    /// `w`, `a`) are omitted because they are content chars in their own right.
    pub transform_trigger_chars: HashSet<char>,
}

impl ComposeOpts {
    /// Derive `ComposeOpts` from a `PipelineConfig`.
    ///
    /// - `segment_mode`: `DirectMap` when `config.native_script_mode`, else `MarkBased`.
    /// - `validator`: parsed from `config.validation.syllable_structure`
    ///   ("vietnamese" → Vietnamese, "hmong" → Hmong, "none" → None, else Vietnamese).
    /// - `tone_enabled`: `!config.tone_map.is_empty()`.
    /// - `tone_style`: via `config.get_tone_style()`.
    pub fn from_config(config: &PipelineConfig) -> Self {
        let segment_mode = if config.native_script_mode {
            SegmentMode::DirectMap
        } else {
            SegmentMode::MarkBased
        };

        let validator = match config
            .validation
            .as_ref()
            .map(|v| v.syllable_structure.as_str())
            .unwrap_or("vietnamese")
        {
            "hmong"  => Validator::Hmong,
            "custom" => Validator::Custom,
            "none"   => Validator::None,
            _        => Validator::Vietnamese,
        };

        // Collect non-alphabetic transform trigger chars (e.g. VNI '6', '7', '8', '9').
        // Letter triggers (Telex 'w', 'a', etc.) are intentionally excluded — they are
        // also content chars and must not be stripped from the uppercase count.
        let transform_trigger_chars: HashSet<char> = config
            .transform_rules
            .keys()
            .filter_map(|k| k.chars().last())
            .filter(|c| !c.is_alphabetic())
            .collect();

        Self {
            transform_rules: config.transform_rules.clone(),
            tone_map: config.tone_map.clone(),
            tone_style: config.get_tone_style(),
            segment_mode,
            validator,
            tone_enabled: !config.tone_map.is_empty(),
            transform_trigger_chars,
        }
    }
}

// ── Output ────────────────────────────────────────────────────────────────────

/// Result of a single `compose()` call.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComposeResult {
    /// The syllable text after all transforms, tone placement, and fallback.
    pub text: String,

    /// `true` when the key sequence looks like an English/non-Vietnamese word
    /// (after undo/toggle detection).  Phase 4 executor uses this to passthrough.
    pub temp_english: bool,
}

// ── Entry point ───────────────────────────────────────────────────────────────

/// Recompute the syllable output from the full raw key buffer.
///
/// ## Contract
///
/// - Pure: no global state read or written, no I/O.
/// - Deterministic: same `raw` + `opts` always yields the same `ComposeResult`.
/// - `raw` may be empty; returns `ComposeResult { text: "", temp_english: false }`.
///
/// ## Steps (Vietnamese `MarkBased` mode)
///
/// 1. [`fallback::check_undo`] — detect double-key undo / toggle patterns first
///    so later steps never see the undo key as a real mark.
/// 2. [`segment::segment`] — split raw into (base, transforms, tones).
/// 3. [`transform::apply_transforms`] — apply diacritic marks, validation-gated.
/// 4. [`assemble::apply_tone`] — place + apply the last tone key (if any).
///
/// ## DirectMap mode (native scripts)
///
/// Segment returns the full base from the transform table; no tone step.
pub fn compose(raw: &[char], opts: &ComposeOpts) -> ComposeResult {
    if raw.is_empty() {
        return ComposeResult { text: String::new(), temp_english: false };
    }

    // Step 1 — fallback / undo detection (reads raw counts only).
    let fb = fallback::check_fallback(raw, opts);
    if fb.is_handled {
        return ComposeResult {
            text: fb.text,
            temp_english: fb.temp_english,
        };
    }

    // Step 2 — segment.
    let seg = segment::segment(raw, opts);

    // Step 3 — apply transform marks.
    let transformed = transform::apply_transforms(&seg.base, &seg.transforms, opts);

    // Step 4 — apply tone (skip for DirectMap / no tone_map).
    let text = if opts.tone_enabled && !seg.tones.is_empty() {
        // Vietnamese: last tone wins.
        let last_tone_key = *seg.tones.last().unwrap();
        assemble::apply_tone(&transformed, last_tone_key, opts)
            .unwrap_or(transformed)
    } else {
        transformed
    };

    ComposeResult { text, temp_english: false }
}
