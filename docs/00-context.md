# buttre system context & design rules
**Last Updated**: 2026-06-13
**Audience**: Developers & AI Agents

---

## Quick Start for AI Agents

**Before assisting with any task, read these in order:**

| File | Description |
| --- | --- |
| `01-architecture.md` | System architecture |
| `02-coding-guide.md` | Coding standards and patterns |
| `ROADMAP.md` | Project roadmap and current status |

---

## Project Metadata

```yaml
name: buttre
tagline: Modern Vietnamese Input Method Engine
type: cross_platform_input_method
version: 0.6.3-alpha
status: beta

platforms:
  - windows (TSF) - ✅ Implemented
  - macos (IMKit) - 🚧 Planned (Q2 2026)
  - linux (IBus) - 🚧 Planned (Q3 2026)

primary_language: rust
rust_version: 1.70+
license: MPL-2.0

repository: https://github.com/dxsl-org/buttre
documentation: docs/
```

---

## Tech Stack

### Core Technologies

```yaml
languages:
  rust:
    version: 1.70+
    edition: 2021
    purpose: Core engine, platform backends

databases:
  sqlite:
    purpose: Hán Nôm dictionary (Phase 4)
    size: ~48,510 entries
    optimization: FTS5 full-text search

frameworks:
  windows-rs:
    version: 0.62.2
    purpose: Windows TSF COM bindings

  future_frameworks:
    cocoa: macOS Objective-C bindings (planned)
    x11: Linux X11 bindings (planned)
```

### Dependencies

**Workspace Dependencies** (from `Cargo.toml`):
```yaml
core:
  - unicode-normalization: 0.1 (NFC/NFD conversion)
  - lazy_static: 1.4 (static lookups)
  - thiserror: 1.0 (library errors)
  - anyhow: 1.0 (application errors)

serialization:
  - serde: 1.0 (config serialization)
  - serde_json: 1.0 (JSON handling)
  - toml: 0.8 (TOML config)

utilities:
  - dirs: 5.0 (platform directories)
  - log: 0.4 (logging facade)
  - tracing: 0.1 (structured logging)
  - tracing-subscriber: 0.3 (tracing backend)

platform_specific:
  - windows: 0.62 (Windows API)
  - cocoa: 0.25 (macOS - future)
  - x11: 2.21 (Linux - future)
```

---

## Crate Structure

buttre uses a **multi-crate workspace** architecture:

```yaml
buttre-engine:
  purpose: Processing pipeline (7-stage config-driven, recompute-from-raw)
  location: crates/buttre-engine/
  responsibility: Vietnamese input transformations, tone marks, undo logic
  status: ✅ Complete (600+ tests passing)
  public_api:
    - PipelineExecutor
    - PipelineConfig
    - telex_config(), vni_config(), viqr_config()

buttre-core:
  purpose: Platform-agnostic keyboard interface
  location: crates/buttre-core/
  responsibility: Keyboard struct, Action types, input method selection
  status: ✅ Complete
  public_api:
    - Keyboard::new(InputMethod)
    - Keyboard::process(char) -> Vec<Action>
    - Action enum (DoNothing, Commit, Replace, etc.)

buttre-platform:
  purpose: Platform-specific backends
  location: crates/buttre-platform/
  responsibility: Windows TSF, macOS IMKit, Linux IBus
  status: ✅ Windows TSF complete, others planned
  components:
    - platforms/windows/tsf/ (TSF implementation)
    - platforms/macos/ (planned)
    - platforms/linux/ (planned)

buttre-test:
  purpose: Testing utilities
  location: crates/buttre-test/
  responsibility: Batch testing, benchmarks, test data
  status: ✅ Complete
```

**Deprecated/Legacy Crates** (mentioned in old docs, now consolidated):
- `buttre-app` → Merged into `buttre-platform`
- `buttre-hotkey` → Merged into `buttre-platform`
- `buttre-vietnamese` → Now `buttre-engine` (universal pipeline)
- `buttre-hannom` → Planned as Phase 4 (Q4 2026)
- `buttre-custom` → Planned as future enhancement
- `buttre-windows`, `buttre-windows-hook`, `buttre-windows-common`, `buttre-windows-tsf` → Consolidated into `buttre-platform/platforms/windows/`

---

## Directory Structure

