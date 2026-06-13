# buttre Coding Guide

> How to write code that fits buttre's conventions and patterns

**Last Updated**: 2026-01-08
**For**: Developers contributing to buttre

---

## Table of Contents

1. [Project Setup](#project-setup)
2. [Code Organization](#code-organization)
3. [Rust Coding Standards](#rust-coding-standards)
4. [Common Patterns](#common-patterns)
5. [Testing Guidelines](#testing-guidelines)
6. [Error Handling](#error-handling)
7. [Performance Guidelines](#performance-guidelines)
8. [How to Add New Features](#how-to-add-new-features)

---

## Project Setup

### Prerequisites

- Rust 1.70+
- (Windows) Visual Studio Build Tools
- (macOS) Xcode Command Line Tools
- (Linux) GCC/Clang

### Workspace Structure

```
buttre/
├── crates/
│   ├── buttre-engine/    # Processing pipeline
│   ├── buttre-core/      # Platform-agnostic interface
│   ├── buttre-platform/  # Platform backends
│   └── buttre-test/      # Testing utilities
├── docs/                # Documentation
├── .agent/              # AI agent artifacts
└── .reference/          # Reference implementations
```

### Building

```bash
# Check all crates
cargo check

# Build specific crate
cargo build --package buttre-engine

# Build release (optimized)
cargo build --release

# Run tests
cargo test

# Run tests for specific crate
cargo test --package buttre-engine
```

---

## Code Organization

### File Naming Conventions

**Modules**: `snake_case`
```
pipeline/
├── mod.rs
├── config.rs
├── context.rs
├── executor.rs
└── stages/
    ├── mod.rs
    ├── stage1_normalization.rs
    ├── stage2_gatekeeper.rs
    └── ...
```

**Tests**: Same file or `tests/` directory
```
src/pipeline/executor.rs
src/pipeline/executor_tests.rs  // (if too large for inline)

tests/
├── integration_tests.rs
└── test_data/
    ├── telex.txt
    └── vni.txt
```

### Module Organization Pattern

**From `buttre-engine/src/pipeline/mod.rs`**:

```rust
//! Pipeline Module — 7-Stage Input Processing Pipeline
//!
//! This module implements a config-driven, 7-stage pipeline for processing
//! Vietnamese input methods (Telex, VNI, etc.) in a flexible and extensible way.
//!
//! ## Architecture
//!
//! The pipeline consists of 7 stages:
//! 1. Normalization — normalize input, push CharInfo to char_buffer
//! 2. Gatekeeper — route non-Vietnamese / temp-English passthrough
//! 3. Compose — recompute-from-raw: segment → transform → tone → fallback
//! 4. Orthography — normalize Unicode form
//! 5. Learning — track patterns (future)
//! 6. Lookup — dictionary lookup (Hán Nôm)
//! 7. Output — diff last_output → syllable_buffer → emit actions
//!
//! ## Design Principles
//!
//! - **Config-Driven**: All input methods are defined via configuration
//! - **Incremental**: Each stage can be tested independently
//! - **Backward Compatible**: Works alongside existing hardcoded methods
//! - **Extensible**: Easy to add new stages or modify existing ones

pub mod config;
pub mod context;
pub mod stage;
pub mod stages;
pub mod executor;
pub mod presets;

// Re-exports for convenience
pub use config::{PipelineConfig, ToneMark};
pub use context::{TypingContext, Candidate, CandidateType};
pub use stage::{PipelineStage, StageResult};
pub use executor::PipelineExecutor;
pub use presets::{telex_config, vni_config, viqr_config};
```

**Pattern Rules**:
- ✅ Use `//!` for module-level documentation
- ✅ Explain architecture and design principles
- ✅ Re-export commonly used types
- ✅ Organize submodules logically

---

## Rust Coding Standards

### 1. Error Handling

**❌ NEVER use these in library code:**
```rust
// WRONG - Will panic in production
let value = result.unwrap();
let value = option.expect("message");
panic!("error");
```

**✅ ALWAYS use Result/Option:**
```rust
// CORRECT - Propagate errors
pub fn parse_config(path: &Path) -> anyhow::Result<Config> {
    let content = std::fs::read_to_string(path)?;  // Propagate error
    let config = toml::from_str(&content)?;        // Propagate error
    Ok(config)
}

// CORRECT - Handle Option
pub fn find_vowel(text: &str) -> Option<char> {
    text.chars().find(|c| is_vowel(*c))
}
```

**From `buttre-core/src/keyboard/keyboard.rs`**:

```rust
/// Create a new keyboard from pipeline config
pub(crate) fn new(config: PipelineConfig) -> anyhow::Result<Self> {
    // Create executor directly from config
    let executor = PipelineExecutor::new(config);

    Ok(Self {
        executor,
        buffer: String::new(),
    })
}

/// Process a keystroke
///
/// Returns a vector of actions to perform. Usually contains 1-2 actions:
/// - Main action (DoNothing/Commit/Replace/UpdateComposition)
/// - Optional ShowCandidates/HideCandidates for Nôm input
pub fn process(&mut self, key: char) -> anyhow::Result<Vec<Action>> {
    // Process through engine pipeline
    let engine_actions = self.executor.process(key);

    // Convert engine actions to our actions
    // ... (no unwrap/expect/panic)
}
```

---

### 2. Documentation Standards

**Public Functions** - MUST have documentation:

```rust
/// Process a keystroke through the pipeline.
///
/// # Arguments
///
/// * `key` - The character to process
///
/// # Returns
///
/// A vector of actions to perform on the text buffer
///
/// # Example
///
/// ```
/// use buttre_core::{Keyboard, InputMethod};
///
/// let mut keyboard = Keyboard::new(InputMethod::Telex)?;
/// let actions = keyboard.process('a')?;
/// ```
pub fn process(&mut self, key: char) -> anyhow::Result<Vec<Action>> {
    // ...
}
```

**Module Documentation**:

```rust
//! Pipeline Module — 7-Stage Input Processing Pipeline
//!
//! This module implements a config-driven, 7-stage pipeline...
//!
//! ## Architecture
//! ...
//!
//! ## Example
//! ```
//! use buttre_engine::pipeline::{PipelineExecutor, telex_config};
//!
//! let mut executor = PipelineExecutor::new(telex_config());
//! let actions = executor.process('a');
//! ```
```

---

### 3. Naming Conventions

**Types**: `PascalCase`
```rust
pub struct PipelineExecutor { }
pub enum InputMethodType { }
pub struct TypingContext { }
```

**Functions/Methods**: `snake_case`
```rust
pub fn process_key(&mut self, key: char) -> Vec<Action> { }
pub fn find_main_vowel(text: &str) -> Option<usize> { }
```

**Constants**: `SCREAMING_SNAKE_CASE`
```rust
const MAX_BUFFER_SIZE: usize = 32;
const DEFAULT_TONE_STYLE: ToneStyle = ToneStyle::Old;
```

**Boolean Functions**: `is_`, `has_`, `can_`
```rust
pub fn is_vowel(c: char) -> bool { }
pub fn has_tone_mark(syllable: &Syllable) -> bool { }
pub fn can_apply_transformation(ctx: &Context) -> bool { }
```

**Conversion Functions**: `to_`, `into_`, `as_`, `from_`
```rust
pub fn to_string(&self) -> String { }
pub fn as_bytes(&self) -> &[u8] { }
pub fn from_config(config: Config) -> Self { }
```

**Fallible Functions**: `try_`
```rust
pub fn try_parse(input: &str) -> Option<Syllable> { }
pub fn try_apply_tone(ctx: &mut Context) -> Result<(), Error> { }
```

---

### 4. Type Safety

**✅ Use newtypes for domain concepts:**

```rust
// GOOD - Type-safe
pub struct UserId(u64);
pub struct TonePosition(usize);

impl TonePosition {
    pub fn new(pos: usize) -> Self {
        TonePosition(pos)
    }

    pub fn value(&self) -> usize {
        self.0
    }
}

// BAD - No type safety
type UserId = u64;
let id: u64 = 123;  // Could be anything
```

**✅ Use enums instead of strings:**

```rust
// GOOD - Type-safe
pub enum InputMethodType {
    Telex,
    Vni,
    Viqr,
    Nom,
}

// BAD - Stringly typed
let method = "telex";  // Typos not caught by compiler
```

---

### 5. Ownership & Borrowing

**Prefer borrowing over ownership:**

```rust
// GOOD - Borrow when possible
pub fn find_main_vowel(text: &str) -> Option<usize> {
    text.chars().position(|c| is_vowel(c))
}

// LESS GOOD - Takes ownership unnecessarily
pub fn find_main_vowel(text: String) -> Option<usize> {
    text.chars().position(|c| is_vowel(c))
}
```

**Document why you clone:**

```rust
// GOOD - Document why clone is necessary
pub fn create_candidate(&self, text: &str) -> Candidate {
    Candidate {
        text: text.to_string(),  // Must own the string for candidate
        score: self.calculate_score(text),
    }
}
```

---

## Common Patterns

### Pattern 1: Pipeline Stage

**From `buttre-engine/src/pipeline/stages/stage4_transform.rs`**:

```rust
use super::super::{PipelineStage, StageResult, TypingContext};

/// Stage 4: Transformation
///
/// Applies transformation rules (aa→â, aw→ă, dd→đ, etc.)
pub struct Stage4Transform;

impl PipelineStage for Stage4Transform {
    fn name(&self) -> &'static str {
        "Transform"
    }

    fn process(&self, key: char, ctx: &mut TypingContext) -> StageResult {
        // 1. Check if this is a transformation key
        // 2. Look up transformation rule
        // 3. Apply transformation
        // 4. Update context
        // 5. Return Continue

        // ... implementation

        StageResult::Continue
    }
}
```

**Pattern Rules**:
- ✅ Each stage is a separate struct implementing `PipelineStage`
- ✅ Stage has a descriptive `name()`
- ✅ `process()` method is pure (no side effects except on `ctx`)
- ✅ Returns `StageResult` to control flow

---

### Pattern 2: Action Enum

**From `buttre-core/src/action.rs`**:

```rust
/// Actions to be performed on the text buffer
#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    /// Do nothing (character was buffered)
    DoNothing,

    /// Commit text to buffer
    Commit(String),

    /// Replace last N characters with new text
    Replace {
        backspace_count: usize,
        text: String,
    },

    /// Update composition string (TSF only)
    UpdateComposition {
        text: String,
        cursor: usize,
    },

    /// Confirm composition
    ConfirmComposition(String),

    /// Show candidate window (Nôm input)
    ShowCandidates {
        candidates: Vec<Candidate>,
        input: String,
    },

    /// Hide candidate window
    HideCandidates,
}
```

**Pattern Rules**:
- ✅ Use `#[derive(Debug, Clone, PartialEq)]` for action enums
- ✅ Document each variant
- ✅ Use struct-style variants for complex data

---

### Pattern 3: Configuration Struct

**From `buttre-engine/src/pipeline/config.rs`**:

```rust
/// Configuration for the processing pipeline
#[derive(Debug, Clone)]
pub struct PipelineConfig {
    /// Input method type
    pub input_method_type: InputMethodType,

    /// Tone configuration
    pub tone_config: ToneConfig,

    /// Transformation rules
    pub transform_rules: Vec<TransformRule>,

    /// Tone application rules
    pub tone_rules: Vec<ToneRule>,
}

impl PipelineConfig {
    /// Create a new pipeline configuration
    pub fn new(input_method_type: InputMethodType) -> Self {
        Self {
            input_method_type,
            tone_config: ToneConfig::default(),
            transform_rules: Vec::new(),
            tone_rules: Vec::new(),
        }
    }

    /// Builder pattern: set tone config
    pub fn with_tone_config(mut self, config: ToneConfig) -> Self {
        self.tone_config = config;
        self
    }
}
```

**Pattern Rules**:
- ✅ Use builder pattern for complex configuration
- ✅ Provide sensible defaults
- ✅ Make fields public for flexibility

---

## Testing Guidelines

### Unit Tests

**From `buttre-engine/src/pipeline/stages/stage4_transform.rs`**:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_aa_to_circumflex() {
        let mut ctx = TypingContext::new();
        ctx.raw_buffer = vec!['a'];
        ctx.current_syllable.text = "a".to_string();

        let stage = Stage4Transform;
        let result = stage.process('a', &mut ctx);

        assert_eq!(result, StageResult::Continue);
        assert_eq!(ctx.current_syllable.text, "â");
    }

    #[test]
    fn test_transform_uo_plus_w_to_horn_pair() {
        let mut ctx = TypingContext::new();
        ctx.raw_buffer = vec!['u', 'o'];
        ctx.current_syllable.text = "uo".to_string();

        let stage = Stage4Transform;
        let result = stage.process('w', &mut ctx);

        assert_eq!(result, StageResult::Continue);
        assert_eq!(ctx.current_syllable.text, "ươ");
    }
}
```

**Test Naming**: `test_<function>_<scenario>_<expected>`

Good names:
- `test_process_key_valid_input_returns_action`
- `test_apply_tone_empty_buffer_returns_none`
- `test_transform_aa_to_circumflex`

Bad names:
- `test_1`
- `test_process`
- `it_works`

---

### Integration Tests

**From `buttre-engine/tests/flexible_typing_test.rs`**:

```rust
#[test]
fn test_flexible_typing_tuongwf_to_truong() {
    let config = telex_config();
    let mut executor = PipelineExecutor::new(config);

    // Type "tuongwf" (out of order)
    let keys = ['t', 'u', 'o', 'n', 'g', 'w', 'f'];

    for key in keys {
        executor.process(key);
    }

    let final_output = executor.get_current_output();
    assert_eq!(final_output, "trường");
}
```

---

### Test Data Files

**From `buttre-test/data/telex.txt`**:

```
# Format: input → expected_output
# One test per line

# Basic transformations
aa → â
aw → ă
dd → đ

# Tone marks
as → á
af → à

# Complex words
nguwowif → người
tuongwf → trường
```

---

## Error Handling

### Using anyhow for Application Errors

```rust
use anyhow::{Result, Context};

pub fn load_config(path: &Path) -> Result<Config> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read config from {}", path.display()))?;

    let config: Config = toml::from_str(&content)
        .context("Failed to parse TOML config")?;

    Ok(config)
}
```

### Using thiserror for Library Errors

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PipelineError {
    #[error("Invalid input method: {0}")]
    InvalidInputMethod(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Internal pipeline error")]
    InternalError(#[from] std::io::Error),
}
```

---

## Performance Guidelines

### 1. Avoid Allocations in Hot Paths

```rust
// GOOD - No allocation
pub fn find_vowel(text: &str) -> Option<usize> {
    text.chars().position(|c| is_vowel(c))
}

// BAD - Unnecessary allocation
pub fn find_vowel(text: &str) -> Option<usize> {
    let chars: Vec<char> = text.chars().collect();  // Allocation!
    chars.iter().position(|c| is_vowel(*c))
}
```

### 2. Use Static Lookups

```rust
// GOOD - O(1) lookup
use lazy_static::lazy_static;
use std::collections::HashSet;

lazy_static! {
    static ref VOWELS: HashSet<char> = {
        ['a', 'à', 'á', 'ả', 'ã', 'ạ',
         'e', 'è', 'é', 'ẻ', 'ẽ', 'ẹ',
         // ... more vowels
        ].iter().copied().collect()
    };
}

pub fn is_vowel(c: char) -> bool {
    VOWELS.contains(&c)
}
```

### 3. Use #[inline] for Tiny Functions

```rust
#[inline]
pub fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'y')
}
```

---

## How to Add New Features

### Example: Adding a New Input Method

**Step 1**: Create preset configuration

**File**: `crates/buttre-engine/src/pipeline/presets.rs`

```rust
/// Create VIQR input method configuration
pub fn viqr_config() -> PipelineConfig {
    PipelineConfig {
        input_method_type: InputMethodType::Viqr,
        tone_config: ToneConfig {
            free_marking: true,
            auto_correct_uo: false,
            max_modify_length: 10,
        },
        transform_rules: vec![
            // VIQR uses different keys
            TransformRule { pattern: "a^", result: "â", ... },
            TransformRule { pattern: "a+", result: "ă", ... },
            // ... more rules
        ],
        tone_rules: vec![
            ToneRule { key: '\'', mark: ToneMark::Acute },
            ToneRule { key: '`', mark: ToneMark::Grave },
            // ... more rules
        ],
    }
}
```

**Step 2**: Add enum variant

**File**: `crates/buttre-engine/src/pipeline/config.rs`

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMethodType {
    Telex,
    Vni,
    Viqr,   // Add this
    Nom,
}
```

**Step 3**: Add tests

**File**: `crates/buttre-engine/tests/viqr_test.rs`

```rust
#[test]
fn test_viqr_circumflex() {
    let config = viqr_config();
    let mut executor = PipelineExecutor::new(config);

    executor.process('a');
    executor.process('^');

    assert_eq!(executor.get_current_output(), "â");
}
```

**Step 4**: Update documentation

- Add to `docs/ARCHITECTURE.md`
- Add to `README.md`

---

## Summary

**buttre Coding Standards**:

✅ **Error Handling**: Use Result/Option, never unwrap/expect in library code
✅ **Documentation**: Document all public APIs with examples
✅ **Naming**: snake_case functions, PascalCase types, SCREAMING_SNAKE constants
✅ **Type Safety**: Use newtypes and enums, avoid strings for domain concepts
✅ **Testing**: Unit tests for all functions, integration tests for workflows
✅ **Performance**: Avoid allocations in hot paths, use static lookups
✅ **Patterns**: Follow established patterns (Pipeline Stage, Action Enum, etc.)

**Before Submitting PR**:

```bash
# 1. Format code
cargo fmt

# 2. Check clippy
cargo clippy --all-targets --all-features

# 3. Run tests
cargo test --all

# 4. Build release
cargo build --release
```

**Questions?** Check existing code for patterns, or ask in issues!
