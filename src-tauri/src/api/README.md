# Tauri API 命令模块

这个文件夹包含了所有提供给前端的 Tauri 命令函数。

## 文件结构

```
src-tauri/src/api/
├── mod.rs            # 模块导出文件
├── general.rs        # 通用命令
├── plugins.rs        # 插件相关命令
└── README.md         # 说明文档
```

## 使用方法

### 1. 添加新的 API 命令

在对应的模块文件中添加新的命令函数：

```rust
#[tauri::command]
pub fn your_new_command(param: String) -> String {
    // 实现你的逻辑
    format!("Result: {}", param)
}
```

### 2. 导出新命令

在 `mod.rs` 中导出新命令：

```rust
pub use your_module::your_new_command;
```

### 3. 注册命令

在 `lib.rs` 中导入并注册命令：

```rust
use api::your_new_command;

// 在 invoke_handler 中添加
.invoke_handler(tauri::generate_handler![greet, scan_plugins, your_new_command])
```

## 现有命令

### 通用命令 (general.rs)
- `greet(name: &str) -> String` - 问候命令

### 插件命令 (plugins.rs)
- `scan_plugins() -> Vec<PluginMetadata>` - 扫描插件列表

## 注意事项

- 所有命令函数必须添加 `#[tauri::command]` 属性
- 命令函数必须是 `pub` 的
- 参数和返回值类型必须实现 `serde::Serialize` 和 `serde::Deserialize`
- 与前端的类型定义保持一致