```
buttre/
├── crates/                    # Rust workspace
│   ├── buttre-engine/         # 7-stage processing pipeline
│   ├── buttre-core/           # Platform-agnostic interface
│   ├── buttre-platform/       # Platform backends
│   └── buttre-test/           # Testing utilities
│
├── docs/                      # Project documentation
│   ├── README.md             # Documentation navigation
│   ├── 00-context.md        # This file
│   ├── 01-architecture.md    # System architecture
│   ├── 02-coding-guide.md    # Coding standards
│   ├── ROADMAP.md            # Project roadmap
│   ├── PIPELINE_ARCHITECTURE.md
│   ├── VIETNAMESE_ACCENT.md
│   ├── MANUAL_TESTING_GUIDE.md
│   ├── FFI_SAFETY_GUIDE.md
│   └── journals/             # Development journals
│
├── .agents/                   # AI agent artifacts
│   └── (planning docs, reports, organized by phase)
│
├── .reference/                # Reference implementations
│   ├── unikey/               # Unikey (C++ reference)
│   ├── openkey/              # OpenKey reference
│   ├── ibus-bamboo/          # IBus Bamboo (Go reference)
│   └── weasel/               # Weasel Hán Nôm reference
│
├── CLAUDE.md                  # This file (AI agent config)
├── README.md                  # Project overview
├── Cargo.toml                 # Workspace configuration
├── LICENSE                    # MPL-2.0 license
└── CODE_OF_CONDUCT.md        # Code of conduct
```

**Note**: `.agents/` directory is used for planning and reports; all shipped documentation lives in `docs/`.

---

## Code Quality Rules

### Mandatory Rules (MUST Follow)

```yaml
error_handling:
  NEVER_use:
    - unwrap() (on Option/Result - causes panic!)
    - expect() (in library code - only in main/tests)
    - panic!() (use Result/Option instead)
    - todo!() (in committed code)
    - unimplemented!() (in committed code)

  ALWAYS_use:
    - Result<T, E> for fallible operations
    - Option<T> for optional values
    - ? operator for error propagation
    - anyhow::Context for human-readable error chains
    - thiserror for domain-specific errors

unsafe_code:
  rules:
    - buttre-engine: MUST be 100% safe Rust (no unsafe)
    - buttre-core: MUST be 100% safe Rust (no unsafe)
    - buttre-platform: Unsafe ONLY for FFI, minimized scope
    - MUST document safety invariants with // SAFETY: comments

type_safety:
  - Use newtypes for domain concepts (UserId, not u64)
  - Use enums instead of strings for types
  - Avoid stringly-typed code
  - Prefer explicit type annotations for complex expressions

code_organization:
  - Follow existing patterns (see docs/CODING_GUIDE.md)
  - Self-documenting names (no abbreviations)
  - Modular design (single responsibility)
  - Test critical logic (unit + integration tests)
```

### Testing Requirements

```yaml
required:
  - unit_tests: Every public function
  - integration_tests: Key workflows (see buttre-engine/tests/)
  - edge_cases: Empty, max, Unicode, special chars
  - error_paths: Test all error conditions

test_naming: "test_<function>_<scenario>_<expected>"

examples:
  good:
    - test_process_key_valid_input_returns_action
    - test_apply_tone_empty_buffer_returns_none
  bad:
    - test_1
    - it_works
```

---

## Workflow for AI Agents

### Development Cycle

When assisting with development, follow this cycle:

```yaml
phases:
  1. Research:
      - Read relevant documentation
      - Understand existing patterns
      - Check reference implementations (.reference/)

  2. Analyze:
      - Identify affected components
      - Review current code
      - Understand dependencies

  3. Plan:
      - Design solution approach
      - Consider edge cases
      - Plan tests

  4. Code:
      - Write implementation following coding guide
      - Add comprehensive tests
      - Document public APIs

  5. Verify:
      - Run cargo check
      - Run cargo test
      - Run cargo clippy
      - Verify all tests pass

  6. Document:
      - Update relevant documentation
      - Add inline comments for complex logic
      - Update CHANGELOG.md if needed

execution: sequential_one_at_a_time
retry_policy: auto_retry_until_exit_conditions
```

### Constraints

```yaml
NEVER:
  - Skip steps in the development cycle
  - Write abbreviated/incomplete code
  - Leave work half-done
  - Use unsafe code without justification + documentation
  - Assume behavior without testing

ALWAYS:
  - Understand project structure first (read docs/)
  - Confirm before making major modifications
  - Complete full scope of assigned task
  - Track progress (use TodoWrite tool)
  - Ask user to continue if task is large
  - Optimize token usage (focused, concise)

IMPORTANT:
  - Use semantic versioning (x.y.z)
  - Update CHANGELOG.md with changes
  - Commit before major changes (backup)
  - Prefer small, incremental changes
  - Reuse existing code, avoid duplicates
  - Document breaking changes + update callers
  - Summarize progress if task fails mid-way
```

