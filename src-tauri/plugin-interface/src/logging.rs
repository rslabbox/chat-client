/// 插件日志宏
/// 提供方便的日志记录宏，自动通过回调函数发送到主程序的日志系统

/// 插件信息日志宏
/// 使用方式: plugin_info!("消息内容")
/// 或: plugin_info!("格式化消息: {}", value)
#[macro_export]
macro_rules! plugin_info {
    ($($arg:tt)*) => {
        $crate::log_info(&format!($($arg)*))
    };
}

/// 插件警告日志宏
/// 使用方式: plugin_warn!("警告消息")
/// 或: plugin_warn!("格式化警告: {}", value)
#[macro_export]
macro_rules! plugin_warn {
    ($($arg:tt)*) => {
        $crate::log_warn(&format!($($arg)*))
    };
}

/// 插件错误日志宏
/// 使用方式: plugin_error!("错误消息")
/// 或: plugin_error!("格式化错误: {}", value)
#[macro_export]
macro_rules! plugin_error {
    ($($arg:tt)*) => {
        $crate::log_error(&format!($($arg)*))
    };
}

/// 插件调试日志宏（如果需要的话）
/// 注意：需要在 api.rs 中添加对应的 log_debug 函数
#[macro_export]
macro_rules! plugin_debug {
    ($($arg:tt)*) => {
        // 暂时使用 info 级别，后续可以添加 debug 级别支持
        $crate::log_info(&format!("[DEBUG] {}", format!($($arg)*)))
    };
}
