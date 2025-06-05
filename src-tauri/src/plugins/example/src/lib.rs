
use plugin_interface::{
    create_plugin_interface_from_handler, log_info, log_warn, pluginui::{Context, Ui}, PluginHandler, PluginInterface, PluginMessage, PluginMetadata
};

/// 示例插件实现 - 使用新的UI框架
pub struct ExamplePlugin {
    metadata: PluginMetadata,
    name: String,
    age: u32,
    selected_option: Option<String>,
}

impl ExamplePlugin {
    // 导出插件的时候调用
    fn new() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
            selected_option: None,
            metadata: PluginMetadata{
                id: "example_plugin".to_string(),
                disabled: false,
                name: "Example Plugin".to_string(),
                description: "Example plugin using new UI framework".to_string(),
                version: "1.0.0".to_string(),
                author: Some("Augment".to_string()),
                library_path: None,
                config_path: "config.toml".to_string(),
            },
        }
    }
    fn theme_switcher(&mut self, ui: &mut Ui, _ctx: &Context) {
        ui.horizontal(|ui| {
            if ui.button("Dark").clicked() {
                log_info!("Dark theme");
                // 使用新的消息发送功能
                self.send_message_to_frontend("Dark theme selected");
            }
            if ui.button("Light").clicked() {
                log_info!("Light theme");
                // 使用新的消息发送功能
                self.send_message_to_frontend("Light theme selected");
            }
        });
    }
}

impl PluginHandler for ExamplePlugin {
    fn update_ui(&mut self, ctx: &Context, ui: &mut Ui) {
        // Simplified UI to test memory safety
        ui.label("Test Plugin");
        ui.label("Simple test without complex components");

        self.theme_switcher(ui, ctx);

        let text_response = ui.text_edit_singleline(&mut self.name);
        if text_response.changed() {
            log_info!("Text field updated: {}", self.name);
        }

        let combo_response = ui.combo_box(vec!["Option 1".to_string(), "Option 2".to_string(), "Option 3".to_string()], &mut self.selected_option, "Select an option");
        if combo_response.clicked() {
            log_info!("Combo box updated: {:?}", self.selected_option);
        }

        ui.label(&format!("Name: {}", self.name));
        ui.label(&format!("Age: {}", self.age));
        ui.label(&format!("Selected Option: {}", self.selected_option.as_ref().unwrap_or(&"None".to_string())));
    }
    
    // 挂载插件的时候调用
    fn on_mount(&mut self, metadata: &PluginMetadata) -> Result<(), Box<dyn std::error::Error>> {
        log_info!("[{}] Plugin mount successfully", self.metadata.name);
        log_info!("Config Metadata: id={}, name={}, version={}",
                  metadata.id, metadata.name, metadata.version);
        self.metadata = metadata.clone();
        Ok(())
    }

    fn on_dispose(&self) -> Result<(), Box<dyn std::error::Error>> {
        log_info!("[{}] Plugin disposed successfully", self.metadata.name);
        Ok(())
    }

    fn on_connect(&self) -> Result<(), Box<dyn std::error::Error>> {
        log_info!("[{}] Connected", self.metadata.name);
        Ok(())
    }

    fn on_disconnect(&self) -> Result<(), Box<dyn std::error::Error>> {
        log_warn!("[{}] Disconnected", self.metadata.name);
        Ok(())
    }

    fn handle_message(&self, message: &str) -> Result<String, Box<dyn std::error::Error>> {
        log_info!("[{}] Received message: {}", self.metadata.name, message);

        let response = format!("Echo from {}: {}", self.metadata.name, message);

        // 向前端发送响应
        // send_message_to_frontend!("收到消息: {}", message);
        Ok(response)
    }

    fn get_metadata(&self) -> PluginMetadata {
        self.metadata.clone()
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
            ((*interface).destroy)((*interface).plugin_ptr);
            let _ = Box::from_raw(interface);
        }
    }
}
