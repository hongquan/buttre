//! Observers for reactive application updates

pub mod ui_observer;
mod ui_callback;
mod keyboard_observer;

// Platform-specific observers
pub use ui_observer::UIObserver;
pub use ui_callback::{MainUICallback, UIEvent};
pub use keyboard_observer::KeyboardObserver;
