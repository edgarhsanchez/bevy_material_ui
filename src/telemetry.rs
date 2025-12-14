//! Telemetry and test automation support
//!
//! This module provides optional telemetry support for UI components,
//! allowing automated testing tools to identify and interact with elements.
//!
//! Telemetry is disabled by default and can be enabled via:
//! - The `BEVY_TELEMETRY` environment variable at runtime
//! - The `TelemetryConfig` resource
//!
//! # Example
//! ```ignore
//! // Enable telemetry programmatically
//! app.insert_resource(TelemetryConfig::enabled());
//!
//! // Or check environment variable (done automatically by TelemetryPlugin)
//! // Set BEVY_TELEMETRY=1 before running
//! ```

use bevy::prelude::*;

/// Plugin for telemetry support
pub struct TelemetryPlugin;

impl Plugin for TelemetryPlugin {
    fn build(&self, app: &mut App) {
        // Initialize telemetry config from environment
        let enabled = std::env::var("BEVY_TELEMETRY").is_ok();
        app.insert_resource(TelemetryConfig { enabled });
        
        if enabled {
            info!("📊 Telemetry enabled");
        }
    }
}

/// Configuration for telemetry features
#[derive(Resource, Clone, Debug, Default)]
pub struct TelemetryConfig {
    /// Whether telemetry is enabled
    pub enabled: bool,
}

impl TelemetryConfig {
    /// Create a config with telemetry enabled
    pub fn enabled() -> Self {
        Self { enabled: true }
    }
    
    /// Create a config with telemetry disabled
    pub fn disabled() -> Self {
        Self { enabled: false }
    }
}

/// Test ID component for automated testing
/// 
/// This component allows test automation tools to find UI elements by a stable identifier.
/// When telemetry is disabled, this component can still be added but won't be written
/// to the telemetry output.
#[derive(Component, Debug, Clone)]
pub struct TestId(pub String);

impl TestId {
    /// Create a new test ID
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
    
    /// Get the ID string
    pub fn id(&self) -> &str {
        &self.0
    }
}

/// Extension trait to conditionally add TestId based on telemetry config
pub trait WithTestId {
    /// Add a TestId component if telemetry is enabled, otherwise a no-op marker
    fn with_test_id(self, id: impl Into<String>, config: &TelemetryConfig) -> Self;
}

/// Trait for inserting test IDs into entity builders
pub trait InsertTestId {
    /// Insert a TestId if telemetry is enabled
    fn insert_test_id(&mut self, id: impl Into<String>, config: &TelemetryConfig) -> &mut Self;
}

impl InsertTestId for EntityCommands<'_> {
    fn insert_test_id(&mut self, id: impl Into<String>, config: &TelemetryConfig) -> &mut Self {
        if config.enabled {
            self.insert(TestId::new(id));
        }
        self
    }
}

/// Helper to create an optional TestId bundle
/// Returns Some(TestId) if telemetry is enabled, None otherwise
pub fn test_id_if_enabled(id: impl Into<String>, config: &TelemetryConfig) -> Option<TestId> {
    if config.enabled {
        Some(TestId::new(id))
    } else {
        None
    }
}

/// Stores element bounds for test automation
#[derive(Debug, Clone)]
pub struct ElementBounds {
    pub test_id: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub parent: Option<String>,
}

impl ElementBounds {
    /// Create new element bounds
    pub fn new(test_id: impl Into<String>, x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            test_id: test_id.into(),
            x,
            y,
            width,
            height,
            parent: None,
        }
    }
    
    /// Set the parent test ID
    pub fn with_parent(mut self, parent: impl Into<String>) -> Self {
        self.parent = Some(parent.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_telemetry_config_default() {
        let config = TelemetryConfig::default();
        assert!(!config.enabled);
    }

    #[test]
    fn test_telemetry_config_enabled() {
        let config = TelemetryConfig::enabled();
        assert!(config.enabled);
    }

    #[test]
    fn test_telemetry_config_disabled() {
        let config = TelemetryConfig::disabled();
        assert!(!config.enabled);
    }

    #[test]
    fn test_test_id_new() {
        let id = TestId::new("my_button");
        assert_eq!(id.id(), "my_button");
    }

    #[test]
    fn test_test_id_if_enabled_true() {
        let config = TelemetryConfig::enabled();
        let id = test_id_if_enabled("test", &config);
        assert!(id.is_some());
        assert_eq!(id.unwrap().id(), "test");
    }

    #[test]
    fn test_test_id_if_enabled_false() {
        let config = TelemetryConfig::disabled();
        let id = test_id_if_enabled("test", &config);
        assert!(id.is_none());
    }

    #[test]
    fn test_element_bounds_new() {
        let bounds = ElementBounds::new("button", 10.0, 20.0, 100.0, 50.0);
        assert_eq!(bounds.test_id, "button");
        assert_eq!(bounds.x, 10.0);
        assert_eq!(bounds.y, 20.0);
        assert_eq!(bounds.width, 100.0);
        assert_eq!(bounds.height, 50.0);
        assert!(bounds.parent.is_none());
    }

    #[test]
    fn test_element_bounds_with_parent() {
        let bounds = ElementBounds::new("child", 0.0, 0.0, 50.0, 50.0)
            .with_parent("parent");
        assert_eq!(bounds.parent, Some("parent".to_string()));
    }
}
