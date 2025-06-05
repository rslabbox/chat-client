use plugin_interface::{
    AsyncPluginHandler, PluginMetadata, PluginMessage, PluginStreamMessage,
    pluginui::{Context, Ui}, callbacks::HostCallbacks
};
use std::future::Future;
use std::pin::Pin;
use tokio::time::{sleep, Duration};
use rand::Rng;
use serde_json::json;

/// 异步示例插件
pub struct AsyncExamplePlugin {
    metadata: PluginMetadata,
    name: String,
    age: u32,
    selected_option: Option<String>,
    dark_mode: bool,
}

impl AsyncExamplePlugin {
    pub fn new() -> Self {
        Self {
            metadata: PluginMetadata::default(),
            name: "AsyncExample".to_string(),
            age: 25,
            selected_option: None,
            dark_mode: false,
        }
    }

    /// 异步AI对话流式输出
    async fn simulate_ai_response_async(&self, user_message: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("Async AI response for: {}", user_message);
        
        // 准备AI回答内容
        let ai_responses = vec![
            "你好！我是一个异步AI助手插件。",
            "我可以在完全异步的环境中运行，享受tokio的所有特性。",
            "这个功能展示了如何在异步插件中实现流式输出。",
            "每个字符都会有随机的延迟，完全异步处理。",
            "你可以看到异步插件的强大之处！",
            "希望这个演示对你有帮助。"
        ];
        
        // 随机选择一个回答
        let mut rng = rand::thread_rng();
        let selected_response = ai_responses[rng.gen_range(0..ai_responses.len())];
        
        // 开始流式传输
        let stream_id = self.send_message_stream_start("async_ai_chat", Some("异步AI回答"))?;
        
        let mut current_text = String::new();
        let chars: Vec<char> = selected_response.chars().collect();
        
        for (i, &char) in chars.iter().enumerate() {
            current_text.push(char);
            
            // 异步延迟 - 可以直接使用tokio的sleep
            let delay = rng.gen_range(50..200);
            sleep(Duration::from_millis(delay)).await;
            
            let is_final = i == chars.len() - 1;
            
            // 直接调用流式消息方法
            self.send_message_stream(&stream_id, &current_text, is_final)?;
        }
        
        // 结束流式传输
        self.send_message_stream_end(&stream_id, true, None)?;
        
        Ok(())
    }

    /// 异步长时间任务模拟
    async fn simulate_long_task_async(&self) -> Result<(), Box<dyn std::error::Error>> {
        let stream_id = self.send_message_stream_start("async_task", Some("异步任务进度"))?;
        
        for progress in 0..=100 {
            let message = format!("异步处理进度: {}% - {}", progress, 
                match progress {
                    0..=20 => "初始化中...",
                    21..=50 => "数据处理中...",
                    51..=80 => "分析结果中...",
                    81..=99 => "生成报告中...",
                    100 => "任务完成！"
                }
            );
            
            let is_final = progress == 100;
            self.send_message_stream(&stream_id, &message, is_final)?;
            
            // 异步延迟
            sleep(Duration::from_millis(50)).await;
        }
        
        self.send_message_stream_end(&stream_id, true, None)?;
        Ok(())
    }

    /// 异步并发任务演示
    async fn demonstrate_concurrency(&self) -> Result<(), Box<dyn std::error::Error>> {
        // 同时启动多个异步任务
        let task1 = self.simulate_ai_response_async("任务1");
        let task2 = self.simulate_long_task_async();
        
        // 并发执行
        let (result1, result2) = tokio::join!(task1, task2);
        
        result1?;
        result2?;
        
        self.send_message_to_frontend("所有并发任务完成！");
        Ok(())
    }
}

