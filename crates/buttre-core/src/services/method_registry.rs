//! Method Registry - Registry of available input methods
//!
//! **Tests**: Integration tests for this module are located in `crates/buttre-core/tests/service_method_registry_tests.rs`.
//!
//! This service maintains a registry of all available input methods,
//! including built-in and custom methods. It integrates with ConfigService
//! to discover methods and provides a unified view of all available options.

use crate::events::{MethodInfo, MethodSource};
use crate::services::ConfigService;
use anyhow::Result;

/// Method Registry - Maintains list of available input methods
///
/// This registry provides a unified view of all available input methods,
/// both built-in and custom.
///
/// # Example
///
/// ```rust,ignore
/// use buttre_core::services::MethodRegistry;
///
/// let mut registry = MethodRegistry::new();
///
/// // Scan for all available methods
/// registry.scan()?;
///
/// // Get all methods
/// let methods = registry.list();
///
/// // Find a specific method
/// if let Some(method) = registry.get("telex") {
///     println!("Found: {}", method.name);
/// }
/// ```
pub struct MethodRegistry {
    /// List of registered methods
    methods: Vec<MethodInfo>,

    /// Config service for discovering custom methods
    config_service: ConfigService,
}

impl MethodRegistry {
    /// Create a new MethodRegistry
    pub fn new() -> Result<Self> {
        let config_service = ConfigService::new()?;

        Ok(Self {
            methods: Vec::new(),
            config_service,
        })
    }

    /// Create a MethodRegistry with a custom ConfigService
    pub fn with_config_service(config_service: ConfigService) -> Self {
        Self {
            methods: Vec::new(),
            config_service,
        }
    }

    /// Scan for all available methods
    ///
    /// This will discover both built-in and custom methods by scanning
    /// the keyboards directory.
    pub fn scan(&mut self) -> Result<()> {
        self.methods.clear();

        // Get all configs from ConfigService
        let configs = self.config_service.list()?;

        // Convert to MethodInfo
        for config in configs {
            self.methods.push(MethodInfo {
                id: config.id,
                name: config.name,
                language: config.language,
                source: match config.source {
                    crate::services::config_service::ConfigSource::Builtin => MethodSource::Builtin,
                    crate::services::config_service::ConfigSource::Custom(path) => {
                        MethodSource::Custom(path.to_string_lossy().to_string())
                    }
                },
            });
        }

        // Sort by name for consistent ordering
        self.methods.sort_by(|a, b| a.name.cmp(&b.name));

        Ok(())
    }

    /// Get a method by ID
    ///
    /// # Arguments
    ///
    /// * `id` - Method ID to find
    ///
    /// # Returns
    ///
    /// The method info if found, or None
    pub fn get(&self, id: &str) -> Option<&MethodInfo> {
        self.methods.iter().find(|m| m.id == id)
    }

    /// List all registered methods
    pub fn list(&self) -> &[MethodInfo] {
        &self.methods
    }

    /// Get methods by language
    ///
    /// # Arguments
    ///
    /// * `language` - Language to filter by (e.g., "vietnamese", "nom")
    pub fn by_language(&self, language: &str) -> Vec<&MethodInfo> {
        self.methods
            .iter()
            .filter(|m| m.language == language)
            .collect()
    }

    /// Get only built-in methods
    pub fn builtins(&self) -> Vec<&MethodInfo> {
        self.methods
            .iter()
            .filter(|m| matches!(m.source, MethodSource::Builtin))
            .collect()
    }

    /// Get only custom methods
    pub fn customs(&self) -> Vec<&MethodInfo> {
        self.methods
            .iter()
            .filter(|m| matches!(m.source, MethodSource::Custom(_)))
            .collect()
    }

    /// Register a custom method manually
    ///
    /// This is useful for adding methods that were discovered through
    /// other means (e.g., file watcher).
    ///
    /// # Arguments
    ///
    /// * `info` - Method information to register
    pub fn register(&mut self, info: MethodInfo) {
        // Remove existing entry with same ID
        self.methods.retain(|m| m.id != info.id);

        // Add new entry
        self.methods.push(info);

        // Re-sort
        self.methods.sort_by(|a, b| a.name.cmp(&b.name));
    }

    /// Unregister a method
    ///
    /// # Arguments
    ///
    /// * `id` - ID of the method to remove
    ///
    /// # Returns
    ///
    /// `true` if the method was found and removed, `false` otherwise
    pub fn unregister(&mut self, id: &str) -> bool {
        let before_len = self.methods.len();
        self.methods.retain(|m| m.id != id);
        self.methods.len() < before_len
    }

    /// Get the number of registered methods
    pub fn count(&self) -> usize {
        self.methods.len()
    }

    /// Check if a method exists
    pub fn contains(&self, id: &str) -> bool {
        self.methods.iter().any(|m| m.id == id)
    }
}

impl Default for MethodRegistry {
    fn default() -> Self {
        Self::new().expect("Failed to create MethodRegistry")
    }
}
