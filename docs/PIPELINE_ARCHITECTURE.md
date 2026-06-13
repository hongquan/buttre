# buttre Engine — 7-Stage Processing Pipeline

**Last Updated**: 2026-06-13

## Overview

buttre Engine uses a 7-stage config-driven pipeline to process Vietnamese input.
Each stage has a single responsibility and returns a `StageResult` that either
continues, short-circuits to passthrough, or emits output actions directly.

The pipeline replaced a former dual-engine design (incremental Transform/Tone
stages + Permutation/Reconciliation/Retrofix stages) with a single
**recompute-from-raw** `ComposeStage`.  Every keystroke recomputes the syllable
from the full raw buffer — no accumulated stage-to-stage state is carried forward
inside the composition logic.

---

## Stage Diagram

```
┌──────────────────────────────────────────────────────┐
│                    INPUT CHARACTER                   │
│                         ↓                            │
│  ┌────────────────────────────────────────────────┐  │
│  │  Stage 1: NORMALIZATION                        │  │
│  │  • Normalize case; push CharInfo to buffer     │  │
│  └────────────────────────────────────────────────┘  │
│                         ↓                            │
│  ┌────────────────────────────────────────────────┐  │
│  │  Stage 2: GATEKEEPER                           │  │
│  │  • Check temp_english_mode → PassThrough       │  │
│  │  • Non-alphabetic → PassThrough (commit word)  │  │
│  └────────────────────────────────────────────────┘  │
│                         ↓                            │
│  ┌────────────────────────────────────────────────┐  │
│  │  Stage 3: COMPOSE  (recompute-from-raw)        │  │
│  │  • segment: base + transform marks + tone keys │  │
│  │  • transform: apply diacritic marks, gated by  │  │
│  │    Vietnamese syllable validator               │  │
│  │  • tone: place tone mark on vowel nucleus      │  │
│  │  • fallback: undo / toggle / English detection │  │
│  │  Writes syllable_buffer; sets temp_english     │  │
│  └────────────────────────────────────────────────┘  │
│                         ↓                            │
│  ┌────────────────────────────────────────────────┐  │
│  │  Stage 4: ORTHOGRAPHY                          │  │
│  │  • Normalize tone position (old/new style)     │  │
│  │  • Unicode NFC normalization                   │  │
│  └────────────────────────────────────────────────┘  │
│                         ↓                            │
│  ┌────────────────────────────────────────────────┐  │
│  │  Stage 5: LEARNING  (no-op until future phase) │  │
│  │  • Track completed syllables for adaptation    │  │
│  └────────────────────────────────────────────────┘  │
│                         ↓                            │
│  ┌────────────────────────────────────────────────┐  │
│  │  Stage 6: LOOKUP                               │  │
│  │  • Optional Hán Nôm dictionary lookup          │  │
│  │  • Populates TypingContext::candidates          │  │
│  └────────────────────────────────────────────────┘  │
│                         ↓                            │
│  ┌────────────────────────────────────────────────┐  │
│  │  Stage 7: OUTPUT                               │  │
│  │  • Diff last_output vs syllable_buffer         │  │
│  │  • Emit Replace{backspace_count, text} action  │  │
│  └────────────────────────────────────────────────┘  │
│                         ↓                            │
│                    OUTPUT ACTIONS                    │
└──────────────────────────────────────────────────────┘
```

---

## Flow Control

Each stage returns a `StageResult`:

```rust
enum StageResult {
    Continue,              // Proceed to next stage
    PassThrough,           // Commit any in-progress composition; commit raw char; reset
    Output(Vec<Action>),   // Short-circuit; return these actions immediately
}
```

---

## Stage Detail

### Stage 1: Normalization

**Purpose**: Normalize the input character and add it to the char buffer.

- Converts the input character to lowercase (stores uppercase flag in `CharInfo`).
- Appends `CharInfo` to `TypingContext::char_buffer`.
- Always returns `Continue`.

---

### Stage 2: Gatekeeper

**Purpose**: Route non-Vietnamese input without touching the composition logic.

- If `temp_english_mode` is true → `PassThrough` (sends raw char as-is).
- If the character is non-alphabetic (space, punctuation, digit) → `PassThrough`
  (commits any pending syllable, then sends the character).
- Otherwise → `Continue`.

---

### Stage 3: ComposeStage (recompute-from-raw)

**Purpose**: Rebuild the entire syllable from `char_buffer` on every keystroke.

This is the heart of the pipeline.  It calls `compose::compose(raw, opts)` and
writes the result to `context.syllable_buffer`.