---

## Current Focus

**Active Phase**: Windows Stability & Polish (Q1 2026)

```yaml
active_crate: buttre-engine
current_status:
  - Core engine: ✅ Complete (7-stage pipeline, recompute-from-raw)
  - Windows TSF: ✅ Implemented
  - Tests: ✅ 600+ passing (3 known failures)
  - Documentation: ✅ Comprehensive (docs/ reorganized)

next_steps:
  immediate:
    - Manual testing in Notepad, Word, browsers
    - Fix 3 pre-existing test failures
    - Rebuild TSF DLL for testing

  short_term:
    - Installer improvements (silent install, upgrade path)
    - User manual (Vietnamese)
    - Video tutorials

  medium_term:
    - buttre 1.0 Windows stable release
    - Begin macOS implementation (Q2 2026)

progress:
  - 600+ tests passing
  - Build status: success (warnings only)
  - Test status: manual_testing_required
  - UX optimization: ✅ Complete (free accent, tone repositioning, auto-correction)
```

### Known Issues

**Pre-existing Test Failures** (3 tests):

1. **test_find_best_permutation_thuwowfngf**
   - File: `crates/buttre-engine/src/pipeline/stages/stage6_permutation.rs`
   - Cause: Duplicate transform mark handling appends extra 'w'
   - Priority: Medium (affects edge case)
   - Fix: Improve permutation duplicate detection

2. **test_telex_settings**
   - File: `crates/buttre-engine/src/pipeline/presets.rs`
   - Cause: Test expects ToneStyle::New but preset uses ToneStyle::Old
   - Priority: Low (test vs preset mismatch)
   - Fix: Align test expectations with preset defaults

3. **test_vni_settings**
   - File: `crates/buttre-engine/src/pipeline/presets.rs`
   - Cause: Same as test_telex_settings
   - Priority: Low
   - Fix: Align test expectations with preset defaults

---

## Vietnamese Input Rules

**Critical domain knowledge for working on the engine**:

### Tone Placement Rules

```yaml
priority_order:
  1. super_vowel:
      chars: [ă, â, ê, ô, ơ, ư]
      rule: Always receive tone
      example: "tuấn" (tone on â)

  2. three_vowel:
      rule: Tone on middle vowel
      example: "uoi → uòi"

  3. two_vowel_closed:
      rule: Tone on 2nd vowel when final consonant present
      example: "toán" (tone on á, final consonant n)

  4. two_vowel_open:
      ia_ua_ưa: Tone on 1st vowel
      oa_oe_uy: Depends on ToneStyle (Old=1st, New=2nd)
      others: Tone on 1st vowel

  5. single_vowel:
      rule: Tone on that vowel
      example: "á"

tone_styles:
  old: "óa, úy (traditional, default)"
  new: "oá, uý (modern)"
```

### Auto-Correction

```yaml
uo_to_ươ:
  trigger: Applying tone to syllable containing 'uo'
  result: "'uo' → 'ươ' before tone application"
  example: "nguoif → người (not nguòi)"
  config: auto_correct_uo (default: false)
```

### English Fallback

```yaml
temp_english_mode:
  trigger: After undo operation (double-key)
  behavior: Next alphabetic key is raw (not Vietnamese)
  reset_on: Non-alphabetic character or word boundary
  example: "Aaron" (aa → â → a [undo] → r → o → n [raw])
```

---

## Pipeline Architecture

**7-Stage Processing Pipeline** (config-driven, recompute-from-raw):

```yaml
stage1_normalization:
  purpose: Normalize input, push CharInfo to char_buffer
  input: char
  output: CharInfo (lowercase ch + uppercase flag)

stage2_gatekeeper:
  purpose: Route non-Vietnamese input
  checks: temp_english_mode, non-alphabetic
  decision: Continue | PassThrough

stage3_compose:
  purpose: Recompute syllable from raw char_buffer (pure function)
  internal_steps:
    fallback: Undo / toggle / English-fallback detection
    segment: Raw keys → base + transform marks + tone keys
    transform: Apply diacritic marks (validation-gated)
    assemble: Place tone mark on vowel nucleus
  output: syllable_buffer; temp_english flag

stage4_orthography:
  purpose: Normalize tone position + Unicode
  apply: ToneStyle (Old/New)
  convert: To NFC (canonical composition)

stage5_learning:
  purpose: User pattern tracking (future, currently no-op)

stage6_lookup:
  purpose: Optional Hán Nôm dictionary lookup
  output: candidates in TypingContext

stage7_output:
  purpose: Generate final actions
  algorithm: Diff last_output vs syllable_buffer → Replace{backspace_count, text}
```

