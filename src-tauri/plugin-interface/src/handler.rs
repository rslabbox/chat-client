use crate::callbacks::{HostCallbacks, set_host_callbacks};
use crate::metadata::PluginMetadata;
use crate::pluginui::{Context,Ui};

/// 插件处理器 trait
/// 定义了插件的生命周期方法
pub trait PluginHandler: Send + Sync {
    /// 插件初始化时调用（在挂载之前，用于设置回调函数）
    fn initialize(&self, callbacks: HostCallbacks) -> Result<(), Box<dyn std::error::Error>> {
        // 默认实现：设置全局回调函数
        set_host_callbacks(callbacks).map_err(|_| "Failed to set host callbacks")?;
        Ok(())
    }

    /// 更新UI（事件驱动）
    /// 当前端用户交互或需要更新UI时调用
    fn update_ui(&mut self, ctx: &Context, ui: &mut Ui);

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