#### Internal steps of `compose()`

| Step | Module | What it does |
|------|--------|--------------|
| 1 | `fallback::check_fallback` | Detect undo / toggle patterns from key counts.  Returns early if handled. |
| 2 | `segment::segment` | Split raw buffer into (base chars, transform mark keys, tone keys). |
| 3 | `transform::apply_transforms` | Apply diacritic marks to base; gated by Vietnamese syllable validator. |
| 4 | `assemble::apply_tone` | Place and apply the last tone mark onto the vowel nucleus. |

#### Superset model

| Axis | Options |
|------|---------|
| `SegmentMode` | `MarkBased` (Telex/VNI) · `DirectMap` (native scripts) |
| `Validator` | `Vietnamese` · `Hmong` · `Custom` · `None` |
| `tone_enabled` | `true` when tone_map is non-empty; `false` skips tone step |
| `ToneStyle` | `Old` (óa placement) · `New` (oá placement, default) |

`ComposeStage` also applies a case mask after `compose()` returns: uppercase flags
from `char_buffer` are mapped back onto the output text.

---

### Stage 4: Orthography

**Purpose**: Ensure the syllable is in canonical Unicode form.

- Applies `ToneStyle`-based tone position normalization when the config requests it.
- Converts to NFC (Unicode Canonical Composition) for correct rendering.
- Always returns `Continue` (modifies `syllable_buffer` in-place).

---

### Stage 5: Learning

**Purpose**: Future user-pattern adaptation (currently a no-op).

- Will track user-confirmed syllables for personalized frequency re-ranking.
- Always returns `Continue`.

---

### Stage 6: Lookup

**Purpose**: Optional Hán Nôm (chữ Nôm) dictionary lookup.

- If a Nôm dictionary is configured and the syllable matches entries, candidates
  are populated in `TypingContext::candidates`.
- Always returns `Continue`; candidates are consumed by the UI layer.

---

### Stage 7: Output

**Purpose**: Generate the final `Vec<Action>` describing what the IME must do.

- Diffs `context.last_output` against `context.syllable_buffer`.
- Finds the first differing character position.
- Emits `Action::Replace { backspace_count, text }` for the changed suffix.
- Updates `context.last_output` to match the new syllable.

---

## Typing State

The pipeline maintains state in `TypingContext`:

```
Keystroke → char_buffer             → syllable_buffer  → last_output
─────────────────────────────────────────────────────────────────────
n         → [n]                     → "n"              → "n"
g         → [ng]                    → "ng"             → "ng"
u         → [ngu]                   → "ngu"            → "ngu"
w         → [nguw]                  → "ngư"            → "ngư"
o         → [nguwo]                 → "ngưo"           → "ngưo"
w         → [nguwow]                → "người"          → "người"
i         → [nguwowi]               → "người"          → "người"
f         → [nguwowif]              → "người"          → "người"
```

---

## Example: Typing "người" with Telex

Input sequence: `n g u w o w i f`

1. `n` — no transform/tone → syllable: `"n"` → Replace{0, "n"}
2. `g` — no match → syllable: `"ng"` → Replace{1, "g"}
3. `u` — no match → syllable: `"ngu"` → Replace{1, "u"}
4. `w` — compose: u+w → ư → syllable: `"ngư"` → Replace{3, "ngư"}
5. `o` — no match → syllable: `"ngưo"` → Replace{1, "o"}
6. `w` — compose: uo+w → ươ → syllable: `"người"` → Replace{4, "người"}
7. `i` — no match → syllable: `"người"` → DoNothing (no diff)
8. `f` — tone huyền on ơ → syllable: `"người"` → Replace{6, "người"}

**Final result**: `"người"` ✓

---

## Performance

- **Target**: well under 1 ms per keystroke (verified by `compose_bench`).
- Recompute cost scales with syllable length (~7 chars max for Vietnamese),
  not with total input history.
- The `compose()` function is pure (no global state, no I/O) — amenable to
  caching by prefix if profiling ever shows it necessary.
- O(1) tone lookups via static arrays in `crate::tone`.

---

## Configuration

The pipeline is fully config-driven via `PipelineConfig`.  Built-in presets:

```rust
// Telex
let config = presets::telex_config();

// VNI
let config = presets::vni_config();

// VIQR
let config = presets::viqr_config();

// Simplified Telex (without some ambiguous rules)
let config = presets::simple_telex_config();
```

Custom configs can specify transform rules, tone maps, tone style, validator, and
the ordered list of middle stages via `config.pipeline.enabled`.
