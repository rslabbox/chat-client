//! Main UI builder implementation

use crate::{send_to_frontend, PluginHandler};

use super::components::{Response, UiComponent, UiComponentType};
use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
};
use uuid::Uuid;

/// Main UI builder - provides immediate mode UI building
pub struct Ui {
    /// Components built in this frame
    pub(crate) components: Vec<UiComponent>,
    /// Plugin ID for this UI
    pub(crate) plugin_id: String,
    /// Current layout context
    pub(crate) layout_stack: Vec<LayoutContext>,
    /// Components that were clicked in this frame
    pub(crate) clicked_components: HashSet<String>,
    /// Components that were changed in this frame
    pub(crate) changed_components: HashSet<String>,
    /// UI event data from frontend (component_id -> value)
    pub(crate) ui_event_data: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub(crate) enum LayoutContext {
    Root,
    Horizontal,
    Vertical,
}

impl Ui {
    pub fn new(plugin_id: String) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self {
            components: Vec::new(),
            plugin_id,
            layout_stack: vec![LayoutContext::Root],
            clicked_components: HashSet::new(),
            changed_components: HashSet::new(),
            ui_event_data: HashMap::new(),
        }))
    }

    /// Get plugin Id
    pub fn plugin_id(&self) -> &str {
        &self.plugin_id
    }

    /// Add a text label
    pub fn label(&mut self, text: &str) {
        let component = UiComponent {
            id: format!("label_{}", Uuid::new_v4()),
            component: UiComponentType::Label {
                text: text.to_string(),
            },
        };
        self.add_component(component);
    }

    /// Add a clickable button
    pub fn button(&mut self, text: &str) -> Response {
        // Use a hash of the text and current component count for stable ID
        let id = format!(
            "button_{}_{}",
            self.components.len(),
            text.replace(" ", "_")
        );
        let component = UiComponent {
            id: id.clone(),
            component: UiComponentType::Button {
                text: text.to_string(),
                enabled: true,
            },
        };
        self.add_component(component);

        // Check if this component was clicked in this frame
        let was_clicked = self.clicked_components.contains(&id);

        // Return a response with click state
        Response::new_with_component_and_state(id, was_clicked, false)
    }

    /// Add a single-line text editor
    pub fn text_edit_singleline(&mut self, value: &mut String) -> Response {
        // Use a stable ID based on component count for consistent identification
        let id = format!("textedit_{}", self.components.len());

        // Check if this component was changed and update the value from frontend data
        let was_changed = self.changed_components.contains(&id);
        if was_changed {
            if let Some(new_value) = self.ui_event_data.get(&id) {
                *value = new_value.clone();
            }
        }

        let component = UiComponent {
            id: id.clone(),
            component: UiComponentType::TextEdit {
                value: value.clone(),
                hint: String::new(),
            },
        };
        self.add_component(component);

        Response::new_with_component_and_state(id, false, was_changed)
    }

    /// Add a combo box (dropdown) widget
    pub fn combo_box<T>(
        &mut self,
        options: Vec<T>,
        selected: &mut Option<T>,
        placeholder: &str,
    ) -> Response
    where
        T: Clone + PartialEq + ToString,
    {
        // Use a stable ID based on component count and placeholder
        let id = format!(
            "combo_{}_{}",
            self.components.len(),
            placeholder.replace(" ", "_")
        );

        // Check if this component was clicked or changed and update the selection from frontend data
        let was_clicked = self.clicked_components.contains(&id);
        let was_changed = self.changed_components.contains(&id);
        if was_changed {
            if let Some(new_value) = self.ui_event_data.get(&id) {
                if let Ok(selection_index) = new_value.parse::<usize>() {
                    if selection_index < options.len() {
                        *selected = Some(options[selection_index].clone());
                    } else {
                        *selected = None;
                    }
                }
            }
        }

        // Validate that selected value exists in options, otherwise set to None
        let selected_index = if let Some(ref selected_value) = selected {
            options.iter().position(|opt| opt == selected_value)
        } else {
            None
        };

        // If selected value doesn't exist in options, set selected to None
        if selected.is_some() && selected_index.is_none() {
            *selected = None;
        }

        let component = UiComponent {
            id: id.clone(),
            component: UiComponentType::ComboBox {
                options: options.iter().map(|opt| opt.to_string()).collect(),
                selected: selected_index,
                placeholder: placeholder.to_string(),
            },
        };
        self.add_component(component);

        // Return a response with event states
        Response::new_with_component_and_state(id, was_clicked, was_changed)
    }

    /// Add a toggle switch
    pub fn toggle(&mut self, value: &mut bool) -> Response {
        // Use a stable ID based on component count
        let id = format!("toggle_{}", self.components.len());

        // Check if this component was clicked and update the value from frontend data
        let was_clicked = self.clicked_components.contains(&id);
        let was_changed = self.changed_components.contains(&id);
        if was_changed {
            if let Some(new_value) = self.ui_event_data.get(&id) {
                if let Ok(toggle_value) = new_value.parse::<bool>() {
                    *value = toggle_value;
                }
            }
        }

        let component = UiComponent {
            id: id.clone(),
            component: UiComponentType::Toggle {
                value: *value,
            },
        };
        self.add_component(component);

        // Return a response with event states
        Response::new_with_component_and_state(id, was_clicked, was_changed)
    }

    /// Create a horizontal layout
    pub fn horizontal<R>(&mut self, add_contents: impl FnOnce(&mut Self) -> R) -> R {
        self.layout_stack.push(LayoutContext::Horizontal);
        let start_index = self.components.len();

        let result = add_contents(self);

        // Collect components added in this horizontal context
        let children = self.components.split_off(start_index);

        if !children.is_empty() {
            let horizontal_component = UiComponent {
                id: format!("horizontal_{}", Uuid::new_v4()),
                component: UiComponentType::Horizontal { children },
            };
            self.components.push(horizontal_component);
        }

        self.layout_stack.pop();
        result
    }

    /// Create a vertical layout
    pub fn vertical<R>(&mut self, add_contents: impl FnOnce(&mut Self) -> R) -> R {
        self.layout_stack.push(LayoutContext::Vertical);
        let start_index = self.components.len();

        let result = add_contents(self);

        // Collect components added in this vertical context
        let children = self.components.split_off(start_index);

        if !children.is_empty() {
            let vertical_component = UiComponent {
                id: format!("vertical_{}", Uuid::new_v4()),
                component: UiComponentType::Vertical { children },
            };
            self.components.push(vertical_component);
        }

        self.layout_stack.pop();
        result
    }

    /// Internal method to add a component
    fn add_component(&mut self, component: UiComponent) {
        self.components.push(component);
    }

    /// Get all components for serialization
    pub fn get_components(&self) -> &[UiComponent] {
        &self.components
    }

    /// Clear all components (called at start of each frame)
    pub fn clear(&mut self) {
        self.components.clear();
        // Clear event tracking - events should only be active for one frame
        self.clicked_components.clear();
        self.changed_components.clear();
        self.ui_event_data.clear();
    }

    /// Clear only components, keep event tracking for current frame
    pub fn clear_components_only(&mut self) {
        self.components.clear();
        // Keep event tracking for this update_ui call
    }

    /// Clear event tracking after update_ui is complete
    pub fn clear_events(&mut self) {
        self.clicked_components.clear();
        self.changed_components.clear();
        self.ui_event_data.clear();
    }

    /// Handle UI events (called when frontend sends UI events)
    pub fn handle_ui_event(&mut self, component_id: &str, value: &str) -> bool {
        // Store the event data for use in the event loop
        self.ui_event_data
            .insert(component_id.to_string(), value.to_string());

        // Track the event based on component type for the event loop pattern
        if component_id.starts_with("combo_") {
            self.clicked_components.insert(component_id.to_string());
            self.changed_components.insert(component_id.to_string());
            true
        } else if component_id.starts_with("button_") {
            self.clicked_components.insert(component_id.to_string());
            true
        } else if component_id.starts_with("textedit_") {
            self.changed_components.insert(component_id.to_string());
            true
        } else if component_id.starts_with("toggle_") {
            self.clicked_components.insert(component_id.to_string());
            self.changed_components.insert(component_id.to_string());
            true
        } else {
            false
        }
    }
}

pub trait PluginUiOption {
    fn refresh_ui(&self) -> bool;
}

impl<T: PluginHandler> PluginUiOption for T {
    fn refresh_ui(&self) -> bool {
        let plugin_id = self.get_metadata().id;
        let payload = serde_json::json!({
            "plugin": plugin_id
        }).to_string();
        send_to_frontend("plugin-ui-refreshed", &payload.as_str())
    }
}
