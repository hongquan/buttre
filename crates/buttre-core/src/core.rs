//! ButtreCore - Main facade for the buttre application
//!
//! **Tests**: Integration tests for this module are located in `crates/buttre-core/tests/core_tests.rs`.
//!
//! This module provides a unified interface to all buttre functionality.
//! Platform code should interact with ButtreCore instead of individual services.

use crate::events::{create_event_bus, AppEvent, HotkeyAction, SharedEventBus};
use crate::services::{
    ConfigService, HotkeyService, KeyboardService, MethodRegistry, Preset, SettingsService,
};
use crate::state::AppState;
use crate::Action;
use anyhow::{Context, Result};

/// ButtreCore - Main application facade
///
/// This struct provides a unified interface to all buttre functionality.
/// It manages all services and coordinates their interactions through the event bus.
///
/// # Example
///
/// ```rust,ignore
/// use buttre_core::ButtreCore;
///
/// // Create and initialize
/// let mut core = ButtreCore::new()?;
/// core.init()?;
///
/// // Subscribe to events
/// core.event_bus().subscribe(|event| {
///     println!("Event: {:?}", event);
/// });
///
/// // Use the API
/// core.switch_method("telex")?;
/// let action = core.process('a')?;
/// ```
pub struct ButtreCore {
    /// Shared event bus
    event_bus: SharedEventBus,

    /// Keyboard service
    keyboard: KeyboardService,

    /// Config service
    config: ConfigService,

    /// Method registry
    registry: MethodRegistry,

    /// Hotkey service
    hotkey: Option<HotkeyService>,

    /// Settings service
    settings: SettingsService,

    /// Application state
    state: AppState,
}

impl ButtreCore {
    /// Create a new ButtreCore instance
    ///
    /// This will:
    /// - Create the event bus
    /// - Initialize all services
    /// - Load settings
    /// - Set up the application state
    ///
    /// Note: This does NOT register hotkeys or load keyboards yet.
    /// Call `init()` to complete initialization.
    pub fn new() -> Result<Self> {
        // Create event bus
        let event_bus = create_event_bus();

        // Create services
        let keyboard = KeyboardService::new(event_bus.clone());
        let config = ConfigService::new().context("Failed to create ConfigService")?;
        let mut registry = MethodRegistry::new().context("Failed to create MethodRegistry")?;

        // Scan for available methods
        registry.scan().context("Failed to scan for methods")?;

        // Try to create hotkey service (may fail if hotkeys are taken)
        let hotkey = HotkeyService::new(event_bus.clone()).ok();
        if hotkey.is_none() {
            event_bus.publish(AppEvent::warn(
                "Failed to register hotkeys - they may be taken by another application",
            ));
        }

        let settings = SettingsService::new(event_bus.clone());
        let state = AppState::new();

        Ok(Self {
            event_bus,
            keyboard,
            config,
            registry,
            hotkey,
            settings,
            state,
        })
    }

    /// Initialize ButtreCore with default keyboards
    ///
    /// This will:
    /// - Create Telex and VNI keyboards
    /// - Load the current method from settings
    /// - Switch to that method
    pub fn init(&mut self) -> Result<()> {
        // Create default keyboards
        self.keyboard
            .create_preset(Preset::Telex)
            .context("Failed to create Telex keyboard")?;
        self.keyboard
            .create_preset(Preset::Vni)
            .context("Failed to create VNI keyboard")?;

        // Load current method from settings
        let current_method = self.settings.input_method().to_string();

        // Switch to it (if it's not english)
        if current_method != "english" && self.keyboard.has(&current_method) {
            self.keyboard.switch(&current_method)?;
        }

        Ok(())
    }

    // ========================================================================
    // Input Processing
    // ========================================================================

    /// Process a keystroke
    ///
    /// # Arguments
    ///
    /// * `key` - The character to process
    ///
    /// # Returns
    ///
    /// The action to perform (DoNothing, Commit, or Replace)
    pub fn process(&mut self, key: char) -> Result<Action> {
        self.keyboard.process(key)
    }

    /// Process backspace
    pub fn backspace(&mut self) -> Result<Action> {
        self.keyboard.backspace()
    }

