//! Stage: Compose — recompute-from-raw core (Phase 4; evidence-based un-latch
//! added in Phase 2 of the event-sourcing-completion plan).
//!
//! Replaces stages 4 (Transform), 5 (Tone), 6 (Permutation), 7 (Reconciliation),
//! and 8 (Retrofix) with a single pure call to [`compose`].
//!
//! ## Contract
//!
//! - When `ctx.temp_english_mode` is `false` at entry: full recompute from the
//!   entire `ctx.char_buffer` via [`compose`].  The case mask is applied to the
//!   result so the output respects the user's capitalisation.
//!
//! - When `ctx.temp_english_mode` is `true` at entry: the buffer is in
//!   English/fallback mode (set by a previous compose call). This is DERIVED
//!   state, not a one-way valve (purity invariant, `AGENTS.md`) — every
//!   keystroke re-probes `compose(&full_raw)` and un-latches the instant the
//!   evidence says Vietnamese (see [`should_unlatch`]). There is no new
//!   persistent field for this: `temp_english_mode` itself is simply
//!   re-derived (set by `compose` as before, cleared here) rather than only
//!   ever growing more latched. Probing is gated by a pre-filter + a run-on
//!   cap exemption so the added cost stays bounded (see the latched branch
//!   below); when neither un-latches, we fall back to the cheap literal
//!   append that was the ENTIRE latched behavior before this phase.
//!
//! ## Case handling (normal mode)
//!
//! `compose` is case-agnostic: it receives lowercase chars and returns a
//! lowercase-anchored string.  We then apply the case mask from `char_buffer`:
//! - All chars uppercase → uppercase the whole result.
//! - Any leading uppercase → capitalise only the first output char.
//! - No uppercase → return as-is.

use std::sync::{Arc, RwLock};

use crate::compose::{compose, is_last_event_undo, ComposeOpts, ComposeResult};
use crate::pipeline::config::PipelineConfig;
use crate::pipeline::context::{CharInfo, CharInfoBufferExt};
use crate::pipeline::validation::is_attested_overlay;
use crate::pipeline::{PipelineStage, StageResult, TypingContext};

/// Maximum raw keystrokes a single Vietnamese syllable can occupy before the
/// recompute path treats the buffer as run-on input and falls back to literal
/// passthrough.  Set generously above the real maximum (~10: "nghieengf",
/// "truwowngf", VNI "nghie6ng2") so it never clips a legitimate syllable.
const MAX_VIET_SYLLABLE_RAW: usize = 16;

