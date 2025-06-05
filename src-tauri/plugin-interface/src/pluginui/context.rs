//! Context types for the plugin UI framework

use std::collections::HashMap;

/// Creation context passed to plugins during initialization
#[derive(Debug)]
pub struct CreationContext<'a> {
    /// Plugin configuration data
    pub config: HashMap<String, String>,
    /// Plugin ID
    pub plugin_id: &'a str,
}

impl<'a> CreationContext<'a> {
    pub fn new(plugin_id: &'a str) -> Self {
        Self {
            config: HashMap::new(),
            plugin_id,
        }
    }

    pub fn with_config(plugin_id: &'a str, config: HashMap<String, String>) -> Self {
        Self { config, plugin_id }
    }
}

/// Runtime context for UI operations
#[derive(Debug)]
pub struct Context {
    /// Current plugin ID
    pub plugin_id: String,
    /// Theme information
    pub theme: Theme,
    /// UI state
    pub ui_state: UiState,
    /// UI event data from frontend
    pub ui_event_data: HashMap<String, String>,
}

impl Context {
    pub fn new(plugin_id: String) -> Self {
        Self {
            plugin_id,
            theme: Theme::default(),
            ui_state: UiState::default(),
            ui_event_data: HashMap::new(),
        }
    }

    /// Create context with UI event data
    pub fn with_ui_event_data(plugin_id: String, ui_event_data: HashMap<String, String>) -> Self {
        Self {
            plugin_id,
            theme: Theme::default(),
            ui_state: UiState::default(),
            ui_event_data,
        }
    }

    /// Get UI event data for a specific component
    pub fn get_ui_event_data(&self, component_id: &str) -> Option<&String> {
        self.ui_event_data.get(component_id)
    }
}

/// Theme information
#[derive(Debug, Clone)]
pub struct Theme {
    pub is_dark: bool,
}

impl Default for Theme {
    fn default() -> Self {
        Self { is_dark: false }
    }
}

/// UI state information
#[derive(Debug, Default)]
pub struct UiState {
    pub frame_count: u64,
    pub time: f64,
}
