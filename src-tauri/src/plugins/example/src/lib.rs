use plugin_interfaces::StreamError;
use plugin_interfaces::{
    create_plugin_interface_from_handler, log_info, log_warn,
    pluginui::{Context, Ui},
    PluginHandler, PluginInstanceContext, PluginInterface, PluginMessage, PluginStreamMessage,
    PluginUiOption,
};
use std::sync::Arc;
use tokio::{runtime::Runtime, sync::Mutex};

/// 示例插件实现 - 使用新的UI框架
#[derive(Clone)]
pub struct ExamplePlugin {
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
        }
    }
    fn theme_switcher(&mut self, ui: &mut Ui, _ctx: &Context, plugin_ctx: &PluginInstanceContext) {
        ui.horizontal(|ui| {
            if ui.button("Short").clicked() {
                log_info!("Send short message");
                // 使用新的上下文传递API发送消息
                self.send_message_to_frontend("Test Message", plugin_ctx);
            }
            if ui.button("Markdown").clicked() {
                log_info!("Send Markdown");
                // 使用新的上下文传递API发送复杂消息
                self.send_message_to_frontend(
                    r"以下是一个代码块和一个数学公式的示例：

### 代码块 (Python)
```python
# 计算斐波那契数列
def fibonacci(n):
    if n <= 1:
        return n
    else:
        return fibonacci(n-1) + fibonacci(n-2)
```

### 数学公式 (欧拉公式)
$$ e^{i\pi} + 1 = 0 $$

#### 公式说明：
这是数学中最著名的公式之一，由莱昂哈德·欧拉提出，它将五个基本数学常数联系在一起：
- $e$ (自然对数的底)
- $i$ (虚数单位)
- $\pi$ (圆周率)
- $1$ (乘法单位元)
- $0$ (加法单位元)

公式在复分析和数学物理中有重要应用，展示了指数函数与三角函数的深刻联系：
$$ e^{i\theta} = \cos\theta + i\sin\theta $$

当 $\theta = \pi$ 时，得到：
$$ e^{i\pi} = \cos\pi + i\sin\pi = -1 + 0i $$",
                    plugin_ctx,
                );
            }
        });
    }

    async fn demo_streaming_message_background_async(
        self: Arc<Self>,
        plugin_ctx: PluginInstanceContext,
    ) {
        // 重新实现流式消息功能，支持上下文传递
        log_info!("Starting background stream demo with context support");

        match self.send_message_stream_start(&plugin_ctx) {
            Ok(stream_id) => {
                log_info!("Started background stream: {}", stream_id);

                // 发送一些示例数据块
                let chunks = vec![
                    "后台第一部分数据...\n",
                    "后台第二部分数据...\n",
                    "后台第三部分数据...\n",
                    "后台第四部分数据...\n",
                    "后台第五部分数据...\n",
                    "后台第六部分数据...\n",
                    "后台第七部分数据...\n",
                    "后台第八部分数据...\n",
                    "后台第九部分数据...\n",
                    "后台第十部分数据, 已完成\n",
                ];

                for (i, chunk) in chunks.iter().enumerate() {
                    let is_final = i == chunks.len() - 1;
                    match self.send_message_stream(&stream_id, chunk, is_final, &plugin_ctx) {
                        Ok(_) => {
                            log_info!("Sent chunk {}/{}: {}", i + 1, chunks.len(), chunk);
                        }
                        Err(e) => {
                            match e {
                                StreamError::StreamCancelled => {
                                    log_info!(
                                        "Stream {} was cancelled by user, stopping gracefully...",
                                        stream_id
                                    );
                                    return; // 用户取消，直接返回，不发送错误消息
                                }
                                _ => {
                                    log_warn!("Failed to send background stream chunk: {}", e);
                                    let _ = self.send_message_stream_end(
                                        &stream_id,
                                        false,
                                        Some(&format!("Error: {}", e)),
                                        &plugin_ctx,
                                    );
                                    return;
                                }
                            }
                        }
                    }

                    // 使用 tokio 异步延迟模拟真实的后台处理
                    tokio::time::sleep(tokio::time::Duration::from_millis(2000)).await;
                }

                // 结束流式传输
                if let Err(e) = self.send_message_stream_end(&stream_id, true, None, &plugin_ctx) {
                    log_warn!("Failed to end background stream: {}", e);
                }

                log_info!("Background stream demo completed");
            }
            Err(e) => {
                log_warn!("Failed to start background stream: {}", e);
            }
        }
    }

    fn demo_streaming_message_background(self: Arc<Self>, plugin_ctx: PluginInstanceContext) {
        // 使用 tokio runtime 执行异步任务
        if let Some(runtime) = self.runtime.clone() {
            let self_clone = self.clone();
            runtime.spawn(async move {
                self_clone
                    .demo_streaming_message_background_async(plugin_ctx)
                    .await;
            });
        } else {
            log_warn!("Tokio runtime not initialized, falling back to thread");
        }
    }

    /// 专门用于演示在异步函数中修改 age 的函数
    /// 通过克隆必要的数据来解决生命周期问题
    async fn modify_age_async(&self, instance_context: PluginInstanceContext) {
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

        // 通知前端 - 使用克隆的上下文
        let new_age = {
            let age_guard = self.age.lock().await;
            *age_guard
        };

        // 现在可以正确地发送消息到前端
        self.send_message_to_frontend(
            &format!("Age updated from {} to {} (+{})", old_age, new_age, 5),
            &instance_context,
        );

        // 刷新UI
        self.refresh_ui(&instance_context);
    }

    /// 启动异步修改 age 的任务
    fn start_async_age_modification(&self, plugin_ctx: &PluginInstanceContext) {
        if let Some(runtime) = &self.runtime {
            let self_clone = Arc::new(self.clone());
            let context_clone = plugin_ctx.clone(); // 克隆上下文以便在异步任务中使用

            runtime.spawn(async move {
                self_clone.modify_age_async(context_clone).await;
            });
        } else {
            log_warn!("Cannot modify age: runtime not initialized");
        }
    }
}

