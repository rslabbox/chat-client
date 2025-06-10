// 示例：实例级别状态管理 - 实例管理器模式

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// 插件实例数据
#[derive(Debug, Clone)]
pub struct PluginInstanceData {
    pub instance_id: String,
    pub plugin_id: String,
    pub metadata: PluginMetadata,
    pub callbacks: HostCallbacks,
    pub state: HashMap<String, String>, // 实例特定的状态
}

/// 实例管理器 - 管理所有插件实例的状态
pub struct InstanceManager {
    instances: Arc<Mutex<HashMap<String, PluginInstanceData>>>,
}

impl InstanceManager {
    pub fn new() -> Self {
        Self {
            instances: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// 注册新的插件实例
    pub fn register_instance(&self, instance_data: PluginInstanceData) -> Result<(), String> {
        let mut instances = self.instances.lock()
            .map_err(|_| "Failed to lock instances")?;
        
        if instances.contains_key(&instance_data.instance_id) {
            return Err(format!("Instance {} already exists", instance_data.instance_id));
        }

        instances.insert(instance_data.instance_id.clone(), instance_data);
        Ok(())
    }

    /// 获取实例数据
    pub fn get_instance(&self, instance_id: &str) -> Option<PluginInstanceData> {
        let instances = self.instances.lock().ok()?;
        instances.get(instance_id).cloned()
    }

    /// 更新实例状态
    pub fn update_instance_state(&self, instance_id: &str, key: &str, value: String) -> Result<(), String> {
        let mut instances = self.instances.lock()
            .map_err(|_| "Failed to lock instances")?;
        
        if let Some(instance) = instances.get_mut(instance_id) {
            instance.state.insert(key.to_string(), value);
            Ok(())
        } else {
            Err(format!("Instance {} not found", instance_id))
        }
    }

    /// 移除实例
    pub fn remove_instance(&self, instance_id: &str) -> bool {
        let mut instances = self.instances.lock().unwrap();
        instances.remove(instance_id).is_some()
    }

    /// 获取实例的回调函数
    pub fn get_callbacks(&self, instance_id: &str) -> Option<HostCallbacks> {
        let instances = self.instances.lock().ok()?;
        instances.get(instance_id).map(|instance| instance.callbacks.clone())
    }
}

/// 全局实例管理器
static INSTANCE_MANAGER: std::sync::OnceLock<InstanceManager> = std::sync::OnceLock::new();

/// 获取全局实例管理器
pub fn get_instance_manager() -> &'static InstanceManager {
    INSTANCE_MANAGER.get_or_init(|| InstanceManager::new())
}

/// 插件处理器 trait - 使用实例ID来访问状态
pub trait PluginHandler: Send + Sync {
    /// 初始化 - 注册实例到管理器
    fn initialize(
        &self,
        instance_id: String,
        callbacks: HostCallbacks,
        metadata: PluginMetadata,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let instance_data = PluginInstanceData {
            instance_id: instance_id.clone(),
            plugin_id: metadata.id.clone(),
            metadata,
            callbacks,
            state: HashMap::new(),
        };

        get_instance_manager().register_instance(instance_data)
            .map_err(|e| e.into())
    }

    /// 挂载 - 传入实例ID
    fn on_mount(&mut self, instance_id: &str) -> Result<(), Box<dyn std::error::Error>>;

    /// 处理消息 - 传入实例ID
    fn handle_message(&mut self, instance_id: &str, message: &str) -> Result<String, Box<dyn std::error::Error>>;

    /// 获取实例数据的辅助方法
    fn get_instance_data(&self, instance_id: &str) -> Option<PluginInstanceData> {
        get_instance_manager().get_instance(instance_id)
    }

    /// 发送消息到前端的辅助方法
    fn send_to_frontend(&self, instance_id: &str, event: &str, payload: &str) -> bool {
        if let Some(callbacks) = get_instance_manager().get_callbacks(instance_id) {
            return (callbacks.send_to_frontend)(event, payload);
        }
        false
    }

    /// 更新实例状态的辅助方法
    fn update_state(&self, instance_id: &str, key: &str, value: String) -> Result<(), String> {
        get_instance_manager().update_instance_state(instance_id, key, value)
    }
}

/// 示例插件实现
pub struct CounterPlugin {
    // 注意：这里不存储实例特定的状态，而是使用实例管理器
}

impl CounterPlugin {
    pub fn new() -> Self {
        Self {}
    }

