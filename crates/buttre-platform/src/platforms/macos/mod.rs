//! buttre macOS Input Method
//!
//! Implementation using macOS Input Method Kit (IMKit)

#![cfg(target_os = "macos")]

pub mod ffi;

use crate::PlatformBackend;
use std::sync::{Arc, Mutex};
use buttre_core::{Action, Engine};
use anyhow::Result;

/// macOS backend implementation
pub struct MacOSBackend {
    enabled: bool,
}

impl PlatformBackend for MacOSBackend {
    fn new() -> Result<Self> {
        Ok(Self { enabled: false })
    }
    
    fn init(&mut self, _engine: Arc<Mutex<Engine>>) -> Result<()> {
        tracing::info!("Initializing macOS (IMKit) backend");
        Ok(())
    }
    
    fn process_key(&mut self, _key: char) -> Action {
        Action::DoNothing
    }
    
    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
    
    fn cleanup(&mut self) {
    }
}

// Re-export FFI functions for C
pub use ffi::{
    buttre_engine_new,
    buttre_engine_free,
    buttre_engine_process_key,
    buttre_engine_process_backspace,
    buttre_engine_reset,
    buttre_engine_set_method,
    buttre_engine_set_enabled,
};
