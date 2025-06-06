use plugin_interface::{
    create_plugin_interface_from_handler, log_info, log_warn,
    pluginui::{Context, Ui},
    PluginHandler, PluginInterface, PluginMessage, PluginMetadata, PluginStreamMessage,
};
use std::sync::Arc;
use tokio::runtime::Runtime;

/// 示例插件实现 - 使用新的UI框架
#[derive(Clone)]
pub struct ExamplePlugin {
    metadata: PluginMetadata,
    name: String,
    age: u32,
    selected_option: Option<String>,
    dark_mode: bool,
    runtime: Option<Arc<Runtime>>, // tokio 异步运行时
}

impl ExamplePlugin {
    // 导出插件的时候调用
    fn new() -> Self {
        Self {
            name: "Debin".to_owned(),
            age: 32,
            selected_option: None,
            dark_mode: false,
            runtime: None, // 在 on_mount 时初始化
            metadata: PluginMetadata {
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

    async fn demo_streaming_message_background_async(self: Arc<Self>) {
        // 演示后台异步流式消息的使用
        match self.send_message_stream_start("demo", Some("Background Streaming demo")) {
            Ok(stream_id) => {
                log_info!("Started background stream: {}", stream_id);

                // 发送一些示例数据块
                let chunks = vec![
                    "后台第一部分数据...",
                    "后台第二部分数据...",
                    "后台第三部分数据...",
                    "后台第四部分数据，请稍等...",
                ];

                for (i, chunk) in chunks.iter().enumerate() {
                    // 使用 tokio 异步延迟模拟真实的后台处理
                    tokio::time::sleep(tokio::time::Duration::from_millis(2000)).await;

                    let is_final = i == chunks.len() - 1;
                    if let Err(e) = self.send_message_stream(&stream_id, chunk, is_final) {
                        log_warn!("Failed to send background stream chunk: {}", e);
                        let _ = self.send_message_stream_end(
                            &stream_id,
                            false,
                            Some(&format!("Error: {}", e)),
                        );
                        return;
                    }
                }

                // 结束流式传输
                if let Err(e) = self.send_message_stream_end(&stream_id, true, None) {
                    log_warn!("Failed to end background stream: {}", e);
                }

                log_info!("Background stream demo completed");
            }
            Err(e) => {
                log_warn!("Failed to start background stream: {}", e);
            }
        }
    }

    fn demo_streaming_message_background(self: Arc<Self>) {
        // 使用 tokio runtime 执行异步任务
        if let Some(runtime) = self.runtime.clone() {
            let self_clone = self.clone();
            runtime.spawn(async move {
                self_clone.demo_streaming_message_background_async().await;
            });
        } else {
            log_warn!("Tokio runtime not initialized, falling back to thread");
            // 如果 runtime 未初始化，回退到原来的线程方式
            std::thread::spawn(move || {
                // 这里可以保留原来的同步实现作为备用
                log_warn!("Using fallback thread implementation");
            });
        }
    }

    /// 演示如何在其他函数中使用 tokio runtime
    fn execute_async_task(&self, task_name: &str) {
        if let Some(runtime) = &self.runtime {
            let task_name = task_name.to_string();
            let self_clone = Arc::new(self.clone());

            runtime.spawn(async move {
                log_info!("Starting async task: {}", task_name);

                // 模拟一些异步工作
                tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;

                // 可以在这里调用其他异步函数
                self_clone.some_async_operation().await;

                log_info!("Completed async task: {}", task_name);
            });
        } else {
            log_warn!("Cannot execute async task '{}': runtime not initialized", task_name);
        }
    }

    /// 示例异步操作
    async fn some_async_operation(&self) {
        self.send_message_to_frontend(&format!("Async operation start: Age={}", self.age));
        log_info!("Performing some async operation...");

        // 模拟网络请求或其他异步操作
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

        // 可以发送消息到前端
        self.send_message_to_frontend(&format!("Async operation completed: Age={}", self.age));


        log_info!("Async operation finished");
    }
}

impl PluginHandler for ExamplePlugin {
    fn update_ui(&mut self, ctx: &Context, ui: &mut Ui) {
        // Simplified UI to test memory safety
        ui.label("Test Plugin");
        ui.label("Simple test without complex components");

        self.theme_switcher(ui, ctx);

        if ui.button("Background Stream Demo").clicked() {
            log_info!("Starting stream demo");
            // 创建Arc包装的self引用，使用 tokio 异步任务
            let self_arc = Arc::new(self.clone());
            self_arc.demo_streaming_message_background();
        }

        if ui.button("Execute Async Task").clicked() {
            log_info!("Starting async task demo");
            self.execute_async_task("demo_task");
        }

        let text_response = ui.text_edit_singleline(&mut self.name);
        if text_response.changed() {
            log_info!("Text field updated: {}", self.name);
        }

        let combo_response = ui.combo_box(
            vec![
                "Option 1".to_string(),
                "Option 2".to_string(),
                "Option 3".to_string(),
            ],
            &mut self.selected_option,
            "Select an option",
        );
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
        ui.label(&format!(
            "Selected Option: {}",
            self.selected_option.as_ref().unwrap_or(&"None".to_string())
        ));
        ui.label(&format!("Dark Mode: {}", self.dark_mode));
    }

    // 挂载插件的时候调用
    fn on_mount(&mut self, metadata: &PluginMetadata) -> Result<(), Box<dyn std::error::Error>> {
        log_info!("[{}] Plugin mount successfully", self.metadata.name);
        log_info!(
            "Config Metadata: id={}, name={}, version={}",
            metadata.id,
            metadata.name,
            metadata.version
        );
        self.metadata = metadata.clone();

        // 初始化 tokio 异步运行时
        match Runtime::new() {
            Ok(runtime) => {
                self.runtime = Some(Arc::new(runtime));
                log_info!("Tokio runtime initialized successfully");
            }
            Err(e) => {
                log_warn!("Failed to initialize tokio runtime: {}", e);
            }
        }

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
