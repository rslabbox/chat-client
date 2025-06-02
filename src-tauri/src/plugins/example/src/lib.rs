use plugin_interface::{
    create_plugin_interface_from_handler, log_info, log_warn, send_message_to_frontend,
    send_to_frontend, PluginHandler, PluginInterface, PluginMetadata, PluginUi,
};
use std::sync::{Arc, Mutex};

/// 示例插件实现
pub struct ExamplePlugin {
    name: String,
    ui: Arc<Mutex<PluginUi>>,
    text_value: Arc<Mutex<String>>,
}

impl ExamplePlugin {
    pub fn new() -> Self {
        let ui = PluginUi::new();

        let mut instance = Self {
            name: "Example Plugin".to_string(),
            ui: Arc::clone(&ui),
            text_value: Arc::new(Mutex::new(String::new())),
        };

        // 在创建完实例后初始化UI
        instance.init_ui();
        instance
    }

    fn init_ui(&mut self) {
        let ui_clone = Arc::clone(&self.ui);

        // 设置UI更新回调，当UI状态改变时通知前端
        {
            let mut ui_guard = ui_clone.lock().unwrap();
            ui_guard.set_update_callback(|| {
                // 当UI状态更新时，发送事件到前端
                send_to_frontend("plugin-ui-updated", r#"{"plugin": "example_plugin"}"#);
            });
        }

        // 创建一个文本框
        let text_value_clone = Arc::clone(&self.text_value);
        let _text_field = PluginUi::textfield(&ui_clone, "输入内容", move |value| {
            if let Ok(mut text) = text_value_clone.lock() {
                *text = value.clone();
            }
            log_info!("收到文本输入: {}", value);
        });

        let text_value_clone = Arc::clone(&self.text_value);
        let _scan_button = PluginUi::button(&ui_clone, "打招呼", Some("refresh"), true, move || {
            log_info!("扫描按钮被点击 - 开始异步操作");
            let text = text_value_clone.lock().unwrap();
            let hi = format!("hi, {}", *text);
            send_message_to_frontend("example_plugin", hi.as_str());
        });
        // scan_button.set_enabled(false);
    }
}

impl PluginHandler for ExamplePlugin {
    fn on_mount(&self) -> Result<(), Box<dyn std::error::Error>> {
        // 使用插件日志宏 - 现在会显示真实的文件名和行号
        log_info!("[{}] Plugin mount successfully", self.name);

        Ok(())
    }

    fn on_dispose(&self) -> Result<(), Box<dyn std::error::Error>> {
        log_info!("[{}] Plugin disposed successfully", self.name);
        Ok(())
    }

    fn on_connect(&self) -> Result<(), Box<dyn std::error::Error>> {
        log_info!("[{}] Connected from", self.name);
        Ok(())
    }

    fn on_disconnect(&self) -> Result<(), Box<dyn std::error::Error>> {
        log_warn!("[{}] Disconnected from", self.name);
        Ok(())
    }

    fn handle_message(&self, message: &str) -> Result<String, Box<dyn std::error::Error>> {
        log_info!("[{}] Received message: {}", self.name, message);

        let response = format!("Echo from {}: {}", self.name, message);

        // 向前端发送响应
        send_message_to_frontend("example_plugin", "pong");
        Ok(response)
    }

    fn get_metadata(&self) -> PluginMetadata {
        PluginMetadata {
            id: "example_plugin".to_string(),
            disabled: false,
            name: self.name.clone(),
            description: "这是一个示例插件，展示插件系统的基本功能和UI组件".to_string(),
            version: "1.0.0".to_string(),
            author: Some("Your Name".to_string()),
            library_path: None,
            config_path: "".to_string(),
        }
    }

    fn get_ui(&self) -> Arc<Mutex<PluginUi>> {
        Arc::clone(&self.ui)
    }
}

/// 创建插件实例的导出函数
#[no_mangle]
pub extern "C" fn create_plugin() -> *mut PluginInterface {
    let plugin = ExamplePlugin::new();
    let handler: Box<dyn PluginHandler> = Box::new(plugin);
    create_plugin_interface_from_handler(handler)
}

/// 销毁插件实例的导出函数
#[no_mangle]
pub extern "C" fn destroy_plugin(interface: *mut PluginInterface) {
    if !interface.is_null() {
        unsafe {
            // 调用接口的销毁函数
            ((*interface).destroy)((*interface).plugin_ptr);
            // 释放接口本身
            let _ = Box::from_raw(interface);
        }
    }
}
