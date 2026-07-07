//! Vowel Processing Module
//!
//! This module provides shared data structures and algorithms for Vietnamese vowel handling.
//! It is used by both the config layer (buttre-core) and pipeline stages (buttre-engine).
//!
//! ## Architecture
//!
//! - **sequences.rs**: VowelSeqInfo struct and lookup tables
//! - **cluster.rs**: Vowel cluster detection algorithms
//! - **positioning.rs**: Tone positioning algorithms
//!
//! ## Design Principle
//!
//! This module follows the **Separation of Concerns** pattern:
//! - Data structures are defined here (shared)
//! - Data population happens in buttre-core/keyboard (config layer)
//! - Algorithm implementation uses these structures (pipeline layer)

pub mod cluster;
pub mod positioning;
pub mod sequences;

// Re-exports for convenience
pub use cluster::{find_vowel_clusters, is_vowel, normalize_vowel, ClusterType, VowelCluster};
pub use positioning::{find_tone_position, is_super_vowel, TonePositioningMode};
pub use sequences::{Mark, VowelSeq, VowelSeqInfo, VowelSeqTable};
