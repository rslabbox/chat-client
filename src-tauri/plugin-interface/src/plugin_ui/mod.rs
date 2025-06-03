//! 插件UI组件模块
//!
//! 提供了一套简单的UI组件系统，允许插件定义自己的UI界面

mod components;
mod component_ref;
mod plugin_ui;
mod builders;

// Re-export public types
pub use components::{ComponentType, Component};
pub use component_ref::ComponentRef;
pub use plugin_ui::PluginUi;