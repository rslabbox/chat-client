use chat_client_lib::plugins::PluginManager;

fn main() {
    println!("Testing plugin system...");

    let manager = PluginManager::new();
    let plugins = manager.scan_plugins();

    println!("Found {} plugins:", plugins.len());
    for plugin in &plugins {
        println!("  - ID: {}", plugin.id);
        println!("    Name: {}", plugin.name);
        println!("    Version: {}", plugin.version);
        println!("    Description: {}", plugin.description);
        if let Some(author) = &plugin.author {
            println!("    Author: {}", author);
        }
        if let Some(library_path) = &plugin.library_path {
            println!("    Library: {}", library_path);
        }
        println!("    Config: {}", plugin.config_path);
        println!("    Disabled: {}", plugin.disabled);
        println!();
    }

    // 测试插件生命周期 - 使用有动态库的插件
    if let Some(plugin) = plugins.iter().find(|p| p.library_path.is_some()) {
        println!("Testing plugin lifecycle with: {}", plugin.name);

        // 测试挂载
        println!("\n1. Testing mount...");
        match manager.mount_plugin(&plugin.id) {
            Ok(msg) => println!("✓ Mount success: {}", msg),
            Err(err) => println!("✗ Mount failed: {}", err),
        }

        // 测试连接
        println!("\n2. Testing connect...");
        match manager.connect_plugin(&plugin.id) {
            Ok(msg) => println!("✓ Connect success: {}", msg),
            Err(err) => println!("✗ Connect failed: {}", err),
        }

        // 测试断开连接
        println!("\n3. Testing disconnect...");
        match manager.disconnect_plugin(&plugin.id) {
            Ok(msg) => println!("✓ Disconnect success: {}", msg),
            Err(err) => println!("✗ Disconnect failed: {}", err),
        }

        // 测试卸载
        println!("\n4. Testing dispose...");
        match manager.dispose_plugin(&plugin.id) {
            Ok(msg) => println!("✓ Dispose success: {}", msg),
            Err(err) => println!("✗ Dispose failed: {}", err),
        }
    }
}
