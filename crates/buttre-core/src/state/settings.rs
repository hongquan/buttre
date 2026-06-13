//! Settings management for buttre application
//!
//! **Tests**: Integration tests for this module are located in `crates/buttre-core/tests/state_tests.rs`.
//!
//! This module handles loading and saving application settings to disk.
//! Settings are stored in a platform-specific location using TOML format.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Application settings
///
/// These settings are persisted to disk and loaded on application startup.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Settings {
    /// Current input method ID (e.g., "english", "telex", "vni", "nom", or custom method ID)
    pub input_method: String,
    
    /// Enable auto-correction features
    pub auto_correct: bool,
    
    /// Enable shorthand/macro expansion
    pub shorthand: bool,
    
    /// Launch buttre on system startup
    pub startup: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            input_method: "english".to_string(),
            auto_correct: false,
            shorthand: false,
            startup: false,
        }
    }
}

impl Settings {
    /// Get the settings file path
    ///
    /// Platform-specific paths:
    /// - Windows: %APPDATA%\buttre\settings.toml
    /// - macOS: ~/Library/Application Support/buttre/settings.toml
    /// - Linux: ~/.config/buttre/settings.toml
    pub fn get_path() -> Result<PathBuf> {
        let data_dir =
            dirs::data_dir().ok_or_else(|| anyhow::anyhow!("Could not find data directory"))?;
        let buttre_dir = data_dir.join("buttre");
        fs::create_dir_all(&buttre_dir)?;
        Ok(buttre_dir.join("settings.toml"))
    }

    /// Load settings from file, or return default if file doesn't exist
    ///
    /// This method will never fail - if the settings file cannot be loaded,
    /// it will return default settings instead.
    pub fn load() -> Self {
        match Self::get_path() {
            Ok(path) => {
                if path.exists() {
                    if let Ok(content) = fs::read_to_string(&path) {
                        if let Ok(settings) = toml::from_str(&content) {
                            return settings;
                        }
                    }
                }
            }
            Err(e) => eprintln!("Failed to get settings path: {:?}", e),
        }
        Self::default()
    }

    /// Save settings to file
    ///
    /// # Errors
    /// Returns an error if the settings file cannot be written.
    pub fn save(&self) -> Result<()> {
        let path = Self::get_path()?;
        let content = toml::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }
}

