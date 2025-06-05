use crate::callbacks::{HostCallbacks, set_host_callbacks};
use crate::metadata::PluginMetadata;
use crate::pluginui::{Context,Ui};
use std::future::Future;
use std::pin::Pin;


/// 插件处理器 trait
/// 定义了插件的生命周期方法
pub trait PluginHandler: Send + Sync {
    /// 插件初始化时调用（在挂载之前，用于设置回调函数）
    fn initialize(&self, callbacks: HostCallbacks) -> Result<(), Box<dyn std::error::Error>> {
        // 默认实现：设置全局回调函数
        set_host_callbacks(callbacks).map_err(|_| "Failed to set host callbacks")?;
        Ok(())
    }

    /// 更新UI（异步版本）
    /// 当前端用户交互或需要更新UI时调用
    fn update_ui_async<'a>(&'a mut self, ctx: &'a Context, ui: &'a mut Ui) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>>;

    /// 插件挂载时调用
    fn on_mount(&mut self, metadata: &PluginMetadata) -> Result<(), Box<dyn std::error::Error>>;

    /// 插件卸载时调用
    fn on_dispose(&self) -> Result<(), Box<dyn std::error::Error>>;

    /// 连接时调用
    fn on_connect(&self) -> Result<(), Box<dyn std::error::Error>>;

    /// 断开连接时调用
    fn on_disconnect(&self) -> Result<(), Box<dyn std::error::Error>>;

    /// 处理消息
    fn handle_message(&self, message: &str) -> Result<String, Box<dyn std::error::Error>>;

    /// 获取插件元数据
    fn get_metadata(&self) -> PluginMetadata;
}

/// 异步插件处理器 trait
/// 提供完全异步的插件生命周期管理
pub trait AsyncPluginHandler: Send + Sync {
    /// 插件初始化时调用（异步版本）
    fn initialize_async(&self, callbacks: HostCallbacks) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn std::error::Error>>> + Send + '_>> {
        Box::pin(async move {
            set_host_callbacks(callbacks).map_err(|_| "Failed to set host callbacks")?;
            Ok(())
        })
    }

    /// 更新UI（异步版本）
    fn update_ui_async(&mut self, ctx: &Context, ui: &mut Ui) -> Pin<Box<dyn Future<Output = ()> + Send + '_>>;

    /// 插件挂载时调用（异步版本）
    fn on_mount_async(&mut self, metadata: &PluginMetadata) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn std::error::Error>>> + Send + '_>>;

    /// 插件卸载时调用（异步版本）
    fn on_dispose_async(&self) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn std::error::Error>>> + Send + '_>>;

    /// 连接时调用（异步版本）
    fn on_connect_async(&self) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn std::error::Error>>> + Send + '_>>;

    /// 断开连接时调用（异步版本）
    fn on_disconnect_async(&self) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn std::error::Error>>> + Send + '_>>;

    /// 处理消息（异步版本）
    fn handle_message_async(&self, message: &str) -> Pin<Box<dyn Future<Output = Result<String, Box<dyn std::error::Error>>> + Send + '_>>;

    /// 获取插件元数据
    fn get_metadata(&self) -> PluginMetadata;

    /// 启动插件的异步运行时
    fn start_runtime(&self) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn std::error::Error>>> + Send + '_>> {
        Box::pin(async move {
            // 默认实现：什么都不做
            Ok(())
        })
    }

    /// 停止插件的异步运行时
    fn stop_runtime(&self) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn std::error::Error>>> + Send + '_>> {
        Box::pin(async move {
            // 默认实现：什么都不做
            Ok(())
        })
    }
}
