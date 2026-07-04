//! Services Layer - Business logic and state management
//!
//! This module provides high-level services that encapsulate business logic
//! and integrate with the event bus for loose coupling.
//!
//! # Services
//!
//! - `KeyboardService` - Manages keyboard instances and input processing
//! - `ConfigService` - Loads and discovers keyboard configurations
//! - `MethodRegistry` - Registry of available input methods
//! - `HotkeyService` - Global hotkey management
//! - `SettingsService` - Application settings persistence
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │                    Services Layer                            │
//! │  ┌───────────────┐  ┌───────────────┐  ┌─────────────────┐  │
//! │  │KeyboardService│  │ ConfigService │  │ MethodRegistry  │  │
//! │  │               │  │               │  │                 │  │
//! │  └───────┬───────┘  └───────┬───────┘  └────────┬────────┘  │
//! │          │                  │                   │           │
//! │          └──────────────────┼───────────────────┘           │
//! │                             │                               │
//! │                      ┌──────▼──────┐                        │
//! │                      │  Event Bus  │                        │
//! │                      └─────────────┘                        │
//! └─────────────────────────────────────────────────────────────┘
//! ```
//!
//! # Example
//!
//! ```rust,ignore
//! use buttre_core::services::*;
//! use buttre_core::events::create_event_bus;
//!
//! // Create event bus
//! let bus = create_event_bus();
//!
//! // Create services
//! let mut keyboard = KeyboardService::new(bus.clone());
//! let config = ConfigService::new()?;
//! let mut registry = MethodRegistry::new()?;
//! let hotkey = HotkeyService::new(bus.clone())?;
//! let mut settings = SettingsService::new(bus.clone());
//!
//! // Use services
//! keyboard.create_preset(Preset::Telex)?;
//! keyboard.switch("telex")?;
//! let action = keyboard.process('a')?;
//! ```

mod config_service;
mod hotkey_service;
mod keyboard_service;
mod method_registry;
mod settings_service;

// Re-export public types
pub use config_service::{ConfigInfo, ConfigService, ConfigSource};
pub use hotkey_service::HotkeyService;
pub use keyboard_service::{KeyboardService, Preset};
pub use method_registry::MethodRegistry;
pub use settings_service::SettingsService;
