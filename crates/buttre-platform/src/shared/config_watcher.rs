//! Config Watcher - Auto-reload menu when keyboard configs change
//!
//! **Tests**: Integration tests for this module are located in `crates/buttre-platform/tests/shared_config_watcher_tests.rs`.
//!
//! This module watches the keyboards/ directory for changes and notifies
//! the UI to rebuild the menu when new configs are added or removed.

use anyhow::Result;
use log::{info, warn};
use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver};
use std::time::Duration;

/// Config change event
#[derive(Debug, Clone)]
pub enum ConfigChangeEvent {
    /// New config file added
    Added(PathBuf),
    /// Config file removed
    Removed(PathBuf),
    /// Config file modified
    Modified(PathBuf),
}

/// Config watcher that monitors the keyboards/ directory
pub struct ConfigWatcher {
    _watcher: RecommendedWatcher,
    receiver: Receiver<ConfigChangeEvent>,
}

impl ConfigWatcher {
    /// Create a new config watcher for the keyboards directory
    pub fn new(keyboards_dir: PathBuf) -> Result<Self> {
        let (tx, rx) = channel();
        
        // Create watcher with event handler
        let mut watcher = RecommendedWatcher::new(
            move |res: notify::Result<Event>| {
                if let Ok(event) = res {
                    if let Some(change_event) = Self::process_event(event) {
                        let _ = tx.send(change_event);
                    }
                }
            },
            Config::default()
                .with_poll_interval(Duration::from_secs(2)),
        )?;
        
        // Watch the keyboards directory
        if keyboards_dir.exists() {
            watcher.watch(&keyboards_dir, RecursiveMode::NonRecursive)?;
            info!("Watching keyboards directory: {:?}", keyboards_dir);
        } else {
            warn!("Keyboards directory does not exist: {:?}", keyboards_dir);
        }
        
        Ok(Self {
            _watcher: watcher,
            receiver: rx,
        })
    }
    
    /// Process file system event and convert to ConfigChangeEvent
    fn process_event(event: Event) -> Option<ConfigChangeEvent> {
        // Only process .toml files
        let path = event.paths.first()?;
        
        if path.extension()?.to_str()? != "toml" {
            return None;
        }
        
        match event.kind {
            EventKind::Create(_) => {
                info!("Config added: {:?}", path);
                Some(ConfigChangeEvent::Added(path.clone()))
            }
            EventKind::Remove(_) => {
                info!("Config removed: {:?}", path);
                Some(ConfigChangeEvent::Removed(path.clone()))
            }
            EventKind::Modify(_) => {
                info!("Config modified: {:?}", path);
                Some(ConfigChangeEvent::Modified(path.clone()))
            }
            _ => None,
        }
    }
    
    /// Try to receive a config change event (non-blocking)
    pub fn try_recv(&self) -> Option<ConfigChangeEvent> {
        self.receiver.try_recv().ok()
    }
}

