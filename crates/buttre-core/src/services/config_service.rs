//! Config Service - Manages keyboard configuration loading and discovery
//!
//! **Tests**: Integration tests for this module are located in `crates/buttre-core/tests/service_config_tests.rs`.
//!
//! This service handles:
//! - Loading built-in configs (Telex, VNI)
//! - Loading custom configs from files
//! - Discovering custom configs in the keyboards directory
//! - Watching for config file changes (future)

use crate::KeyboardConfig;
use anyhow::{Result, Context};
use std::path::{Path, PathBuf};
use std::fs;

/// Config Service - Manages keyboard configurations
///
/// This service provides access to both built-in and custom keyboard configurations.
///
/// # Example
///
/// ```rust,ignore
/// use buttre_core::services::ConfigService;
///
/// let service = ConfigService::new();
///
/// // Load built-in config
/// let telex = service.load("telex")?;
///
/// // Load custom config
/// let custom = service.load_file("keyboards/my_method.toml")?;
///
/// // List all available configs
/// let configs = service.list()?;
/// ```
pub struct ConfigService {
    /// Directory for custom keyboard configs
    custom_dir: PathBuf,
}

/// Information about an available configuration
#[derive(Debug, Clone)]
pub struct ConfigInfo {
    /// Config ID
    pub id: String,
    /// Display name
    pub name: String,
    /// Language
    pub language: String,
    /// Source of the config
    pub source: ConfigSource,
}

/// Source of a configuration
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigSource {
    /// Built-in configuration
    Builtin,
    /// Custom configuration from file
    Custom(PathBuf),
}

impl ConfigService {
    /// Create a new ConfigService
    ///
    /// This will use the default custom keyboards directory:
    /// - Windows: `%APPDATA%/buttre/keyboards`
    /// - macOS: `~/Library/Application Support/buttre/keyboards`
    /// - Linux: `~/.config/buttre/keyboards`
    pub fn new() -> Result<Self> {
        let custom_dir = Self::get_custom_dir()?;
        
        // Ensure directory exists
        fs::create_dir_all(&custom_dir)
            .context("Failed to create keyboards directory")?;
        
        Ok(Self { custom_dir })
    }
    
    /// Create a ConfigService with a custom directory
    ///
    /// # Arguments
    ///
    /// * `custom_dir` - Path to the custom keyboards directory
    pub fn with_custom_dir(custom_dir: PathBuf) -> Result<Self> {
        fs::create_dir_all(&custom_dir)
            .context("Failed to create keyboards directory")?;
        
        Ok(Self { custom_dir })
    }
    
    /// Get the default custom keyboards directory
    fn get_custom_dir() -> Result<PathBuf> {
        let data_dir = dirs::data_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find data directory"))?;
        
        Ok(data_dir.join("buttre").join("keyboards"))
    }
    
    /// Load a configuration by ID
    ///
    /// This will first check for built-in configs (telex, vni),
    /// then look for custom configs in the keyboards directory.
    ///
    /// # Arguments
    ///
    /// * `id` - Config ID (e.g., "telex", "vni", "my_custom")
    ///
    /// # Returns
    ///
    /// The loaded configuration, or an error if not found
    pub fn load(&self, id: &str) -> Result<KeyboardConfig> {
        // Check built-in configs first
        match id {
            "telex" => return KeyboardConfig::telex(),
            "vni" => return KeyboardConfig::vni(),
            _ => {}
        }
        
        // Look for custom config
        let path = self.custom_dir.join(format!("{}.toml", id));
        if path.exists() {
            return self.load_file(&path);
        }
        
        Err(anyhow::anyhow!("Config '{}' not found", id))
    }
    
    /// Load a configuration from a file
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the TOML configuration file
    ///
    /// # Returns
    ///
    /// The loaded configuration
    pub fn load_file(&self, path: impl AsRef<Path>) -> Result<KeyboardConfig> {
        let path = path.as_ref();
        KeyboardConfig::load(path.to_str().ok_or_else(|| {
            anyhow::anyhow!("Invalid path")
        })?)
        .with_context(|| format!("Failed to load config from {:?}", path))
    }
    
    /// List all available configurations
    ///
    /// This includes both built-in and custom configurations.
    ///
    /// # Returns
    ///
    /// A vector of configuration information
    pub fn list(&self) -> Result<Vec<ConfigInfo>> {
        let mut configs = Vec::new();
        
        // Add built-in configs
        configs.push(ConfigInfo {
            id: "telex".to_string(),
            name: "Telex".to_string(),
            language: "vietnamese".to_string(),
            source: ConfigSource::Builtin,
        });
        
        configs.push(ConfigInfo {
            id: "vni".to_string(),
            name: "VNI".to_string(),
            language: "vietnamese".to_string(),
            source: ConfigSource::Builtin,
        });
        
        // Scan custom directory
        if self.custom_dir.exists() {
            for entry in fs::read_dir(&self.custom_dir)? {
                let entry = entry?;
                let path = entry.path();
                
                // Only process .toml files
                if path.extension().and_then(|s| s.to_str()) != Some("toml") {
                    continue;
                }
                
                // Try to load the config to get metadata
                if let Ok(config) = self.load_file(&path) {
                    configs.push(ConfigInfo {
                        id: config.metadata.id.clone(),
                        name: config.metadata.name.clone(),
                        language: config.metadata.language.clone(),
                        source: ConfigSource::Custom(path),
                    });
                }
            }
        }
        
        Ok(configs)
    }
    
    /// Get the custom keyboards directory path
    pub fn custom_dir(&self) -> &Path {
        &self.custom_dir
    }
    
    /// Check if a config exists
    ///
    /// # Arguments
    ///
    /// * `id` - Config ID to check
    pub fn exists(&self, id: &str) -> bool {
        // Check built-in
        if matches!(id, "telex" | "vni") {
            return true;
        }
        
        // Check custom
        let path = self.custom_dir.join(format!("{}.toml", id));
        path.exists()
    }
}

impl Default for ConfigService {
    fn default() -> Self {
        Self::new().expect("Failed to create ConfigService")
    }
}

