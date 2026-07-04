//! buttre Linux Input Method
//!
//! Supports IBus via D-Bus (zbus 3).

#![cfg(target_os = "linux")]

pub mod ibus;

use crate::PlatformBackend;
use anyhow::Result;
use buttre_core::state::{Settings, StateObserver};
use buttre_core::{Action, Keyboard};
use std::sync::{Arc, Mutex, RwLock};

/// Linux backend — spawns the IBus engine in a background thread.
///
/// Fields are wrapped in `Mutex` so the struct is `Sync`, satisfying the
/// `StateObserver: Send + Sync` bound when registered as an observer.
pub struct LinuxBackend {
    enabled: bool,
    shutdown_tx: Mutex<Option<tokio::sync::oneshot::Sender<()>>>,
    engine_thread: Mutex<Option<std::thread::JoinHandle<()>>>,
}

impl PlatformBackend for LinuxBackend {
    fn new() -> Result<Self> {
        Ok(Self {
            enabled: false,
            shutdown_tx: Mutex::new(None),
            engine_thread: Mutex::new(None),
        })
    }

    fn init(&mut self, _keyboard: Arc<RwLock<Option<Keyboard>>>) -> Result<()> {
        tracing::info!("Initializing Linux (IBus) backend");

        let (tx, rx) = tokio::sync::oneshot::channel::<()>();
        *self.shutdown_tx.lock().unwrap() = Some(tx);

        let handle = std::thread::Builder::new()
            .name("buttre-ibus-engine".to_string())
            .spawn(move || {
                let rt = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .expect("Failed to build tokio runtime for IBus engine");
                if let Err(e) = rt.block_on(ibus::run_engine_with_shutdown(rx)) {
                    tracing::error!("IBus engine exited with error: {}", e);
                }
            })?;

        *self.engine_thread.lock().unwrap() = Some(handle);
        self.enabled = true;
        Ok(())
    }

    fn process_key(&mut self, _key: char) -> Action {
        Action::DoNothing
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    fn cleanup(&mut self) {
        if let Some(tx) = self.shutdown_tx.lock().unwrap().take() {
            let _ = tx.send(());
        }
        if let Some(handle) = self.engine_thread.lock().unwrap().take() {
            if let Err(e) = handle.join() {
                tracing::warn!("IBus engine thread join error: {:?}", e);
            }
        }
    }
}

impl StateObserver for LinuxBackend {
    fn on_method_changed(&self, _method: &str, enabled: bool) {
        tracing::info!("LinuxBackend: method changed, enabled={}", enabled);
    }

    fn on_settings_changed(&self, _settings: &Settings) {}
}
