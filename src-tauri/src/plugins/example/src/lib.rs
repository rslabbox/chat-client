use plugin_interface::{PluginHandler, PluginMetadata, log_info, log_warn, get_app_config};
use std::os::raw::c_char;

/// 示例插件实现
pub struct ExamplePlugin {
    name: String,
}

impl ExamplePlugin {
    pub fn new() -> Self {
        Self {
            name: "Example Plugin".to_string(),
        }
    }
}

impl PluginHandler for ExamplePlugin {
    fn on_mount(&self) -> Result<(), Box<dyn std::error::Error>> {
        // 使用主程序提供的日志函数
        log_info(&format!("[{}] Plugin mounted successfully", self.name));


        // 获取应用配置示例
        if let Some(config) = get_app_config("app_version") {
            log_info(&format!("[{}] App version: {}", self.name, config));
        }

        Ok(())
    }

    fn on_dispose(&self) -> Result<(), Box<dyn std::error::Error>> {
        log_info(&format!("[{}] Plugin disposed successfully", self.name));
        Ok(())
    }

    fn on_connect(&self) -> Result<(), Box<dyn std::error::Error>> {
        log_info(&format!("[{}] Connected", self.name));
        Ok(())
    }

    fn on_disconnect(&self) -> Result<(), Box<dyn std::error::Error>> {
        log_warn(&format!("[{}] Disconnected", self.name));
        Ok(())
    }

    fn handle_message(&self, message: &str) -> Result<String, Box<dyn std::error::Error>> {
        log_info(&format!("[{}] Received message: {}", self.name, message));

        let response = format!("Echo from {}: {}", self.name, message);

        // 向前端发送响应
        self.send_message_to_frontend("pong");
        Ok(response)
    }

    fn get_metadata(&self) -> PluginMetadata {
        PluginMetadata {
            id: "example_plugin".to_string(),
            disabled: false,
            name: self.name.clone(),
            description: "这是一个示例插件，展示插件系统的基本功能".to_string(),
            version: "1.0.0".to_string(),
            author: Some("Your Name".to_string()),
            library_path: None,
            config_path: "".to_string(),
        }
    }
}

/// 创建插件实例的导出函数
#[no_mangle]
pub extern "C" fn create_plugin() -> *mut dyn PluginHandler {
    let plugin = ExamplePlugin::new();
    Box::into_raw(Box::new(plugin))
}

/// 销毁插件实例的导出函数
#[no_mangle]
pub extern "C" fn destroy_plugin(plugin: *mut dyn PluginHandler) {
    if !plugin.is_null() {
        unsafe {
            let _ = Box::from_raw(plugin);
        }
    }
}

/// 获取插件信息的导出函数（可选）
#[no_mangle]
pub extern "C" fn get_plugin_info() -> *const c_char {
    let info = r#"{"name":"Example Plugin","version":"1.0.0","description":"示例插件"}"#;
    info.as_ptr() as *const c_char
}
