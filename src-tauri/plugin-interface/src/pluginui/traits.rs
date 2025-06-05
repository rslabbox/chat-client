//! Plugin UI trait definition

use super::{Context, Ui};

/// Main trait that plugins implement to define their UI
pub trait PluginUiApp {
    /// Create a new instance of the plugin
    // fn new(cc: &CreationContext<'_>) -> Self;

    /// Update and render the plugin UI (event-driven)
    /// This method is called when UI needs to be updated
    fn update_ui(&mut self, ctx: &Context, ui: &mut Ui);
}
