// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod api;
pub mod plugins;
mod logging;

// 导入所有 API 命令
use api::{greet, scan_plugins, mount_plugin, dispose_plugin, connect_plugin, disconnect_plugin, get_plugin_status, get_current_plugin, send_message_to_plugin};
use log::{error, info};
use tauri::{RunEvent, WindowEvent};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 初始化日志系统
    if let Err(e) = logging::init_logging() {
        error!("Failed to initialize logging: {}", e);
    }

    // 日志系统初始化完成
    info!("日志系统初始化完成");

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
            get_current_plugin,
            send_message_to_plugin
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    // 初始化插件管理器
    api::plugins::initialize_plugin_manager(app.handle().clone());

    app.run(|_app_handle, event| {
        match event {
            RunEvent::ExitRequested { .. } => {
                info!("应用即将退出，正在清理插件...");
                api::plugins::cleanup_all_plugins();
            }
            RunEvent::WindowEvent { event: WindowEvent::CloseRequested { .. }, .. } => {
                // 当最后一个窗口关闭时，也执行清理
                info!("窗口关闭，正在清理插件...");
                api::plugins::cleanup_all_plugins();
            }
            _ => {}
        }
    });
}
