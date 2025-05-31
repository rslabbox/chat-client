# 聊天客户端

一个基于 Tauri、Vue 3 和 Rust 构建的现代化、可扩展的聊天客户端。具有强大的插件架构，支持动态加载和管理聊天插件。

[English Documentation](./README.md)

## 特性

- 🚀 **现代技术栈**：基于 Tauri、Vue 3、TypeScript 和 Rust 构建
- 🔌 **插件架构**：动态插件加载和管理系统
- 🎨 **响应式界面**：可调整大小的面板，使用 Element Plus 组件
- 💬 **消息系统**：实时消息显示和输入处理
- 🔄 **插件生命周期**：完整的插件生命周期管理（挂载、卸载、连接、断开连接）
- 📦 **跨平台**：支持 Windows、macOS 和 Linux 的桌面应用程序

## 架构

### 前端 (Vue 3 + TypeScript)
- **Vue 3** 使用 Composition API 和 `<script setup>` 语法
- **Element Plus** 提供 UI 组件
- **Pinia** 进行状态管理
- **Vue Router** 处理路由导航
- **Vite** 作为构建工具

### 后端 (Rust + Tauri)
- **Tauri** 框架构建桌面应用程序
- **插件接口** 库用于插件开发
- **动态加载** 使用 `libloading` crate
- **插件管理器** 处理生命周期管理

### 插件系统
- **PluginHandler Trait**：定义插件生命周期方法
- **动态加载**：运行时从 `.dll` 文件加载插件
- **双向通信**：插件可以发送/接收消息
- **配置管理**：每个插件都有自己的 `config.toml`

## 快速开始

### 前置要求

- [Node.js](https://nodejs.org/) (v18 或更高版本)
- [Rust](https://rustup.rs/) (最新稳定版)
- [pnpm](https://pnpm.io/) (推荐的包管理器)

### 安装

1. **克隆仓库**
   ```bash
   git clone <repository-url>
   cd chat-client
   ```

2. **安装依赖**
   ```bash
   pnpm install
   ```

3. **开发模式运行**
   ```bash
   pnpm tauri dev
   ```

4. **生产环境构建**
   ```bash
   pnpm tauri build
   ```

## 开发

### 项目结构

```
chat-client/
├── src/                    # Vue 前端
│   ├── components/         # Vue 组件
│   ├── views/             # Vue 视图
│   ├── stores/            # Pinia 状态管理
│   └── api/               # API 接口
├── src-tauri/             # Rust 后端
│   ├── src/
│   │   ├── api/           # Tauri 命令
│   │   ├── plugins/       # 插件管理
│   │   └── lib.rs         # 主库文件
│   ├── plugin-interface/  # 插件开发库
│   └── src/plugins/       # 插件实现
└── dist/                  # 构建输出
```

### 插件开发

1. **创建新插件**
   ```bash
   cd src-tauri/src/plugins
   mkdir my-plugin
   cd my-plugin
   ```

2. **添加 Cargo.toml**
   ```toml
   [package]
   name = "my-plugin"
   version = "0.1.0"
   edition = "2021"

   [lib]
   crate-type = ["cdylib"]

   [dependencies]
   plugin-interface = { path = "../../plugin-interface" }
   ```

3. **实现插件**
   ```rust
   use plugin_interface::{PluginHandler, PluginMetadata};

   pub struct MyPlugin;

   impl PluginHandler for MyPlugin {
       fn on_mount(&self) -> Result<(), Box<dyn std::error::Error>> {
           // 插件初始化
           Ok(())
       }

       fn handle_message(&self, message: &str) -> Result<String, Box<dyn std::error::Error>> {
           // 处理传入消息
           Ok(format!("响应: {}", message))
       }

       // 实现其他必需的方法...
   }
   ```

4. **添加配置文件 config.toml**
   ```toml
   [plugin]
   id = "my-plugin"
   name = "我的插件"
   description = "这是一个示例插件"
   version = "1.0.0"
   author = "您的名字"
   disabled = false
   ```

5. **导出插件函数**
   ```rust
   #[no_mangle]
   pub extern "C" fn create_plugin() -> *mut dyn PluginHandler {
       let plugin = MyPlugin::new();
       Box::into_raw(Box::new(plugin))
   }

   #[no_mangle]
   pub extern "C" fn destroy_plugin(plugin: *mut dyn PluginHandler) {
       if !plugin.is_null() {
           unsafe {
               let _ = Box::from_raw(plugin);
           }
       }
   }
   ```

### 可用脚本

- `pnpm dev` - 启动开发服务器
- `pnpm build` - 构建 Vue 前端
- `pnpm tauri dev` - 以开发模式运行 Tauri
- `pnpm tauri build` - 构建 Tauri 应用程序

### 插件生命周期

插件系统支持以下生命周期方法：

- **on_mount()**: 插件挂载时调用
- **on_dispose()**: 插件卸载时调用
- **on_connect()**: 连接时调用
- **on_disconnect()**: 断开连接时调用
- **handle_message()**: 处理消息时调用

### UI 组件

应用程序包含以下主要组件：

- **ConfigPanel**: 左侧配置面板，用于插件管理
- **MessageDisplay**: 消息显示区域
- **MessageInput**: 消息输入区域

界面支持拖拽调整面板大小，提供良好的用户体验。

## 推荐的 IDE 设置

- [VS Code](https://code.visualstudio.com/) + [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## 贡献

1. Fork 这个仓库
2. 创建您的功能分支 (`git checkout -b feature/amazing-feature`)
3. 提交您的更改 (`git commit -m 'Add some amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 打开一个 Pull Request

## 许可证

本项目基于 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 致谢

- [Tauri](https://tauri.app/) - 提供了出色的桌面应用框架
- [Vue.js](https://vuejs.org/) - 提供了响应式前端框架
- [Element Plus](https://element-plus.org/) - 提供了美观的 UI 组件

## 常见问题

### 如何添加新的插件？

1. 在 `src-tauri/src/plugins/` 目录下创建新的插件文件夹
2. 实现 `PluginHandler` trait
3. 添加 `config.toml` 配置文件
4. 编译插件为 `.dll` 文件
5. 在应用程序中扫描并加载插件

### 如何调试插件？

使用 `src-tauri/src/bin/test_plugins.rs` 进行插件测试：

```bash
cd src-tauri
cargo run --bin test_plugins
```
