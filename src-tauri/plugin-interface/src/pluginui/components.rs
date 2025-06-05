//! UI component definitions for the new framework

use serde::{Deserialize, Serialize};

/// Response from UI interactions
pub struct Response {
    pub clicked: bool,
    pub changed: bool,
    pub hovered: bool,
    /// Component ID for event handling
    pub component_id: Option<String>,
}

impl Response {
    pub fn new() -> Self {
        Self {
            clicked: false,
            changed: false,
            hovered: false,
            component_id: None,
        }
    }

    pub fn new_with_component_and_state(
        component_id: String,
        clicked: bool,
        changed: bool,
    ) -> Self {
        Self {
            clicked,
            changed,
            hovered: false,
            component_id: Some(component_id),
        }
    }

    pub fn clicked(&self) -> bool {
        self.clicked
    }

    pub fn changed(&self) -> bool {
        self.changed
    }

    pub fn hovered(&self) -> bool {
        self.hovered
    }

    pub fn with_clicked() -> Self {
        Self {
            clicked: true,
            changed: false,
            hovered: false,
            component_id: None,
        }
    }

    pub fn with_changed() -> Self {
        Self {
            clicked: false,
            changed: true,
            hovered: false,
            component_id: None,
        }
    }
}

/// Internal component representation for serialization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiComponent {
    pub id: String,
    pub component: UiComponentType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum UiComponentType {
    Label {
        text: String,
    },
    Button {
        text: String,
        enabled: bool,
    },
    TextEdit {
        value: String,
        hint: String,
    },
    SelectableValue {
        options: Vec<String>,
        selected: usize,
    },
    ComboBox {
        options: Vec<String>,
        selected: Option<usize>,
        placeholder: String,
    },
    Toggle {
        value: bool,
    },
    Horizontal {
        children: Vec<UiComponent>,
    },
    Vertical {
        children: Vec<UiComponent>,
    },
}
