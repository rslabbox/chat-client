use serde::{Deserialize, Serialize};

/// 插件元数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    pub id: String,
    pub disabled: bool,
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: Option<String>,
    pub library_path: Option<String>, // 动态库文件路径
    pub config_path: String,          // 配置文件路径
}

/// 插件处理器 trait
/// 定义了插件的生命周期方法
pub trait PluginHandler: Send + Sync {
    /// 插件挂载时调用
    fn on_mount(&self) -> Result<(), Box<dyn std::error::Error>>;
    
    /// 插件卸载时调用
    fn on_dispose(&self) -> Result<(), Box<dyn std::error::Error>>;
    
    /// 连接时调用
    fn on_connect(&self) -> Result<(), Box<dyn std::error::Error>>;
    
    /// 断开连接时调用
    fn on_disconnect(&self) -> Result<(), Box<dyn std::error::Error>>;
    
    /// 获取插件元数据
    fn get_metadata(&self) -> PluginMetadata;
}

/// 插件创建函数类型
/// 用于从动态库中创建插件实例
pub type CreatePluginFn = unsafe extern "C" fn() -> *mut dyn PluginHandler;

/// 插件销毁函数类型
/// 用于销毁插件实例
pub type DestroyPluginFn = unsafe extern "C" fn(*mut dyn PluginHandler);

/// 插件导出符号名称
pub const CREATE_PLUGIN_SYMBOL: &[u8] = b"create_plugin";
pub const DESTROY_PLUGIN_SYMBOL: &[u8] = b"destroy_plugin";
