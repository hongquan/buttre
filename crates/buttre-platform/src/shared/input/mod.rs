//! Input method management for buttre application

pub mod keyboard_manager;
pub mod method_registry;

// Re-export commonly used items
pub use keyboard_manager::KeyboardManager;
pub use method_registry::{MethodRegistry, MethodInfo, MethodSource};
