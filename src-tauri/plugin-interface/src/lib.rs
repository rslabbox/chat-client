// 模块声明
pub mod metadata;
pub mod callbacks;
pub mod api;
pub mod handler;
pub mod symbols;
pub mod logging;
pub mod pluginui;
pub mod config;
pub mod message;
pub mod async_manager;

// 重新导出所有公共接口
pub use metadata::*;
pub use callbacks::*;
pub use api::*;
pub use handler::*;
pub use symbols::*;
pub use logging::*;
pub use pluginui::{CreationContext, Context, Ui};
pub use config::*;
pub use message::*;

// 异步功能（可选）
#[cfg(feature = "async")]
pub use async_manager::*;
