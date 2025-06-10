// 示例：实例级别状态管理 - 上下文传递模式

use std::collections::HashMap;

/// 插件实例上下文 - 包含单个插件实例的所有状态
#[derive(Debug, Clone)]
pub struct PluginInstanceContext {
    pub instance_id: String,
    pub plugin_id: String,
    pub metadata: PluginMetadata,
    pub callbacks: Option<HostCallbacks>,
    pub config: HashMap<String, String>,
}

impl PluginInstanceContext {
    pub fn new(instance_id: String, plugin_id: String, metadata: PluginMetadata) -> Self {
        Self {
            instance_id,
            plugin_id,
            metadata,
            callbacks: None,
            config: HashMap::new(),
        }
    }

    /// 向前端发送消息 - 使用实例自己的回调函数
    pub fn send_to_frontend(&self, event: &str, payload: &str) -> bool {
        if let Some(callbacks) = &self.callbacks {
            // 调用实例专属的回调函数
            return (callbacks.send_to_frontend)(event, payload);
        }
        false
    }

    /// 获取配置 - 使用实例自己的配置
    pub fn get_config(&self, key: &str) -> Option<&String> {
        self.config.get(key)
    }
}

/// 插件处理器 trait - 所有方法都接收上下文参数
pub trait PluginHandler: Send + Sync {
    /// 初始化 - 创建并返回插件实例上下文
    fn initialize(
        &mut self,
        callbacks: HostCallbacks,
        metadata: PluginMetadata,
    ) -> Result<PluginInstanceContext, Box<dyn std::error::Error>> {
        let instance_id = metadata.instance_id.clone().unwrap_or_default();
        let plugin_id = metadata.id.clone();
        
        let mut context = PluginInstanceContext::new(instance_id, plugin_id, metadata);
        context.callbacks = Some(callbacks);
        
        Ok(context)
    }

    /// 挂载 - 传入上下文
    fn on_mount(&mut self, context: &PluginInstanceContext) -> Result<(), Box<dyn std::error::Error>>;

    /// 更新UI - 传入上下文
    fn update_ui(&mut self, ui_context: &UIContext, ui: &mut UI, plugin_context: &PluginInstanceContext);

    /// 处理消息 - 传入上下文
    fn handle_message(&mut self, message: &str, context: &PluginInstanceContext) -> Result<String, Box<dyn std::error::Error>>;
}

/// 插件管理器 - 管理多个插件实例
pub struct PluginManager {
    /// 存储每个实例的上下文
    instances: HashMap<String, PluginInstanceContext>,
    /// 存储每个实例的处理器
    handlers: HashMap<String, Box<dyn PluginHandler>>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            instances: HashMap::new(),
            handlers: HashMap::new(),
        }
    }

    /// 挂载插件实例
    pub fn mount_plugin(
        &mut self,
        plugin_id: &str,
        instance_id: Option<String>,
        handler: Box<dyn PluginHandler>,
    ) -> Result<String, String> {
        let instance_id = instance_id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        // 检查实例是否已存在
        if self.instances.contains_key(&instance_id) {
            return Err(format!("Instance {} already exists", instance_id));
        }

        // 创建元数据
        let metadata = PluginMetadata {
            id: plugin_id.to_string(),
            instance_id: Some(instance_id.clone()),
            name: format!("Plugin {}", plugin_id),
            // ... 其他字段
        };

        // 创建回调函数
        let callbacks = HostCallbacks {
            send_to_frontend: |event, payload| {
                println!("Sending to frontend: {} -> {}", event, payload);
                true
            },
            // ... 其他回调
        };

        // 初始化插件，获取上下文
        let mut plugin_handler = handler;
        let context = plugin_handler.initialize(callbacks, metadata)
            .map_err(|e| format!("Failed to initialize plugin: {}", e))?;

        // 调用挂载方法，传入上下文
        plugin_handler.on_mount(&context)
            .map_err(|e| format!("Failed to mount plugin: {}", e))?;

        // 存储实例和处理器
        self.instances.insert(instance_id.clone(), context);
        self.handlers.insert(instance_id.clone(), plugin_handler);

        Ok(instance_id)
    }

    /// 向插件实例发送消息
    pub fn send_message(&mut self, instance_id: &str, message: &str) -> Result<String, String> {
        // 获取实例上下文
        let context = self.instances.get(instance_id)
            .ok_or_else(|| format!("Instance {} not found", instance_id))?;

        // 获取处理器
        let handler = self.handlers.get_mut(instance_id)
            .ok_or_else(|| format!("Handler for instance {} not found", instance_id))?;

        // 调用处理器方法，传入上下文
        handler.handle_message(message, context)
            .map_err(|e| format!("Failed to handle message: {}", e))
    }

    /// 卸载插件实例
    pub fn dispose_plugin(&mut self, instance_id: &str) -> Result<(), String> {
        // 移除实例和处理器
        self.instances.remove(instance_id);
        self.handlers.remove(instance_id);
        Ok(())
    }
}

