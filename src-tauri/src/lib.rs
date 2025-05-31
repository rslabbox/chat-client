// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod api;
pub mod plugins;

// 导入所有 API 命令
use api::{greet, scan_plugins};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, scan_plugins])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
