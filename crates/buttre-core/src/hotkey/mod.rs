//! buttre Hotkey Module
//!
//! Provides global hotkey management for buttre using the global-hotkey crate.

mod error;
mod manager;

pub use error::{HotkeyError, Result};
pub use manager::{ButtreHotkeyManager, HotkeyAction};

// Re-export commonly used types
pub use global_hotkey::hotkey::{Code, Modifiers};
