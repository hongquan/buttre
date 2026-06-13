//! Platform-specific backend implementations
//!
//! This module contains implementations for different operating systems.
//! The correct backend is selected at compile-time based on target OS.
//!
//! # Supported Platforms
//!
//! - **Windows:** TSF (Text Services Framework) + Keyboard Hook fallback
//! - **macOS:** IMKit (Input Method Kit)
//! - **Linux:** IBus (Intelligent Input Bus)

#[cfg(platform_windows)]
pub mod windows;

#[cfg(platform_macos)]
pub mod macos;

#[cfg(platform_linux)]
pub mod linux;

// Re-export platform-specific backend as unified type
#[cfg(platform_windows)]
pub use windows::WindowsBackend as Backend;

#[cfg(platform_macos)]
pub use macos::MacOSBackend as Backend;

#[cfg(platform_linux)]
pub use linux::LinuxBackend as Backend;
