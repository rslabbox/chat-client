use crate::plugins::{PluginLoader, PluginMetadata};

/// 扫描并返回所有可用的插件列表
#[tauri::command]
pub fn scan_plugins() -> Vec<PluginMetadata> {
    let loader = PluginLoader::new();
    loader.scan_plugins()
}
