use crate::plugins::{
    AvailablePluginInfo, DownloadResponse, PluginDownloadResult, PluginManager, PluginMetadata, PluginRepository
};
use plugin_interfaces::metadata::HistoryMessage;
use std::sync::{Arc, OnceLock};
use tauri::AppHandle;

// 全局插件管理器实例
static PLUGIN_MANAGER: OnceLock<Arc<PluginManager>> = OnceLock::new();

/// 初始化插件管理器（应用启动时调用）
pub fn initialize_plugin_manager(app_handle: AppHandle) {
    let manager = Arc::new(PluginManager::new(app_handle));
    PLUGIN_MANAGER
        .set(manager)
        .expect("Failed to initialize plugin manager");
}

/// 获取插件管理器实例
fn get_plugin_manager() -> Result<&'static Arc<PluginManager>, String> {
    PLUGIN_MANAGER
        .get()
        .ok_or_else(|| "Plugin manager not initialized".to_string())
}

/// 扫描并返回所有可用的插件列表
#[tauri::command]
pub fn scan_plugins() -> Result<Vec<PluginMetadata>, String> {
    let manager = get_plugin_manager()?;
    Ok(manager.scan_plugins())
}

/// 挂载插件实例
#[tauri::command]
pub fn mount_plugin(plugin_id: String, instance_id: Option<String>) -> Result<String, String> {
    let manager = get_plugin_manager()?;
    manager.mount_plugin(&plugin_id, instance_id)
}

/// 卸载插件实例
#[tauri::command]
pub fn dispose_plugin(instance_id: String) -> Result<String, String> {
    let manager = get_plugin_manager()?;
    manager.dispose_plugin(&instance_id)
}

/// 连接插件实例
#[tauri::command]
pub fn connect_plugin(instance_id: String) -> Result<String, String> {
    let manager = get_plugin_manager()?;
    manager.connect_plugin(&instance_id)
}

/// 断开插件实例连接
#[tauri::command]
pub fn disconnect_plugin(instance_id: String) -> Result<String, String> {
    let manager = get_plugin_manager()?;
    manager.disconnect_plugin(&instance_id)
}

/// 获取插件实例状态
#[tauri::command]
pub fn get_plugin_status(instance_id: String) -> Result<Option<(bool, bool)>, String> {
    let manager = get_plugin_manager()?;
    Ok(manager.get_plugin_status(&instance_id))
}

/// 向指定插件实例发送消息
#[tauri::command]
pub fn send_message_to_plugin(
    plugin_id: String,
    instance_id: String,
    message: String,
    history: Option<Vec<HistoryMessage>>,
) -> Result<String, String> {
    let manager = get_plugin_manager()?;
    manager.send_message_to_plugin_instance(&plugin_id, &instance_id, &message, history)
}

/// 获取插件实例UI定义
#[tauri::command]
pub fn get_plugin_ui(instance_id: String) -> Result<String, String> {
    let manager = get_plugin_manager()?;
    manager.get_plugin_ui(&instance_id)
}

/// 处理插件实例UI更新
#[tauri::command]
pub fn handle_plugin_ui_update(
    instance_id: String,
    component_id: String,
    value: String,
) -> Result<bool, String> {
    let manager = get_plugin_manager()?;
    manager.handle_plugin_ui_update(&instance_id, &component_id, &value)
}

/// 处理插件实例UI事件
#[tauri::command]
pub fn handle_plugin_ui_event(
    instance_id: String,
    component_id: String,
    value: String,
) -> Result<bool, String> {
    let manager = get_plugin_manager()?;
    manager.handle_plugin_ui_event(&instance_id, &component_id, &value)
}

#[tauri::command]
pub async fn download_github_repo() -> Result<DownloadResponse, String> {
    let repository = PluginRepository::new();
    Ok(repository.download_github_repo().await?)
}

/// 扫描可用插件列表（从插件仓库）
#[tauri::command]
pub fn scan_available_plugins() -> Result<Vec<AvailablePluginInfo>, String> {
    let repository = PluginRepository::new();
    Ok(repository.scan_available_plugins())
}

/// 下载并安装插件
#[tauri::command]
pub async fn download_plugin(plugin_id: String) -> Result<PluginDownloadResult, String> {
    let repository = PluginRepository::new();
    Ok(repository.download_plugin(&plugin_id).await)
}

/// 卸载已安装的插件
#[tauri::command]
pub fn uninstall_plugin(plugin_id: String) -> Result<PluginDownloadResult, String> {
    let repository = PluginRepository::new();
    Ok(repository.uninstall_plugin(&plugin_id))
}

/// 清理所有插件（应用关闭时调用）
pub fn cleanup_all_plugins() {
    if let Ok(manager) = get_plugin_manager() {
        manager.cleanup_all_plugins();
    }
}