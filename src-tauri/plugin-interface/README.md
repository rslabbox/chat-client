# Plugin Interface 日志宏使用指南

## 概述

`plugin-interface` 库提供了方便的日志宏，让插件开发者可以轻松地将日志信息发送到主程序的日志系统。

## 可用的日志宏

### 1. `plugin_info!` - 信息日志
用于记录一般信息。

```rust
use plugin_interface::PluginHandler;

impl PluginHandler for MyPlugin {
    fn on_mount(&self) -> Result<(), Box<dyn std::error::Error>> {
        // 简单消息
        plugin_interface::plugin_info!("插件已挂载");
        
        // 格式化消息
        plugin_interface::plugin_info!("插件 {} 版本 {} 已挂载", self.name, self.version);
        
        Ok(())
    }
}
```

### 2. `plugin_warn!` - 警告日志
用于记录警告信息。

```rust
impl PluginHandler for MyPlugin {
    fn on_disconnect(&self) -> Result<(), Box<dyn std::error::Error>> {
        plugin_interface::plugin_warn!("连接已断开，正在尝试重连...");
        Ok(())
    }
}
```

### 3. `plugin_error!` - 错误日志
用于记录错误信息。

```rust
impl PluginHandler for MyPlugin {
    fn handle_message(&self, message: &str) -> Result<String, Box<dyn std::error::Error>> {
        if message.is_empty() {
            plugin_interface::plugin_error!("收到空消息");
            return Err("消息不能为空".into());
        }
        
        Ok("处理成功".to_string())
    }
}
```

### 4. `plugin_debug!` - 调试日志
用于记录调试信息（当前使用 INFO 级别显示）。

```rust
impl PluginHandler for MyPlugin {
    fn handle_message(&self, message: &str) -> Result<String, Box<dyn std::error::Error>> {
        plugin_interface::plugin_debug!("处理消息: {}", message);
        
        // 处理逻辑...
        
        Ok("处理完成".to_string())
    }
}
```

## 日志格式

所有日志都会按照主程序配置的格式显示：
```
[时:分:秒 文件名:行数] 级别 消息内容
```

例如：
```
[14:30:25 lib.rs:20] INFO [Example Plugin] Plugin mounted successfully
[14:30:26 lib.rs:41] WARN [Example Plugin] Disconnected
[14:30:27 lib.rs:47] ERROR [Example Plugin] Failed to process message
```

## 与旧版本的兼容性

如果您之前使用的是函数形式的日志记录：

```rust
// 旧方式
use plugin_interface::{log_info, log_warn, log_error};

log_info(&format!("消息: {}", value));
log_warn(&format!("警告: {}", value));
log_error(&format!("错误: {}", value));
```

可以迁移到新的宏形式：

```rust
// 新方式 - 更简洁
plugin_interface::plugin_info!("消息: {}", value);
plugin_interface::plugin_warn!("警告: {}", value);
plugin_interface::plugin_error!("错误: {}", value);
```

## 注意事项

1. **宏调用方式**：需要使用 `plugin_interface::plugin_info!()` 的完整路径形式
2. **格式化支持**：所有宏都支持 `format!` 宏的语法
3. **性能**：宏会在编译时展开，性能与直接调用函数相同
4. **错误处理**：如果主程序回调函数不可用，日志调用会静默失败

## 完整示例

```rust
use plugin_interface::{PluginHandler, PluginMetadata};

pub struct MyPlugin {
    name: String,
}

impl PluginHandler for MyPlugin {
    fn on_mount(&self) -> Result<(), Box<dyn std::error::Error>> {
        plugin_interface::plugin_info!("[{}] 插件挂载开始", self.name);
        
        // 初始化逻辑...
        
        plugin_interface::plugin_info!("[{}] 插件挂载完成", self.name);
        Ok(())
    }
    
    fn on_dispose(&self) -> Result<(), Box<dyn std::error::Error>> {
        plugin_interface::plugin_info!("[{}] 插件正在卸载", self.name);
        Ok(())
    }
    
    fn handle_message(&self, message: &str) -> Result<String, Box<dyn std::error::Error>> {
        plugin_interface::plugin_debug!("[{}] 收到消息: {}", self.name, message);
        
        if message.trim().is_empty() {
            plugin_interface::plugin_error!("[{}] 消息为空", self.name);
            return Err("消息不能为空".into());
        }
        
        let response = format!("Echo: {}", message);
        plugin_interface::plugin_info!("[{}] 发送响应: {}", self.name, response);
        
        Ok(response)
    }
    
    // 其他必需的方法...
}
```