// Test-only instrumentation (red-team M2/M3, perf): counts how many times the
// latched branch actually ran a probe `compose()` call. Used to PROVE the
// pre-filter and run-on-cap exemption skip probing entirely for non-trigger
// keystrokes and for run-on buffers, rather than merely returning the right
// answer after doing the work anyway (mirrors the same idiom already used by
// `compose::fallback`'s `PREFIX_COMPOSE_CALLS`).
#[cfg(test)]
thread_local! {
    static PROBE_CALLS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// Stage: Compose (replaces stages 4–8).
///
/// Holds the LIVE, already-merged `ComposeOpts` behind one lock: the static
/// per-method baseline from `from_config`, with the two learning fields
/// (`user_attested`/`raw_prefs`, event-sourcing-completion Phase 5) written
/// in place by `PipelineExecutor::set_learning_snapshot` at word boundaries.
/// [`Self::process`] just takes a read guard — the previous design cloned
/// the whole `ComposeOpts` (≈10 Arc bumps + drops) on EVERY keystroke to
/// merge a snapshot that only ever changes at word boundaries; profiling
/// put the compose-stage wrapper at ~870 ns/key on top of `compose()`'s
/// ~450 ns, and this per-key clone was a major slice of it. `RwLock` (not
/// `RefCell`) because `PipelineStage: Send + Sync` — see `pipeline::stage`.
#[derive(Debug, Clone)]
pub struct ComposeStage {
    live_opts: Arc<RwLock<ComposeOpts>>,
}

impl ComposeStage {
    /// Build a `ComposeStage` from a pipeline configuration.
    pub fn from_config(config: &PipelineConfig) -> Self {
        Self {
            live_opts: Arc::new(RwLock::new(ComposeOpts::from_config(config))),
        }
    }

    /// Shared handle for `PipelineExecutor::new` to retain at construction
    /// time, before this stage is boxed into the type-erased
    /// `Vec<Box<dyn PipelineStage>>` (event-sourcing-completion Phase 5) —
    /// see `PipelineExecutor::set_learning_snapshot`, which writes the two
    /// learning fields directly into these live opts at word boundaries.
    pub(crate) fn live_opts_handle(&self) -> Arc<RwLock<ComposeOpts>> {
        Arc::clone(&self.live_opts)
    }
}

impl PipelineStage for ComposeStage {
    fn process(&self, ctx: &mut TypingContext, _input: char) -> StageResult {
        // Event-sourcing-completion Phase 5: the learning fields are already
        // merged into these live opts (written in place at word boundaries by
        // `PipelineExecutor::set_learning_snapshot`) — a read guard is all a
        // keystroke needs. Uncontended in practice: the IME pipeline runs a
        // keystroke to completion before any boundary-time snapshot write.
        let opts_guard = match self.live_opts.read() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let opts = &*opts_guard;

        // ── English / fallback: evidence-based un-latch (Phase 2) ─────────────
        // `temp_english_mode` is re-derived every keystroke, not a one-way
        // valve: Normalization (stage 1) has already populated `char_buffer`
        // in full by the time we get here, so the raw below is always
        // complete for this keystroke.
        if ctx.temp_english_mode {
            // Run-on cap exemption (perf guard, red-team M2/M3): the cap
            // below is a pure length rule and monotonic within a word (raw
            // only grows), so once it has already fired, every subsequent
            // keystroke is STILL run-on with zero chance of un-latching —
            // probing would be pure waste on a buffer this large.
            let past_cap = ctx.char_buffer.len() > MAX_VIET_SYLLABLE_RAW;

            // Trigger pre-filter (red-team M2/M3): probe ONLY when the
            // just-typed key could possibly matter — a tone key or a
            // transform trigger for this config. Mirrors the
            // `is_transform_trigger_char` idiom `compose::fallback` uses for
            // the same reason (re-implemented here, not imported, so this
            // stage does not reach into `fallback`'s private surface).
            // Plain letters — the overwhelming majority of keystrokes typed
            // while latched (`dessign`'s `ign`, `tissot`'s `t`) — fail this
            // O(1) check and skip probing entirely. Both checks run on
            // `char_buffer` directly so the `Vec<char>` raw copy (below) is
            // only ever built for the rare keystroke that actually probes.
            let is_trigger = ctx
                .char_buffer
                .last()
                .is_some_and(|c| is_probe_trigger_char(c.ch.to_ascii_lowercase(), opts));

            if !past_cap && is_trigger {
                #[cfg(test)]
                PROBE_CALLS.with(|c| c.set(c.get() + 1));

                // char_buffer is normalized-lowercase (see `to_char_vec`
                // doc) — the same projection `compose()` expects and every
                // raw_pos in `applied_marks`/`consumed_tone` indexes into.
                let raw: Vec<char> = ctx.char_buffer.to_char_vec();
                let probe = compose(&raw, opts);
                if should_unlatch(&probe, &raw, opts) {
                    // Evidence says Vietnamese: adopt the recompute and
                    // un-latch. Goes through the same case mask as the
                    // normal path so casing stays correct. OutputStage's
                    // diff emits the corrective Replace automatically.
                    ctx.syllable_buffer = apply_case_mask(&probe.text, &ctx.char_buffer, opts);
                    ctx.temp_english_mode = false;
                    return StageResult::Continue;
                }
            }

            // Cheap fallback: no evidence of Vietnamese (or probing was
            // skipped) — append the new key literally, exactly as before
            // this phase. Recomposing from raw here would re-interpret the
            // whole buffer and could overwrite an already-correct fallback
            // text with a spurious one.
            if let Some(last) = ctx.char_buffer.last() {
                // Preserve original case for the appended char.
                let ch = if last.is_uppercase {
                    last.ch.to_uppercase().next().unwrap_or(last.ch)
                } else {
                    last.ch
                };
                ctx.syllable_buffer.push(ch);
            }
            // temp_english_mode stays true — re-derived again next keystroke.
            return StageResult::Continue;
        }

        // ── Normal recompute path ─────────────────────────────────────────────
        // Extract lowercase raw keys from the char buffer Stage 1 populated.
        let raw: Vec<char> = ctx.char_buffer.to_char_vec(); // normalized (lowercase)

        // ── Defensive syllable-length cap ─────────────────────────────────────
        // A single Vietnamese syllable never exceeds ~10 raw keystrokes, even at
        // its longest with tone + transform keys ("nghieengf", "truwowngf",
        // VNI "nghie6ng2").  Past a generous cap the buffer is unavoidably run-on
        // input — multiple syllables typed with no separator.  Recomputing the
        // whole thing from raw on every keystroke is wasted O(n²) work, and it
        // makes the entire long buffer a single desync unit (one leaked/dropped
        // key corrupts all of it).  Switch to literal passthrough — the same path
        // temp_english uses — to bound both the cost and the blast radius.
        //
        // Skipped in Nôm multi-keyword candidate mode, where a space-joined query
        // ("thien thuong …") legitimately grows past the cap.
        if !ctx.showing_candidates
            && raw.len() > MAX_VIET_SYLLABLE_RAW
            && !raw.iter().any(|c| c.is_whitespace())
        {
            if let Some(last) = ctx.char_buffer.last() {
                let ch = if last.is_uppercase {
                    last.ch.to_uppercase().next().unwrap_or(last.ch)
                } else {
                    last.ch
                };
                ctx.syllable_buffer.push(ch);
            }
            // Latch passthrough so the rest of the run-on word also appends
            // literally until the next separator resets the engine.
            ctx.temp_english_mode = true;
            return StageResult::Continue;
        }

        // Run the pure recompute engine.
        let result = compose(&raw, opts);

        // Apply the case mask from the original keystrokes.
        let text = apply_case_mask(&result.text, &ctx.char_buffer, opts);

        ctx.syllable_buffer = text;
        ctx.temp_english_mode = result.temp_english;

        StageResult::Continue
    }

    fn name(&self) -> &'static str {
        "ComposeStage"
    }

    fn reset(&mut self) {
        // No internal state — compose is stateless.
    }
}

