//! Keyboard builder
//!
//! **Tests**: Integration tests for this module are located in `crates/buttre-core/tests/keyboard_builder_tests.rs`.
//!
//! Provides a fluent interface for creating Keyboard instances.
//! Supports both TOML-based Config and hardcoded PipelineConfig.

use super::{Config, Keyboard};
use std::path::PathBuf;
use std::sync::Arc;
use buttre_engine::pipeline::config::{PipelineConfig, LookupSettings};
use buttre_engine::pipeline::nom_dictionary::NomDictionary;

/// Builder for creating keyboards from configurations
pub struct KeyboardBuilder {
    config: Option<Config>,
    pipeline_config: Option<PipelineConfig>,
    language: Option<String>,
    nom_dictionary_path: Option<PathBuf>,
    use_composition: bool,
}

impl KeyboardBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            config: None,
            pipeline_config: None,
            language: None,
            nom_dictionary_path: None,
            use_composition: false,
        }
    }
    
    /// Create a Telex keyboard (using hardcoded config)
    pub fn telex() -> anyhow::Result<Keyboard> {
        Self::telex_with_composition(false)
    }

    /// Create a Telex keyboard with optional composition support
    pub fn telex_with_composition(use_composition: bool) -> anyhow::Result<Keyboard> {
        let config = crate::keyboard::telex::build_config();
        Self::new()
            .with_pipeline_config(config)
            .with_composition(use_composition)
            .build()
    }
    
    /// Create a VNI keyboard (using hardcoded config)
    pub fn vni() -> anyhow::Result<Keyboard> {
        Self::vni_with_composition(false)
    }

    /// Create a VNI keyboard with optional composition support
    pub fn vni_with_composition(use_composition: bool) -> anyhow::Result<Keyboard> {
        let config = crate::keyboard::vni::build_config();
        Self::new()
            .with_pipeline_config(config)
            .with_composition(use_composition)
            .build()
    }

    /// Create a Nôm keyboard
    pub fn nom(dictionary_path: Option<PathBuf>) -> anyhow::Result<Keyboard> {
        // Nôm is based on Telex rules
        let config = crate::keyboard::telex::build_config();
        let mut builder = Self::new().with_pipeline_config(config);
        
        if let Some(path) = dictionary_path {
            builder = builder.with_nom_dictionary(path);
        }
        
        builder.build()
    }
    
    /// Create a Nôm keyboard with composition mode (for TSF)
    pub fn nom_with_composition(dictionary_path: Option<PathBuf>, use_composition: bool) -> anyhow::Result<Keyboard> {
        // Nôm is based on Telex rules
        let config = crate::keyboard::telex::build_config();
        let mut builder = Self::new()
            .with_pipeline_config(config)
            .with_composition(use_composition);
        
        if let Some(path) = dictionary_path {
            builder = builder.with_nom_dictionary(path);
        }
        
        builder.build()
    }
    
    /// Set the configuration (TOML based)
    pub fn with_config(mut self, config: Config) -> Self {
        self.config = Some(config);
        self
    }
    
    /// Set the pipeline configuration (Hardcoded/Direct)
    pub fn with_pipeline_config(mut self, config: PipelineConfig) -> Self {
        self.pipeline_config = Some(config);
        self
    }
    
    /// Set the language
    pub fn with_language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
    }
    
    /// Set Nôm dictionary path
    pub fn with_nom_dictionary(mut self, path: PathBuf) -> Self {
        self.nom_dictionary_path = Some(path);
        self
    }
    
    /// Enable TSF composition mode
    pub fn with_composition(mut self, use_composition: bool) -> Self {
        self.use_composition = use_composition;
        self
    }
    
    /// Build the keyboard
    pub fn build(self) -> anyhow::Result<Keyboard> {
        // 1. Determine base config
        let mut pipeline_config = if let Some(pc) = self.pipeline_config {
            pc
        } else if let Some(c) = self.config {
            c.to_pipeline_config()
        } else {
            return Err(anyhow::anyhow!("Config not set (neither TOML Config nor PipelineConfig provided)"));
        };
        
        // Apply composition setting
        pipeline_config.pipeline.use_composition = self.use_composition;
        
        // 2. Apply Nôm dictionary if requested
        if let Some(path) = self.nom_dictionary_path {
            match NomDictionary::open(path) {
                Ok(dict) => {
                    pipeline_config.dictionary = Some(Arc::new(dict));
                    // Enable lookup with candidate UI (no auto-replace)
                    // User will see numbered candidates 1-5 to choose from
                    pipeline_config.lookup = Some(LookupSettings {
                        auto_replace: false, // Show candidates instead of auto-replacing
                        ..Default::default()
                    });
                    // Legacy flag (keeping for compatibility)
                    pipeline_config.enable_lookup = true;
                }
                Err(e) => {
                    // Log error but continue without dictionary
                    eprintln!("Failed to load Nom dictionary: {}", e);
                }
            }
        }
        
        // 3. Create keyboard
        Keyboard::new(pipeline_config)
    }
}

impl Default for KeyboardBuilder {
    fn default() -> Self {
        Self::new()
    }
}

