//! Pipeline Stages — 7-stage processing pipeline (Phase 4+).
//!
//! ## Stage Architecture (post-Phase-4)
//!
//! | Stage | Name | Responsibility |
//! |-------|------|----------------|
//! | 1 | Normalization | Normalize input, populate char_buffer |
//! | 2 | Gatekeeper | Route non-Vietnamese / temp-English passthrough |
//! | 3 | Compose | Recompute-from-raw: segment → transform → tone → fallback |
//! | 4 | Orthography | Normalize Unicode form (NFC/NFD) |
//! | 5 | Learning | Track user patterns (future, currently no-op) |
//! | 6 | Lookup | Dictionary lookup (Nôm candidates) |
//! | 7 | Output | Generate diff actions |
//!
//! The former stages (Transform, Tone, Permutation, Reconciliation, Retrofix)
//! have been retired and replaced by ComposeStage (Stage 3).

// Core stages (always present)
pub mod stage1_normalization;
pub mod stage2_gatekeeper;
pub mod stage3_validation;

// Compose stage — replaces old stages 4-8
pub mod compose_stage;

// Post-compose stages
pub mod stage9_orthography;
pub mod stage10_learning;
pub mod stage11_lookup;
pub mod stage12_output;

// VNI-specific optimizations (retained for reference; not used in default pipeline)
pub mod vni_optimized;

// Telex static table (retained for reference)
pub mod telex_table;

#[cfg(test)]
mod integration_tests;

// Re-exports for convenience
pub use stage1_normalization::NormalizationStage;
pub use stage2_gatekeeper::GatekeeperStage;
pub use stage3_validation::ValidationStage;
pub use compose_stage::ComposeStage;
pub use stage9_orthography::OrthographyStage;
pub use stage10_learning::LearningStage;
pub use stage11_lookup::LookupStage;
pub use stage12_output::OutputStage;
pub use vni_optimized::VniOptimizedTransformStage;
