use plugin_interface::{
    get_app_config, plugin_info, plugin_warn, PluginHandler, PluginMetadata, PluginInterface,
    create_plugin_interface_from_handler,
};

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
        // 使用插件日志宏
        plugin_info!("[{}] Plugin mounted successfully", self.name);

        // 获取应用配置示例
        if let Some(config) = get_app_config("app_version") {
            plugin_info!("[{}] App version: {}", self.name, config);
        }

        Ok(())
    }

    fn on_dispose(&self) -> Result<(), Box<dyn std::error::Error>> {
        plugin_info!("[{}] Plugin disposed successfully", self.name);
        Ok(())
    }

    fn on_connect(&self) -> Result<(), Box<dyn std::error::Error>> {
        plugin_info!("[{}] Connected", self.name);
        Ok(())
    }

    fn on_disconnect(&self) -> Result<(), Box<dyn std::error::Error>> {
        plugin_warn!("[{}] Disconnected", self.name);
        Ok(())
    }

    fn handle_message(&self, message: &str) -> Result<String, Box<dyn std::error::Error>> {
        plugin_info!("[{}] Received message: {}", self.name, message);

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