//! Keyboard observer for input method updates

use buttre_core::state::{StateObserver, Settings};
use crate::shared::KeyboardManager;
use log::info;

/// Observer that updates the KeyboardManager when input method changes
pub struct KeyboardObserver {
    keyboard_manager: KeyboardManager,
}

impl KeyboardObserver {
    /// Create a new KeyboardObserver
    ///
    /// # Arguments
    /// * `keyboard_manager` - The keyboard manager to update
    pub fn new(keyboard_manager: KeyboardManager) -> Self {
        Self { keyboard_manager }
    }
}

impl StateObserver for KeyboardObserver {
    fn on_method_changed(&self, method: &str, _enabled: bool) {
        info!("KeyboardObserver: Updating keyboard to method '{}'", method);
        
        if let Err(e) = self.keyboard_manager.set_method(method) {
            log::error!("Failed to set keyboard method: {:?}", e);
        }
    }

    fn on_settings_changed(&self, _settings: &Settings) {
        // Keyboard doesn't need to react to other settings changes
    }
}
