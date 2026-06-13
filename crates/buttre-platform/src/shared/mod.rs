//! Shared cross-platform code
//!
//! This module contains code that is used across all platforms,
//! including UI components, keyboard management, and IPC.

pub mod input;
pub mod observers;
pub mod ui;
pub mod pipe_server;
pub mod config_watcher;

// Re-export commonly used types
pub use input::{KeyboardManager, MethodRegistry, MethodInfo, MethodSource};
pub use observers::{UIObserver, MainUICallback, UIEvent, KeyboardObserver};
pub use ui::{build_menu, create_tray_icon, show_help_dialog, MenuItems};
pub use config_watcher::{ConfigWatcher, ConfigChangeEvent};
