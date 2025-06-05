use plugin_interface::{
    create_plugin_interface_from_handler, log_info, log_warn,
    pluginui::{Context, Ui},
    PluginHandler, PluginInterface, PluginMetadata,
};
use rand::Rng;
use serde_json::json;
use tokio::time::{sleep, Duration, Instant};
use tokio::sync::{RwLock, mpsc, broadcast};
use tokio::task::{JoinHandle, spawn};
use std::sync::Arc;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

/// 异步任务状态
#[derive(Debug, Clone)]
pub struct AsyncTaskState {
    pub task_id: String,
    pub status: String,
    pub progress: f32,
    pub message: String,
}

/// 示例插件实现 - 完全支持tokio异步
pub struct ExamplePlugin {
    metadata: PluginMetadata,
    name: String,
    age: u32,
    selected_option: Option<String>,
    dark_mode: bool,
    // 异步运行时相关
    runtime_handle: Option<tokio::runtime::Handle>,
    active_tasks: Arc<RwLock<HashMap<String, JoinHandle<()>>>>,
    task_states: Arc<RwLock<HashMap<String, AsyncTaskState>>>,
    shutdown_sender: Option<broadcast::Sender<()>>,
    message_channel: Option<mpsc::UnboundedSender<String>>,
}

impl ExamplePlugin {
    // 导出插件的时候调用
    fn new() -> Self {
        let (shutdown_sender, _) = broadcast::channel(1);
        let (message_sender, _message_receiver) = mpsc::unbounded_channel();

        Self {
            name: "AsyncDebin".to_owned(),
            age: 32,
            selected_option: None,
            dark_mode: false,
            metadata: PluginMetadata {
                id: "async_example_plugin".to_string(),
                disabled: false,
                name: "Async Example Plugin".to_string(),
                description: "Fully async plugin using tokio runtime".to_string(),
                version: "2.0.0".to_string(),
                author: Some("Augment".to_string()),
                library_path: None,
                config_path: "config.toml".to_string(),
            },
            runtime_handle: None,
            active_tasks: Arc::new(RwLock::new(HashMap::new())),
            task_states: Arc::new(RwLock::new(HashMap::new())),
            shutdown_sender: Some(shutdown_sender),
            message_channel: Some(message_sender),
        }
    }
    /// 异步主题切换器 - 支持动画效果
    fn async_theme_switcher(&mut self, ui: &mut Ui, _ctx: &Context) {
        ui.horizontal(|ui| {
            if ui.button("🌙 Dark Theme").clicked() {
                log_info!("Switching to dark theme with animation");
                self.spawn_theme_transition_task("dark".to_string());
            }
            if ui.button("☀️ Light Theme").clicked() {
                log_info!("Switching to light theme with animation");
                self.spawn_theme_transition_task("light".to_string());
            }
            if ui.button("🌊 Stream Demo").clicked() {
                log_info!("Starting async stream demo");
                self.spawn_async_stream_demo();
            }
            if ui.button("🤖 AI Chat").clicked() {
                log_info!("Starting AI chat with streaming");
                self.spawn_ai_chat_task("你好，请介绍一下异步插件的功能".to_string());
            }
            if ui.button("📊 Progress Task").clicked() {
                log_info!("Starting long-running progress task");
                self.spawn_progress_task();
            }
            if ui.button("🔄 Concurrent Tasks").clicked() {
                log_info!("Starting multiple concurrent tasks");
                self.spawn_concurrent_tasks();
            }
        });
    }

    /// 生成唯一任务ID
    fn generate_task_id(&self) -> String {
        format!("task_{}", Instant::now().elapsed().as_nanos())
    }

    /// 启动主题切换任务（带动画效果）
    fn spawn_theme_transition_task(&mut self, theme: String) {
        let task_id = self.generate_task_id();
        let task_id_for_handle = task_id.clone();
        let plugin_id = self.metadata.id.clone();
        let active_tasks = self.active_tasks.clone();
        let task_states = self.task_states.clone();

        let handle = spawn(async move {
            log_info!("Starting theme transition to: {}", theme);

            // 模拟主题切换动画
            for i in 0..=10 {
                let progress = i as f32 / 10.0;
                let message = format!("切换到{}主题... {}%", theme, (progress * 100.0) as u32);

                // 更新任务状态
                {
                    let mut states = task_states.write().await;
                    states.insert(task_id_for_handle.clone(), AsyncTaskState {
                        task_id: task_id_for_handle.clone(),
                        status: "running".to_string(),
                        progress,
                        message: message.clone(),
                    });
                }

                // 发送进度消息
                Self::send_async_message(&plugin_id, &message).await;

                // 模拟动画延迟
                sleep(Duration::from_millis(200)).await;
            }

            Self::send_async_message(&plugin_id, &format!("✅ {}主题切换完成！", theme)).await;
            log_info!("Theme transition completed: {}", theme);

            // 清理任务
            {
                let mut tasks = active_tasks.write().await;
                tasks.remove(&task_id_for_handle);
                let mut states = task_states.write().await;
                states.remove(&task_id_for_handle);
            }
        });

        // 保存任务句柄
        if let Ok(mut tasks) = self.active_tasks.try_write() {
            tasks.insert(task_id, handle);
        }
    }