**Key Types**:
```rust
struct TypingContext {
    raw_buffer: Vec<char>,           // Raw input history
    current_syllable: Syllable,      // Current syllable
    temp_english_mode: bool,         // English fallback
    last_transformation: Option<TransformRecord>,
    last_output: String,             // For incremental updates
    tone_config: ToneConfig,
    candidates: Vec<Candidate>,      // For Hán Nôm
}

struct ToneConfig {
    free_marking: bool,              // Allow tone before transformation
    auto_correct_uo: bool,           // uo → ươ before tone
    max_modify_length: usize,        // Max backtrack length
}

enum ToneStyle { Old, New }          // óa vs oá
enum ToneMark { None, Acute, Grave, Hook, Tilde, Dot }
```

**Key Methods** (in `buttre-engine`):
```rust
fn find_main_vowel(text: &str) -> Option<usize>
fn auto_correct_uo(syllable: &mut Syllable)
fn reposition_existing_tone(syllable: &mut Syllable)
fn move_tone(text: &mut String, from: usize, to: usize)
```

---

## Build Commands

### Development

```bash
# Check code
cargo check
cargo check --package buttre-engine

# Run tests
cargo test
cargo test --package buttre-engine
cargo test --package buttre-engine -- --skip test_find_best_permutation

# Build
cargo build                    # Debug
cargo build --release          # Release (optimized)

# Code quality
cargo fmt                      # Format code
cargo clippy --all-targets --all-features  # Lints
```

### Windows TSF Deployment

```powershell
# Rebuild TSF DLL (requires Admin)
./rebuild-tsf.ps1

# Rebuild TSF DLL (debug mode)
./rebuild-tsf-debug.ps1

# Register DLL (requires Admin)
regsvr32 target/release/buttre_platform.dll

# Unregister DLL (requires Admin)
regsvr32 /u target/release/buttre_platform.dll
```

---

## Reference Implementations

**Unikey** (C++ reference for Vietnamese algorithms):
```yaml
location: .reference/unikey/
key_files:
  - vietkey.cpp:
      functions: putToneMark, putBreveMark, doubleChar, tempVietOff
      purpose: Tone application, character transformations, English fallback

  - ukengine.cpp:
      functions: processTone, getTonePosition, VSeqList, processRoof, processHook
      purpose: Tone positioning logic, vowel sequence detection
      data: VSeqList (70 predefined Vietnamese vowel sequences)

usage:
  - Tone placement algorithms
  - Vowel sequence detection
  - Buffer management (tempVietOff = temp English mode)
  - Auto-correction (uo → ươ on tone application)
  - ToneStyle support (Old/New)
```

**Other References**:
- **OpenKey**: `.reference/openkey/` (alternative Vietnamese IME)
- **IBus Bamboo**: `.reference/ibus-bamboo/` (Go-based IBus engine)
- **Weasel**: `.reference/weasel/` (Hán Nôm Rime engine)

---

## Rust Coding Rules

### Error Handling (CRITICAL)

**❌ FORBIDDEN** (will cause panics in production):
```rust
// NEVER do this:
let value = result.unwrap();              // PANIC on Err
let value = option.expect("message");     // PANIC on None
panic!("error");                           // Always crashes
todo!();                                   // Not implemented
unimplemented!();                          // Not implemented
```

**✅ CORRECT**:
```rust
// Use Result for fallible operations
pub fn parse_config(path: &Path) -> Result<Config, ConfigError> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| ConfigError::IoError { path: path.into(), source: e })?;
    toml::from_str(&content)
        .map_err(|e| ConfigError::ParseError { source: e })
}

// Use Option for optional values
pub fn find_vowel(text: &str) -> Option<char> {
    text.chars().find(|c| is_vowel(*c))
}

// Use ? for error propagation
pub fn load_config() -> anyhow::Result<Config> {
    let path = get_config_path()?;  // Propagate error
    let config = parse_config(&path)?;  // Propagate error
    Ok(config)
}
```

### Type Safety

**✅ Use newtypes**:
```rust
// GOOD - Type-safe
struct UserId(u64);
struct TonePosition(usize);
enum InputMethod { Telex, Vni, Viqr }

// BAD - No type safety
type UserId = u64;
let method = "telex";  // Stringly typed
```

### Performance

