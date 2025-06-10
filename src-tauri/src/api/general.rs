/// 通用的问候命令
#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/// 测试多实例插件挂载
#[tauri::command]
pub fn test_multi_instance(app_handle: tauri::AppHandle) -> String {
    crate::test_multi_instance::test_multi_instance_mounting(app_handle);
    "多实例测试完成".to_string()
}
