use crate::plugins::{PluginMetadata, PluginManager};
use once_cell::sync::Lazy;
use std::sync::Arc;

// 全局插件管理器实例
static PLUGIN_MANAGER: Lazy<Arc<PluginManager>> = Lazy::new(|| {
    Arc::new(PluginManager::new())
});

/// 扫描并返回所有可用的插件列表
#[tauri::command]
pub fn scan_plugins() -> Vec<PluginMetadata> {
    PLUGIN_MANAGER.scan_plugins()
}

/// 挂载插件
#[tauri::command]
pub fn mount_plugin(plugin_id: String) -> Result<String, String> {
    PLUGIN_MANAGER.mount_plugin(&plugin_id)
}

/// 卸载插件
#[tauri::command]
pub fn dispose_plugin(plugin_id: String) -> Result<String, String> {
    PLUGIN_MANAGER.dispose_plugin(&plugin_id)
}

/// 连接插件
#[tauri::command]
pub fn connect_plugin(plugin_id: String) -> Result<String, String> {
    PLUGIN_MANAGER.connect_plugin(&plugin_id)
}

/// 断开插件连接
#[tauri::command]
pub fn disconnect_plugin(plugin_id: String) -> Result<String, String> {
    PLUGIN_MANAGER.disconnect_plugin(&plugin_id)
}

/// 获取插件状态
#[tauri::command]
pub fn get_plugin_status(plugin_id: String) -> Option<(bool, bool)> {
    PLUGIN_MANAGER.get_plugin_status(&plugin_id)
}

/// 获取当前活跃插件
#[tauri::command]
pub fn get_current_plugin() -> Option<String> {
    PLUGIN_MANAGER.get_current_plugin()
}

/// 向当前插件发送消息
#[tauri::command]
pub fn send_message_to_plugin(message: String) -> Result<String, String> {
    PLUGIN_MANAGER.send_message_to_current_plugin(&message)
}

/// 清理所有插件（应用关闭时调用）
pub fn cleanup_all_plugins() {
    PLUGIN_MANAGER.cleanup_all_plugins()
}
