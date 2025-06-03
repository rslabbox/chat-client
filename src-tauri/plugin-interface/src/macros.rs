/// 自动使用plugin id的宏定义
/// 这些宏需要在插件代码中定义PLUGIN_ID常量

/// 向前端发送插件消息响应，自动使用当前插件的ID
/// 使用方法:
/// - send_message_to_frontend!("简单消息")
/// - send_message_to_frontend!("格式化消息: {}", value)
/// - send_message_to_frontend!("多个参数: {} {}", arg1, arg2)
#[macro_export]
macro_rules! send_message_to_frontend {
    ($($arg:tt)*) => {
        $crate::handler::send_message_to_frontend(PLUGIN_ID, &format!($($arg)*))
    };
}

/// 向前端发送插件UI更新事件，自动使用当前插件的ID
/// 使用方法: send_ui_updated!()
#[macro_export]
macro_rules! send_ui_updated {
    () => {
        $crate::api::send_to_frontend("plugin-ui-updated", &format!(r#"{{"plugin": "{}"}}"#, PLUGIN_ID))
    };
}

/// 创建带有自动UI更新回调的PluginUi
/// 使用方法: create_plugin_ui!()
#[macro_export]
macro_rules! create_plugin_ui {
    () => {
        {
            let ui = $crate::PluginUi::new();
            // 自动设置UI更新回调
            {
                let mut ui_guard = ui.lock().unwrap();
                ui_guard.set_update_callback(|| {
                    // 当UI状态更新时，发送事件到前端
                    $crate::send_ui_updated!();
                });
            }
            ui
        }
    };
}

/// 重新导出原始函数，避免名称冲突
pub use crate::api::send_to_frontend as send_to_frontend_raw;
pub use crate::handler::send_message_to_frontend as send_message_to_frontend_raw;
