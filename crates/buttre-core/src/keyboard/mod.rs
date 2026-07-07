//! Keyboard module - Configuration and keyboard management
//!
//! This module contains the core keyboard types that were previously in buttre-keyboard.
//! It provides:
//! - `Config` - TOML configuration loading
//! - `Keyboard` - Main keyboard wrapper around buttre-engine
//! - `KeyboardBuilder` - Builder pattern for creating keyboards
//!
//! # Example
//!
//! ```rust,ignore
//! use buttre_core::keyboard::{KeyboardBuilder, Config};
//!
//! // Use built-in preset
//! let mut keyboard = KeyboardBuilder::telex()?;
//!
//! // Or load custom config
//! let config = Config::load("my_method.toml")?;
//! let mut keyboard = KeyboardBuilder::new()
//!     .with_config(config)
//!     .build()?;
//!
//! // Process keystrokes
//! let action = keyboard.process('a')?;
//! ```

mod builder;
mod config;
// Filename deliberately NOT `keyboard.rs` — `mod keyboard;` inside the
// `keyboard` module directory triggers clippy::module_inception (a module
// with the same name as its containing module).
mod engine;

// Input method modules (hardcoded configs)
pub mod nom;
pub mod telex;
pub mod vni;

#[cfg(test)]
mod vni_debug_test;

// Re-export public types
pub use builder::KeyboardBuilder;
pub use config::{Config, Metadata, Rules};
pub use engine::{BackspaceMode, Keyboard, ToggleSignal};
