//! buttre Hotkey Manager - Global hotkey management
//!
//! **Tests**: Integration tests for this module are located in `crates/buttre-core/tests/hotkey_tests.rs`.

use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager,
};
use log::{debug, info, warn};
use crate::hotkey::error::{HotkeyError, Result};
use std::collections::HashMap;

/// Hotkey event types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HotkeyAction {
    /// Toggle between current method and English
    Toggle,
    /// Switch to Telex
    Telex,
    /// Switch to VNI
    Vni,
    Nom,  // Unified Nôm method
    /// Switch to custom method (index)
    Custom(usize),
}

/// Manages global hotkeys for buttre
pub struct ButtreHotkeyManager {
    manager: GlobalHotKeyManager,
    hotkeys: HashMap<u32, HotkeyAction>,
}

impl ButtreHotkeyManager {
    /// Create new hotkey manager with default hotkeys
    pub fn new() -> Result<Self> {
        info!("Creating hotkey manager");
        
        let manager = GlobalHotKeyManager::new()
            .map_err(|e| HotkeyError::ManagerCreationFailed(e.to_string()))?;
        
        let mut hotkeys = HashMap::new();
        
        // Register default hotkeys
        let hotkey_configs = vec![
            // Toggle: Ctrl+Shift+Space (closest to Unikey's Ctrl+Shift)
            // Note: Can't register pure Ctrl+Shift (no key), so we use Space
            (Modifiers::CONTROL | Modifiers::SHIFT, Code::Space, HotkeyAction::Toggle),
            // Telex: Ctrl+Shift+F1 or Ctrl+Shift+1
            (Modifiers::CONTROL | Modifiers::SHIFT, Code::F1, HotkeyAction::Telex),
            (Modifiers::CONTROL | Modifiers::SHIFT, Code::Digit1, HotkeyAction::Telex),
            
            // VNI: Ctrl+Shift+F2 or Ctrl+Shift+2
            (Modifiers::CONTROL | Modifiers::SHIFT, Code::F2, HotkeyAction::Vni),
            (Modifiers::CONTROL | Modifiers::SHIFT, Code::Digit2, HotkeyAction::Vni),
            
            // Chữ Nôm: Ctrl+Shift+F3 or Ctrl+Shift+3
            (Modifiers::CONTROL | Modifiers::SHIFT, Code::F3, HotkeyAction::Nom),
            (Modifiers::CONTROL | Modifiers::SHIFT, Code::Digit3, HotkeyAction::Nom),
        ];
        
        for (mods, code, action) in hotkey_configs {
            let hotkey = HotKey::new(Some(mods), code);
            
            manager.register(hotkey)
                .map_err(|e| HotkeyError::RegistrationFailed(format!("{:?}: {}", action, e)))?;
            
            info!("Registered hotkey {:?} + {:?} -> {:?} (ID: {})", mods, code, action, hotkey.id());
            hotkeys.insert(hotkey.id(), action);
        }
        
        info!("Hotkey manager initialized with {} hotkeys", hotkeys.len());
        
        Ok(Self {
            manager,
            hotkeys,
        })
    }
    
    /// Register custom method hotkeys (Digit 4-9, 0)
    ///
    /// Custom 1 -> Ctrl+Shift+4
    /// Custom 2 -> Ctrl+Shift+5
    /// ...
    /// Custom 7 -> Ctrl+Shift+0
    pub fn register_custom_methods(&mut self, count: usize) -> Result<()> {
        let digit_keys = [
            Code::Digit4, Code::Digit5, Code::Digit6, Code::Digit7,
            Code::Digit8, Code::Digit9, Code::Digit0,
        ];
        
        for (i, code) in digit_keys.iter().take(count).enumerate() {
            let hotkey = HotKey::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), *code);
            let action = HotkeyAction::Custom(i);
            
            // Check if hotkey is already registered (to avoid duplicates or errors)
            if self.hotkeys.contains_key(&hotkey.id()) {
                info!("Hotkey for Custom {} already registered, skipping", i);
                continue;
            }

            match self.manager.register(hotkey) {
                Ok(_) => {
                    info!("Registered custom hotkey {:?} -> Custom {} (ID: {})", code, i, hotkey.id());
                    self.hotkeys.insert(hotkey.id(), action);
                }
                Err(e) => {
                     // Log warning but continue registering others
                     // Some hotkeys might be taken by system
                     warn!("Failed to register hotkey for Custom {}: {}", i, e);
                     continue;
                }
            }
        }
        
        Ok(())
    }
    
    /// Check for hotkey events
    /// Returns the action if a hotkey was pressed
    /// Note: Only processes Pressed events, ignores Released events
    pub fn check_hotkey(&self) -> Option<HotkeyAction> {
        use global_hotkey::HotKeyState;
        
        let mut last_action = None;
        let mut event_count = 0;
        let mut pressed_count = 0;
        let mut released_count = 0;
        
        // Drain all pending events
        while let Ok(event) = GlobalHotKeyEvent::receiver().try_recv() {
            event_count += 1;
            
            // IMPORTANT: Only process Pressed events, ignore Released
            // This prevents double-triggering and spurious events
            match event.state {
                HotKeyState::Pressed => {
                    pressed_count += 1;
                    debug!("Received hotkey PRESSED event #{}: ID {}", pressed_count, event.id);
                    
                    if let Some(action) = self.hotkeys.get(&event.id) {
                        info!("Hotkey pressed: {:?}", action);
                        last_action = Some(*action);
                    } else {
                        debug!("Unknown hotkey ID: {}", event.id);
                    }
                }
                HotKeyState::Released => {
                    released_count += 1;
                    debug!("Ignoring hotkey RELEASED event (ID {})", event.id);
                }
            }
        }
        
        if event_count > 0 {
            debug!("Total events: {} (Pressed: {}, Released: {})", event_count, pressed_count, released_count);
        }
        
        last_action
    }
}

impl Drop for ButtreHotkeyManager {
    fn drop(&mut self) {
        for (id, action) in &self.hotkeys {
            // Note: We can't easily unregister by ID, but GlobalHotKeyManager
            // will clean up on drop
            debug!("Cleaning up hotkey {:?} (ID: {})", action, id);
        }
    }
}

