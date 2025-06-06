use std::ffi::CString;
use crate::callbacks::get_host_callbacks;

/// 向前端发送消息（供插件使用）
/// 内部调用 host_send_to_frontend 函数
pub fn send_to_frontend(event: &str, payload: &str) -> bool {
    host_send_to_frontend(event, payload)
}

/// 调用主程序的 host_send_to_frontend 函数
/// 这是实际执行向前端发送消息的函数
pub fn host_send_to_frontend(event: &str, payload: &str) -> bool {
    if let Some(callbacks) = get_host_callbacks() {
        if let (Ok(event_str), Ok(payload_str)) = (CString::new(event), CString::new(payload)) {
            // 调用主程序提供的 host_send_to_frontend 函数
            return (callbacks.send_to_frontend)(event_str.as_ptr(), payload_str.as_ptr());
        }
    }
    false
}

/// 获取应用配置（供插件使用）
pub fn get_app_config(key: &str) -> Option<String> {
    if let Some(callbacks) = get_host_callbacks() {
        if let Ok(key_str) = CString::new(key) {
            let result_ptr = (callbacks.get_app_config)(key_str.as_ptr());
            if !result_ptr.is_null() {
                unsafe {
                    let c_str = std::ffi::CStr::from_ptr(result_ptr);
                    return c_str.to_str().ok().map(|s| s.to_string());
                }
            }
        }
    }
    None
}

/// 调用其他插件（供插件使用）
pub fn call_other_plugin(plugin_id: &str, message: &str) -> Option<String> {
    if let Some(callbacks) = get_host_callbacks() {
        if let (Ok(id_str), Ok(msg_str)) = (CString::new(plugin_id), CString::new(message)) {
            let result_ptr = (callbacks.call_other_plugin)(id_str.as_ptr(), msg_str.as_ptr());
            if !result_ptr.is_null() {
                unsafe {
                    let c_str = std::ffi::CStr::from_ptr(result_ptr);
                    return c_str.to_str().ok().map(|s| s.to_string());
                }
            }
        }
    }
    None
}
