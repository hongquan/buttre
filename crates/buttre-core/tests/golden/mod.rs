//! Golden regression harness — shared support for snapshot testing.
//!
//! ## Purpose
//!
//! Pins the current engine behaviour before the refactor so we can detect
//! regressions.  The two key primitives are:
//!
//! - [`replay`] — convert a flat `Vec<Action>` into the visible text a host
//!   app would show after applying them.
//! - [`type_sequence`] — drive a fresh `PipelineExecutor` with a key string,
//!   collect all actions and replay them.
//!
//! ## Composition mode
//!
//! Both Telex and VNI `build_config()` leave `pipeline.use_composition = false`
//! (the default), so the engine emits `Commit` / `Replace` actions only.
//! `UpdateComposition` is not emitted for these configs, and
//! `ConfirmComposition` only fires on a word-boundary flush — so snapshot
//! replay models committed text for `use_composition = false` configs.
//! `replay` handles all `Action` variants defensively to avoid panics if a
//! different config is passed, but it does not model in-progress composition
//! text (unterminated syllables under composition mode are not captured).

use buttre_engine::pipeline::{PipelineConfig, PipelineExecutor};
use buttre_engine::types::Action;

pub mod corpus_data;

// ============================================================================
// Corpus types
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

/// One snapshot case.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Case {
    /// Raw keystroke string fed to the engine.
    pub keys: &'static str,
    /// Classification for Phase 4 change management.
    pub tag: Tag,
}

impl Case {
    #[allow(dead_code)]
    pub const fn new(keys: &'static str, tag: Tag) -> Self {
        Self { keys, tag }
    }
}

// ============================================================================
// replay — simulate the host-app text buffer
// ============================================================================

/// Simulate the host-application text buffer by applying a sequence of engine
/// actions in order.
///
/// ## Semantics by action variant
///
/// | Variant                        | Effect on accumulator                             |
/// |-------------------------------|---------------------------------------------------|
/// | `Commit(s)`                   | append `s`                                        |
/// | `Replace { backspace_count, text }` | remove `backspace_count` chars from the end, then append `text` |
/// | `ConfirmComposition(s)`       | append `s` (composition confirmed)                |
/// | `UpdateComposition { text, .. }` | ignored — only committed text matters          |
/// | `DoNothing`                   | no-op                                             |
/// | `ShowCandidates` / `HideCandidates` | no-op (UI state only)                     |
///
/// ## Notes
///
/// - `backspace_count` operates on Unicode scalar values (chars), not bytes.
/// - If `backspace_count` exceeds the current buffer length the buffer is
///   cleared without underflowing.
pub fn replay(actions: &[Action]) -> String {
    let mut buf: Vec<char> = Vec::new();

    for action in actions {
        match action {
            Action::Commit(s) => {
                buf.extend(s.chars());
            }
            Action::Replace {
                backspace_count,
                text,
            } => {
                let remove = (*backspace_count).min(buf.len());
                buf.truncate(buf.len() - remove);
                buf.extend(text.chars());
            }
            Action::ConfirmComposition(s) => {
                // Composition confirmed: the text is now committed.
                buf.extend(s.chars());
            }
            // Preview only — committed text is captured via Commit/Replace/Confirm.
            Action::UpdateComposition { .. } => {}
            // UI events and no-ops.
            Action::DoNothing | Action::ShowCandidates { .. } | Action::HideCandidates => {}
        }
    }

    buf.into_iter().collect()
}

// ============================================================================
// type_sequence
// ============================================================================

/// Drive a fresh `PipelineExecutor` with `keys` and return the final visible
/// text produced.
///
/// Each character in `keys` — including spaces — is fed to `executor.process`.
/// Spaces trigger a word boundary / syllable reset inside the engine, but the
/// space itself is passed through as a `Commit(' ')` action, so the returned
/// string includes inter-word spaces exactly as a host app would see them.
///
/// The function collects *all* actions returned across every keystroke and
/// passes the full list to [`replay`].  This means multi-syllable input like
/// `"nguwowif theo"` is reconstructed correctly across the space-triggered
/// reset.
// This module is included via `#[path]` by two independent integration-test
// binaries (`golden_regression.rs`, which calls this, and
// `compose_isolation.rs`, which doesn't) — each binary compiles its own copy,
// so this is "unused" only from the latter's perspective.
#[allow(dead_code)]
pub fn type_sequence(config: PipelineConfig, keys: &str) -> String {
    let mut executor = PipelineExecutor::new(config);
    let mut all_actions: Vec<Action> = Vec::new();

    for ch in keys.chars() {
        let actions = executor.process(ch);
        all_actions.extend(actions);
    }

    replay(&all_actions)
}

// ============================================================================
// Unit tests for replay itself
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replay_pure_commit() {
        let actions = vec![
            Action::Commit("h".to_string()),
            Action::Commit("i".to_string()),
        ];
        assert_eq!(replay(&actions), "hi");
    }

    #[test]
    fn replay_replace_shrink() {
        // "th" → replace last 1 char with "ú" → "thú"
        let actions = vec![
            Action::Commit("t".to_string()),
            Action::Commit("h".to_string()),
            Action::Commit("u".to_string()),
            Action::Replace {
                backspace_count: 1,
                text: "ú".to_string(),
            },
        ];
        assert_eq!(replay(&actions), "thú");
    }

    #[test]
    fn replay_replace_grow() {
        // "a" → replace 1 with "aw" composite → "ă"
        let actions = vec![
            Action::Commit("a".to_string()),
            Action::Replace {
                backspace_count: 1,
                text: "ă".to_string(),
            },
        ];
        assert_eq!(replay(&actions), "ă");
    }

    #[test]
    fn replay_multi_syllable_with_space() {
        // First syllable committed, space committed, second syllable
        let actions = vec![
            Action::Commit("b".to_string()),
            Action::Commit("a".to_string()),
            Action::Commit("n".to_string()),
            // Space passthrough from engine
            Action::Commit(" ".to_string()),
            Action::Commit("t".to_string()),
            Action::Commit("a".to_string()),
            Action::Commit("y".to_string()),
        ];
        assert_eq!(replay(&actions), "ban tay");
    }

    #[test]
    fn replay_confirm_composition() {
        // If composition mode were used: UpdateComposition is ignored,
        // ConfirmComposition appends.
        let actions = vec![
            Action::UpdateComposition {
                text: "thu".to_string(),
                cursor: 3,
            },
            Action::UpdateComposition {
                text: "thú".to_string(),
                cursor: 3,
            },
            Action::ConfirmComposition("thú".to_string()),
        ];
        assert_eq!(replay(&actions), "thú");
    }

    #[test]
    fn replay_do_nothing_and_ui_events() {
        let actions = vec![
            Action::DoNothing,
            Action::Commit("ok".to_string()),
            Action::HideCandidates,
        ];
        assert_eq!(replay(&actions), "ok");
    }

    #[test]
    fn replay_backspace_does_not_underflow() {
        let actions = vec![Action::Replace {
            backspace_count: 100,
            text: "safe".to_string(),
        }];
        assert_eq!(replay(&actions), "safe");
    }
}
