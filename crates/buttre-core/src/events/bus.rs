//! Event Bus implementation
//!
//! **Tests**: Integration tests for this module are located in `crates/buttre-core/tests/event_bus_tests.rs`.
//!
//! Provides a thread-safe, centralized event distribution system.
//! Components can subscribe to events and publish events without knowing about each other.

use super::types::AppEvent;
use std::sync::{Arc, RwLock};

/// Event handler function type
///
/// Handlers receive a reference to the event and can perform any action.
/// Handlers must be Send + Sync to work across threads.
pub type EventHandler = Box<dyn Fn(&AppEvent) + Send + Sync>;

/// Event Bus - Central event distribution system
///
/// The EventBus allows components to communicate through events without
/// tight coupling. Publishers emit events, subscribers receive them.
///
/// # Thread Safety
///
/// EventBus is thread-safe and can be shared across threads using Arc.
/// Multiple threads can publish and subscribe simultaneously.
///
/// # Example
///
/// ```rust,ignore
/// use buttre_core::events::{EventBus, AppEvent};
/// use std::sync::Arc;
///
/// let bus = Arc::new(EventBus::new());
///
/// // Subscribe to events
/// bus.subscribe(|event| {
///     println!("Event received: {:?}", event);
/// });
///
/// // Publish an event
/// bus.publish(AppEvent::info("Application started"));
/// ```
pub struct EventBus {
    /// List of registered event handlers
    handlers: RwLock<Vec<EventHandler>>,
}

impl EventBus {
    /// Create a new EventBus
    pub fn new() -> Self {
        Self {
            handlers: RwLock::new(Vec::new()),
        }
    }

    /// Subscribe to all events
    ///
    /// The provided handler will be called for every event published to the bus.
    /// Handlers are called synchronously in the order they were registered.
    ///
    /// # Arguments
    ///
    /// * `handler` - A function or closure that takes `&AppEvent`
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// bus.subscribe(|event| {
    ///     match event {
    ///         AppEvent::MethodChanged { method, .. } => {
    ///             println!("Method changed to: {}", method);
    ///         }
    ///         _ => {}
    ///     }
    /// });
    /// ```
    pub fn subscribe<F>(&self, handler: F)
    where
        F: Fn(&AppEvent) + Send + Sync + 'static,
    {
        let mut handlers = self.handlers.write().unwrap();
        handlers.push(Box::new(handler));
    }

    /// Publish an event to all subscribers
    ///
    /// All registered handlers will be called synchronously with the event.
    /// If a handler panics, it will not affect other handlers.
    ///
    /// # Arguments
    ///
    /// * `event` - The event to publish
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// bus.publish(AppEvent::method_changed("telex", true));
    /// ```
    pub fn publish(&self, event: AppEvent) {
        let handlers = self.handlers.read().unwrap();

        for handler in handlers.iter() {
            // Catch panics to prevent one bad handler from breaking others
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                handler(&event);
            }));

            if let Err(e) = result {
                eprintln!("Event handler panicked: {:?}", e);
            }
        }
    }

    /// Emit an event (alias for publish)
    ///
    /// This is a convenience method that does the same as `publish`.
    pub fn emit(&self, event: AppEvent) {
        self.publish(event);
    }

    /// Get the number of registered handlers
    ///
    /// Useful for debugging and testing.
    pub fn subscriber_count(&self) -> usize {
        self.handlers.read().unwrap().len()
    }

    /// Clear all subscribers
    ///
    /// Removes all registered event handlers.
    /// Useful for testing or resetting the bus.
    pub fn clear(&self) {
        let mut handlers = self.handlers.write().unwrap();
        handlers.clear();
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

/// Shared EventBus type
///
/// This is the recommended way to share an EventBus across components.
/// Use `create_event_bus()` to create a new shared instance.
pub type SharedEventBus = Arc<EventBus>;

/// Create a new shared EventBus
///
/// Returns an Arc-wrapped EventBus that can be cloned and shared across threads.
///
/// # Example
///
/// ```rust,ignore
/// use buttre_core::events::create_event_bus;
///
/// let bus = create_event_bus();
/// let bus_clone = bus.clone();
///
/// // Use in different components
/// keyboard_service.set_event_bus(bus.clone());
/// ui_observer.set_event_bus(bus.clone());
/// ```
pub fn create_event_bus() -> SharedEventBus {
    Arc::new(EventBus::new())
}
