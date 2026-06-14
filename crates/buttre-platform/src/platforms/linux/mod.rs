//! buttre Linux Input Method
//!
//! Supports IBus via D-Bus (zbus 3).

#![cfg(target_os = "linux")]

pub mod ibus;

use crate::PlatformBackend;
use std::sync::{Arc, RwLock};
use buttre_core::{Action, Keyboard};
use anyhow::Result;

/// Linux backend — spawns the IBus engine in a background thread.
pub struct LinuxBackend {
    enabled: bool,
    shutdown_tx: Option<tokio::sync::oneshot::Sender<()>>,
    engine_thread: Option<std::thread::JoinHandle<()>>,
}

impl PlatformBackend for LinuxBackend {
    fn new() -> Result<Self> {
        Ok(Self {
            enabled: false,
            shutdown_tx: None,
            engine_thread: None,
        })
    }

    fn init(&mut self, _keyboard: Arc<RwLock<Option<Keyboard>>>) -> Result<()> {
        tracing::info!("Initializing Linux (IBus) backend");

        let (tx, rx) = tokio::sync::oneshot::channel::<()>();
        self.shutdown_tx = Some(tx);

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

        self.engine_thread = Some(handle);
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
        // Signal the engine thread to stop
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(());
        }
        // Join the thread (best-effort)
        if let Some(handle) = self.engine_thread.take() {
            if let Err(e) = handle.join() {
                tracing::warn!("IBus engine thread join error: {:?}", e);
            }
        }
    }
}
