//! Event system for buttre
//!
//! This module provides a centralized event bus for loose coupling between components.
//! Components can publish events and subscribe to events without knowing about each other.
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────┐
//! │  Component  │──publish──┐
//! └─────────────┘           │
//!                           ▼
//! ┌─────────────┐      ┌─────────┐      ┌─────────────┐
//! │  Component  │◄─────│EventBus │─────►│  Component  │
//! └─────────────┘      └─────────┘      └─────────────┘
//!                           ▲
//! ┌─────────────┐           │
//! │  Component  │──publish──┘
//! └─────────────┘
//! ```
//!
//! # Usage
//!
//! ```rust,ignore
//! use buttre_core::events::{create_event_bus, AppEvent};
//!
//! // Create event bus
//! let bus = create_event_bus();
//!
//! // Subscribe to events
//! bus.subscribe(|event| {
//!     match event {
//!         AppEvent::MethodChanged { method, enabled } => {
//!             println!("Method changed: {} (enabled: {})", method, enabled);
//!         }
//!         AppEvent::Error { source, message } => {
//!             eprintln!("Error from {}: {}", source, message);
//!         }
//!         _ => {}
//!     }
//! });
//!
//! // Publish events
//! bus.publish(AppEvent::method_changed("telex", true));
//! bus.publish(AppEvent::info("Application started"));
//! ```

mod types;
mod bus;

// Re-export all public types
pub use types::{
    AppEvent,
    HotkeyAction,
    MethodInfo,
    MethodSource,
    LogLevel,
};

pub use bus::{
    EventBus,
    SharedEventBus,
    EventHandler,
    create_event_bus,
};
