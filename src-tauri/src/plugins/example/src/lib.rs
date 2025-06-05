
use plugin_interface::{
    create_plugin_interface_from_handler, log_info, log_warn, pluginui::{Context, Ui},
    PluginHandler, PluginInterface, PluginMessage, PluginStreamMessage, PluginMetadata
};

/// 示例插件实现 - 使用新的UI框架
pub struct ExamplePlugin {
    metadata: PluginMetadata,
    name: String,
    age: u32,
    selected_option: Option<String>,
    dark_mode: bool,
}

impl ExamplePlugin {
    // 导出插件的时候调用
    fn new() -> Self {
        Self {
            name: "Debin".to_owned(),
            age: 32,
            selected_option: None,
            dark_mode: false,
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
            if ui.button("Stream Demo").clicked() {
                log_info!("Starting stream demo");
                // 演示流式消息功能
                self.demo_streaming_message();
            }
        });
    }

    fn demo_streaming_message(&self) {
        // 演示流式消息的使用
        match self.send_message_stream_start("demo", Some("Streaming demo")) {
            Ok(stream_id) => {
                log_info!("Started stream: {}", stream_id);

                // 发送一些示例数据块
                let chunks = vec![
                    "这是第一部分数据...",
                    "这是第二部分数据...",
                    "这是第三部分数据...",
                ];

                for (i, chunk) in chunks.iter().enumerate() {
                    let is_final = i == chunks.len() - 1;
                    if let Err(e) = self.send_message_stream(&stream_id, chunk, is_final) {
                        log_warn!("Failed to send stream chunk: {}", e);
                        let _ = self.send_message_stream_end(&stream_id, false, Some(&format!("Error: {}", e)));
                        return;
                    }
                }

                // 结束流式传输
                if let Err(e) = self.send_message_stream_end(&stream_id, true, None) {
                    log_warn!("Failed to end stream: {}", e);
                }

                log_info!("Stream demo completed");
            }
            Err(e) => {
                log_warn!("Failed to start stream: {}", e);
            }
        }
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

        // 开关组件示例 - 在水平布局中使用
        ui.horizontal(|ui| {
            ui.label("Dark Mode:");
            let toggle_response = ui.toggle(&mut self.dark_mode);
            if toggle_response.changed() {
                log_info!("Dark mode toggled: {}", self.dark_mode);
            }
        });

        ui.label(&format!("Name: {}", self.name));
        ui.label(&format!("Age: {}", self.age));
        ui.label(&format!("Selected Option: {}", self.selected_option.as_ref().unwrap_or(&"None".to_string())));
        ui.label(&format!("Dark Mode: {}", self.dark_mode));
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
        log_info!("Config Metadata: id={}, name={}, version={}",
                  self.metadata.id, self.metadata.name, self.metadata.version);
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
