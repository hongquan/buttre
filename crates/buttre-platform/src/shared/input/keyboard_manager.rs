//! Keyboard management for buttre application
//!
//! ## Flow:
//! ```text
//! Platform → KeyboardManager → buttre-keyboard::Keyboard → buttre-engine
//! ```

use anyhow::Result;
use std::sync::{Arc, Mutex, RwLock};
use buttre_core::{Keyboard, KeyboardBuilder};

/// Keyboard manager that wraps buttre-keyboard with Arc<Mutex<>>
pub struct KeyboardManager {
    keyboard: Arc<RwLock<Option<Keyboard>>>,
    current_method: Arc<Mutex<String>>,
}

impl KeyboardManager {
    /// Create a new keyboard manager (starts with no keyboard - English mode)
    pub fn new() -> Result<Self> {
        Ok(Self {
            keyboard: Arc::new(RwLock::new(None)),
            current_method: Arc::new(Mutex::new("english".to_string())),
        })
    }

    /// Get a clone of the keyboard Arc for sharing across threads
    pub fn get_keyboard(&self) -> Arc<RwLock<Option<Keyboard>>> {
        self.keyboard.clone()
    }

    /// Set the input method
    pub fn set_method(&self, method: &str) -> Result<()> {
        tracing::info!("KeyboardManager: Setting method to '{}'", method);
        
        *self.current_method.lock().unwrap() = method.to_string();
        
        if method == "english" {
            // English mode - clear keyboard
            *self.keyboard.write().unwrap() = None;
            return Ok(());
        }

        // Load keyboard for this method
        let keyboard = match method {
            "telex" => {
                match KeyboardBuilder::telex() {
                    Ok(kb) => kb,
                    Err(e) => {
                        tracing::error!("Failed to load Telex keyboard: {}", e);
                        *self.keyboard.write().unwrap() = None;
                        return Err(e);
                    }
                }
            }
            "vni" => {
                match KeyboardBuilder::vni() {
                    Ok(kb) => kb,
                    Err(e) => {
                        tracing::error!("Failed to load VNI keyboard: {}", e);
                        *self.keyboard.write().unwrap() = None;
                        return Err(e);
                    }
                }
            }
            "nom" => {
                let nom_path = buttre_core::vietnamese::get_nom_db_path();
                if nom_path.is_none() {
                    tracing::warn!("Nôm dictionary not found! Using Telex fallback mode.");
                } else {
                    tracing::info!("Replacing text with Nôm using dictionary: {:?}", nom_path);
                }
                
                match KeyboardBuilder::nom(nom_path) {
                    Ok(kb) => kb,
                    Err(e) => {
                        tracing::error!("Failed to load Nôm keyboard: {}", e);
                        *self.keyboard.write().unwrap() = None;
                        return Err(e);
                    }
                }
            }
            _ => {
                // Try to load custom keyboard from file
                tracing::info!("Attempting to load custom keyboard: {}", method);
                let custom_dir = buttre_core::vietnamese::get_custom_dir();
                let config_path = custom_dir.join(format!("{}.toml", method));
                
                if config_path.exists() {
                    match buttre_core::Config::load(config_path.to_str().unwrap()) {
                        Ok(config) => {
                            tracing::info!("✓ Loaded custom keyboard from {:?}", config_path);
                            KeyboardBuilder::new().with_config(config).build()?
                        }
                        Err(e) => {
                            tracing::warn!("Failed to load custom keyboard: {}, falling back to English", e);
                            *self.keyboard.write().unwrap() = None;
                            return Ok(());
                        }
                    }
                } else {
                    tracing::warn!("Custom keyboard '{}' not found at {:?}, falling back to English", method, config_path);
                    *self.keyboard.write().unwrap() = None;
                    return Ok(());
                }
            }
        };

        *self.keyboard.write().unwrap() = Some(keyboard);
        tracing::info!("✓ Loaded keyboard for method '{}'", method);
        Ok(())
    }
    
    /// Get current method
    pub fn current_method(&self) -> String {
        self.current_method.lock().unwrap().clone()
    }
}

impl Default for KeyboardManager {
    fn default() -> Self {
        Self::new().expect("Failed to create keyboard manager")
    }
}
