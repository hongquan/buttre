// SPDX-License-Identifier: GPL-3.0-only
// Vietnamese Engine Integration for TSF
//
// **Tests**: Integration tests for this module are located in `crates/buttre-platform/tests/platform_windows_tsf_tests.rs`.

use buttre_core::InputBuffer;
use buttre_core::Action;
use buttre_core::Keyboard;
use buttre_core::KeyboardBuilder;
use super::candidate_ui::CandidateItem;
use std::path::PathBuf;

/// Vietnamese input mode
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum VietnameseMode {
    Telex,
    VNI,
    Nom,
    Custom(String), // Custom config with method ID
}

/// Vietnamese Engine for TSF
/// Wraps buttre-keyboard and provides TSF-compatible interface
pub struct VietnameseEngine {
    mode: VietnameseMode,
    keyboard: Option<Keyboard>,
    buffer: InputBuffer,
}

impl VietnameseEngine {
    /// Create a new Vietnamese engine
    pub fn new(mode: VietnameseMode) -> Self {
        let keyboard = Self::load_keyboard(&mode);

        Self {
            mode,
            keyboard,
            buffer: InputBuffer::new(),
        }
    }
    
    /// Load keyboard instance for given mode
    fn load_keyboard(mode: &VietnameseMode) -> Option<Keyboard> {
        match mode {
            VietnameseMode::Telex => {
                KeyboardBuilder::telex_with_composition(true).ok()
            }
            VietnameseMode::VNI => {
                KeyboardBuilder::vni_with_composition(true).ok()
            }
            VietnameseMode::Nom => {
                // Load Nôm dictionary and create keyboard with TSF composition mode
                let nom_path = buttre_core::vietnamese::get_nom_db_path();
                KeyboardBuilder::nom_with_composition(nom_path, true).ok()
            }
            VietnameseMode::Custom(method_id) => {
                // Load custom config from file (same as Hook)
                tracing::info!("TSF: Loading custom keyboard: {}", method_id);
                let custom_dir = buttre_core::vietnamese::get_custom_dir();
                let config_path = custom_dir.join(format!("{}.toml", method_id));
                
                if config_path.exists() {
                    match buttre_core::Config::load(config_path.to_str().unwrap()) {
                        Ok(config) => {
                            tracing::info!(“TSF: ✓ Loaded custom keyboard from {:?}”, config_path);
                            // Create keyboard with composition mode for TSF
                            KeyboardBuilder::new()
                                .with_config(config)
                                .with_composition(true)
                                .build()
                                .ok()
                        }
                        Err(e) => {
                            tracing::warn!("TSF: Failed to load custom keyboard: {}", e);
                            None
                        }
                    }
                } else {
                    tracing::warn!("TSF: Custom config not found: {:?}", config_path);
                    None
                }
            }
        }
    }

    /// Process a key press
    /// Returns first action (main typing action), ignoring candidate UI actions
    pub fn process_key(&mut self, ch: char) -> Action {
        if let Some(ref mut kb) = self.keyboard {
            match kb.process(ch) {
                Ok(actions) => {
                    // Take first action (main typing action)
                    actions.into_iter().next().unwrap_or(Action::DoNothing)
                }
                Err(e) => {
                    tracing::warn!("Keyboard process error: {}", e);
                    Action::DoNothing
                }
            }
        } else {
            Action::DoNothing
        }
    }

    /// Process backspace
    pub fn process_backspace(&mut self) -> Action {
        if let Some(ref mut kb) = self.keyboard {
            match kb.backspace() {
                Ok(action) => action,
                Err(e) => {
                    tracing::warn!("Keyboard backspace error: {}", e);
                    Action::DoNothing
                }
            }
        } else {
            Action::DoNothing
        }
    }

    /// Reset the engine state
    pub fn reset(&mut self) {
        self.buffer.clear();
        if let Some(ref mut kb) = self.keyboard {
            kb.reset();
        }
    }

    /// Get current buffer content
    pub fn buffer_content(&self) -> String {
        if let Some(ref kb) = self.keyboard {
            kb.buffer().to_string()
        } else {
            self.buffer.to_string()
        }
    }

    /// Switch input mode
    pub fn set_mode(&mut self, mode: VietnameseMode) {
        if self.mode != mode {
            self.keyboard = Self::load_keyboard(&mode);
            self.mode = mode;
            self.reset();
        }
    }

    /// Generate candidate list (stub for Nom support)
    pub fn generate_candidates(&self, _input: &str) -> Vec<CandidateItem> {
        // TODO: Implement Nom candidate generation when needed
        Vec::new()
    }
}