    /// 获取计数器值
    fn get_counter(&self, instance_id: &str) -> i32 {
        if let Some(instance) = self.get_instance_data(instance_id) {
            instance.state.get("counter")
                .and_then(|s| s.parse().ok())
                .unwrap_or(0)
        } else {
            0
        }
    }

    /// 增加计数器
    fn increment_counter(&self, instance_id: &str) -> Result<i32, String> {
        let current = self.get_counter(instance_id);
        let new_value = current + 1;
        self.update_state(instance_id, "counter", new_value.to_string())?;
        Ok(new_value)
    }
}

impl PluginHandler for CounterPlugin {
    fn on_mount(&mut self, instance_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("Counter plugin mounted for instance: {}", instance_id);
        
        // 初始化计数器
        self.update_state(instance_id, "counter", "0".to_string())?;
        
        // 发送挂载事件
        self.send_to_frontend(instance_id, "plugin-mounted", 
            &format!("{{\"instance\": \"{}\"}}", instance_id));
        
        Ok(())
    }

    fn handle_message(&mut self, instance_id: &str, message: &str) -> Result<String, Box<dyn std::error::Error>> {
        match message {
            "increment" => {
                let new_count = self.increment_counter(instance_id)?;
                let response = format!("Counter for instance {} is now: {}", instance_id, new_count);
                
                // 发送更新事件
                self.send_to_frontend(instance_id, "counter-updated", 
                    &format!("{{\"instance\": \"{}\", \"count\": {}}}", instance_id, new_count));
                
                Ok(response)
            }
            "get" => {
                let count = self.get_counter(instance_id);
                Ok(format!("Counter for instance {} is: {}", instance_id, count))
            }
            _ => Ok(format!("Unknown command: {}", message))
        }
    }
}

/// 插件管理器
pub struct PluginManager {
    handlers: HashMap<String, Box<dyn PluginHandler>>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    /// 挂载插件
    pub fn mount_plugin(
        &mut self,
        plugin_id: &str,
        instance_id: Option<String>,
        mut handler: Box<dyn PluginHandler>,
    ) -> Result<String, String> {
        let instance_id = instance_id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        // 创建元数据和回调
        let metadata = PluginMetadata {
            id: plugin_id.to_string(),
            name: format!("Plugin {}", plugin_id),
        };

        let callbacks = HostCallbacks {
            send_to_frontend: |event, payload| {
                println!("Frontend event: {} -> {}", event, payload);
                true
            },
        };

        // 初始化插件
        handler.initialize(instance_id.clone(), callbacks, metadata)
            .map_err(|e| format!("Failed to initialize: {}", e))?;

        // 挂载插件
        handler.on_mount(&instance_id)
            .map_err(|e| format!("Failed to mount: {}", e))?;

        // 存储处理器
        self.handlers.insert(instance_id.clone(), handler);

        Ok(instance_id)
    }

    /// 发送消息
    pub fn send_message(&mut self, instance_id: &str, message: &str) -> Result<String, String> {
        let handler = self.handlers.get_mut(instance_id)
            .ok_or_else(|| format!("Instance {} not found", instance_id))?;

        handler.handle_message(instance_id, message)
            .map_err(|e| format!("Failed to handle message: {}", e))
    }
}

// 使用示例
fn main() {
    let mut manager = PluginManager::new();

    // 挂载同一个插件的两个实例
    let instance1 = manager.mount_plugin(
        "counter-plugin",
        Some("counter-1".to_string()),
        Box::new(CounterPlugin::new()),
    ).unwrap();

    let instance2 = manager.mount_plugin(
        "counter-plugin",
        Some("counter-2".to_string()),
        Box::new(CounterPlugin::new()),
    ).unwrap();

    // 测试独立状态
    println!("{}", manager.send_message(&instance1, "increment").unwrap()); // Counter for instance counter-1 is now: 1
    println!("{}", manager.send_message(&instance1, "increment").unwrap()); // Counter for instance counter-1 is now: 2
    println!("{}", manager.send_message(&instance2, "increment").unwrap()); // Counter for instance counter-2 is now: 1
    println!("{}", manager.send_message(&instance1, "get").unwrap());       // Counter for instance counter-1 is: 2
    println!("{}", manager.send_message(&instance2, "get").unwrap());       // Counter for instance counter-2 is: 1
}

// 辅助类型
#[derive(Debug, Clone)]
pub struct PluginMetadata {
    pub id: String,
    pub name: String,
}

#[derive(Clone)]
pub struct HostCallbacks {
    pub send_to_frontend: fn(&str, &str) -> bool,
}