// 示例插件实现
pub struct ExamplePlugin {
    counter: i32,
}

impl ExamplePlugin {
    pub fn new() -> Self {
        Self { counter: 0 }
    }
}

impl PluginHandler for ExamplePlugin {
    fn on_mount(&mut self, context: &PluginInstanceContext) -> Result<(), Box<dyn std::error::Error>> {
        println!("Plugin {} (instance {}) mounted", context.plugin_id, context.instance_id);
        
        // 使用实例上下文发送消息
        context.send_to_frontend("plugin-mounted", &format!("{{\"instance\": \"{}\"}}", context.instance_id));
        
        Ok(())
    }

    fn update_ui(&mut self, _ui_context: &UIContext, _ui: &mut UI, context: &PluginInstanceContext) {
        println!("Updating UI for instance {}", context.instance_id);
        // UI更新逻辑，可以访问实例特定的状态
    }

    fn handle_message(&mut self, message: &str, context: &PluginInstanceContext) -> Result<String, Box<dyn std::error::Error>> {
        self.counter += 1;
        
        // 使用实例上下文
        let response = format!(
            "Instance {} (plugin {}) received: {} (count: {})",
            context.instance_id,
            context.plugin_id,
            message,
            self.counter
        );

        // 向前端发送响应
        context.send_to_frontend("plugin-response", &response);

        Ok(response)
    }
}

// 使用示例
fn main() {
    let mut manager = PluginManager::new();

    // 挂载同一个插件的两个实例
    let instance1 = manager.mount_plugin(
        "example-plugin",
        Some("instance-1".to_string()),
        Box::new(ExamplePlugin::new()),
    ).unwrap();

    let instance2 = manager.mount_plugin(
        "example-plugin", 
        Some("instance-2".to_string()),
        Box::new(ExamplePlugin::new()),
    ).unwrap();

    // 向不同实例发送消息 - 它们有独立的状态
    println!("Response 1: {}", manager.send_message(&instance1, "Hello from instance 1").unwrap());
    println!("Response 2: {}", manager.send_message(&instance2, "Hello from instance 2").unwrap());
    println!("Response 1 again: {}", manager.send_message(&instance1, "Second message").unwrap());

    // 输出：
    // Instance instance-1 (plugin example-plugin) received: Hello from instance 1 (count: 1)
    // Instance instance-2 (plugin example-plugin) received: Hello from instance 2 (count: 1)  
    // Instance instance-1 (plugin example-plugin) received: Second message (count: 2)
}

// 辅助类型定义
#[derive(Debug, Clone)]
pub struct PluginMetadata {
    pub id: String,
    pub instance_id: Option<String>,
    pub name: String,
}

#[derive(Clone)]
pub struct HostCallbacks {
    pub send_to_frontend: fn(&str, &str) -> bool,
}

pub struct UIContext;
pub struct UI;
