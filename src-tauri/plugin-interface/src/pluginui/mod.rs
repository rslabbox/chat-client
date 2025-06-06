//! New Plugin UI Framework
//!
//! A simplified, egui-inspired UI framework for plugins that provides
//! immediate mode UI with a clean, simple API.

mod components;
mod context;
mod ui;

pub use components::{UiComponent, Response, UiComponentType};
pub use context::{Context, CreationContext};
pub use ui::Ui;
pub use ui::PluginUiOption;

// Re-export commonly used macros
pub use crate::{log_error, log_info, log_warn};
