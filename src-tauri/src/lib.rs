// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod api;
pub mod plugins;

// 导入所有 API 命令
use api::{
    connect_plugin, disconnect_plugin, dispose_plugin, download_github_repo, download_plugin,
    get_plugin_status, get_plugin_ui, greet, handle_plugin_ui_event, handle_plugin_ui_update,
    mount_plugin, scan_available_plugins, scan_plugins, send_message_to_plugin, uninstall_plugin,
};

use plugin_interfaces::log_info;
use tauri::{RunEvent, WindowEvent};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            scan_plugins,
            mount_plugin,
            dispose_plugin,
            connect_plugin,
            disconnect_plugin,
            get_plugin_status,
            send_message_to_plugin,
            get_plugin_ui,
            handle_plugin_ui_update,
            handle_plugin_ui_event,
            download_github_repo,
            scan_available_plugins,
            download_plugin,
            uninstall_plugin
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    // 初始化插件管理器
    api::plugins::initialize_plugin_manager(app.handle().clone());

    app.run(|_app_handle, event| {
        match event {
            RunEvent::ExitRequested { .. } => {
                log_info!("应用即将退出，正在清理插件...");
                api::plugins::cleanup_all_plugins();
            }
            RunEvent::WindowEvent {
                event: WindowEvent::CloseRequested { .. },
                ..
            } => {
                // 当最后一个窗口关闭时，也执行清理
                log_info!("窗口关闭，正在清理插件...");
                api::plugins::cleanup_all_plugins();
            }
            _ => {}
        }
    });
}
