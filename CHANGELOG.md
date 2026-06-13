# Changelog

All notable changes to buttre are documented here. Format follows [Keep a Changelog](https://keepachangelog.com); versions follow SemVer.

## [Unreleased]

### Engine — recompute refactor (12 → 7 stages)
- Replaced the dual-engine design (incremental Transform/Tone + Permutation/Reconciliation/Retrofix) with one pure `compose()` that rebuilds the syllable from the raw key buffer each keystroke — no inter-stage state, fully deterministic. Stages 4–8 retired; logic now in `crates/buttre-engine/src/compose/`.
- Unified all tone tables and placement into `crates/buttre-engine/src/tone/` (single source of truth).
- One config-driven pipeline serves Telex, VNI, VIQR, and Nôm; segment mode (`MarkBased`/`DirectMap`) and validator (`Vietnamese`/`Hmong`/`Custom`/`None`) are config-selected, not hard-coded.
- Behavior: VNI `u7o7` compounds compose correctly in any order; validation-first English fallback (no duplicate-key trigger needed); undo preserves transforms, matching Unikey/OpenKey.
- Performance: ~250 ns–8 µs/keystroke (well under 1 ms).

### Platforms
- **macOS FFI (BREAKING ABI):** `buttre_engine_process_key` gains a 4th param `capslock: bool` (uppercase = `capslock XOR shift`) — **Swift hosts must update call sites**. Added full US-ANSI punctuation keycodes, runtime `set_method` (telex/vni), and `set_enabled`. Ported off the removed `vietnamese::methods` API to the `Keyboard` pipeline.
- **Linux IBus:** real CommitText / UpdatePreeditText / DeleteSurroundingText signals; break-key and modifier-combo passthrough; `CapsLock XOR Shift`; method selected via `~/.config/buttre/method`; background tokio runtime with clean shutdown. Ported to the `Keyboard` pipeline.

### Installers & CI
- Windows MSI (`cargo wix`), Linux `.deb`/`.rpm` (IBus component XML + cache refresh), macOS universal `.dylib` zip.
- GitHub Actions: 3-platform release matrix; Linux/macOS build+test jobs enabled; release workflow modernized (Linux pinned to `ubuntu-22.04`).
- Fixed: TSF `CLSID_buttre_TEXT_SERVICE` placeholder mismatch causing silent COM activation failure.

## [0.6.2-alpha] - 2026-01-13
- Fixed "H2O"-style digit drop in alphanumeric input; better literal-mark preservation; refined digit-penalty scoring.
- Removed debug logging from production builds.

## [0.6.1-alpha] - 2026-01-10
- Added agentic maintenance workflows.
- Fixed cross-word backspace desync; broadened separator detection.

## [0.6.0-alpha] - 2026-01-05
Core-architecture milestone: 12-stage pipeline, PGO (~1 µs/keystroke), flexible (permutation) typing, cross-word sync, hybrid Hook+TSF backend, retrofix/undo. _(The 12-stage pipeline was later superseded by the 7-stage recompute engine — see [Unreleased].)_

## [0.2.0] - 2025-12-27
- VNI performance: precomputed tone tables + range-based detection; PGO engine core.

## [0.1.0] - 2025-12-19
Initial release. Methods: Telex, VNI, Nôm. Platforms: Windows (Hook+TSF), Linux (IBus), macOS. Features: English fallback, raw mode, tone toggle, undo.