// ── Evidence-based un-latch (Phase 2) ──────────────────────────────────────────

/// True when `key_lc` (already lowercased) could possibly change an
/// evidence-based un-latch decision: either it is a tone key, or it can
/// trigger a transform mark (config-driven, no hardcoded key set).
///
/// Mirrors `compose::fallback::is_transform_trigger_char`'s idiom rather than
/// importing it, so this stage does not reach into `fallback`'s private
/// surface — same tables (`opts.transform_rules`/`opts.tone_map`), same O(1)
/// cost per keystroke.
fn is_probe_trigger_char(key_lc: char, opts: &ComposeOpts) -> bool {
    // Same key classes as the old full `transform_rules.keys()` rescan, via
    // the precomputed hot-path tables (`ComposeOpts::from_config`): a trigger
    // is the second char of a 2-char rule or the sole char of a 1-char rule.
    opts.tone_map.contains_key(&key_lc)
        || opts.single_rules.contains_key(&key_lc)
        || opts.pair_rules.keys().any(|&(_, b)| b == key_lc)
}

/// Evidence-based un-latch decision: given a fresh `probe = compose(&raw,
/// opts)` computed from the FULL raw buffer, decide whether a currently
/// LATCHED word should adopt the probe and clear `temp_english_mode`.
///
/// All four conditions must hold (strict, flip-flop-proof — phase-02 doc):
///
/// (a) the probe itself does not classify the word as English;
/// (b) the probe's text is EXACT-attested, OR-ing in the user-attested
///     overlay (`pipeline::validation::is_attested_overlay`,
///     event-sourcing-completion Phase 5 — the SAME consult point P3's
///     word-boundary closed gate uses, via `opts.user_attested`) — shape
///     alone is not enough, so an in-progress VNI intermediate form never
///     falsely un-latches;
/// (c) the keystroke that just fired — a transform mark or the consumed
///     tone — is pinned to the LAST raw position, never "whatever key was
///     physically typed" (red-team M5: position-independence would let an
///     unrelated EARLIER mark resurrect on a later, unrelated keystroke —
///     `"vieteje"`'s immediacy contract is the same principle applied to
///     the sibling undo check in `compose::fallback`). Covers both the
///     transform-mark path (`probe.applied_marks`, Telex letters AND VNI
///     digits alike — condition (c) does not care which) and the
///     tone-consumption path (`probe.consumed_tone`) so a plain literal
///     letter can never satisfy it (red-team m1) — only an ACTUALLY FIRED
///     mark or tone can.
/// (d) the word is NOT currently sitting in a just-fired undo/toggle state
///     per P6's last-event parity fold (`is_last_event_undo`), AND is not in
///     the "3+ repeated same key" toggle zone that fold cannot see (see
///     [`is_repeated_trigger_tap`]'s doc — a gap discovered while
///     implementing this phase, not part of P6's original fold). This is a
///     COUNTERFACTUAL replay (red-team M4): the fold's detectors never ran
///     while latched, so it may classify events the live session never
///     fired — that is the intended semantics (state = fold(log)), pinned
///     to reproduce today's detector outcomes (`dessign`, `a6116`, `seess`,
///     `vie65t5` — P6's table) by the executor regression suite.
///
/// (a) and the `is_last_event_undo` half of (d) happen to overlap in every
/// shipped config WITHOUT a P5 preference for this raw sequence: `probe` and
/// the fold both start from the identical `check_fallback(raw, opts, true)`
/// call, so `is_last_event_undo(raw) == true` already forces
/// `probe.temp_english == true`, which (a) alone would already reject. Now
/// that P5's preference lookup runs BEFORE the fallback checks (Combined
/// Contract, `compose::compose_internal` step 0), that implication no
/// longer holds universally — a raw sequence with a stored `Pref::Composed`
/// entry short-circuits past `check_fallback` entirely, so `probe` could
/// come back non-English while `is_last_event_undo` (which does NOT consult
/// prefs — it calls `compose::fallback::check_fallback` directly) still
/// says "undone". The explicit (d) check below is what keeps this function
/// correct in that case, not just a redundant belt-and-suspenders check.
fn should_unlatch(probe: &ComposeResult, raw: &[char], opts: &ComposeOpts) -> bool {
    if probe.temp_english {
        return false; // (a)
    }
    if !is_attested_overlay(&probe.text, opts.user_attested.as_deref()) {
        return false; // (b)
    }
    let Some(last_idx) = raw.len().checked_sub(1) else {
        return false;
    };
    let trigger_is_last = probe.applied_marks.iter().any(|m| m.raw_pos == last_idx)
        || probe.consumed_tone.is_some_and(|(_, pos)| pos == last_idx);
    if !trigger_is_last {
        return false; // (c)
    }
    if is_repeated_trigger_tap(raw) {
        return false; // (d), repeated-tap half
    }
    !is_last_event_undo(raw, opts) // (d), parity-fold half
}

