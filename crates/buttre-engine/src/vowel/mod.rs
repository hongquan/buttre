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

pub mod sequences;
pub mod cluster;
pub mod positioning;

// Re-exports for convenience
pub use sequences::{VowelSeqInfo, VowelSeq, Mark, VowelSeqTable};
pub use cluster::{VowelCluster, find_vowel_clusters, ClusterType, is_vowel, normalize_vowel};
pub use positioning::{find_tone_position, TonePositioningMode, is_super_vowel};
