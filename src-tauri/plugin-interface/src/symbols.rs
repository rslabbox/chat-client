use crate::handler::PluginHandler;

/// 插件创建函数类型
/// 用于从动态库中创建插件实例
pub type CreatePluginFn = unsafe extern "C" fn() -> *mut dyn PluginHandler;

/// 插件销毁函数类型
/// 用于销毁插件实例
pub type DestroyPluginFn = unsafe extern "C" fn(*mut dyn PluginHandler);

/// 插件导出符号名称
pub const CREATE_PLUGIN_SYMBOL: &[u8] = b"create_plugin";
pub const DESTROY_PLUGIN_SYMBOL: &[u8] = b"destroy_plugin";