/// True when the trailing raw characters (case-insensitive) repeat the SAME
/// key three or more times.
///
/// ## Why this closes a fold gap discovered while implementing P2
///
/// `check_tone_toggle`'s Path 1 explicitly defers ODD trailing runs ≥ 3 to
/// normal compose ("let normal compose handle") instead of classifying them
/// as an undo — a design choice that was invisible pre-P2 because the OLD
/// executor never re-ran `compose()` once latched, so the deferred tone
/// application never actually surfaced (`vni_a111_is_a11_not_reapply`'s own
/// comment: the literal `"a11"` — not the tone-applied `"á1"` — came from
/// the EXECUTOR's literal-append latch, not from `compose()`;
/// `test_multiple_tone_keys_after_fallback`'s `"tisssot"` is the same shape:
/// the 3rd `'s'` composes cleanly to attested `"tí"`, which would otherwise
/// pass (a)-(c) above and wrongly resurrect the tone Unikey says is spent).
/// `check_transform_toggle` has the analogous gap for a non-doubling
/// trigger (Telex `'w'`: `"awww"`'s tail `"www"` never matches the
/// `[rc1, rc2, rc2]` pattern, since `rc1` must be the DIFFERENT preceding
/// vowel, not another `'w'`). `is_last_event_undo` — built directly on
/// `check_fallback` — is blind to both, since neither detector ever
/// classifies these tails as handled.
///
/// A trailing run of exactly two is the NORMAL shape of a legitimate
/// adjacent doubling mark firing for the first time (`"oo"`, the second `'w'`
/// of `"aw"`+`'w'`) and must NOT be blocked. Three or more is always past
/// that: either an existing detector already classifies it as undone
/// (doubling transform triggers, even-parity tone runs — both go through
/// `is_last_event_undo` unaffected by this check), or it falls in the gap
/// above — either way, Unikey's "no re-apply" zone, never a fresh mark.
fn is_repeated_trigger_tap(raw: &[char]) -> bool {
    let Some(&last) = raw.last() else {
        return false;
    };
    let last_lc = last.to_ascii_lowercase();
    raw.iter()
        .rev()
        .take_while(|&&c| c.to_ascii_lowercase() == last_lc)
        .count()
        >= 3
}

// ── Case application ──────────────────────────────────────────────────────────