impl AsyncPluginHandler for AsyncExamplePlugin {
    fn update_ui_async(&mut self, _ctx: &Context, ui: &mut Ui) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        Box::pin(async move {
            ui.label("异步测试插件");
            ui.label("支持完整的异步操作");

            if ui.button("异步AI对话").clicked() {
                let _ = self.simulate_ai_response_async("你好，请介绍一下异步插件").await;
            }

            if ui.button("异步长任务").clicked() {
                let _ = self.simulate_long_task_async().await;
            }

            if ui.button("并发演示").clicked() {
                let _ = self.demonstrate_concurrency().await;
            }

            let text_response = ui.text_edit_singleline(&mut self.name);
            if text_response.changed() {
                println!("异步文本字段更新: {}", self.name);
            }

            ui.horizontal(|ui| {
                ui.label("Dark Mode:");
                let toggle_response = ui.toggle(&mut self.dark_mode);
                if toggle_response.changed() {
                    println!("异步暗色模式切换: {}", self.dark_mode);
                }
            });
        })
    }

    fn on_mount_async(&mut self, metadata: &PluginMetadata) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn std::error::Error>>> + Send + '_>> {
        Box::pin(async move {
            println!("[{}] 异步插件挂载成功", metadata.name);
            self.metadata = metadata.clone();
            
            // 可以在这里执行异步初始化操作
            sleep(Duration::from_millis(100)).await;
            
            Ok(())
        })
    }

    fn on_dispose_async(&self) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn std::error::Error>>> + Send + '_>> {
        Box::pin(async move {
            println!("[{}] 异步插件卸载", self.metadata.name);
            
            // 可以在这里执行异步清理操作
            sleep(Duration::from_millis(50)).await;
            
            Ok(())
        })
    }

    fn on_connect_async(&self) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn std::error::Error>>> + Send + '_>> {
        Box::pin(async move {
            println!("[{}] 异步连接", self.metadata.name);
            Ok(())
        })
    }

    fn on_disconnect_async(&self) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn std::error::Error>>> + Send + '_>> {
        Box::pin(async move {
            println!("[{}] 异步断开连接", self.metadata.name);
            Ok(())
        })
    }

    fn handle_message_async(&self, message: &str) -> Pin<Box<dyn Future<Output = Result<String, Box<dyn std::error::Error>>> + Send + '_>> {
        Box::pin(async move {
            println!("[{}] 异步处理消息: {}", self.metadata.name, message);
            
            // 模拟异步处理
            sleep(Duration::from_millis(10)).await;
            
            let response = format!("异步回应来自 {}: {}", self.metadata.name, message);
            Ok(response)
        })
    }

    fn get_metadata(&self) -> PluginMetadata {
        self.metadata.clone()
    }

    fn start_runtime(&self) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn std::error::Error>>> + Send + '_>> {
        Box::pin(async move {
            println!("[{}] 启动异步运行时", self.metadata.name);
            
            // 可以在这里启动后台任务
            tokio::spawn(async {
                loop {
                    sleep(Duration::from_secs(30)).await;
                    println!("异步插件心跳检测");
                }
            });
            
            Ok(())
        })
    }

    fn stop_runtime(&self) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn std::error::Error>>> + Send + '_>> {
        Box::pin(async move {
            println!("[{}] 停止异步运行时", self.metadata.name);
            
            // 可以在这里清理后台任务
            sleep(Duration::from_millis(100)).await;
            
            Ok(())
        })
    }
}

// 为了兼容性，也实现PluginMessage和PluginStreamMessage
impl PluginMessage for AsyncExamplePlugin {
    fn send_message_to_frontend_typed(&self, content: &str, message_type: plugin_interface::MessageType) -> bool {
        // 实现消息发送逻辑
        println!("发送消息: {} (类型: {:?})", content, message_type);
        true
    }
}

impl PluginStreamMessage for AsyncExamplePlugin {
    fn send_message_stream_start(&self, stream_type: &str, metadata: Option<&str>) -> Result<String, plugin_interface::StreamError> {
        // 实现流式消息开始逻辑
        let stream_id = format!("async_stream_{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis());
        println!("开始异步流: {} (类型: {})", stream_id, stream_type);
        Ok(stream_id)
    }

    fn send_message_stream(&self, stream_id: &str, chunk: &str, is_final: bool) -> Result<(), plugin_interface::StreamError> {
        println!("异步流数据: {} -> {} (final: {})", stream_id, chunk, is_final);
        Ok(())
    }

    fn send_message_stream_end(&self, stream_id: &str, success: bool, error_msg: Option<&str>) -> Result<(), plugin_interface::StreamError> {
        println!("结束异步流: {} (成功: {})", stream_id, success);
        if let Some(error) = error_msg {
            println!("错误: {}", error);
        }
        Ok(())
    }

    fn send_message_stream_pause(&self, stream_id: &str) -> Result<(), plugin_interface::StreamError> {
        println!("暂停异步流: {}", stream_id);
        Ok(())
    }

    fn send_message_stream_resume(&self, stream_id: &str) -> Result<(), plugin_interface::StreamError> {
        println!("恢复异步流: {}", stream_id);
        Ok(())
    }

    fn send_message_stream_cancel(&self, stream_id: &str) -> Result<(), plugin_interface::StreamError> {
        println!("取消异步流: {}", stream_id);
        Ok(())
    }

    fn get_stream_status(&self, stream_id: &str) -> Option<plugin_interface::StreamStatus> {
        println!("获取异步流状态: {}", stream_id);
        Some(plugin_interface::StreamStatus::Active)
    }

    fn list_active_streams(&self) -> Vec<String> {
        vec![]
    }

    fn send_message_stream_batch(&self, stream_id: &str, chunks: &[&str]) -> Result<(), plugin_interface::StreamError> {
        println!("批量发送异步流数据: {} -> {:?}", stream_id, chunks);
        Ok(())
    }
}
