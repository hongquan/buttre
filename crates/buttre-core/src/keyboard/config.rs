//! Config Loader - Load keyboard configurations from TOML files
//!
//! **Tests**: Integration tests for this module are located in `crates/buttre-core/tests/keyboard_config_tests.rs`.

use serde::Deserialize;
use std::collections::HashMap;

/// Keyboard configuration loaded from TOML
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub metadata: Metadata,
    pub transformations: HashMap<String, String>,

    /// Shift layer transformations (for native script keyboards like Khmer NiDA)
    /// Keys pressed with Shift modifier
    #[serde(default)]
    pub transformations_shift: HashMap<String, String>,

    /// AltGr layer transformations (for extended characters)
    /// Keys pressed with AltGr (Right Alt) modifier
    #[serde(default)]
    pub transformations_altgr: HashMap<String, String>,

    pub tones: HashMap<String, String>,
    pub rules: Rules,
    /// Deprecated: Separators are now handled at engine level (key_utils.rs)
    /// This field is kept for backward compatibility with existing config files
    #[serde(default, skip_serializing)]
    #[allow(dead_code)]
    separators: Option<DeprecatedSeparators>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Metadata {
    pub id: String,
    pub name: String,
    pub language: String,
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub description: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Rules {
    #[serde(default = "default_tone_position")]
    pub tone_position: String,
    #[serde(default)]
    pub validate_syllables: bool,
    #[serde(default = "default_unicode_form")]
    pub unicode_form: String,
    /// Enable native script mode for direct mapping keyboards (Khmer, Cham).
    ///
    /// When true, enables:
    /// - Single-char transforms (k → ꨆ)
    /// - Double-key patterns via raw_buffer (kk → ꩀ)
    /// - Pending prefix resolution
    ///
    /// When false (default), these features are disabled to not affect Telex/VNI.
    #[serde(default)]
    pub native_script_mode: bool,
}

/// Deprecated: Buffer termination is now handled at engine level (key_utils.rs)
/// This struct exists only for backward compatibility when parsing old config files
#[derive(Debug, Clone, Deserialize, Default)]
#[allow(dead_code)]
struct DeprecatedSeparators {
    #[serde(default)]
    keys: Vec<String>,
}

fn default_tone_position() -> String {
    "modern".to_string()
}

fn default_unicode_form() -> String {
    "nfc".to_string()
}

// Note: default_separator_keys() removed - buffer termination handled at engine level

impl Config {
    /// Load config from TOML file
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        Self::from_toml_str(&content)
    }

    /// Parse config from a TOML string.
    ///
    /// Named `from_toml_str` (not `from_str`) so it is never confused with
    /// `std::str::FromStr::from_str` — this method's signature otherwise
    /// matches that trait method exactly.
    pub fn from_toml_str(toml: &str) -> anyhow::Result<Self> {
        let config: Config = toml::from_str(toml)?;
        Ok(config)
    }

    /// Load built-in Telex config
    pub fn telex() -> anyhow::Result<Self> {
        // Embedded Telex config
        let toml = include_str!("../../configs/telex.toml");
        Self::from_toml_str(toml)
    }

    /// Load built-in VNI config
    pub fn vni() -> anyhow::Result<Self> {
        // Embedded VNI config
        let toml = include_str!("../../configs/vni.toml");
        Self::from_toml_str(toml)
    }

    /// Convert to PipelineConfig
    pub fn to_pipeline_config(&self) -> buttre_engine::pipeline::config::PipelineConfig {
        use buttre_engine::pipeline::config::PipelineConfig;

        let mut pipeline_config = PipelineConfig::new(&self.metadata.id);

        // Add transformations
        for (from, to) in &self.transformations {
            pipeline_config.add_transform(from, to);
        }

        // Add Shift layer transformations (for native scripts)
        for (from, to) in &self.transformations_shift {
            // Prefix with "S-" to indicate Shift modifier
            pipeline_config.add_transform(format!("S-{from}"), to);
        }

        // Add AltGr layer transformations
        for (from, to) in &self.transformations_altgr {
            // Prefix with "A-" to indicate AltGr modifier
            pipeline_config.add_transform(format!("A-{from}"), to);
        }

        // Add tones
        for (key_str, tone_str) in &self.tones {
            if let Some(key) = key_str.chars().next() {
                if let Some(tone) = parse_tone(tone_str) {
                    pipeline_config.add_tone(key, tone);
                }
            }
        }

        // Pass native script mode flag
        pipeline_config.native_script_mode = self.rules.native_script_mode;

        pipeline_config
    }
}

/// Parse tone from string
fn parse_tone(s: &str) -> Option<buttre_engine::pipeline::config::ToneMark> {
    use buttre_engine::pipeline::config::ToneMark;

    match s.to_lowercase().as_str() {
        "none" => Some(ToneMark::None),
        "acute" => Some(ToneMark::Acute),
        "grave" => Some(ToneMark::Grave),
        "hook" => Some(ToneMark::Hook),
        "tilde" => Some(ToneMark::Tilde),
        "dot" => Some(ToneMark::Dot),
        _ => None,
    }
}
