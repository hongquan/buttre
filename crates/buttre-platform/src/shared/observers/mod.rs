//! Observers for reactive application updates

mod keyboard_observer;
mod ui_callback;
pub mod ui_observer;

// Platform-specific observers
pub use keyboard_observer::KeyboardObserver;
pub use ui_callback::{MainUICallback, UIEvent};
pub use ui_observer::UIObserver;
