# Project Changelog

> Significant architectural changes, feature additions, and bug fixes.
> For detailed release notes see the root [`CHANGELOG.md`](../CHANGELOG.md).

**Last Updated**: 2026-06-13

---

## 2026-06-13 — Recompute Engine Refactor (Phases 1–5)

### Summary

The buttre engine has been refactored from a dual-engine incremental pipeline to a
single **recompute-from-raw** `compose()` architecture.

### What changed

| Before | After |
|--------|-------|
| 12-stage pipeline | 7-stage pipeline |
| Incremental Transform stage (stage 4) | `compose::segment` + `compose::transform` |
| Incremental Tone stage (stage 5) | `compose::assemble::apply_tone` |
| Permutation stage (stage 6) | Eliminated: `compose()` handles all orderings |
| Reconciliation stage (stage 7) | Eliminated: single deterministic result |
| Retrofix stage (stage 8) | `compose::fallback::check_fallback` |
| Per-stage tone tables (duplicate) | `crates/buttre-engine/src/tone/` (unified) |

### Behavior improvements

- VNI `u7o7` compound vowel sequences compose correctly in all orderings.
- Validation-first English fallback: raw Latin output when no valid syllable is
  possible, without requiring a duplicate-key undo.
- Transform-preserving undo matching aligns with reference IME behavior.
- Config-selected `Validator` (`Vietnamese`, `Hmong`, `Custom`, `None`) and
  `SegmentMode` (`MarkBased` / `DirectMap`) enable a single pipeline for all input
  methods including non-Vietnamese native scripts.

### Performance

All measurements below the 1 ms/keystroke target by >200×:
- Single char: ~230–250 ns
- Simple transform (e.g. "aa"→"â"): ~550 ns
- Complex syllable (e.g. "người" 8 keys): ~5–6 µs per recompute call
- Full per-keystroke executor path: ~1–5 µs

See `.agents/260613-1204-recompute-engine-refactor/reports/perf-comparison.md`
for detailed benchmark numbers.

### Tests

Golden harness (22 tests), isolation tests, generality tests, and all
unit/integration tests remain green with zero regressions.

---

## 2026-05-19 — Windows TSF Port Completion

See `docs/journals/2026-05-19-tsf-port-completion.md`.

---

## 2026-01-13 — VNI "H20" Bug + Tone Logic Fixes

- Fixed digit-dropping in alphanumeric contexts (e.g. "H20" no longer → "H0").
- Improved tone mark preservation when no phonetic placement is possible.
- Removed all internal debug `println!` and file-based logging.

---

## 2026-01-05 — Version 0.6.0-alpha: The 6 Pillars

1. 12-stage pipeline (superseded by 7-stage in 2026-06-13 refactor).
2. PGO optimization (~1.0 µs per keystroke baseline).
3. Permutation-based flexible typing (superseded by recompute model).
4. Cross-word backspace sync.
5. Hybrid LL Hook + TSF backend (Windows).
6. Retrofix + undo logic (superseded by `compose::fallback`).
