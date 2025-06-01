use std::ffi::c_char;
use std::sync::OnceLock;

/// 主程序提供给插件的回调函数集合
/// 这些函数指针在插件加载时由主程序传递给插件
#[repr(C)]
pub struct HostCallbacks {

    /// 向前端发送消息
    pub send_to_frontend: extern "C" fn(*const c_char, *const c_char) -> bool,

    /// 获取应用配置
    pub get_app_config: extern "C" fn(*const c_char) -> *const c_char,

    /// 调用其他插件
    pub call_other_plugin: extern "C" fn(*const c_char, *const c_char) -> *const c_char,
}

/// 全局回调函数存储
static HOST_CALLBACKS: OnceLock<HostCallbacks> = OnceLock::new();

/// 设置主程序回调函数（由主程序调用）
pub fn set_host_callbacks(callbacks: HostCallbacks) -> Result<(), HostCallbacks> {
    HOST_CALLBACKS.set(callbacks)
}

/// 获取主程序回调函数（由插件调用）
pub fn get_host_callbacks() -> Option<&'static HostCallbacks> {
    HOST_CALLBACKS.get()
}
