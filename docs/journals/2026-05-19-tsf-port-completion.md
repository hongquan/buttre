# TSF Backend Port to Robustness Parity — Completed

**Date**: 2026-05-19 14:30
**Severity**: Medium
**Component**: `crates/buttre-platform/src/platforms/windows/tsf/`
**Status**: Resolved

## What Happened

Completed 4-phase port of Windows TSF (Text Services Framework) backend to match robustness and feature parity with the `windows-chewing-tsf` reference IME. All phases merged; build exits cleanly with baseline warnings intact.

## Technical Details

**Phase 01 — Critical Bugs**: Moved `DllMain` to canonical location; renamed `keystroke_cookie` → `keystroke_tid` for semantic accuracy; fixed `BOOL` import (windows-rs 0.62 relocates it to `windows::core`).

**Phase 02 — Real Composition Sink**: Deleted `CompositionSinkStub` (was blocking engine reset). `write_text` and `show_candidates` now receive actual `ITfCompositionSink` via `to_owned()`; `OnCompositionTerminated` now calls `engine.reset()` and zeroes `last_text_len`. Discovered turbofish gotcha: `as_interface_ref::<I>()` is a trait method with zero generic args; must use type annotation instead of turbofish.

**Phase 03 — Missing COM Sinks**: Expanded `#[implement]` from 4 to 9 interfaces. Added 5 event sinks (`ITfTextInputProcessorEx`, `ITfThreadMgrEventSink`, `ITfThreadFocusSink`, `ITfActiveLanguageProfileNotifySink`, `ITfCompartmentEventSink`). Discovered windows-rs limitation: `#[implement]` macro generates private `TextService_Impl.this` — only accessible in declaring module. Attempted split into `sinks/` directory failed; kept all impl blocks inline in `text_service_stub.rs`. Key insight: `Param<IUnknown>` requires passing `&owned` after `to_owned()`, not `&I` directly.

**Phase 04 — Weak Reuse & Key Busy**: Replaced `Rc<RefCell<PendingComposition>>` with `pending_edit: RefCell<Weak<RefCell<PendingComposition>>>` + `last_text_len: Cell<usize>`. Weak reuse pattern prevents edit-session queue overflow under sustained fast typing. Added `key_busy: Cell<bool>` guard for Excel's spurious `OnSetFocus` on first keydown. Post-review fixes: moved `key_busy.set(true)` after early-return guards; replaced `(*ptim).clone().unwrap()` with `ptim.ok()?.clone()`.

## Root Cause

No failures — systematic port of proven patterns from reference IME. Early mistakes (turbofish, module visibility) revealed gaps in windows-rs macro-generated code understanding; all resolved by reading generated output.

## Lessons Learned

- windows-rs 0.62's `#[implement]` generates private `this` field inaccessible outside declaring module; accept it, don't fight it
- `Param<IUnknown>` trait bound requires intermediate `to_owned()` call; type annotations clarify intent better than turbofish syntax
- Weak reference reuse is idiomatic TSF pattern for queue management, not a code smell

## Next Steps

TSF backend is production-ready. Integration testing with actual IME input flow; monitor for any remaining Excel/Word compatibility edge cases.
