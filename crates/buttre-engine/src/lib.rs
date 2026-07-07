//! # buttre Engine - Vietnamese Input Processing Pipeline
//!
//! **Architecture Layer**: Foundation (Tier 1)
//!
//! ## 🎯 Purpose
//!
//! Pure Vietnamese language processing algorithms. This crate contains the core
//! 7-stage pipeline that transforms raw keystrokes into Vietnamese characters.

// `clippy::pedantic`/`clippy::nursery` are deliberately NOT enabled here —
// the workspace's own lint policy (`Cargo.toml` [workspace.lints.clippy])
// already states style/complexity are "disabled for now (too many warnings
// in existing code)". Warning on them at the crate level while CI runs
// `cargo clippy -- -D warnings` promoted ~500 pedantic/nursery style
// findings (doc-comment backticks, `use Self`, etc. — no correctness bugs)
// into hard build failures, silently breaking CI for every release.
#![warn(clippy::all, missing_docs)]
#![deny(unsafe_code)]
#![allow(clippy::module_name_repetitions, clippy::must_use_candidate)]
//!
//! ## 📊 Architecture Position
//!
//! ```text
//! buttre-platform (UI + OS Integration)
//!        ↓
//! buttre-keyboard (Configuration Management)
//!        ↓
//! buttre-engine ← YOU ARE HERE (Foundation Layer)
//! ```
//!
//! ## ✅ Responsibilities
//!
//! - Character transformations (aa→â, aw→ă, dd→đ)
//! - Tone mark placement (s→acute, f→grave)
//! - Syllable validation
//! - Unicode normalization
//! - 7-stage pipeline execution
//!
//! ## ❌ Does NOT Handle
//!
//! - TOML configuration (→ buttre-keyboard)
//! - Keyboard layouts (→ buttre-keyboard)
//! - Platform integration (→ buttre-platform)
//! - UI/Settings (→ buttre-platform)
//!
//! ## 🔧 Key Components
//!
//! - `PipelineExecutor` - Orchestrates the 7 stages
//! - `PipelineStage` - Trait implemented by each stage
//! - `Action` - Output actions (DoNothing/Commit/Replace)
//! - `InputBuffer` - Character buffer management
//!
//! ## 📝 Usage (from buttre-keyboard)
//!
//! ```rust,ignore
//! use buttre_engine::pipeline::{PipelineExecutor, PipelineConfig};
//!
//! let mut config = PipelineConfig::new("telex");
//! config.add_transform("aa", "â");
//! config.add_tone('s', ToneMark::Acute);
//!
//! let mut executor = PipelineExecutor::new(config);
//! let actions = executor.process('a'); // First 'a'
//! let actions = executor.process('a'); // Second 'a' → â
//! ```
//!
//! See `ARCHITECTURE.md` for full system design.

pub mod buffer;
pub mod compose;
pub mod pipeline;
pub mod tone; // Single source of truth for tone char tables and placement
pub mod types;
pub mod unicode;
pub mod vowel; // NEW: Vowel processing module for flexible typing // Phase 3: pure recompute-from-raw compose engine

// Re-export core types
pub use buffer::InputBuffer;
pub use pipeline::{PipelineConfig, PipelineExecutor, PipelineStage, StageResult, TypingContext};
pub use types::{Action, CharInfo, Config, WordForm};
pub use unicode::{normalize_nfc, normalize_nfd, sanitize_filename, str_eq_normalized};
pub use vowel::{find_vowel_clusters, VowelCluster, VowelSeqInfo, VowelSeqTable}; // NEW

/// Engine version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize tracing subscriber for logging
///
/// This should be called once at the start of your application.
/// It sets up structured logging with the tracing framework.
///
/// ## Example
///
/// ```rust,no_run
/// use buttre_engine::init_tracing;
///
/// init_tracing();
/// // Your code here
/// ```
pub fn init_tracing() {
    use tracing_subscriber::{fmt, EnvFilter};

    // Set up filter from environment variable RUST_LOG
    // Default to "info" level if not set
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    // Initialize subscriber with pretty formatting
    fmt()
        .with_env_filter(filter)
        .with_target(true)
        .with_thread_ids(false)
        .with_file(false)
        .with_line_number(false)
        .init();
}
