use buttre_core::keyboard::{Keyboard, Config, KeyboardBuilder};
use buttre_engine::pipeline::presets::vni_config;

#[test]
fn test_keyboard_creation() {
    let toml = r#"
[metadata]
id = "test"
name = "Test"
language = "vietnamese"

[transformations]
"aa" = "â"

[tones]
"s" = "acute"

[rules]
tone_position = "modern"
"#;
    
    // Test with old way converted to new way
    let config = Config::from_str(toml).unwrap();
    let pipeline_config = config.to_pipeline_config();
    let keyboard = Keyboard::new(pipeline_config);
    assert!(keyboard.is_ok());
}

#[test]
fn test_thuowr_via_keyboard() {
    let mut keyboard = KeyboardBuilder::telex().unwrap();
    
    for ch in "thuowr".chars() {
        keyboard.process(ch).unwrap();
    }
    
    assert_eq!(keyboard.buffer(), "thuở", "thuowr should produce thuở");
}

// ── Backspace: grapheme-aware, keeps the word editable, no desync ─────────────

#[test]
fn test_backspace_deletes_grapheme_keeps_tone() {
    use buttre_core::Action;
    let mut kb = KeyboardBuilder::telex().unwrap();

    // "vieetj" → "việt".  Backspace deletes the last grapheme 't' but KEEPS the
    // tone → "việ" (raw order ≠ display order: the tone key 'j' is typed last).
    for ch in "vieetj".chars() {
        kb.process(ch).unwrap();
    }
    assert_eq!(kb.buffer(), "việt");

    match kb.backspace().unwrap() {
        Action::Replace { backspace_count, text } => {
            assert_eq!(backspace_count, 1, "exactly one displayed char deleted");
            assert_eq!(text, "");
        }
        other => panic!("expected Replace{{1,\"\"}}, got {other:?}"),
    }
    assert_eq!(kb.buffer(), "việ", "tone preserved; only final consonant removed");

    // Composition stays alive: a tone key now re-tones the edited word.
    kb.process('s').unwrap();
    assert_eq!(kb.buffer(), "viế", "re-toning after backspace works");
}

#[test]
fn test_backspace_no_desync_then_fresh_word() {
    use buttre_core::Action;
    let mut kb = KeyboardBuilder::telex().unwrap();
    for ch in "ngayf".chars() {
        kb.process(ch).unwrap();
    }
    assert_eq!(kb.buffer(), "ngày");
    // Each backspace deletes exactly one displayed grapheme — no over-deletion
    // reaching into a previous word.
    assert!(matches!(kb.backspace().unwrap(), Action::Replace { backspace_count: 1, .. }));
    assert_eq!(kb.buffer(), "ngà");
    assert!(matches!(kb.backspace().unwrap(), Action::Replace { backspace_count: 1, .. }));
    assert_eq!(kb.buffer(), "ng");
}

// ── Multi-word rolling window: edit/re-tone a previous word (Cách B) ───────────

#[test]
fn test_multiword_retone_previous_word() {
    let mut kb = KeyboardBuilder::telex().unwrap();
    for ch in "ban cas".chars() {
        kb.process(ch).unwrap();
    }
    assert_eq!(kb.buffer(), "ban cá");
    // Backspace across the space, deleting the second word entirely.
    kb.backspace().unwrap(); // "ban c"
    kb.backspace().unwrap(); // "ban "
    kb.backspace().unwrap(); // "ban"
    assert_eq!(kb.buffer(), "ban");
    // The previous word is still composable: apply a tone to it.
    kb.process('f').unwrap();
    assert_eq!(kb.buffer(), "bàn", "must re-tone the previous word after backspace");
}

#[test]
fn test_multiword_window_cap_freezes_oldest() {
    // Window keeps the last 3 words; a 4th word scrolls the oldest into the
    // frozen prefix (still shown, no longer recomposed).
    let mut kb = KeyboardBuilder::telex().unwrap();
    for ch in "mot hai ba bon".chars() {
        kb.process(ch).unwrap();
    }
    assert_eq!(kb.buffer(), "mot hai ba bon");
}

