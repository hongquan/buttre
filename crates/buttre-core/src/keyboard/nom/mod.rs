//! Nôm Input Method - Hardcoded Configuration
//!
//! This module provides the hardcoded Nôm configuration.
//! Nôm is a traditional Vietnamese input method.

pub mod transforms;
pub mod special;

use buttre_engine::pipeline::config::{PipelineConfig, ToneMark, ToneStyle, UnicodeForm};
use std::sync::Arc;
use std::collections::HashMap;

/// Build complete Nôm configuration
///
/// ## Returns
/// PipelineConfig ready for use with PipelineExecutor
///
/// ## Example
/// ```rust
/// use buttre_core::keyboard::nom;
/// use buttre_engine::pipeline::PipelineExecutor;
///
/// let config = nom::build_config();
/// let mut executor = PipelineExecutor::new(config);
/// ```
pub fn build_config() -> PipelineConfig {
    let mut config = PipelineConfig::new("nom");
    
    // Add transformation rules
    config.transform_rules = transforms::get_rules();
    
    // Add tone mappings (same as Telex for now)
    config.tone_map = get_tone_map();
    
    // Add special context rules
    config.context_rules = Arc::new(special::get_rules());
    
    // Settings
    config.enable_lookup = false;
    config.tone_style = ToneStyle::New;
    config.unicode_form = UnicodeForm::NFC;
    
    config
}

/// Get Nôm tone mappings
///
/// For now, uses same tone keys as Telex
fn get_tone_map() -> HashMap<char, ToneMark> {
    let mut map = HashMap::new();
    
    // Same as Telex
    map.insert('s', ToneMark::Acute);
    map.insert('f', ToneMark::Grave);
    map.insert('r', ToneMark::Hook);
    map.insert('x', ToneMark::Tilde);
    map.insert('j', ToneMark::Dot);
    
    // Uppercase
    map.insert('S', ToneMark::Acute);
    map.insert('F', ToneMark::Grave);
    map.insert('R', ToneMark::Hook);
    map.insert('X', ToneMark::Tilde);
    map.insert('J', ToneMark::Dot);
    
    map
}
