//! Tauri API 命令模块
//! 
//! 这个模块包含了所有提供给前端的 Tauri 命令函数

pub mod general;
pub mod plugins;

// 重新导出所有 API 命令函数，方便在 lib.rs 中使用
pub use general::*;
pub use plugins::*;