    /// Reset input buffer
    pub fn reset(&mut self) {
        self.keyboard.reset();
    }

    // ========================================================================
    // Method Switching
    // ========================================================================

    /// Switch to a different input method
    ///
    /// # Arguments
    ///
    /// * `id` - Method ID (e.g., "telex", "vni", "english")
    pub fn switch_method(&mut self, id: &str) -> Result<()> {
        // If switching to a Vietnamese method, ensure keyboard exists
        if id != "english" && !self.keyboard.has(id) {
            // Try to load the config and create the keyboard
            let config = self
                .config
                .load(id)
                .context(format!("Method '{}' not found", id))?;
            self.keyboard.create(id, config)?;
        }

        // Switch keyboard (or set to None for english)
        if id != "english" {
            self.keyboard.switch(id)?;
        }

        // Update settings
        self.settings.set_input_method(id)?;

        // Update state
        self.state.set_method(id)?;

        Ok(())
    }

    /// Toggle between Vietnamese and English
    ///
    /// If currently in English, switches to the last Vietnamese method.
    /// If currently in Vietnamese, switches to English.
    pub fn toggle(&mut self) -> Result<()> {
        self.state.toggle()?;

        let new_method = self.state.current_method().to_string();
        self.switch_method(&new_method)?;

        Ok(())
    }

    /// Get the current input method ID
    pub fn current_method(&self) -> &str {
        self.state.current_method()
    }

    /// Check if Vietnamese input is enabled
    pub fn is_enabled(&self) -> bool {
        self.state.is_enabled()
    }

    // ========================================================================
    // Methods Registry
    // ========================================================================

    /// List all available input methods
    pub fn list_methods(&self) -> &[crate::events::MethodInfo] {
        self.registry.list()
    }

    /// Refresh the methods registry
    ///
    /// This will rescan the keyboards directory for new custom methods.
    pub fn refresh_methods(&mut self) -> Result<()> {
        self.registry.scan()
    }

    // ========================================================================
    // Settings
    // ========================================================================

    /// Get current settings
    pub fn settings(&self) -> &crate::state::Settings {
        self.settings.get()
    }

    /// Update settings
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// core.update_settings(|settings| {
    ///     settings.auto_correct = true;
    /// })?;
    /// ```
    pub fn update_settings<F>(&mut self, f: F) -> Result<()>
    where
        F: FnOnce(&mut crate::state::Settings),
    {
        self.settings.update(f)
    }

    /// Save settings to disk
    pub fn save_settings(&self) -> Result<()> {
        self.settings.save()
    }

    // ========================================================================
    // Hotkeys
    // ========================================================================

    /// Poll for hotkey events
    ///
    /// This should be called regularly in your event loop.
    /// When a hotkey is pressed, it will publish a HotkeyPressed event.
    pub fn poll_hotkeys(&self) {
        if let Some(hotkey) = &self.hotkey {
            hotkey.poll();
        }
    }

    /// Handle a hotkey action
    ///
    /// This is a convenience method that handles common hotkey actions.
    /// You can also subscribe to HotkeyPressed events directly.
    ///
    /// # Arguments
    ///
    /// * `action` - The hotkey action to handle
    pub fn handle_hotkey(&mut self, action: HotkeyAction) -> Result<()> {
        match action {
            HotkeyAction::Toggle => self.toggle(),
            HotkeyAction::Telex => self.switch_method("telex"),
            HotkeyAction::Vni => self.switch_method("vni"),
            HotkeyAction::Nom => self.switch_method("nom"),
            HotkeyAction::Custom(i) => {
                // Find the i-th custom method
                let customs = self.registry.customs();
                if let Some(method) = customs.get(i) {
                    let method_id = method.id.clone(); // Clone to avoid borrow issues
                    self.switch_method(&method_id)
                } else {
                    Ok(()) // Ignore if custom method doesn't exist
                }
            }
        }
    }

    // ========================================================================
    // Event Bus
    // ========================================================================

    /// Get the event bus for subscribing to events
    pub fn event_bus(&self) -> SharedEventBus {
        self.event_bus.clone()
    }
}
