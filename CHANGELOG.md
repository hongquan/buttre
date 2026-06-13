# Changelog

All notable changes to buttre will be documented in this file.

## [Unreleased]

### Changed — Engine Architecture (Recompute Refactor)

- **Single recompute engine**: The dual-engine design (incremental Transform + Tone
  stages followed by Permutation + Reconciliation + Retrofix stages) has been replaced
  by a single `compose()` function that rebuilds the syllable from the raw key buffer
  on every keystroke.  No accumulated inter-stage state; pure and deterministic.
- **7-stage pipeline**: Stage count reduced from 12 to 7.  Stages 4–8 (Transform,
  Tone, Permutation, Reconciliation, Retrofix) are retired; their logic lives in
  `crates/buttre-engine/src/compose/`.
- **Tone module unified**: All tone-related tables and placement logic consolidated
  into `crates/buttre-engine/src/tone/`.  Single source of truth for tone char
  lookups; removes the former per-stage duplicated tables.
- **Behavior improvements**:
  - VNI `u7o7` compound vowel sequences now compose correctly in all orderings.
  - Validation-first English fallback: if the current raw buffer can't form a valid
    Vietnamese syllable after transforms, it falls back to raw Latin immediately
    rather than waiting for a duplicate-key undo trigger.
  - Transform-preserving undo: undo/toggle matching uses the transform record from
    `compose()`, which aligns with reference IME behavior (Unikey/OpenKey).
- **Generality**: One pipeline handles Telex, VNI, VIQR, and Hán Nôm via config;
  segment mode (`MarkBased` vs `DirectMap`) and validator (`Vietnamese`, `Hmong`,
  `Custom`, `None`) are config-selected rather than hard-coded.
- **Performance**: Recompute path is well under 1 ms/keystroke on all measured
  inputs (~250 ns – 8 µs/keystroke depending on syllable complexity).

### Added — macOS FFI
- **Punctuation keycodes**: `keycode_to_char` now covers all US ANSI punctuation (`-`, `=`, `[`, `]`, `\`, `;`, `'`, `,`, `.`, `/`, `` ` ``) and their shifted variants.
- **`buttre_engine_set_method(engine_id, method)`**: Switch telex (0) / vni (1) at runtime without re-creating the engine.
- **`buttre_engine_set_enabled(engine_id, enabled)`**: Disable the engine without freeing it; `process_key` returns null while disabled.

### Changed — macOS FFI (BREAKING ABI)
- **`buttre_engine_process_key`** gains a 4th parameter `capslock: bool`.
  - Uppercase now uses `capslock XOR shift`, matching system behavior (CapsLock+Shift = lowercase).
  - **Swift host call sites must be updated** to pass the fourth argument.

### Added — Linux IBus
- **CommitText / UpdatePreeditText / DeleteSurroundingText** D-Bus signals are now emitted (no longer stub TODOs).
- **Break-key detection**: Space, punctuation, arrows, Tab, Escape commit preedit and pass the key through.
- **Modifier-combo skip**: Ctrl+X / Alt+F / Super+… are passed through without engine interaction.
- **CapsLock XOR Shift**: Letter case correctly handles CapsLock state.
- **Method config**: `~/.config/buttre/method` selects `telex` (default) or `vni` on startup.
- **`LinuxBackend::init`** spawns a background thread with a dedicated tokio runtime running the IBus engine; `cleanup()` shuts it down cleanly.

### Added — Installers & CI
- **Windows MSI** (`installers/windows/`): `product.wxs` rewritten for buttre (correct CLSID, branding, keyboard files); `build_installer.ps1` uses `cargo wix --package buttre-platform`.
- **Linux packages** (`installers/linux/`): `.deb` and `.rpm` via `cargo-deb` + `cargo-generate-rpm`; IBus component XML (`buttre.xml`); `postinst`/`postrm` run `ibus write-cache --system`.
- **macOS developer artifact** (`installers/macos/`): `build_dylib.sh` produces a universal (x86_64 + arm64) `libbuttre_platform.dylib` zip; artifact README documents Gatekeeper quarantine workaround.
- **GitHub Actions release matrix** (`.github/workflows/release.yml`): 3-platform parallel build jobs; `workflow_dispatch` for dry-run testing; all artifacts uploaded to a single GH Release via `softprops/action-gh-release@v2`.

### Fixed — Installers
- **TSF CLSID mismatch**: `CLSID_buttre_TEXT_SERVICE` in `tsf/mod.rs` was a placeholder; now matches `registration.rs` (`{E6B8A6C0-1234-5678-9ABC-DEF012345678}`). Silent COM activation failure is resolved.

### Changed — CI
- **Release workflow** rewritten; deprecated `actions/create-release@v1` and `actions/upload-release-asset@v1` removed; Linux runner pinned to `ubuntu-22.04` (glibc 2.35) for broad compatibility.

---

## [0.6.2-alpha] - 2026-01-13

### Fixed
- **VNI "H20" Bug**: Prevented dropping digits in alphanumeric contexts (e.g., "H20" no longer becomes "H0").
- **Tone Logic**: Improved preservation of literal marks when no phonetic tone application is possible.
- **Security**: Refined digit penalty scoring to avoid incorrect character replacements.

### Changed
- **Cleanup**: Removed all internal debug `println!` and file-based logging for production build.

---

## [0.6.1-alpha] - 2026-01-10

### Added
- **Workflows**: Integrated agentic workflows (`/analyze`, `/debug`, `/audit`) for automated maintenance.

### Fixed
- **Backspace Sync**: Resolved desynchronization between platform history and engine buffer during cross-word deletion.
- **Separators**: Enhanced `is_separator` logic to cover a broader set of transitional characters.

---

## [0.6.0-alpha] - 2026-01-05

### 🎉 Major Milestone: The 6 Pillars
Completion of core architectural features defining the buttre next-gen Vietnamese engine.

1. **12-Stage Pipeline**: Modular, high-precision processing architecture.
2. **PGO Optimization**: Industry-leading latency (~1.0µs per keystroke).
3. **Flexible Typing**: Permutation-based logic for out-of-order mark placement.
4. **Cross-word Sync**: Reliable backspace and state management across boundaries.
5. **Hybrid Backend**: Concurrent LL Hook and TSF support for Windows apps.
6. **Retrofix & Undo**: Intelligent triple-toggle logic and automatic vowel upgrades.

---

## [0.2.0] - 2025-12-27

### Optimized
- **VNI Performance**: 3-phase optimization using pre-computed tone tables and range-based detection.
- **Engine Core**: Implemented PGO (Profile-Guided Optimization) reducing latency to world-class levels.

---

## [0.1.0] - 2025-12-19

### 🎉 Initial Release
First production-ready version of buttre Vietnamese IME.

- **Methods**: Telex, VNI, and Chữ Nôm support.
- **Platforms**: Windows (Hook + TSF), Linux (IBus), macOS (IMKit).
- **Features**: English fallback, Raw mode, Tone toggle, and Undo operations.