// ── Phase 5: Keyboard-level performance check (red-team M4) ──────────────────
//
// `compose_bench` (buttre-engine) measures a single `compose()` call in
// isolation. The Hook backend's real per-keystroke cost is higher:
// `Keyboard::process_multiword` recomposes the ENTIRE rolling window (up to
// `MAX_WINDOW_WORDS` words) on every keystroke, and each word inside the
// window may independently pay the attestation-gate's demote-and-recompose
// cost (see `compose::compose_internal`). A worst-case English input that
// repeatedly triggers BOTH the gate's demote path AND window scrolling
// exercises what compose_bench alone cannot see. Repeating the flagship
// `"data"` case is the most direct worst case: every occurrence independently
// fires the non-adjacent gate, demotes, and recomposes, while the 4-char word
// length keeps the window scrolling on almost every keystroke (>3 words).
//
// This is NOT a tight perf gate (see phase-05-regression-suite.md Risk
// Notes) — it records real numbers and asserts only a generous sanity
// ceiling, so a genuine algorithmic regression (e.g. an accidental
// O(n^2)/O(n^3) reintroduced upstream) fails loudly without making CI flaky
// on ordinary machine variance. See phase-05-regression-suite.md for the
// actual measured numbers on the reference machine.
#[test]
fn test_keyboard_multiword_worst_case_perf() {
    use std::time::Instant;

    // 8 repetitions of the flagship gate/demote case, space-separated: the
    // window (cap 3 words) scrolls on almost every subsequent word boundary.
    let typing_input = ["data"; 8].join(" ");
    let keystroke_count = typing_input.chars().count();

    let mut kb = KeyboardBuilder::telex().expect("telex keyboard");
    let start = Instant::now();
    for ch in typing_input.chars() {
        kb.process(ch).expect("process must not error");
    }
    let typing_elapsed = start.elapsed();
    let per_keystroke = typing_elapsed / keystroke_count as u32;

    // Backspace storm: delete everything just typed, one displayed grapheme
    // at a time — the worst case for `find_window_backspace_raw`'s
    // remove-one-key subset search over the live window.
    let start = Instant::now();
    for _ in 0..keystroke_count {
        kb.backspace().expect("backspace must not error");
    }
    let backspace_elapsed = start.elapsed();
    let per_backspace = backspace_elapsed / keystroke_count as u32;

    eprintln!(
        "[perf] keyboard multiword worst-case ({keystroke_count} keystrokes of \"{typing_input}\"): \
         typing {typing_elapsed:?} total ({per_keystroke:?}/keystroke); \
         backspace storm {backspace_elapsed:?} total ({per_backspace:?}/backspace)"
    );

    // Loose sanity ceilings only — see the doc comment above.
    assert!(
        per_keystroke.as_millis() < 5,
        "typing got unexpectedly slow: {per_keystroke:?}/keystroke (budget: well under 1ms typical)"
    );
    assert!(
        per_backspace.as_millis() < 20,
        "backspace storm got unexpectedly slow: {per_backspace:?}/backspace"
    );
}

// ── Phase 3: word-boundary final repair — multiword (Hook) delivery ─────────
// Test Scenario Matrix from phase-03-boundary-repair.md, replayed through the
// REAL Hook-backend code path (`Keyboard::process` → `process_multiword` →
// `compose_window`/`compose_one_word`/`diff_to_action`) rather than a mock —
// this is the same mechanism `hook.rs` drives via `send_replacement`.

fn type_str(kb: &mut Keyboard, s: &str) {
    for ch in s.chars() {
        kb.process(ch).expect("process must not error");
    }
}

#[test]
fn boundary_repair_vni_nhat6_space_restores_literal() {
    let mut kb = KeyboardBuilder::vni().expect("vni keyboard");
    type_str(&mut kb, "nhat6 ");
    assert_eq!(kb.buffer(), "nhat6 ", "shape-only inferred mark must repair to literal raw at the boundary");
}