    /// 启动异步流式演示
    fn spawn_async_stream_demo(&mut self) {
        let task_id = self.generate_task_id();
        let task_id_for_handle = task_id.clone();
        let plugin_id = self.metadata.id.clone();
        let active_tasks = self.active_tasks.clone();

        let handle = spawn(async move {
            log_info!("Starting async stream demo");

            // 模拟流式数据生成
            let data_chunks = vec![
                "🚀 初始化异步流...",
                "📡 建立数据连接...",
                "🔄 开始数据传输...",
                "📊 处理数据块 1/5...",
                "📊 处理数据块 2/5...",
                "📊 处理数据块 3/5...",
                "📊 处理数据块 4/5...",
                "📊 处理数据块 5/5...",
                "✅ 流式传输完成！",
            ];

            for (i, chunk) in data_chunks.iter().enumerate() {
                Self::send_async_message(&plugin_id, chunk).await;

                // 随机延迟模拟真实数据处理
                let delay = rand::thread_rng().gen_range(300..800);
                sleep(Duration::from_millis(delay)).await;

                log_info!("Sent chunk {}/{}: {}", i + 1, data_chunks.len(), chunk);
            }

            // 清理任务
            {
                let mut tasks = active_tasks.write().await;
                tasks.remove(&task_id_for_handle);
            }
        });

        if let Ok(mut tasks) = self.active_tasks.try_write() {
            tasks.insert(task_id, handle);
        }
    }

    /// 启动AI聊天任务
    fn spawn_ai_chat_task(&mut self, user_message: String) {
        let task_id = self.generate_task_id();
        let task_id_for_handle = task_id.clone();
        let plugin_id = self.metadata.id.clone();
        let active_tasks = self.active_tasks.clone();

        // 预生成随机数以避免Send问题
        let mut rng = rand::thread_rng();
        let ai_responses = vec![
            "🤖 你好！我是一个完全异步的AI助手插件。",
            "⚡ 我使用tokio运行时来处理所有操作，包括这个对话。",
            "🔄 我可以同时处理多个任务，而不会阻塞用户界面。",
            "📡 每个字符都是通过异步流式传输发送的。",
            "🚀 这展示了异步插件架构的强大功能！",
            "💡 你可以同时启动多个任务来测试并发能力。",
        ];
        let selected_index = rng.gen_range(0..ai_responses.len());
        let selected_response = ai_responses[selected_index].to_string();

        // 预生成所有延迟时间
        let delays: Vec<u64> = (0..selected_response.chars().count())
            .map(|_| rng.gen_range(50..150))
            .collect();

        let handle = spawn(async move {
            log_info!("Starting AI chat for message: {}", user_message);

            // 逐字符流式输出
            let mut current_text = String::new();
            for (i, char) in selected_response.chars().enumerate() {
                current_text.push(char);

                // 发送当前累积的文本
                Self::send_async_message(&plugin_id, &current_text).await;

                // 模拟AI思考延迟
                if i < delays.len() {
                    sleep(Duration::from_millis(delays[i])).await;
                }
            }

            log_info!("AI chat completed");

            // 清理任务
            {
                let mut tasks = active_tasks.write().await;
                tasks.remove(&task_id_for_handle);
            }
        });

        if let Ok(mut tasks) = self.active_tasks.try_write() {
            tasks.insert(task_id, handle);
        }
    }

