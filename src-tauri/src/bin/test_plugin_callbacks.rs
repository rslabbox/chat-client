use chat_client_lib::plugins::PluginManager;

fn main() {
    println!("=== 测试插件回调函数功能 ===");
    
    // 创建插件管理器
    let manager = PluginManager::new();
    
    // 扫描插件
    println!("\n1. 扫描插件...");
    let plugins = manager.scan_plugins();
    for plugin in &plugins {
        println!("  发现插件: {} ({})", plugin.name, plugin.id);
        if let Some(lib_path) = &plugin.library_path {
            println!("    库文件: {}", lib_path);
        }
    }
    
    // 查找示例插件
    let example_plugin = plugins.iter().find(|p| p.id == "example_plugin");
    
    if let Some(plugin) = example_plugin {
        println!("\n2. 测试示例插件...");
        
        // 挂载插件
        println!("  挂载插件...");
        match manager.mount_plugin(&plugin.id) {
            Ok(msg) => println!("  ✓ {}", msg),
            Err(e) => {
                println!("  ✗ 挂载失败: {}", e);
                return;
            }
        }
        
        // 连接插件
        println!("  连接插件...");
        match manager.connect_plugin(&plugin.id) {
            Ok(msg) => println!("  ✓ {}", msg),
            Err(e) => println!("  ✗ 连接失败: {}", e),
        }
        
        // 发送消息测试
        println!("  发送测试消息...");
        match manager.send_message_to_current_plugin("Hello from host!") {
            Ok(response) => println!("  ✓ 插件响应: {}", response),
            Err(e) => println!("  ✗ 消息发送失败: {}", e),
        }
        
        // 断开连接
        println!("  断开插件连接...");
        match manager.disconnect_plugin(&plugin.id) {
            Ok(msg) => println!("  ✓ {}", msg),
            Err(e) => println!("  ✗ 断开失败: {}", e),
        }
        
        // 卸载插件
        println!("  卸载插件...");
        match manager.dispose_plugin(&plugin.id) {
            Ok(msg) => println!("  ✓ {}", msg),
            Err(e) => println!("  ✗ 卸载失败: {}", e),
        }
        
    } else {
        println!("\n❌ 未找到示例插件 (example_plugin)");
        println!("请确保已编译示例插件: cargo build --package example-plugin --release");
    }
    
    println!("\n=== 测试完成 ===");
}