impl PluginHandler for ExamplePlugin {
    fn update_ui(&mut self, ctx: &Context, ui: &mut Ui, plugin_ctx: &PluginInstanceContext) {
        // Simplified UI to test memory safety
        ui.label("Test Plugin");
        ui.label("Simple test without complex components");

        self.theme_switcher(ui, ctx, plugin_ctx);

        if ui.button("Background Stream Demo").clicked() {
            log_info!("Starting stream demo");
            // 创建Arc包装的self引用，使用 tokio 异步任务
            let self_arc = Arc::new(self.clone());
            let context_clone = plugin_ctx.clone();
            self_arc.demo_streaming_message_background(context_clone);
        }

        if ui.button("Age +5 (Async)").clicked() {
            log_info!("Starting async age increment by 5");
            self.start_async_age_modification(plugin_ctx);
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
            ui.label("Toggle Mode:");
            let toggle_response = ui.toggle(&mut self.dark_mode);
            if toggle_response.changed() {
                log_info!("Mode toggled: {}", self.dark_mode);
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
        ui.label(&format!("Mode: {}", self.dark_mode));
    }

    // 挂载插件的时候调用
    fn on_mount(
        &mut self,
        plugin_ctx: &PluginInstanceContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let metadata = plugin_ctx.get_metadata();
        log_info!("[{}] Plugin mount successfully", metadata.name);
        log_info!(
            "Config Metadata: id={}, name={}, version={}, instance_id={}",
            metadata.id,
            metadata.name,
            metadata.version,
            metadata.instance_id.clone().unwrap_or("None".to_string())
        );

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

    fn on_dispose(
        &mut self,
        plugin_ctx: &PluginInstanceContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let metadata = plugin_ctx.get_metadata();
        log_info!(
            "Plugin disposed successfully. Metadata: id={}, name={}, version={}, instance_id={}",
            metadata.id,
            metadata.name,
            metadata.version,
            metadata.instance_id.clone().unwrap_or("None".to_string())
        );
        // 关闭 tokio 异步运行时
        if let Some(runtime) = self.runtime.clone() {
            // Use Arc::try_unwrap to get ownership if this is the last reference
            match Arc::try_unwrap(runtime) {
                Ok(runtime) => {
                    runtime.shutdown_timeout(std::time::Duration::from_millis(10));
                    log_info!("Tokio runtime shutdown successfully");
                }
                Err(_) => {
                    log_warn!("Cannot shutdown runtime: other references still exist");
                }
            }
        } else {
            log_warn!("Tokio runtime not initialized, cannot shutdown");
        }
        Ok(())
    }

    fn on_connect(
        &mut self,
        plugin_ctx: &PluginInstanceContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let metadata = plugin_ctx.get_metadata();
        log_info!(
            "Plugin connect successfully. Metadata: id={}, name={}, version={}, instance_id={}",
            metadata.id,
            metadata.name,
            metadata.version,
            metadata.instance_id.clone().unwrap_or("None".to_string())
        );
        Ok(())
    }

    fn on_disconnect(
        &mut self,
        plugin_ctx: &PluginInstanceContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let metadata = plugin_ctx.get_metadata();
        log_info!(
            "Plugin disconnect successfully. Metadata: id={}, name={}, version={}, instance_id={}",
            metadata.id,
            metadata.name,
            metadata.version,
            metadata.instance_id.clone().unwrap_or("None".to_string())
        );
        Ok(())
    }

    fn handle_message(
        &mut self,
        message: &str,
        plugin_ctx: &PluginInstanceContext,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let metadata = plugin_ctx.get_metadata();
        log_info!(
            "Plugin Recive Message. Metadata: id={}, name={}, version={}, instance_id={}, require_history={}",
            metadata.id,
            metadata.name,
            metadata.version,
            metadata.instance_id.clone().unwrap_or("None".to_string()),
            metadata.require_history
        );

        // 检查是否有历史记录
        let history_info = if metadata.require_history {
            if let Some(history) = plugin_ctx.get_history() {
                format!("（包含 {} 条历史记录）", history.len())
            } else {
                "（无历史记录）".to_string()
            }
        } else {
            "".to_string()
        };

        let response = format!(
            "Echo from {}: {}{}",
            plugin_ctx.get_metadata().name,
            message,
            history_info
        );

        // 向前端发送响应
        self.send_message_to_frontend(
            &format!("[{}]收到消息：{}{}", metadata.name, message, history_info),
            plugin_ctx,
        );
        Ok(response)
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
