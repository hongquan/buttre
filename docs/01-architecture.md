# buttre Architecture Documentation

> Complete architectural overview of the buttre Vietnamese Input Method Engine

**Last Updated**: 2026-06-13
**Version**: 0.6.3-alpha
**Status**: Production-Ready Core, Platform Integration In Progress

---

## Table of Contents

1. [System Overview](#system-overview)
2. [Crate Architecture](#crate-architecture)
3. [Pipeline Architecture](#pipeline-architecture)
4. [State Management](#state-management)
5. [Data Flow](#data-flow)
6. [Platform Integration](#platform-integration)
7. [Design Principles](#design-principles)

---

## System Overview

buttre is a cross-platform Vietnamese input method engine written in Rust, designed for:
- **Performance**: Sub-millisecond keystroke processing
- **Correctness**: 100% compliant with Vietnamese orthography rules
- **Flexibility**: Support for Telex, VNI, VIQR, and Hán Nôm input methods
- **Cross-platform**: Windows (TSF), macOS (IMKit), Linux (IBus/Fcitx5)

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                     Platform Layer                              │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │
│  │ Windows TSF  │  │  macOS IMKit │  │ Linux IBus   │          │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘          │
└─────────┼──────────────────┼──────────────────┼─────────────────┘
          │                  │                  │
┌─────────┼──────────────────┼──────────────────┼─────────────────┐
│         │      buttre-core (Platform Agnostic)│                  │
│         └──────────────────┴──────────────────┘                 │
│              Keyboard Interface + Action Types                  │
└─────────────────────────┬───────────────────────────────────────┘
                          │
┌─────────────────────────┴───────────────────────────────────────┐
│                   buttre-engine (Pipeline)                        │
│  ┌────────────────────────────────────────────────────────┐    │
│  │   7-Stage Processing Pipeline (Config-Driven)           │    │
│  │   Telex | VNI | VIQR | Hán Nôm                         │    │
│  └────────────────────────────────────────────────────────┘    │
└──────────────────────────────────────────────────────────────────┘
```

---

## Crate Architecture

buttre uses a workspace-based multi-crate architecture for separation of concerns:

### Core Crates

#### 1. `buttre-engine` - Processing Pipeline

**Purpose**: Universal, config-driven input processing pipeline

**Location**: `crates/buttre-engine/`

**Responsibilities**:
- Implements the 7-stage processing pipeline
- Handles Vietnamese input transformations (aa→â, aw→ă, etc.)
- Tone mark application and positioning
- Undo/redo logic
- Dictionary lookup (Hán Nôm)
- Generates processing actions

**Key Modules**:
```
buttre-engine/
├── src/
│   ├── pipeline/
│   │   ├── config.rs          # Pipeline configuration
│   │   ├── context.rs         # Typing context state
│   │   ├── executor.rs        # Pipeline executor (7 stages)
│   │   ├── presets.rs         # Telex/VNI/VIQR presets
│   │   └── stages/
│   │       ├── stage1_normalization.rs
│   │       ├── stage2_gatekeeper.rs
│   │       ├── compose_stage.rs    # Stage 3: recompute-from-raw
│   │       ├── stage9_orthography.rs
│   │       ├── stage10_learning.rs
│   │       ├── stage11_lookup.rs
│   │       └── stage12_output.rs
│   ├── compose/               # Pure recompute engine
│   │   ├── mod.rs             # compose() entry point + ComposeOpts
│   │   ├── segment.rs         # Raw keys → base + marks + tones
│   │   ├── transform.rs       # Apply diacritic marks (validation-gated)
│   │   ├── assemble.rs        # Place tone mark on vowel nucleus
│   │   └── fallback.rs        # Undo / toggle / English-fallback detection
│   ├── types.rs               # Action types
│   └── lib.rs
└── tests/                     # Integration tests
```

**Public API**:
```rust
// Create pipeline executor
let config = telex_config();
let mut executor = PipelineExecutor::new(config);

// Process keystroke
let actions = executor.process('a');  // Returns Vec<Action>
```

---

#### 2. `buttre-core` - Platform-Agnostic Interface

**Purpose**: Platform-independent keyboard interface and action types

**Location**: `crates/buttre-core/`

**Responsibilities**:
- Defines the `Keyboard` interface
- Defines action types (DoNothing, Commit, Replace, etc.)
- Wraps `buttre-engine` with a clean API
- Input method selection (Telex/VNI/VIQR/Nôm)

**Key Modules**:
```
buttre-core/
├── src/
│   ├── keyboard/
│   │   ├── keyboard.rs        # Main Keyboard struct
│   │   ├── telex/             # Telex-specific logic
│   │   ├── vni/               # VNI-specific logic
│   │   └── nom/               # Hán Nôm logic
│   ├── action.rs              # Action enum
│   └── lib.rs
└── tests/
```

**Public API**:
```rust
use buttre_core::{Keyboard, InputMethod, Action};

// Create keyboard
let mut keyboard = Keyboard::new(InputMethod::Telex)?;

// Process keystroke
let actions = keyboard.process('a')?;

// Handle actions
for action in actions {
    match action {
        Action::DoNothing => { /* buffer */ }
        Action::Commit(text) => { /* send text */ }
        Action::Replace { backspace_count, text } => { /* replace */ }
        _ => {}
    }
}
```

---

#### 3. `buttre-platform` - Platform Backends

**Purpose**: Platform-specific implementations (Windows TSF, macOS, Linux)

**Location**: `crates/buttre-platform/`

**Responsibilities**:
- Windows: TSF (Text Services Framework) implementation
- macOS: IMKit integration (planned)
- Linux: IBus/Fcitx5 integration (planned)
- System tray UI
- Settings management

**Key Modules**:
```
buttre-platform/
├── src/
│   ├── platforms/
│   │   └── windows/
│   │       └── tsf/
│   │           ├── com.rs                      # COM utilities (DllMain, ref count)
│   │           ├── factory.rs                  # COM class factory
│   │           ├── registration.rs             # TSF registration
│   │           ├── text_ops.rs                 # Text manipulation
│   │           ├── ipc.rs                      # Inter-process communication
│   │           ├── logging.rs                  # Debug logging
│   │           ├── text_service/
│   │           │   ├── text_service_stub.rs    # ITfTextInputProcessorEx + ITfKeyEventSink
│   │           │   ├── composition.rs          # Composition state
│   │           │   ├── edit_session.rs         # Edit session handling
│   │           │   ├── display_attribute.rs    # Display attributes
│   │           │   ├── candidate_ui.rs         # Candidate window
│   │           │   ├── vietnamese_engine.rs    # Vietnamese processing
│   │           │   └── mod.rs
│   │           └── mod.rs
│   └── lib.rs
└── Cargo.toml
```

**Platform Integration**:
- **Windows**: Compiled as DLL, registered via `regsvr32`
- **macOS**: Framework bundle with Objective-C bridge (planned)
- **Linux**: Shared object loaded by IBus (planned)

---

#### 4. `buttre-test` - Testing Utilities

**Purpose**: Cross-platform testing infrastructure

**Location**: `crates/buttre-test/`

**Responsibilities**:
- Batch testing from text files
- Performance benchmarking
- Test data management

---

## Pipeline Architecture

### 7-Stage Processing Pipeline

The engine uses a **7-stage pipeline** for processing Vietnamese input.
The core innovation is **Stage 3: Compose** — a pure recompute-from-raw engine
that replaces the former incremental Transform/Tone/Permutation/Reconciliation/Retrofix
stages with a single deterministic function call.

```
Input Key
    ↓
┌────────────────────────────────────────────┐
│ Stage 1: NORMALIZATION                     │
│ • Normalize case; push CharInfo to buffer  │
└────────────────┬───────────────────────────┘
                 ↓
┌────────────────────────────────────────────┐
│ Stage 2: GATEKEEPER                        │
│ • temp_english_mode → PassThrough          │
│ • Non-alphabetic → PassThrough             │
│ • Otherwise → Continue                     │
└────────────────┬───────────────────────────┘
                 ↓
┌────────────────────────────────────────────┐
│ Stage 3: COMPOSE  (recompute-from-raw)     │
│ Internal steps (compose/mod.rs):           │
│  1. fallback — undo/toggle detection       │
│  2. segment — base + transforms + tones   │
│  3. transform — apply diacritics (gated)  │
│  4. assemble — place tone on nucleus      │
│ Writes syllable_buffer; sets temp_english  │
└────────────────┬───────────────────────────┘
                 ↓
┌────────────────────────────────────────────┐
│ Stage 4: ORTHOGRAPHY                       │
│ • Normalize tone position                  │
│ • Apply ToneStyle (Old: óa, New: oá)       │
│ • Convert to NFC                           │
└────────────────┬───────────────────────────┘
                 ↓
┌────────────────────────────────────────────┐
│ Stage 5: LEARNING  (no-op, future)         │
│ • User pattern tracking                    │
└────────────────┬───────────────────────────┘
                 ↓
┌────────────────────────────────────────────┐
│ Stage 6: LOOKUP                            │
│ • Hán Nôm dictionary candidates            │
└────────────────┬───────────────────────────┘
                 ↓
┌────────────────────────────────────────────┐
│ Stage 7: OUTPUT                            │
│ • Diff last_output vs syllable_buffer      │
│ • Emit Replace{backspace_count, text}      │
└────────────────┬───────────────────────────┘
                 ↓
                Output Actions
```

### Stage Results

Each stage returns a `StageResult`:

```rust
pub enum StageResult {
    Continue,              // Proceed to next stage
    PassThrough,           // Stop, send input as-is
    Output(Vec<Action>),   // Stop, return actions
}
```

### Pipeline Configuration

The pipeline is **config-driven**, allowing easy addition of new input methods:

```rust
pub struct PipelineConfig {
    pub input_method_type: InputMethodType,
    pub tone_config: ToneConfig,
    pub transform_rules: Vec<TransformRule>,
    pub tone_rules: Vec<ToneRule>,
    pub special_handlers: Vec<SpecialHandler>,
}
```

**Preset Configurations**:
- `telex_config()` - Telex input method
- `vni_config()` - VNI input method
- `viqr_config()` - VIQR input method
- `nom_config()` - Hán Nôm input method

---

## State Management

### Typing Context

The pipeline maintains mutable state in `TypingContext`:

```rust
pub struct TypingContext {
    /// Raw input history (for undo/redo)
    pub raw_buffer: Vec<char>,

    /// Current syllable being built
    pub current_syllable: Syllable,

    /// English fallback mode (after undo)
    pub temp_english_mode: bool,

    /// Last transformation applied
    pub last_transformation: Option<TransformRecord>,

    /// Last output (for incremental updates)
    pub last_output: String,

    /// Tone configuration
    pub tone_config: ToneConfig,

    /// Candidates (for Hán Nôm)
    pub candidates: Vec<Candidate>,
}
```

### State Evolution Example

Typing "người" (nguwowif):

```
Key → Raw Buffer   → Syllable → Output
──────────────────────────────────────────
n   → [n]         → n        → "n"
g   → [ng]        → ng       → "ng"
u   → [ngu]       → ngu      → "ngu"
w   → [nguw]      → ngư      → "ngư"       (u→ư)
o   → [nguwo]     → ngưo     → "ngưo"
w   → [nguwow]    → người    → "người"     (uo→ươ)
i   → [nguwowi]   → người    → "người"
f   → [nguwowif]  → người    → "người"     (tone)
```

---

## Data Flow

### Keystroke Processing Flow

```
┌──────────────────────────────────────────────────────────┐
│ 1. Platform Layer (Windows TSF/macOS/Linux)             │
│    Captures raw keystroke from OS                        │
└───────────────────────┬──────────────────────────────────┘
                        ↓
┌──────────────────────────────────────────────────────────┐
│ 2. buttre-core::Keyboard                                  │
│    keyboard.process(key) → Vec<Action>                   │
└───────────────────────┬──────────────────────────────────┘
                        ↓
┌──────────────────────────────────────────────────────────┐
│ 3. buttre-engine::PipelineExecutor                        │
│    executor.process(key) → Vec<Action>                   │
│    ┌──────────────────────────────────────────────────┐ │
│    │ Stage 1 → Stage 2 → ... → Stage 7               │ │
│    │ (Each stage returns Continue/PassThrough/Output)│ │
│    └──────────────────────────────────────────────────┘ │
└───────────────────────┬──────────────────────────────────┘
                        ↓
┌──────────────────────────────────────────────────────────┐
│ 4. Action Processing (back to buttre-core)                │
│    DoNothing | Commit | Replace | UpdateComposition     │
└───────────────────────┬──────────────────────────────────┘
                        ↓
┌──────────────────────────────────────────────────────────┐
│ 5. Platform Layer                                        │
│    - Replace: Send backspaces + text                     │
│    - Commit: Send text directly                          │
│    - UpdateComposition: Update composition string (TSF)  │
└──────────────────────────────────────────────────────────┘
```

### Action Types

```rust
pub enum Action {
    DoNothing,                              // No output
    Commit(String),                         // Append text
    Replace { backspace_count: usize, text: String },  // Replace
    UpdateComposition { text: String, cursor: usize }, // TSF composition
    ConfirmComposition(String),             // Confirm composition
    ShowCandidates { candidates: Vec<Candidate>, input: String }, // Hán Nôm
    HideCandidates,                         // Hide candidates
}
```

---

## Platform Integration

### Windows TSF (Text Services Framework)

**Status**: ✅ Implemented and Working

**Architecture**:
- COM DLL registered as Text Input Processor (TIP)
- Implements `ITfTextInputProcessorEx` interface
- Implements `ITfKeyEventSink` for keystroke capture
- Uses composition string for incremental updates

**Registration**:
```powershell
# Build DLL
cargo build --release --package buttre-platform

# Register (requires Admin)
regsvr32 target/release/buttre_platform.dll
```

**Key Files**:
- `crates/buttre-platform/src/platforms/windows/tsf/text_service/text_service_stub.rs` (839 lines) - Implements both ITfTextInputProcessorEx and ITfKeyEventSink
- `crates/buttre-platform/src/platforms/windows/tsf/com.rs` - Contains DllMain entry point and COM utilities

---

### macOS IMKit

**Status**: 🚧 Planned

**Architecture**:
- Framework bundle with Objective-C bridge
- Uses `InputMethodKit` framework
- Rust core wrapped by Objective-C wrapper

**Reference**: See `docs/MACOS_IMPLEMENTATION_PLAN.md`

---

### Linux IBus/Fcitx5

**Status**: 🚧 Planned

**Architecture**:
- Shared object (.so) loaded by IBus/Fcitx5
- D-Bus communication
- Rust core exposed via C FFI

**Reference**: See `docs/LINUX_IMPLEMENTATION_PLAN.md`

---

## Design Principles

### 1. **Config-Driven Architecture**

All input methods are defined via configuration, not hardcoded logic:

```rust
// Adding a new input method is just configuration
let custom_config = PipelineConfig {
    input_method_type: InputMethodType::Custom,
    transform_rules: vec![
        TransformRule { pattern: "aa", result: "â", ... },
        // ... more rules
    ],
    // ...
};
```

### 2. **Zero Unsafe in Core**

- `buttre-engine`: 100% safe Rust
- `buttre-core`: 100% safe Rust
- Unsafe code only in `buttre-platform` for FFI (minimized)

### 3. **Testability**

- **Unit tests**: Each module has comprehensive unit tests
- **Integration tests**: 600+ tests in `buttre-engine/tests/`
- **Property-based tests**: Fuzzing for edge cases
- **Test coverage**: >85%

### 4. **Performance**

- **Zero-allocation hot paths**: Fixed-size buffers
- **O(1) lookups**: Static tone maps, hash tables
- **Incremental updates**: Only send changed portions
- **Lazy evaluation**: Delay expensive operations

### 5. **Incremental Development**

Each stage can be tested independently:

```rust
#[test]
fn test_stage4_transform() {
    let mut ctx = TypingContext::new();
    ctx.raw_buffer = vec!['a', 'a'];

    let result = Stage4Transform.process('a', &mut ctx);

    assert_eq!(ctx.current_syllable.text, "â");
}
```

---

## Summary

**buttre Architecture** provides:

✅ **Separation of Concerns**: Engine ← Core ← Platform
✅ **Config-Driven**: Easy to add new input methods
✅ **Testability**: Each component tested independently
✅ **Performance**: Sub-ms processing, zero-allocation hot paths
✅ **Cross-Platform**: Same core for Windows/macOS/Linux
✅ **Maintainability**: Clean architecture, clear boundaries

**Key Files**:
- `crates/buttre-engine/src/pipeline/executor.rs` - Pipeline execution
- `crates/buttre-core/src/keyboard/keyboard.rs` - Keyboard interface
- `crates/buttre-platform/src/platforms/windows/tsf/` - Windows TSF
- `docs/PIPELINE_ARCHITECTURE.md` - Detailed pipeline docs
- `docs/VIETNAMESE_ACCENT.md` - Vietnamese orthography rules
