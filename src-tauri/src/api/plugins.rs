use crate::plugins::{PluginMetadata, PluginManager};
use std::sync::{Arc, OnceLock};
use tauri::AppHandle;

// 全局插件管理器实例
static PLUGIN_MANAGER: OnceLock<Arc<PluginManager>> = OnceLock::new();

/// 初始化插件管理器（应用启动时调用）
pub fn initialize_plugin_manager(app_handle: AppHandle) {
    let manager = Arc::new(PluginManager::new(app_handle));
    PLUGIN_MANAGER.set(manager).expect("Failed to initialize plugin manager");
}

/// 获取插件管理器实例
fn get_plugin_manager() -> Result<&'static Arc<PluginManager>, String> {
    PLUGIN_MANAGER.get().ok_or_else(|| "Plugin manager not initialized".to_string())
}

/// 扫描并返回所有可用的插件列表
#[tauri::command]
pub fn scan_plugins() -> Result<Vec<PluginMetadata>, String> {
    let manager = get_plugin_manager()?;
    Ok(manager.scan_plugins())
}

/// 挂载插件
#[tauri::command]
pub fn mount_plugin(plugin_id: String) -> Result<String, String> {
    let manager = get_plugin_manager()?;
    manager.mount_plugin(&plugin_id)
}

/// 卸载插件
#[tauri::command]
pub fn dispose_plugin(plugin_id: String) -> Result<String, String> {
    let manager = get_plugin_manager()?;
    manager.dispose_plugin(&plugin_id)
}

/// 连接插件
#[tauri::command]
pub fn connect_plugin(plugin_id: String) -> Result<String, String> {
    let manager = get_plugin_manager()?;
    manager.connect_plugin(&plugin_id)
}

/// 断开插件连接
#[tauri::command]
pub fn disconnect_plugin(plugin_id: String) -> Result<String, String> {
    let manager = get_plugin_manager()?;
    manager.disconnect_plugin(&plugin_id)
}

/// 获取插件状态
#[tauri::command]
pub fn get_plugin_status(plugin_id: String) -> Result<Option<(bool, bool)>, String> {
    let manager = get_plugin_manager()?;
    Ok(manager.get_plugin_status(&plugin_id))
}

/// 获取当前活跃插件
#[tauri::command]
pub fn get_current_plugin() -> Result<Option<String>, String> {
    let manager = get_plugin_manager()?;
    Ok(manager.get_current_plugin())
}

/// 向当前插件发送消息
#[tauri::command]
pub fn send_message_to_plugin(message: String) -> Result<String, String> {
    let manager = get_plugin_manager()?;
    manager.send_message_to_current_plugin(&message)
}

/// 获取插件UI定义
#[tauri::command]
pub fn get_plugin_ui(plugin_id: String) -> Result<String, String> {
    let manager = get_plugin_manager()?;
    manager.get_plugin_ui(&plugin_id)
}

/// 处理插件UI事件
#[tauri::command]
pub fn handle_plugin_ui_event(plugin_id: String, component_id: String, value: String) -> Result<bool, String> {
    let manager = get_plugin_manager()?;
    manager.handle_plugin_ui_event(&plugin_id, &component_id, &value)
}

/// 清理所有插件（应用关闭时调用）
pub fn cleanup_all_plugins() {
    if let Ok(manager) = get_plugin_manager() {
        manager.cleanup_all_plugins();
    }
}
