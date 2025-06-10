use plugin_interfaces::{
    create_plugin_interface_from_handler, log_info, log_warn,
    pluginui::{Context, Ui},
    PluginHandler, PluginInterface, PluginMessage, PluginMetadata, PluginStreamMessage,
    PluginUiOption,
};
use std::sync::Arc;
use tokio::{runtime::Runtime, sync::Mutex};

/// 示例插件实现 - 使用新的UI框架
#[derive(Clone)]
pub struct ExamplePlugin {
    metadata: PluginMetadata,
    name: String,
    age: Arc<Mutex<u32>>, // 使用 Arc<Mutex<T>> 包装以支持异步修改
    selected_option: Option<String>,
    dark_mode: bool,
    runtime: Option<Arc<Runtime>>, // tokio 异步运行时
}

impl ExamplePlugin {
    // 导出插件的时候调用
    fn new() -> Self {
        Self {
            name: "Debin".to_owned(),
            age: Arc::new(Mutex::new(32)), // 使用 Arc<Mutex<T>> 包装
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
                instance_id: None,
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

                    // 使用 tokio 异步延迟模拟真实的后台处理
                    tokio::time::sleep(tokio::time::Duration::from_millis(2000)).await;
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

    /// 专门用于演示在异步函数中修改 age 的函数
    async fn modify_age_async(&self) {
        log_info!("Starting async age modification...");

        // 模拟一些异步工作
        tokio::time::sleep(tokio::time::Duration::from_millis(2000)).await;

        // 读取当前值
        let old_age = {
            let age_guard = self.age.lock().await;
            *age_guard
        };

        // 修改 age 值
        {
            let mut age_guard = self.age.lock().await;
            *age_guard += 5;
            log_info!("Age modified from {} to {}", old_age, *age_guard);
        }

        // 通知前端
        let new_age = {
            let age_guard = self.age.lock().await;
            *age_guard
        };

        self.send_message_to_frontend(&format!(
            "Age updated from {} to {} (+{})",
            old_age, new_age, 5
        ));
        self.refresh_ui();
    }

    /// 启动异步修改 age 的任务
    fn start_async_age_modification(&self) {
        if let Some(runtime) = &self.runtime {
            let self_clone = Arc::new(self.clone());

            runtime.spawn(async move {
                self_clone.modify_age_async().await;
            });
        } else {
            log_warn!("Cannot modify age: runtime not initialized");
        }
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

        if ui.button("Age +5 (Async)").clicked() {
            log_info!("Starting async age increment by 5");
            self.start_async_age_modification();
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

        // 安全地读取 age 值用于显示
        let current_age = {
            let age_guard = self.age.blocking_lock();
            *age_guard
        };
        ui.label(&format!("Age: {}", current_age));

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

    fn on_dispose(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        log_info!("[{}] Plugin disposed successfully", self.metadata.name);
        Ok(())
    }

    fn on_connect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        log_info!("[{}] Connected", self.metadata.name);
        Ok(())
    }

    fn on_disconnect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
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
