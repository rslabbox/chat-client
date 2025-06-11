// 测试多实例插件挂载功能

use crate::plugins::manager::PluginManager;

/// 测试多实例插件挂载
/// 注意：这个函数需要在有AppHandle的上下文中调用
pub fn test_multi_instance_mounting(app_handle: tauri::AppHandle) {
    println!("🧪 开始测试多实例插件挂载...");

    let plugin_manager = PluginManager::new(app_handle);

    // 测试1: 挂载同一个插件的第一个实例
    println!("\n📦 测试1: 挂载第一个实例");
    let instance1_result =
        plugin_manager.mount_plugin("example-plugin", Some("instance-1".to_string()));

    match instance1_result {
        Ok(instance_id) => {
            println!("✅ 第一个实例挂载成功: {}", instance_id);
        }
        Err(e) => {
            println!("❌ 第一个实例挂载失败: {}", e);
            return;
        }
    }

    // 测试2: 挂载同一个插件的第二个实例
    println!("\n📦 测试2: 挂载第二个实例");
    let instance2_result =
        plugin_manager.mount_plugin("example-plugin", Some("instance-2".to_string()));

    match instance2_result {
        Ok(instance_id) => {
            println!("✅ 第二个实例挂载成功: {}", instance_id);
            println!("🎉 多实例挂载测试通过！");
        }
        Err(e) => {
            println!("❌ 第二个实例挂载失败: {}", e);
            println!("这证实了之前的问题：同一个插件无法挂载多次");
            return;
        }
    }

    // 测试3: 验证实例隔离
    println!("\n🔍 测试3: 验证实例隔离");
    test_instance_isolation(&plugin_manager);

    // 测试4: 测试实例独立的消息发送
    println!("\n📨 测试4: 测试实例独立的消息发送");
    test_independent_messaging(&plugin_manager);

    println!("\n🎯 多实例测试完成！");
}

/// 测试实例隔离
fn test_instance_isolation(plugin_manager: &PluginManager) {
    println!("验证两个实例是否有独立的状态...");

    // 向第一个实例发送消息
    let response1 = plugin_manager.send_message_to_plugin_instance(
        "example-plugin",
        "instance-1",
        "Hello from test 1",
    );
    match response1 {
        Ok(resp) => println!("实例1响应: {}", resp),
        Err(e) => println!("实例1错误: {}", e),
    }

    // 向第二个实例发送消息
    let response2 = plugin_manager.send_message_to_plugin_instance(
        "example-plugin",
        "instance-2",
        "Hello from test 2",
    );
    match response2 {
        Ok(resp) => println!("实例2响应: {}", resp),
        Err(e) => println!("实例2错误: {}", e),
    }

    // 再次向第一个实例发送消息，验证状态独立性
    let response1_again = plugin_manager.send_message_to_plugin_instance(
        "example-plugin",
        "instance-1",
        "Second message",
    );
    match response1_again {
        Ok(resp) => println!("实例1第二次响应: {}", resp),
        Err(e) => println!("实例1第二次错误: {}", e),
    }
}

/// 测试独立的消息发送
fn test_independent_messaging(plugin_manager: &PluginManager) {
    println!("测试每个实例是否可以独立发送消息到前端...");

    // 发送触发消息发送的指令
    let trigger1 = plugin_manager.send_message_to_plugin_instance(
        "example-plugin",
        "instance-1",
        "trigger_frontend_message",
    );
    match trigger1 {
        Ok(resp) => println!("实例1触发消息发送: {}", resp),
        Err(e) => println!("实例1触发失败: {}", e),
    }

    let trigger2 = plugin_manager.send_message_to_plugin_instance(
        "example-plugin",
        "instance-2",
        "trigger_frontend_message",
    );
    match trigger2 {
        Ok(resp) => println!("实例2触发消息发送: {}", resp),
        Err(e) => println!("实例2触发失败: {}", e),
    }
}