    /// 启动进度任务
    fn spawn_progress_task(&mut self) {
        let task_id = self.generate_task_id();
        let task_id_for_handle = task_id.clone();
        let plugin_id = self.metadata.id.clone();
        let active_tasks = self.active_tasks.clone();
        let task_states = self.task_states.clone();

        let handle = spawn(async move {
            log_info!("Starting progress task");

            for i in 0..=100 {
                let progress = i as f32 / 100.0;
                let message = format!("📊 处理进度: {}% - {}", i,
                    match i {
                        0..=20 => "初始化中...",
                        21..=40 => "加载数据...",
                        41..=60 => "处理数据...",
                        61..=80 => "分析结果...",
                        81..=99 => "生成报告...",
                        100 => "完成！",
                        _ => "处理中...",
                    }
                );

                // 更新任务状态
                {
                    let mut states = task_states.write().await;
                    states.insert(task_id_for_handle.clone(), AsyncTaskState {
                        task_id: task_id_for_handle.clone(),
                        status: if i == 100 { "completed".to_string() } else { "running".to_string() },
                        progress,
                        message: message.clone(),
                    });
                }

                Self::send_async_message(&plugin_id, &message).await;

                // 模拟处理时间
                sleep(Duration::from_millis(100)).await;
            }

            log_info!("Progress task completed");

            // 清理任务
            {
                let mut tasks = active_tasks.write().await;
                tasks.remove(&task_id_for_handle);
                let mut states = task_states.write().await;
                states.remove(&task_id_for_handle);
            }
        });

        if let Ok(mut tasks) = self.active_tasks.try_write() {
            tasks.insert(task_id, handle);
        }
    }

    /// 启动并发任务演示
    fn spawn_concurrent_tasks(&mut self) {
        let plugin_id = self.metadata.id.clone();
        let active_tasks = self.active_tasks.clone();

        let handle = spawn(async move {
            log_info!("Starting concurrent tasks demo");

            Self::send_async_message(&plugin_id, "🔄 启动并发任务演示...").await;

            // 创建多个并发任务
            let task1 = Self::simulate_network_request("API-1", 1000);
            let task2 = Self::simulate_network_request("API-2", 1500);
            let task3 = Self::simulate_network_request("API-3", 800);
            let task4 = Self::simulate_file_processing("file.txt", 1200);

            // 并发执行所有任务
            let results = tokio::join!(task1, task2, task3, task4);

            Self::send_async_message(&plugin_id, &format!(
                "✅ 所有并发任务完成！结果: {:?}",
                (results.0, results.1, results.2, results.3)
            )).await;

            log_info!("Concurrent tasks completed");

            // 清理任务
            {
                let mut tasks = active_tasks.write().await;
                tasks.retain(|_, handle| !handle.is_finished());
            }
        });

        if let Ok(mut tasks) = self.active_tasks.try_write() {
            tasks.insert(self.generate_task_id(), handle);
        }
    }

    /// 异步发送消息到前端
    async fn send_async_message(plugin_id: &str, message: &str) {
        let _payload = json!({
            "type": "plugin_message",
            "plugin_id": plugin_id,
            "content": message,
            "message_type": "normal",
            "timestamp": std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis()
        });

        // 模拟异步发送
        tokio::task::yield_now().await;

        // 这里应该调用实际的前端发送函数
        // 在实际实现中，可以使用 send_to_frontend("plugin-message", &payload.to_string())
        log_info!("Async message sent: {}", message);
        println!("📤 [{}] {}", plugin_id, message);
    }

    /// 模拟网络请求
    async fn simulate_network_request(api_name: &str, delay_ms: u64) -> String {
        log_info!("Starting network request to {}", api_name);
        sleep(Duration::from_millis(delay_ms)).await;
        let result = format!("✅ {} 请求完成 ({}ms)", api_name, delay_ms);
        log_info!("{}", result);
        result
    }

    /// 模拟文件处理
    async fn simulate_file_processing(filename: &str, delay_ms: u64) -> String {
        log_info!("Starting file processing: {}", filename);
        sleep(Duration::from_millis(delay_ms)).await;
        let result = format!("📁 {} 处理完成 ({}ms)", filename, delay_ms);
        log_info!("{}", result);
        result
    }

    /// 获取活跃任务状态
    async fn get_active_task_states(&self) -> Vec<AsyncTaskState> {
        let states = self.task_states.read().await;
        states.values().cloned().collect()
    }

    /// 取消所有活跃任务
    async fn cancel_all_tasks(&self) {
        {
            let mut tasks = self.active_tasks.write().await;
            for (task_id, handle) in tasks.drain() {
                handle.abort();
                log_info!("Cancelled task: {}", task_id);
            }
        }

        {
            let mut states = self.task_states.write().await;
            states.clear();
        }
    }

}

