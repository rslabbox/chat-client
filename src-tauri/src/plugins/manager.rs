use crate::plugins::PluginLoader;
use libloading::{Library, Symbol};
use plugin_interface::{
    CreatePluginFn, DestroyPluginFn, PluginHandler, PluginMetadata, CREATE_PLUGIN_SYMBOL,
    DESTROY_PLUGIN_SYMBOL,
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// 插件实例信息
#[derive(Debug)]
pub struct PluginInstance {
    pub metadata: PluginMetadata,
    pub handler: *mut dyn PluginHandler,
    pub library: Library,
    pub is_mounted: bool,
    pub is_connected: bool,
}

unsafe impl Send for PluginInstance {}
unsafe impl Sync for PluginInstance {}

/// 插件管理器
pub struct PluginManager {
    loader: PluginLoader,
    instances: Arc<Mutex<HashMap<String, PluginInstance>>>,
    current_plugin: Arc<Mutex<Option<String>>>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            loader: PluginLoader::new(),
            instances: Arc::new(Mutex::new(HashMap::new())),
            current_plugin: Arc::new(Mutex::new(None)),
        }
    }

    /// 扫描插件列表
    pub fn scan_plugins(&self) -> Vec<PluginMetadata> {
        self.loader.scan_plugins()
    }

    /// 挂载插件
    pub fn mount_plugin(&self, plugin_id: &str) -> Result<String, String> {
        // 获取当前插件ID
        let current_plugin_id = {
            let current = self.current_plugin.lock().unwrap();
            current.clone()
        }; 
           // 先卸载当前插件
        if let Some(current_id) = current_plugin_id {
            if current_id != plugin_id {
                if let Err(e) = self.dispose_plugin(&current_id) {
                    eprintln!("Failed to dispose current plugin: {}", e);
                }
            }
        }

        let mut instances = self.instances.lock().unwrap();

        // 如果插件已经存在且已挂载，直接返回成功
        if let Some(instance) = instances.get(plugin_id) {
            if instance.is_mounted {
                *self.current_plugin.lock().unwrap() = Some(plugin_id.to_string());
                return Ok(format!("插件 {} 已经挂载", instance.metadata.name));
            }
        }

        // 加载插件
        let plugin_metadata = self.find_plugin_metadata(plugin_id)?;
        let library_path = plugin_metadata
            .library_path
            .as_ref()
            .ok_or_else(|| format!("插件 {} 没有找到动态库文件", plugin_id))?;

        // 动态加载库
        let library = unsafe {
            Library::new(library_path).map_err(|e| format!("加载动态库失败: {}", e))?
        };

        // 获取创建函数
        let create_plugin: Symbol<CreatePluginFn> = unsafe {
            library
                .get(CREATE_PLUGIN_SYMBOL)
                .map_err(|e| format!("找不到插件创建函数: {}", e))?
        };

        // 创建插件实例
        let handler = unsafe { create_plugin() };
        if handler.is_null() {
            return Err("插件创建失败".to_string());
        }

        // 调用 on_mount
        let result = unsafe { (*handler).on_mount() };

        match result {
            Ok(_) => {
                // 创建插件实例
                let instance = PluginInstance {
                    metadata: plugin_metadata.clone(),
                    handler,
                    library,
                    is_mounted: true,
                    is_connected: false,
                };

                instances.insert(plugin_id.to_string(), instance);
                *self.current_plugin.lock().unwrap() = Some(plugin_id.to_string());

                Ok(format!("插件 {} 挂载成功", plugin_metadata.name))
            }
            Err(e) => {
                // 清理失败的插件实例
                unsafe {
                    let destroy_plugin: Result<Symbol<DestroyPluginFn>, _> =
                        library.get(DESTROY_PLUGIN_SYMBOL);
                    if let Ok(destroy_fn) = destroy_plugin {
                        destroy_fn(handler);
                    }
                }
                Err(format!("插件挂载失败: {}", e))
            }
        }
    }

    /// 卸载插件
    pub fn dispose_plugin(&self, plugin_id: &str) -> Result<String, String> {
        let mut instances = self.instances.lock().unwrap();

        if let Some(instance) = instances.get_mut(plugin_id) {
            if !instance.is_mounted {
                return Ok(format!("插件 {} 已经卸载", instance.metadata.name));
            }

            // 先断开连接
            if instance.is_connected {
                let _ = unsafe { (*instance.handler).on_disconnect() };
                instance.is_connected = false;
            }

            // 调用 on_dispose
            let result = unsafe { (*instance.handler).on_dispose() };

            // 销毁插件实例
            unsafe {
                let destroy_plugin: Result<Symbol<DestroyPluginFn>, _> =
                    instance.library.get(DESTROY_PLUGIN_SYMBOL);
                if let Ok(destroy_fn) = destroy_plugin {
                    destroy_fn(instance.handler);
                }
            }

            instance.is_mounted = false;

            // 如果这是当前插件，清除当前插件状态
            let mut current = self.current_plugin.lock().unwrap();
            if current.as_ref() == Some(&plugin_id.to_string()) {
                *current = None;
            }

            match result {
                Ok(_) => Ok(format!("插件 {} 卸载成功", instance.metadata.name)),
                Err(e) => Ok(format!(
                    "插件 {} 卸载完成，但有警告: {}",
                    instance.metadata.name, e
                )),
            }
        } else {
            Err(format!("插件 {} 未找到", plugin_id))
        }
    }

    /// 连接插件
    pub fn connect_plugin(&self, plugin_id: &str) -> Result<String, String> {
        let mut instances = self.instances.lock().unwrap();

        if let Some(instance) = instances.get_mut(plugin_id) {
            if !instance.is_mounted {
                return Err(format!("插件 {} 未挂载", instance.metadata.name));
            }

            if instance.is_connected {
                return Ok(format!("插件 {} 已经连接", instance.metadata.name));
            }

            let result = unsafe { (*instance.handler).on_connect() };

            match result {
                Ok(_) => {
                    instance.is_connected = true;
                    Ok(format!("插件 {} 连接成功", instance.metadata.name))
                }
                Err(e) => Err(format!("插件连接失败: {}", e)),
            }
        } else {
            Err(format!("插件 {} 未找到", plugin_id))
        }
    }

    /// 断开插件连接
    pub fn disconnect_plugin(&self, plugin_id: &str) -> Result<String, String> {
        let mut instances = self.instances.lock().unwrap();

        if let Some(instance) = instances.get_mut(plugin_id) {
            if !instance.is_mounted {
                return Err(format!("插件 {} 未挂载", instance.metadata.name));
            }

            if !instance.is_connected {
                return Ok(format!("插件 {} 已经断开连接", instance.metadata.name));
            }

            let result = unsafe { (*instance.handler).on_disconnect() };

            instance.is_connected = false;

            match result {
                Ok(_) => Ok(format!("插件 {} 断开连接成功", instance.metadata.name)),
                Err(e) => Ok(format!(
                    "插件 {} 断开连接完成，但有警告: {}",
                    instance.metadata.name, e
                )),
            }
        } else {
            Err(format!("插件 {} 未找到", plugin_id))
        }
    }

    /// 获取当前插件状态
    pub fn get_plugin_status(&self, plugin_id: &str) -> Option<(bool, bool)> {
        let instances = self.instances.lock().unwrap();
        instances
            .get(plugin_id)
            .map(|instance| (instance.is_mounted, instance.is_connected))
    }

    /// 获取当前活跃插件
    pub fn get_current_plugin(&self) -> Option<String> {
        self.current_plugin.lock().unwrap().clone()
    }

    /// 向当前活跃插件发送消息
    pub fn send_message_to_current_plugin(&self, message: &str) -> Result<String, String> {
        let instances = self.instances.lock().unwrap();
        let current_plugin_id = self.current_plugin.lock().unwrap();

        if let Some(plugin_id) = current_plugin_id.as_ref() {
            if let Some(instance) = instances.get(plugin_id) {
                if instance.is_mounted {
                    unsafe {
                        match (*instance.handler).handle_message(message) {
                            Ok(response) => Ok(response),
                            Err(e) => Err(format!("插件处理消息失败: {}", e))
                        }
                    }
                } else {
                    Err("当前插件未挂载".to_string())
                }
            } else {
                Err("当前插件未找到".to_string())
            }
        } else {
            Err("没有活跃的插件".to_string())
        }
    }

    /// 清理所有已挂载的插件（应用关闭时调用）
    pub fn cleanup_all_plugins(&self) {
        let mut instances = self.instances.lock().unwrap();

        // 收集所有已挂载的插件ID
        let mounted_plugin_ids: Vec<String> = instances
            .iter()
            .filter(|(_, instance)| instance.is_mounted)
            .map(|(id, _)| id.clone())
            .collect();

        // 逐个清理插件
        for plugin_id in mounted_plugin_ids {
            if let Some(instance) = instances.get_mut(&plugin_id) {
                if instance.is_mounted {
                    println!("正在清理插件: {}", instance.metadata.name);

                    // 先断开连接
                    if instance.is_connected {
                        let _ = unsafe { (*instance.handler).on_disconnect() };
                        instance.is_connected = false;
                    }

                    // 调用 on_dispose
                    let _ = unsafe { (*instance.handler).on_dispose() };

                    // 销毁插件实例
                    unsafe {
                        let destroy_plugin: Result<Symbol<DestroyPluginFn>, _> =
                            instance.library.get(DESTROY_PLUGIN_SYMBOL);
                        if let Ok(destroy_fn) = destroy_plugin {
                            destroy_fn(instance.handler);
                        }
                    }

                    instance.is_mounted = false;
                    println!("插件 {} 清理完成", instance.metadata.name);
                }
            }
        }

        // 清除当前插件状态
        *self.current_plugin.lock().unwrap() = None;

        println!("所有插件清理完成");
    }

    /// 查找插件元数据
    fn find_plugin_metadata(&self, plugin_id: &str) -> Result<PluginMetadata, String> {
        let plugins = self.scan_plugins();
        plugins
            .into_iter()
            .find(|p| p.id == plugin_id)
            .ok_or_else(|| format!("插件 {} 未找到", plugin_id))
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}