/// Apply the original case mask to a compose-produced (lowercase-anchored) string.
///
/// ## Algorithm
///
/// 1. No uppercase chars in buffer → return as-is (pure lowercase).
/// 2. Build a list of "case-bearing" input chars: those that are NOT
///    tone-map keys and NOT non-alphabetic transform triggers (VNI digits).
///    These are the chars whose case should influence the output.
/// 3. All case-bearing chars uppercase → full uppercase result.
/// 4. Otherwise: walk output chars left-to-right, assigning each the case of
///    the case-bearing input char at the same index, with two special rules:
///    - **Doubling collapse**: when two consecutive case-bearing chars have the
///      SAME character (e.g. `DD`, `AA`) they form a doubling transform and both
///      map to a single output char; consume both but use the first one's case.
///    - **Overflow**: output chars beyond the case-bearing list default to
///      lowercase.
///
/// This produces correct case for:
/// - All-caps (`NGUOI+f` → `NGƯỜI`)
/// - Mixed leading-caps (`NGuwow+f` → `NGười`)
/// - Title-case (`Nguow+f` → `Người`)
/// - Doubling transforms (`DDaay` → `Đây`, not `ĐÂy`)
/// - VNI digit triggers (`VIE65T` → `VIỆT`)
///
/// `pub(crate)` (not private): also called by `pipeline::executor`'s
/// word-boundary repair probe (phase-03-boundary-repair) — the repair diff
/// MUST be computed against this SAME case-masked display form, or a
/// mixed-case word like `"Vieejt"` would downcase to `"việt"` on repair.
pub(crate) fn apply_case_mask(text: &str, char_buffer: &[CharInfo], opts: &ComposeOpts) -> String {
    if text.is_empty() || char_buffer.is_empty() {
        return text.to_string();
    }

    let upper_count = char_buffer.iter().filter(|c| c.is_uppercase).count();
    if upper_count == 0 {
        return text.to_string();
    }

    // Build list of case-bearing chars.
    //
    // Strip:
    //   • Lowercase tone-map keys (e.g. Telex 's','f','r','x','j' when lowercase):
    //     These are tone markers and carry no content case.  When the user types
    //     uppercase 'R' they mean the consonant R, not the hook tone key 'r'.
    //   • Non-alphabetic transform triggers (e.g. VNI digits '6','7','8','9'):
    //     These are pure trigger characters with no inherent case.
    let case_chars: Vec<&CharInfo> = char_buffer
        .iter()
        .filter(|c| {
            let is_lowercase_tone_key = !c.is_uppercase && opts.tone_map.contains_key(&c.ch);
            let is_non_alpha_trigger = opts.transform_trigger_chars.contains(&c.ch);
            !is_lowercase_tone_key && !is_non_alpha_trigger
        })
        .collect();

    if case_chars.is_empty() {
        return text.to_string();
    }

    // Fast path: all case-bearing chars are uppercase → full uppercase result.
    if case_chars.iter().all(|c| c.is_uppercase) {
        return text.to_uppercase();
    }

    // Per-output-char case application with doubling-collapse.
    //
    // Walk case_chars with a cursor. For each output char:
    //   - Check if the current and next case_char are the same character
    //     (doubling, e.g. 'D'+'D' or 'A'+'A'). If so, use the first char's
    //     case and advance the cursor by 2.
    //   - Otherwise, use the current char's case and advance by 1.
    //   - If we run off the end of case_chars, output lowercase.
    let output_chars: Vec<char> = text.chars().collect();
    let mut result = String::with_capacity(text.len());
    let mut ci = 0usize; // index into case_chars

    for &out_ch in &output_chars {
        let is_upper = if ci < case_chars.len() {
            let cur = case_chars[ci];
            // Detect doubling: same char appears consecutively in case_chars.
            // This means two input chars merged into one output char.
            let is_doubling = ci + 1 < case_chars.len()
                && case_chars[ci + 1].ch == cur.ch
                && opts
                    .transform_rules
                    .contains_key(&format!("{0}{0}", cur.ch));
            if is_doubling {
                ci += 2; // consume both
            } else {
                ci += 1; // consume one
            }
            cur.is_uppercase
        } else {
            false
        };

        if is_upper {
            for uc in out_ch.to_uppercase() {
                result.push(uc);
            }
        } else {
            result.push(out_ch);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pipeline::config::PipelineConfig;
    use crate::pipeline::context::CharInfo;

    fn make_buf(s: &str) -> Vec<CharInfo> {
        s.chars().map(CharInfo::new).collect()
    }

    fn no_tone_opts() -> ComposeOpts {
        ComposeOpts::from_config(&PipelineConfig::new("test"))
    }

    fn telex_opts() -> ComposeOpts {
        ComposeOpts::from_config(&crate::pipeline::presets::telex_config())
    }

    #[test]
    fn lowercase_unchanged() {
        let buf = make_buf("aa");
        assert_eq!(apply_case_mask("â", &buf, &no_tone_opts()), "â");
    }

    #[test]
    fn first_upper_capitalizes_first_char() {
        // "Aa" — content chars: a(upper), a(lower) → NOT all-content-upper → first-cap
        let buf = vec![
            CharInfo::with_case('a', true),
            CharInfo::with_case('a', false),
        ];
        assert_eq!(apply_case_mask("â", &buf, &no_tone_opts()), "Â");
    }

    #[test]
    fn all_caps_uppercases_result() {
        let buf = vec![
            CharInfo::with_case('a', true),
            CharInfo::with_case('a', true),
        ];
        assert_eq!(apply_case_mask("â", &buf, &no_tone_opts()), "Â");
    }

    #[test]
    fn all_caps_multi_char() {
        let buf = vec![
            CharInfo::with_case('t', true),
            CharInfo::with_case('h', true),
            CharInfo::with_case('u', true),
        ];
        assert_eq!(apply_case_mask("thu", &buf, &no_tone_opts()), "THU");
    }

    #[test]
    fn mixed_case_per_char() {
        // T(upper), H(upper), u(lower): per-char alignment → TH uppercase, ư lowercase.
        // New algorithm: per-char with doubling-collapse; no doubling here → direct 1:1.
        let buf = vec![
            CharInfo::with_case('t', true),
            CharInfo::with_case('h', true),
            CharInfo::with_case('u', false),
        ];
        assert_eq!(apply_case_mask("thư", &buf, &no_tone_opts()), "THư");
    }

    #[test]
    fn mixed_case_title_only() {
        // T(upper), h(lower), u(lower) → only first char uppercase.
        let buf = vec![
            CharInfo::with_case('t', true),
            CharInfo::with_case('h', false),
            CharInfo::with_case('u', false),
        ];
        assert_eq!(apply_case_mask("thư", &buf, &no_tone_opts()), "Thư");
    }

    #[test]
    fn doubling_collapse_dd() {
        // DDa: D+D detected as doubling (transform_rules has "dd") → both D's consumed
        // for output[0]='Đ'; then 'a' at output[1] gets case_chars[2]=a(F) → lowercase.
        // Expected: Đa (not ĐA).
        let buf = vec![
            CharInfo::with_case('d', true),
            CharInfo::with_case('d', true),
            CharInfo::with_case('a', false),
        ];
        assert_eq!(
            apply_case_mask("đa", &buf, &telex_opts()),
            "Đa",
            "DDa: DD collapse → Đ; a stays lowercase"
        );
    }

    #[test]
    fn partial_caps_ng_prefix() {
        // NGuwowif → NGười: N+G are different chars (no doubling), so 1:1 → N(T)→N, G(T)→G.
        // Then remaining content chars u,w,o,w,i are lowercase → người → NGười.
        let buf = vec![
            CharInfo::with_case('n', true),
            CharInfo::with_case('g', true),
            CharInfo::with_case('u', false),
            CharInfo::with_case('w', false),
            CharInfo::with_case('o', false),
            CharInfo::with_case('w', false),
            CharInfo::with_case('i', false),
            CharInfo::with_case('f', false), // tone key — excluded from case_chars
        ];
        assert_eq!(
            apply_case_mask("người", &buf, &telex_opts()),
            "NGười",
            "NGuwowif: N+G prefix uppercase preserved"
        );
    }

    #[test]
    fn tone_key_not_counted_as_content() {
        // NGUOI + f (tone key) — content chars N,G,U,O,I are all upper → ALL-CAPS
        let buf = vec![
            CharInfo::with_case('n', true),
            CharInfo::with_case('g', true),
            CharInfo::with_case('u', true),
            CharInfo::with_case('o', true),
            CharInfo::with_case('i', true),
            CharInfo::with_case('f', false), // tone key
        ];
        assert_eq!(apply_case_mask("người", &buf, &telex_opts()), "NGƯỜI");
    }

    // ── Phase 2: evidence-based un-latch ───────────────────────────────────────

    fn vni_opts() -> ComposeOpts {
        ComposeOpts::from_config(&crate::pipeline::presets::vni_config())
    }

    fn raw(s: &str) -> Vec<char> {
        s.chars().collect()
    }

    fn mark(key: char, raw_pos: usize) -> crate::compose::AppliedMark {
        crate::compose::AppliedMark {
            key,
            raw_pos,
            non_adjacent: true,
        }
    }

    #[test]
    fn probe_trigger_char_plain_letter_never_triggers() {
        let opts = telex_opts();
        // 'i', 'g', 'n' never appear as a tone key or a transform-rule trigger
        // char in Telex — the overwhelming majority of latched keystrokes.
        for c in ['i', 'g', 'n', 'b', 'c', 't'] {
            assert!(
                !is_probe_trigger_char(c, &opts),
                "'{c}' must not be a probe trigger"
            );
        }
    }

    #[test]
    fn probe_trigger_char_tone_key_triggers() {
        let opts = telex_opts();
        for c in ['s', 'f', 'r', 'x', 'j'] {
            assert!(
                is_probe_trigger_char(c, &opts),
                "tone key '{c}' must be a probe trigger"
            );
        }
    }

    #[test]
    fn probe_trigger_char_transform_trigger_triggers() {
        let opts = telex_opts();
        // Second char of a 2-char doubling/compound rule (aa/aw/dd/ee/oo/ow/uw).
        for c in ['a', 'w', 'd', 'e', 'o'] {
            assert!(
                is_probe_trigger_char(c, &opts),
                "transform trigger '{c}' must be a probe trigger"
            );
        }
    }

    #[test]
    fn probe_trigger_char_vni_digit_triggers() {
        let opts = vni_opts();
        for c in ['6', '7', '8', '9'] {
            assert!(
                is_probe_trigger_char(c, &opts),
                "VNI digit '{c}' must be a probe trigger"
            );
        }
        // VNI tone digits are also probe triggers (tone_map, not transform_rules).
        for c in ['1', '2', '3', '4', '5'] {
            assert!(
                is_probe_trigger_char(c, &opts),
                "VNI tone digit '{c}' must be a probe trigger"
            );
        }
    }

    #[test]
    fn repeated_trigger_tap_two_is_not_repeated() {
        // A trailing run of exactly two is a legitimate first-time doubling
        // mark (e.g. "oo") — must never be treated as a spent toggle.
        assert!(!is_repeated_trigger_tap(&raw("boo")));
        assert!(!is_repeated_trigger_tap(&raw("a")));
        assert!(!is_repeated_trigger_tap(&raw("")));
    }

    #[test]
    fn repeated_trigger_tap_three_or_more_is_repeated() {
        assert!(
            is_repeated_trigger_tap(&raw("tisss")),
            "3x trailing 's' is the Unikey no-reapply zone"
        );
        assert!(
            is_repeated_trigger_tap(&raw("a1111")),
            "4x trailing '1' is also caught"
        );
        assert!(
            is_repeated_trigger_tap(&raw("awww")),
            "3x trailing 'w' (non-doubling trigger) is caught too"
        );
    }

    #[test]
    fn repeated_trigger_tap_case_insensitive() {
        assert!(
            is_repeated_trigger_tap(&raw("tiSsS")),
            "run detection must be case-insensitive"
        );
    }

    #[test]
    fn should_unlatch_fires_for_attested_mark_at_last_position() {
        // Mirrors the "vietje" flagship fix at the unit level.
        let opts = telex_opts();
        let r = ComposeResult {
            text: "việt".to_string(),
            temp_english: false,
            applied_marks: vec![mark('e', 5)],
            consumed_tone: None,
            demoted: false,
        };
        assert!(should_unlatch(&r, &raw("vietje"), &opts));
    }

    #[test]
    fn should_unlatch_digit_trigger_parity() {
        // Condition (c) is data-driven, not order/alphabet-dependent: a VNI
        // digit mark at the last raw position must un-latch exactly like a
        // Telex letter mark (matrix row: "method parity").
        let opts = vni_opts();
        let r = ComposeResult {
            text: "cân".to_string(),
            temp_english: false,
            applied_marks: vec![mark('6', 3)],
            consumed_tone: None,
            demoted: false,
        };
        assert!(should_unlatch(&r, &raw("can6"), &opts));
    }

    #[test]
    fn should_unlatch_via_consumed_tone_when_no_mark_fired() {
        // The tone-only half of condition (c): no transform mark fired, but
        // the tone key IS the last raw position.
        let opts = vni_opts();
        let r = ComposeResult {
            text: "cán".to_string(),
            temp_english: false,
            applied_marks: Vec::new(),
            consumed_tone: Some(('1', 3)),
            demoted: false,
        };
        assert!(should_unlatch(&r, &raw("can1"), &opts));
    }

    #[test]
    fn should_unlatch_rejects_when_probe_is_english() {
        let opts = telex_opts();
        let r = ComposeResult {
            text: "vietje".to_string(),
            temp_english: true, // (a) fails
            applied_marks: Vec::new(),
            consumed_tone: None,
            demoted: false,
        };
        assert!(!should_unlatch(&r, &raw("vietje"), &opts));
    }

    #[test]
    fn should_unlatch_rejects_unattested_shape() {
        let opts = telex_opts();
        let r = ComposeResult {
            text: "xyzzz".to_string(), // (b) fails: not attested
            temp_english: false,
            applied_marks: vec![mark('z', 4)],
            consumed_tone: None,
            demoted: false,
        };
        assert!(!should_unlatch(&r, &raw("xyzzz"), &opts));
    }

    #[test]
    fn should_unlatch_overlay_rescues_unattested_probe_text() {
        // Event-sourcing-completion Phase 5: condition (b) ORs in the
        // user-attested overlay via the SAME single consult point
        // (`is_attested_overlay`) P3's closed gate uses. Without an
        // overlay, "xyzzz" fails exactly like the test above; WITH it,
        // condition (b) must pass.
        use std::collections::HashSet;
        use std::sync::Arc;
        // A real decomposable-but-unattested shape ("dât") so the overlay
        // bit is meaningful — "xyzzz" above isn't a decomposable Vietnamese
        // shape at all (invalid onset/nucleus/coda), so it could never gain
        // an overlay bit either way.
        let (o, n, c, t) = crate::pipeline::validation::decompose_ids("dât")
            .expect("'dât' must be a decomposable shape");
        let mut overlay = HashSet::new();
        overlay.insert(crate::pipeline::validation::bit_index(o, n, c, t) as u32);
        let opts = ComposeOpts {
            user_attested: Some(Arc::new(overlay)),
            ..telex_opts()
        };
        let r = ComposeResult {
            text: "dât".to_string(),
            temp_english: false,
            applied_marks: vec![mark('a', 3)],
            consumed_tone: None,
            demoted: false,
        };
        assert!(
            should_unlatch(&r, &raw("data"), &opts),
            "overlay-attested text must satisfy condition (b)"
        );
        // Same probe, WITHOUT the overlay, must still be rejected.
        let plain_opts = telex_opts();
        assert!(!should_unlatch(&r, &raw("data"), &plain_opts));
    }

    #[test]
    fn should_unlatch_rejects_when_trigger_is_not_last_key() {
        // The mark fired earlier in the raw, not at the just-typed key —
        // condition (c), pinned to raw position (red-team M5).
        let opts = telex_opts();
        let r = ComposeResult {
            text: "việt".to_string(),
            temp_english: false,
            applied_marks: vec![mark('e', 3)], // raw_pos 3, but raw has 6 chars
            consumed_tone: None,
            demoted: false,
        };
        assert!(!should_unlatch(&r, &raw("vietje"), &opts));
    }

    #[test]
    fn should_unlatch_rejects_plain_letter_with_no_mark_or_tone() {
        // Red-team m1: an attested probe text with an empty applied_marks AND
        // no consumed_tone (a plain literal recompute with nothing fired)
        // must never satisfy (c), however coincidentally attested the text.
        let opts = telex_opts();
        let r = ComposeResult {
            text: "loan".to_string(),
            temp_english: false,
            applied_marks: Vec::new(),
            consumed_tone: None,
            demoted: false,
        };
        assert!(!should_unlatch(&r, &raw("loan"), &opts));
    }

    #[test]
    fn should_unlatch_rejects_repeated_tap_even_when_otherwise_eligible() {
        // "tisss": the 3rd 's' composes cleanly to attested "tí" with the
        // tone consumed at the last position — (a),(b),(c) all hold — but
        // Unikey's no-reapply-after-undo contract must still win.
        let opts = {
            let mut cfg = PipelineConfig::new("telex");
            cfg.add_tone('s', crate::pipeline::config::ToneMark::Acute);
            ComposeOpts::from_config(&cfg)
        };
        let r = ComposeResult {
            text: "tí".to_string(),
            temp_english: false,
            applied_marks: Vec::new(),
            consumed_tone: Some(('s', 4)),
            demoted: false,
        };
        assert!(!should_unlatch(&r, &raw("tisss"), &opts));
    }

    #[test]
    fn should_unlatch_rejects_undone_state_condition_d() {
        // "canaa": the tail IS a just-fired non-adjacent undo per the P6
        // parity fold, even though this synthetic probe pretends (a)-(c)
        // would otherwise pass.
        let opts = telex_opts();
        let r = ComposeResult {
            text: "cana".to_string(),
            temp_english: false, // artificially forced true→false to isolate (d)
            applied_marks: vec![mark('a', 4)],
            consumed_tone: None,
            demoted: false,
        };
        assert!(
            !should_unlatch(&r, &raw("canaa"), &opts),
            "condition (d) must veto even when (a)-(c) are synthetically satisfied"
        );
    }

    // ── Executor-level: probe instrumentation (red-team M2/M3) ────────────────
    // Prove — via call-count instrumentation, not just the returned answer —
    // that the pre-filter and run-on cap actually skip the probe `compose()`
    // call, rather than merely returning the right answer after doing it.

    fn reset_probe_calls() {
        PROBE_CALLS.with(|c| c.set(0));
    }

    fn probe_calls() -> usize {
        PROBE_CALLS.with(std::cell::Cell::get)
    }

    #[test]
    fn no_probe_for_non_trigger_key_while_latched() {
        use crate::pipeline::PipelineExecutor;
        let mut ex = PipelineExecutor::new(crate::pipeline::presets::telex_config());
        for ch in "dess".chars() {
            ex.process(ch);
        } // latches at the 2nd 's' ("des")
        assert!(ex.context().temp_english_mode);
        reset_probe_calls();
        for ch in "ign".chars() {
            ex.process(ch);
        } // none of i/g/n are triggers
        assert_eq!(
            probe_calls(),
            0,
            "plain letters must never trigger a probe compose"
        );
        assert_eq!(ex.context().syllable_buffer, "design");
    }

    #[test]
    fn probe_fires_for_trigger_key_while_latched() {
        use crate::pipeline::PipelineExecutor;
        let mut ex = PipelineExecutor::new(crate::pipeline::presets::telex_config());
        for ch in "vietj".chars() {
            ex.process(ch);
        } // latches ("vietj")
        assert!(ex.context().temp_english_mode);
        reset_probe_calls();
        ex.process('e'); // 'e' is a transform trigger — must probe
        assert!(
            probe_calls() > 0,
            "a trigger key must perform at least one probe compose"
        );
        assert_eq!(ex.context().syllable_buffer, "việt");
        assert!(!ex.context().temp_english_mode);
    }

    #[test]
    fn no_probe_past_run_on_cap() {
        use crate::pipeline::PipelineExecutor;
        let mut ex = PipelineExecutor::new(crate::pipeline::presets::telex_config());
        // 17 distinct consonants: trips the run-on cap and latches.
        for ch in "bcdfghklmnpqrtvzb".chars() {
            ex.process(ch);
        }
        assert!(ex.context().temp_english_mode, "run-on buffer must latch");
        reset_probe_calls();
        // Further trigger-eligible keys ('a', 's') must still never probe —
        // the cap exemption is monotonic once tripped.
        ex.process('a');
        ex.process('s');
        assert_eq!(
            probe_calls(),
            0,
            "past the run-on cap, probing must be fully exempt"
        );
    }
}
