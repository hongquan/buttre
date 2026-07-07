//! Pipeline Module — 7-Stage Input Processing Pipeline (Phase 4+)
//!
//! Config-driven pipeline for Vietnamese input methods (Telex, VNI, Nôm, etc.).
//!
//! ## Architecture (post-Phase-4)
//!
//! | # | Stage | Responsibility |
//! |---|-------|----------------|
//! | 1 | Normalization | Normalize input, populate char_buffer |
//! | 2 | Gatekeeper | Route non-Vietnamese / temp-English passthrough |
//! | 3 | Compose | Recompute-from-raw: segment → transform → tone → fallback |
//! | 4 | Orthography | Normalize Unicode form |
//! | 5 | Learning | Track patterns (no-op until Phase 5) |
//! | 6 | Lookup | Dictionary lookup (Nôm candidates) |
//! | 7 | Output | Diff last_output → syllable_buffer → emit actions |
//!
//! The former dual-engine stages (Transform, Tone, Permutation, Reconciliation,
//! Retrofix) have been retired and replaced by `ComposeStage`.

mod attested_data; // Generated bitset — see examples/gen_attested_syllables.rs
pub mod config;
pub mod context;
pub mod dictionary;
pub mod executor;
pub mod nom_dictionary;
pub mod permutation;
pub mod presets;
pub mod rules; // Enhanced rules system (Phase 1)
pub mod stage;
pub mod stages;
pub mod validation; // Permutation engine for flexible typing (Phase 2)

// Re-exports for convenience
pub use config::{PipelineConfig, ToneMark};
pub use context::{Candidate, CandidateType, TransformRecord, TransformType, TypingContext};
pub use executor::PipelineExecutor;
pub use rules::{ConditionalRule, ContextRule, RuleAction, RuleMatcher};
pub use stage::{PipelineStage, StageResult};
// Note: SpecialHandler moved to buttre-core/keyboard/{telex,vni,nom}/special.rs

// Re-export preset functions
pub use presets::{simple_telex_config, telex_config, viqr_config, vni_config};
