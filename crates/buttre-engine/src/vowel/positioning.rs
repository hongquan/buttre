//! Tone Positioning Algorithms
//!
//! **Tests**: Integration tests for this module are located in `crates/buttre-engine/tests/vowel_positioning_tests.rs`.
//!
//! This module provides algorithms for determining where to place tone marks
//! in Vietnamese syllables.
//!
//! ## Vietnamese Tone Positioning Rules
//!
//! The rules for placing tone marks follow Vietnamese orthography:
//!
//! 1. **Single vowel:** Tone goes on the vowel
//! 2. **Super vowels (ă, â, ê, ô, ơ):** Tone ALWAYS goes on the super vowel
//! 3. **Double vowels:**
//!    - With super vowel: Tone on super vowel
//!    - oa, oe, uy: Depends on style (old vs new)
//!    - qu, gi: Tone on vowel AFTER qu/gi
//! 4. **Triple vowels:** Tone on the middle vowel
//!
//! ## Free Marking Mode
//!
//! When `free_marking = true`, users can place tones on any vowel in the cluster,
//! overriding the phonology rules. This provides maximum flexibility.

use super::cluster::{VowelCluster, ClusterType};
use super::sequences::VowelSeqTable;

/// Tone Positioning Mode
///
/// Determines how tone marks are placed in syllables.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TonePositioningMode {
    /// Follow Vietnamese phonology rules (default)
    Phonology,
    
    /// Allow tone on any vowel (free marking)
    Free,
}

/// Check if a vowel is a "super vowel" (ă, â, ê, ô, ơ)
///
/// Super vowels ALWAYS receive the tone mark in Vietnamese orthography.
pub fn is_super_vowel(ch: char) -> bool {
    matches!(
        ch.to_lowercase().next().unwrap_or(ch),
        'ă' | 'â' | 'ê' | 'ô' | 'ơ'
    )
}

/// Find the tone position in a syllable
///
/// ## Algorithm
///
/// This is the main entry point for tone positioning. It dispatches to either
/// phonology-based or free-marking positioning based on the mode.
///
/// ## Arguments
///
/// - `syllable`: The syllable string
/// - `cluster`: The vowel cluster in the syllable
/// - `mode`: Positioning mode (Phonology or Free)
/// - `input_pos`: Position where the tone key was typed (for free marking)
///
/// ## Returns
///
/// The character index where the tone should be placed
pub fn find_tone_position(
    syllable: &str,
    cluster: &VowelCluster,
    mode: TonePositioningMode,
    input_pos: Option<usize>,
) -> Option<usize> {
    match mode {
        TonePositioningMode::Phonology => find_phonology_position(syllable, cluster),
        TonePositioningMode::Free => find_free_position(cluster, input_pos),
    }
}

/// Find tone position using Vietnamese phonology rules
///
/// ## Algorithm
///
/// 1. **Priority 1: Super vowels** (ă, â, ê, ô, ơ)
///    - If cluster contains a super vowel, tone goes there
///
/// 2. **Priority 2: Special patterns**
///    - qu/gi: Skip the 'u'/'i' and tone the next vowel
///
/// 3. **Priority 3: Triple vowels**
///    - Tone goes on the middle vowel (index 1)
///
/// 4. **Priority 4: Double vowels with context rules**
///    - oa, oe, uy: Depends on final consonant and style
///    - Default: First vowel
///
/// ## Arguments
///
/// - `syllable`: The full syllable (needed for qu/gi detection)
/// - `cluster`: The vowel cluster
///
/// ## Returns
///
/// The absolute character index in the syllable where tone should be placed
pub fn find_phonology_position(syllable: &str, cluster: &VowelCluster) -> Option<usize> {
    // PRIORITY 1: Super vowels
    for (i, vowel) in cluster.vowels.iter().enumerate() {
        if is_super_vowel(*vowel) {
            return Some(cluster.start_pos + i);
        }
    }
    
    // PRIORITY 2: Special patterns (qu, gi)
    let syllable_lower: String = syllable.to_lowercase();
    if syllable_lower.starts_with("qu") && cluster.vowels.len() >= 2 {
        // Skip 'u' in 'qu', tone goes on next vowel
        // Example: "quá" - tone on 'á' (position after 'u')
        return Some(cluster.start_pos + 1);
    }
    if syllable_lower.starts_with("gi") && cluster.vowels.len() >= 2 {
        // Skip 'i' in 'gi', tone goes on next vowel
        // Example: "giá" - tone on 'á'
        return Some(cluster.start_pos + 1);
    }
    
    // PRIORITY 3: Triple vowels - tone on middle
    if cluster.vowels.len() == 3 {
        return Some(cluster.start_pos + 1);
    }
    
    // PRIORITY 4: Double vowels with special rules
    if cluster.vowels.len() == 2 {
        match cluster.cluster_type {
            ClusterType::DoubleOA => {
                // oa, oe: Modern style → tone on second vowel (a/e)
                // Old style → tone on first vowel (o)
                // Default to modern style
                return Some(cluster.start_pos + 1);
            },
            ClusterType::CompoundUO => {
                // ươ: Tone on ơ (second vowel)
                return Some(cluster.start_pos + 1);
            },
            _ => {
                // Default for double vowels: First vowel
                return Some(cluster.start_pos);
            }
        }
    }
    
    // Single vowel or fallback: First vowel
    Some(cluster.start_pos)
}

/// Find tone position using free marking (nearest vowel)
///
/// ## Algorithm
///
/// In free marking mode, the tone is placed on the vowel nearest to where
/// the user typed the tone key.
///
/// ## Arguments
///
/// - `cluster`: The vowel cluster
/// - `input_pos`: Position where the tone key was typed
///
/// ## Returns
///
/// The character index closest to `input_pos` within the cluster
pub fn find_free_position(cluster: &VowelCluster, input_pos: Option<usize>) -> Option<usize> {
    let input_pos = input_pos?;
    
    // Find the vowel position closest to input_pos
    let mut closest_pos = cluster.start_pos;
    let mut min_distance = (input_pos as i32 - cluster.start_pos as i32).abs();
    
    for i in 0..cluster.vowels.len() {
        let pos = cluster.start_pos + i;
        let distance = (input_pos as i32 - pos as i32).abs();
        
        if distance < min_distance {
            min_distance = distance;
            closest_pos = pos;
        }
    }
    
    Some(closest_pos)
}

/// Find tone position using vowel sequence table
///
/// ## Algorithm
///
/// This function uses the pre-defined vowel sequence table to find the
/// correct tone position. This is more accurate than the heuristic approach.
///
/// ## Arguments
///
/// - `cluster`: The vowel cluster
/// - `table`: The vowel sequence table (from config)
///
/// ## Returns
///
/// The character index where tone should be placed according to the table
pub fn find_tone_position_from_table(
    cluster: &VowelCluster,
    table: &VowelSeqTable,
) -> Option<usize> {
    // Look up the cluster in the table
    let seq_info = table.find_by_vowels(&cluster.vowels)?;
    
    // Get the primary tone position from the table
    let relative_pos = seq_info.primary_tone_position()?;
    
    // Convert to absolute position
    Some(cluster.start_pos + relative_pos)
}

