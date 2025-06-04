use std::cell::RefCell;

thread_local! {
    /// 线程本地插件ID存储
    static PLUGIN_ID: RefCell<Option<String>> = RefCell::new(None);
}

/// 设置当前线程的插件ID（在插件创建时调用）
pub fn set_plugin_id(id: String) {
    PLUGIN_ID.with(|plugin_id| {
        *plugin_id.borrow_mut() = Some(id);
    });
}

/// 获取当前线程的插件ID
pub fn get_plugin_id() -> Option<String> {
    PLUGIN_ID.with(|plugin_id| {
        plugin_id.borrow().clone()
    })
}

/// 自动使用plugin id的宏定义
/// 这些宏会自动使用全局的 PLUGIN_ID 来获取插件ID

/// 向前端发送插件消息响应，自动使用当前插件的ID
/// 使用方法:
/// - send_message_to_frontend!("简单消息")
/// - send_message_to_frontend!("格式化消息: {}", value)
/// - send_message_to_frontend!("多个参数: {} {}", arg1, arg2)
#[macro_export]
macro_rules! send_message_to_frontend {
    ($($arg:tt)*) => {
        {
            if let Some(plugin_id) = $crate::macros::get_plugin_id() {
                $crate::handler::send_message_to_frontend(&plugin_id, &format!($($arg)*))
            } else {
                eprintln!("Warning: PLUGIN_ID not initialized, cannot send message to frontend");
                false
            }
        }
    };
}

/// 向前端发送插件UI更新事件，自动使用当前插件的ID
/// 使用方法: send_ui_updated!()
#[macro_export]
macro_rules! send_ui_updated {
    () => {
        {
            if let Some(plugin_id) = $crate::macros::get_plugin_id() {
                $crate::api::send_to_frontend("plugin-ui-updated", &format!(r#"{{"plugin": "{}"}}"#, plugin_id))
            } else {
                eprintln!("Warning: PLUGIN_ID not initialized, cannot send UI update");
                false
            }
        }
    };
}

/// 创建带有自动UI更新回调的PluginUi
/// 使用方法: create_plugin_ui!()
/// 注意：这个宏使用全局的 PLUGIN_ID
#[macro_export]
macro_rules! create_plugin_ui {
    () => {
        {
            let ui = $crate::PluginUi::new();
            if let Some(plugin_id) = $crate::macros::get_plugin_id() {
                let plugin_id = plugin_id.clone();
                // 自动设置UI更新回调
                {
                    let mut ui_guard = ui.lock().unwrap();
                    ui_guard.set_update_callback(move || {
                        // 当UI状态更新时，发送事件到前端
                        $crate::api::send_to_frontend("plugin-ui-updated", &format!(r#"{{"plugin": "{}"}}"#, plugin_id));
                    });
                }
            } else {
                eprintln!("Warning: PLUGIN_ID not initialized, UI update callback not set");
            }
            ui
        }
    };
}

/// 重新导出原始函数，避免名称冲突
pub use crate::api::send_to_frontend as send_to_frontend_raw;
pub use crate::handler::send_message_to_frontend as send_message_to_frontend_raw;
