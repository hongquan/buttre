//! VNI Input Method - Hardcoded Configuration
//!
//! This module provides the hardcoded VNI configuration.
//! VNI is another popular Vietnamese input method using numbers.

pub mod special;
pub mod tones;
pub mod transforms;
pub mod vowel_sequences; // NEW: Vowel sequence data for flexible typing

use buttre_engine::pipeline::config::{PipelineConfig, ToneStyle, UnicodeForm};
use buttre_engine::vowel::TonePositioningMode;
use std::sync::Arc;

/// Build complete VNI configuration
///
/// ## Returns
/// PipelineConfig ready for use with PipelineExecutor
///
/// ## Example
/// ```rust
/// use buttre_core::keyboard::vni;
/// use buttre_engine::pipeline::PipelineExecutor;
///
/// let config = vni::build_config();
/// let mut executor = PipelineExecutor::new(config);
/// ```
pub fn build_config() -> PipelineConfig {
    let mut config = PipelineConfig::new("vni");

    // Add transformation rules
    config.transform_rules = transforms::get_rules();

    // Add tone mappings
    config.tone_map = tones::get_map();

    // Add special context rules
    config.context_rules = Arc::new(special::get_rules());

    // NEW: Flexible typing configuration
    config.tone.free_marking = false; // Default: strict phonology
    config.tone.allow_permutation = true; // Enable flexible typing order
    config.tone.vowel_sequences = vowel_sequences::get_table();
    config.tone.positioning_mode = TonePositioningMode::Phonology;

    // Settings
    config.enable_lookup = false;
    config.tone_style = ToneStyle::Old;
    config.unicode_form = UnicodeForm::NFC;

    config
}
