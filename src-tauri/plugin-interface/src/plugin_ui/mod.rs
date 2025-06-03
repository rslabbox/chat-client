//! 插件UI组件模块
//!
//! 提供了一套简单的UI组件系统，允许插件定义自己的UI界面

mod components;
mod component_ref;
mod plugin_ui;
mod builders;

#[cfg(test)]
mod tests;

// Re-export public types
pub use components::{ComponentType, Component, ContainerLayout};
pub use component_ref::{ComponentRef, ButtonRef, TextFieldRef, TextRef, SelectRef, ContainerRef};
pub use builders::{ContainerBuilder, NestedContainerBuilder};
pub use plugin_ui::PluginUi;