**✅ Avoid allocations in hot paths**:
```rust
// GOOD - No allocation
fn find_vowel(text: &str) -> Option<usize> {
    text.chars().position(|c| is_vowel(c))
}

// BAD - Unnecessary allocation
fn find_vowel(text: &str) -> Option<usize> {
    let chars: Vec<char> = text.chars().collect();  // Allocation!
    chars.iter().position(|c| is_vowel(*c))
}
```

**✅ Use static lookups**:
```rust
use lazy_static::lazy_static;
use std::collections::HashSet;

lazy_static! {
    static ref VOWELS: HashSet<char> = {
        ['a', 'e', 'i', 'o', 'u', 'y',
         'à', 'á', 'ả', 'ã', 'ạ',
         // ... more vowels
        ].iter().copied().collect()
    };
}

pub fn is_vowel(c: char) -> bool {
    VOWELS.contains(&c)  // O(1) lookup
}
```

---

## Documentation Standards

### Public API Documentation

**Required for all public functions**:
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
/// # Errors
///
/// Returns an error if... (describe error conditions)
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

### Module Documentation

```rust
//! Pipeline Module — 7-Stage Input Processing Pipeline
//!
//! This module implements a config-driven, 7-stage pipeline for processing
//! Vietnamese input methods (Telex, VNI, etc.).
//!
//! ## Architecture
//!
//! The pipeline consists of 7 stages:
//! 1. Normalization
//! 2. Gatekeeper
//! 3. Compose (recompute-from-raw)
//! 4. Orthography
//! 5. Learning (future)
//! 6. Lookup
//! 7. Output
```

---

## Common Patterns

### Pipeline Stage Pattern

```rust
use super::super::{PipelineStage, StageResult, TypingContext};

/// Stage 4: Transformation
pub struct Stage4Transform;

impl PipelineStage for Stage4Transform {
    fn name(&self) -> &'static str {
        "Transform"
    }

    fn process(&self, key: char, ctx: &mut TypingContext) -> StageResult {
        // 1. Check transformation key
        // 2. Apply transformation
        // 3. Update context
        // 4. Return Continue

        StageResult::Continue
    }
}
```

### Action Enum Pattern

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    DoNothing,
    Commit(String),
    Replace { backspace_count: usize, text: String },
    UpdateComposition { text: String, cursor: usize },
    ShowCandidates { candidates: Vec<Candidate>, input: String },
    HideCandidates,
}
```

---

## AI Agent Best Practices

### Before Starting Any Task

1. **Read Documentation**:
   - `CLAUDE.md` (this file) for context
   - `docs/ARCHITECTURE.md` for architecture
   - `docs/CODING_GUIDE.md` for coding standards
   - `docs/ROADMAP.md` for current status

2. **Understand Scope**:
   - What crate(s) are affected?
   - What existing patterns should be followed?
   - What tests need to be written?

3. **Check Current Status**:
   - What's the current phase? (See "Current Focus")
   - Any known issues to avoid? (See "Known Issues")
   - Any related work in progress?

### During Development

1. **Follow Patterns**:
   - Use existing patterns from `docs/CODING_GUIDE.md`
   - Look at similar code for reference
   - Don't reinvent the wheel

2. **Write Tests**:
   - Unit tests for new functions
   - Integration tests for workflows
   - Test edge cases and error paths

3. **Track Progress**:
   - Use TodoWrite tool for multi-step tasks
   - Mark tasks complete as you finish
   - Keep user informed

### After Completion

1. **Verify Quality**:
   ```bash
   cargo fmt
   cargo clippy --all-targets
   cargo test --all
   cargo build --release
   ```

2. **Update Documentation**:
   - Add/update inline docs
   - Update relevant docs/ files
   - Update CHANGELOG.md if needed

3. **Summarize Work**:
   - What was done
   - What tests were added
   - Any known limitations

---

## Resources

### Documentation

- **Main Docs**: `docs/README.md` - Navigation guide
- **Architecture**: `docs/ARCHITECTURE.md` - Complete system architecture
- **Coding**: `docs/CODING_GUIDE.md` - How to code in buttre
- **Roadmap**: `docs/ROADMAP.md` - Project roadmap and timeline
- **Pipeline**: `docs/PIPELINE_ARCHITECTURE.md` - Detailed pipeline docs
- **Vietnamese**: `docs/VIETNAMESE_ACCENT.md` - Orthography rules

### External Resources

- **Rust**: https://doc.rust-lang.org/
- **windows-rs**: https://github.com/microsoft/windows-rs
- **Unicode**: https://unicode.org/reports/tr15/ (NFC/NFD normalization)
- **Vietnamese**: Vietnamese orthography standards


_This configuration file ensures AI agents have complete context to assist effectively with buttre development._