impl PluginHandler for ExamplePlugin {
    fn update_ui_async<'a>(&'a mut self, ctx: &'a Context, ui: &'a mut Ui) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
        Box::pin(async move {
            // 异步UI更新 - 现在可以在UI更新中使用await
            ui.label("🚀 异步插件 - 完全支持tokio");
            ui.label("⚡ UI更新现在是异步的！");

            // 异步主题切换器
            self.async_theme_switcher(ui, ctx);

            // 显示实时任务状态（异步获取）
            let task_count = {
                let tasks = self.active_tasks.read().await;
                tasks.len()
            };
            ui.label(&format!("🔄 活跃任务数量: {}", task_count));

            // 显示任务状态详情
            let task_states = {
                let states = self.task_states.read().await;
                states.values().cloned().collect::<Vec<_>>()
            };

            if !task_states.is_empty() {
                ui.label("📊 任务状态详情:");
                for state in task_states.iter().take(3) { // 只显示前3个任务
                    ui.label(&format!("  • {} - {:.1}% - {}",
                        state.task_id.chars().take(8).collect::<String>(),
                        state.progress * 100.0,
                        state.status
                    ));
                }
                if task_states.len() > 3 {
                    ui.label(&format!("  ... 还有 {} 个任务", task_states.len() - 3));
                }
            }

            // AI聊天按钮 - 现在可以直接在UI中启动异步任务
            if ui.button("🤖 AI Chat Demo").clicked() {
                log_info!("Starting AI chat demo from async UI");
                self.spawn_ai_chat_task("你好，这是从异步UI启动的对话".to_string());
            }

            // 文本输入 - 异步处理变化
            let text_response = ui.text_edit_singleline(&mut self.name);
            if text_response.changed() {
                log_info!("Text field updated asynchronously: {}", self.name);

                // 可以在这里执行异步操作，比如保存到数据库
                tokio::task::yield_now().await; // 模拟异步操作
            }

            // 下拉框 - 异步处理选择
            let combo_response = ui.combo_box(
                vec![
                    "异步选项 1".to_string(),
                    "异步选项 2".to_string(),
                    "异步选项 3".to_string(),
                ],
                &mut self.selected_option,
                "选择一个异步选项",
            );
            if combo_response.clicked() {
                log_info!("Combo box updated asynchronously: {:?}", self.selected_option);

                // 异步处理选择变化
                tokio::task::yield_now().await;
            }

            // 开关组件 - 异步主题切换
            ui.horizontal(|ui| {
                ui.label("异步暗色模式:");
                let toggle_response = ui.toggle(&mut self.dark_mode);
                if toggle_response.changed() {
                    log_info!("Dark mode toggled asynchronously: {}", self.dark_mode);

                    // 启动异步主题切换任务
                    let theme = if self.dark_mode { "dark" } else { "light" };
                    self.spawn_theme_transition_task(theme.to_string());
                }
            });

            // 显示当前状态
            ui.label("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            ui.label(&format!("📝 名称: {}", self.name));
            ui.label(&format!("🎂 年龄: {}", self.age));
            ui.label(&format!(
                "📋 选择的选项: {}",
                self.selected_option.as_ref().unwrap_or(&"无".to_string())
            ));
            ui.label(&format!("🌙 暗色模式: {}", if self.dark_mode { "开启" } else { "关闭" }));

            // 异步任务控制按钮
            ui.label("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            ui.label("🎮 异步任务控制:");

            // 取消所有任务按钮 - 异步执行
            if ui.button("❌ 取消所有任务").clicked() {
                log_info!("Cancelling all active tasks asynchronously");

                // 直接在UI中执行异步取消操作
                {
                    let mut tasks = self.active_tasks.write().await;
                    for (task_id, handle) in tasks.drain() {
                        handle.abort();
                        log_info!("Cancelled task: {}", task_id);
                    }
                }
                {
                    let mut states = self.task_states.write().await;
                    states.clear();
                }
            }

            // 刷新状态按钮 - 演示异步UI刷新
            if ui.button("🔄 刷新状态").clicked() {
                log_info!("Refreshing UI state asynchronously");

                // 模拟异步刷新操作
                sleep(Duration::from_millis(10)).await;

                // 可以在这里更新插件状态
                self.age += 1; // 示例：增加年龄
            }
        })
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
        log_info!(
            "Config Metadata: id={}, name={}, version={}",
            self.metadata.id,
            self.metadata.name,
            self.metadata.version
        );
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
