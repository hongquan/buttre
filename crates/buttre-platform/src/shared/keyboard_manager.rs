//! Keyboard Manager - Manages keyboard instances
//!
//! This module provides a simple wrapper around buttre-keyboard
//! for use in buttre-platform

use buttre_core::{Config, KeyboardBuilder, Keyboard, Action};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use anyhow::Result;

/// Keyboard Manager - manages multiple keyboard instances
pub struct KeyboardManager {
    keyboards: HashMap<String, Arc<Mutex<Keyboard>>>,
    current: Option<String>,
}

impl KeyboardManager {
    /// Create a new keyboard manager
    pub fn new() -> Self {
        Self {
            keyboards: HashMap::new(),
            current: None,
        }
    }
    
    /// Load a keyboard from config file
    pub fn load_keyboard(&mut self, id: &str, config_path: &str) -> Result<()> {
        let config = Config::load(config_path)?;
        let keyboard = KeyboardBuilder::new()
            .with_config(config)
            .with_language("vietnamese")
            .build()?;
        
        self.keyboards.insert(id.to_string(), Arc::new(Mutex::new(keyboard)));
        Ok(())
    }
    
    /// Set current keyboard
    pub fn set_current(&mut self, id: &str) -> Result<()> {
        if self.keyboards.contains_key(id) {
            self.current = Some(id.to_string());
            Ok(())
        } else {
            Err(anyhow::anyhow!("Keyboard not found: {}", id))
        }
    }
    
    /// Get current keyboard
    pub fn get_current(&self) -> Option<Arc<Mutex<Keyboard>>> {
        self.current.as_ref()
            .and_then(|id| self.keyboards.get(id))
            .cloned()
    }
    
    /// Process a keystroke with current keyboard
    pub fn process(&self, key: char) -> Result<Action> {
        if let Some(keyboard) = self.get_current() {
            let mut kb = keyboard.lock().unwrap();
            kb.process(key)
        } else {
            Ok(Action::DoNothing)
        }
    }
    
    /// Process backspace
    pub fn backspace(&self) -> Result<Action> {
        if let Some(keyboard) = self.get_current() {
            let mut kb = keyboard.lock().unwrap();
            kb.backspace()
        } else {
            Ok(Action::DoNothing)
        }
    }
    
    /// Reset current keyboard
    pub fn reset(&self) {
        if let Some(keyboard) = self.get_current() {
            let mut kb = keyboard.lock().unwrap();
            kb.reset();
        }
    }
}

impl Default for KeyboardManager {
    fn default() -> Self {
        Self::new()
    }
}
