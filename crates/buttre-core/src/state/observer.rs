//! Observer pattern for state change notifications
//!
//! This module provides the `StateObserver` trait that allows components
//! to react to state changes without tight coupling.

use super::Settings;

/// Observer trait for reacting to application state changes
///
/// Implementors of this trait will be notified when the application state changes,
/// allowing them to update UI, backend systems, or perform other side effects.
pub trait StateObserver: Send + Sync {
    /// Called when the input method changes
    ///
    /// # Arguments
    /// * `method` - The new input method ID (e.g., "telex", "vni", "nom", "english")
    /// * `enabled` - Whether Vietnamese input is enabled (false for "english")
    fn on_method_changed(&self, method: &str, enabled: bool);

    /// Called when settings are updated
    ///
    /// # Arguments
    /// * `settings` - The updated settings object
    fn on_settings_changed(&self, settings: &Settings);
}