#[test]
fn boundary_repair_vni_nhat61_space_untouched_exact_attested() {
    let mut kb = KeyboardBuilder::vni().expect("vni keyboard");
    type_str(&mut kb, "nhat61 ");
    assert_eq!(kb.buffer(), "nhất ", "exact-attested word must be untouched");
}

#[test]
fn boundary_repair_telex_vietej_space_untouched_exact_path() {
    let mut kb = KeyboardBuilder::telex().expect("telex keyboard");
    type_str(&mut kb, "vietej ");
    assert_eq!(kb.buffer(), "việt ", "Telex's exact-attestation path is already correct");
}

#[test]
fn boundary_repair_data_space_no_double_repair() {
    let mut kb = KeyboardBuilder::telex().expect("telex keyboard");
    type_str(&mut kb, "data ");
    assert_eq!(kb.buffer(), "data ", "already-literal word must not be touched again");
}

#[test]
fn boundary_repair_reset_space_accepted_collision_untouched() {
    let mut kb = KeyboardBuilder::telex().expect("telex keyboard");
    type_str(&mut kb, "reset ");
    assert_eq!(kb.buffer(), "rết ", "exact-attested collision must not be repaired");
}

#[test]
fn boundary_repair_adjacent_vieet_space_never_repaired() {
    let mut kb = KeyboardBuilder::telex().expect("telex keyboard");
    type_str(&mut kb, "vieet ");
    assert_eq!(kb.buffer(), "viêt ", "direct/adjacent typing carries no inferred mark, never repaired");
}

#[test]
fn boundary_repair_case_masked_diff_vieejt_space() {
    // Red-team M2: the repair diff must be computed against the CASE-MASKED
    // display form. "Vieejt" is already exact-attested ("Việt"), so this
    // also serves as a case-preservation regression guard for the no-op path.
    let mut kb = KeyboardBuilder::telex().expect("telex keyboard");
    type_str(&mut kb, "Vieejt ");
    assert_eq!(kb.buffer(), "Việt ", "case must survive the boundary-repair probe");
}

#[test]
fn boundary_repair_disabled_flag_keeps_old_behavior() {
    let mut config = vni_config();
    config.boundary_repair = false;
    let mut kb = KeyboardBuilder::new()
        .with_pipeline_config(config)
        .build()
        .expect("vni keyboard with boundary_repair disabled");
    type_str(&mut kb, "nhat6 ");
    assert_eq!(kb.buffer(), "nhât ", "boundary_repair=false must reproduce the old shape-attested-only behavior exactly");
}

#[test]
fn boundary_repair_multiword_reopens_on_backspace_over_separator() {
    // Bidirectional projection: a closed word's repair is NOT a one-way
    // ratchet — backspacing the separator away re-opens the word and the
    // shape-attested intermediate reappears automatically (the very next
    // window recompute simply sees `closed = false` for it again).
    let mut kb = KeyboardBuilder::vni().expect("vni keyboard");
    type_str(&mut kb, "nhat6 xin");
    assert_eq!(kb.buffer(), "nhat6 xin", "word closed by the separator is repaired while composing the rest");

    // Backspace "xin" away, then backspace over the separator itself.
    kb.backspace().unwrap(); // "nhat6 xi"
    kb.backspace().unwrap(); // "nhat6 x"
    kb.backspace().unwrap(); // "nhat6 "
    assert_eq!(kb.buffer(), "nhat6 ");
    kb.backspace().unwrap(); // removes the separator — word re-opens
    assert_eq!(kb.buffer(), "nhât", "re-opened word must show the live per-keystroke (open) projection again");
}

#[test]
fn boundary_repair_noop_after_p2_unlatch_single_word() {
    // Interaction with Phase 2: "vietje" un-latches mid-word to the
    // exact-attested "việt" — boundary repair at the following separator
    // must be a complete no-op (no double-Replace, no flicker back to any
    // literal form).
    let mut kb = KeyboardBuilder::telex().expect("telex keyboard");
    type_str(&mut kb, "vietj");
    // Still mid-word: "vietj" alone has not unlatched yet.
    type_str(&mut kb, "e ");
    assert_eq!(kb.buffer(), "việt ", "P2 un-latch result must survive the boundary-repair probe untouched");
}
