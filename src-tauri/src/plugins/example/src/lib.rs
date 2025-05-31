use plugin_interface::{PluginHandler, PluginMetadata};
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
        println!("[{}] Plugin mounted successfully", self.name);
        Ok(())
    }

    fn on_dispose(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("[{}] Plugin disposed successfully", self.name);
        Ok(())
    }

    fn on_connect(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("[{}] Connected", self.name);
        Ok(())
    }

    fn on_disconnect(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("[{}] Disconnected", self.name);
        Ok(())
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
