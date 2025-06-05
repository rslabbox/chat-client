use crate::plugins::loader::PluginLoader;
use plugin_interface::{
    PluginHandler, PluginMetadata, PluginInterface,
    pluginui::{Context, Ui},
    callbacks::HostCallbacks,
    log_info, log_warn, log_error
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::RwLock;
use serde_json;

/// 异步插件实例
pub struct AsyncPluginInstance {
    pub metadata: PluginMetadata,
    pub handler: *const PluginInterface,
    pub ui_instance: Option<Arc<Mutex<Ui>>>,
    pub ui_data: Option<String>,
    pub is_mounted: bool,
    pub is_connected: bool,
}

unsafe impl Send for AsyncPluginInstance {}
unsafe impl Sync for AsyncPluginInstance {}

/// 异步插件管理器
/// 专门处理支持异步UI的插件
pub struct AsyncPluginManager {
    instances: Arc<RwLock<HashMap<String, AsyncPluginInstance>>>,
    current_plugin: Arc<RwLock<Option<String>>>,
    loader: PluginLoader,
}

impl AsyncPluginManager {
    pub fn new() -> Self {
        Self {
            instances: Arc::new(RwLock::new(HashMap::new())),
            current_plugin: Arc::new(RwLock::new(None)),
            loader: PluginLoader::new(),
        }
    }

    /// 异步挂载插件
    pub async fn mount_plugin_async(&self, plugin_id: &str) -> Result<String, String> {
        log_info!("开始异步挂载插件: {}", plugin_id);

        // 先卸载当前插件
        let current_plugin_id = {
            let current = self.current_plugin.read().await;
            current.clone()
        };

        if let Some(current_id) = current_plugin_id {
            if current_id != plugin_id {
                if let Err(e) = self.dispose_plugin_async(&current_id).await {
                    log_error!("Failed to dispose current plugin: {}", e);
                }
            }
        }

        let mut instances = self.instances.write().await;

        // 如果插件已经存在且已挂载，直接返回成功
        if let Some(instance) = instances.get(plugin_id) {
            if instance.is_mounted {
                *self.current_plugin.write().await = Some(plugin_id.to_string());
                return Ok(format!("插件 {} 已经挂载", instance.metadata.name));
            }
        }

        // 创建示例插件元数据（简化版本）
        let plugin_metadata = PluginMetadata {
            id: plugin_id.to_string(),
            disabled: false,
            name: format!("异步插件 {}", plugin_id),
            description: "异步插件示例".to_string(),
            version: "1.0.0".to_string(),
            author: Some("Augment".to_string()),
            library_path: None,
            config_path: "config.toml".to_string(),
        };

        // 创建示例处理器（这里需要实际的插件加载逻辑）
        // 暂时返回错误，因为我们需要实际的插件加载机制
        return Err("异步插件管理器需要完整的插件加载机制".to_string());
    }

    /// 异步初始化UI
    async fn initialize_ui_async(&self, plugin_id: &str, handler: *const PluginInterface) -> Result<String, String> {
        log_info!("开始异步初始化UI: {}", plugin_id);

        // 创建UI上下文和实例
        let context = Context::new(plugin_id.to_string());
        let ui_arc = Ui::new(plugin_id.to_string());
        let mut ui = ui_arc.lock().unwrap();

        // 异步调用update_ui
        let update_result = unsafe {
            ((*handler).update_ui)(
                (*handler).plugin_ptr,
                &context as *const Context as *const std::ffi::c_void,
                &mut *ui as *mut Ui as *mut std::ffi::c_void
            )
        };

        if update_result != 0 {
            log_warn!("异步UI初始化返回非零值: {}", update_result);
        }

        // 序列化UI数据
        let ui_data = match serde_json::to_string(&ui.get_components()) {
            Ok(json) => json,
            Err(e) => {
                log_error!("序列化UI数据失败: {}", e);
                "[]".to_string()
            }
        };

        log_info!("UI异步初始化完成，组件数量: {}", ui.get_components().len());
        Ok(ui_data)
    }

    /// 异步卸载插件
    pub async fn dispose_plugin_async(&self, plugin_id: &str) -> Result<String, String> {
        log_info!("开始异步卸载插件: {}", plugin_id);

        let mut instances = self.instances.write().await;

        if let Some(instance) = instances.remove(plugin_id) {
            if !instance.is_mounted {
                return Ok(format!("插件 {} 已经卸载", instance.metadata.name));
            }

            // 先断开连接
            if instance.is_connected {
                let disconnect_result = unsafe {
                    ((*instance.handler).on_disconnect)((*instance.handler).plugin_ptr)
                };
                if disconnect_result != 0 {
                    log_warn!("插件断开连接失败，错误代码: {}", disconnect_result);
                }
            }

            // 异步调用on_dispose
            let dispose_result = unsafe {
                ((*instance.handler).on_dispose)((*instance.handler).plugin_ptr)
            };

            if dispose_result != 0 {
                log_warn!("插件卸载失败，错误代码: {}", dispose_result);
            }

            // 销毁插件处理器
            unsafe {
                ((*instance.handler).destroy)((*instance.handler).plugin_ptr);
            }

            // 如果这是当前插件，清除当前插件状态
            let mut current = self.current_plugin.write().await;
            if current.as_ref() == Some(&plugin_id.to_string()) {
                *current = None;
            }

            log_info!("插件 {} 异步卸载成功", instance.metadata.name);
            Ok(format!("插件 {} 异步卸载成功", instance.metadata.name))
        } else {
            Err(format!("插件 {} 未找到", plugin_id))
        }
    }

    /// 异步处理UI事件
    pub async fn handle_ui_event_async(&self, plugin_id: &str, component_id: &str, value: &str) -> Result<bool, String> {
        log_info!("异步处理UI事件: {} -> {} = {}", plugin_id, component_id, value);

        let instances = self.instances.read().await;

        if let Some(instance) = instances.get(plugin_id) {
            if !instance.is_mounted {
                return Err("插件未挂载".to_string());
            }

            // 创建包含UI事件数据的Context
            let mut ui_event_data = std::collections::HashMap::new();
            ui_event_data.insert(component_id.to_string(), value.to_string());
            let context = Context::with_ui_event_data(plugin_id.to_string(), ui_event_data);

            // 创建新的UI实例来处理事件
            let ui_arc = Ui::new(plugin_id.to_string());
            let mut ui = ui_arc.lock().unwrap();

            // 设置事件数据
            ui.handle_ui_event(component_id, value);

            // 异步调用update_ui
            let update_result = unsafe {
                ((*instance.handler).update_ui)(
                    (*instance.handler).plugin_ptr,
                    &context as *const Context as *const std::ffi::c_void,
                    &mut *ui as *mut Ui as *mut std::ffi::c_void
                )
            };

            if update_result == 0 {
                // 更新UI数据
                let ui_data = match serde_json::to_string(&ui.get_components()) {
                    Ok(json) => json,
                    Err(e) => {
                        log_error!("序列化UI数据失败: {}", e);
                        "[]".to_string()
                    }
                };

                // 这里需要更新实例的UI数据，但由于我们持有的是读锁，需要重新获取写锁
                drop(instances);
                let mut instances_write = self.instances.write().await;
                if let Some(instance_mut) = instances_write.get_mut(plugin_id) {
                    instance_mut.ui_data = Some(ui_data);
                }

                log_info!("UI事件异步处理完成");
                Ok(true)
            } else {
                log_warn!("UI事件处理失败，错误代码: {}", update_result);
                Ok(false)
            }
        } else {
            Err("插件未找到".to_string())
        }
    }

    /// 获取当前插件ID
    pub async fn get_current_plugin_id(&self) -> Option<String> {
        let current = self.current_plugin.read().await;
        current.clone()
    }

    /// 获取插件UI数据
    pub async fn get_plugin_ui_data(&self, plugin_id: &str) -> Option<String> {
        let instances = self.instances.read().await;
        instances.get(plugin_id).and_then(|instance| instance.ui_data.clone())
    }

    /// 列出所有插件
    pub async fn list_plugins(&self) -> Vec<PluginMetadata> {
        let instances = self.instances.read().await;
        instances.values().map(|instance| instance.metadata.clone()).collect()
    }

    /// 清理所有插件
    pub async fn cleanup_all_plugins(&self) {
        log_info!("开始异步清理所有插件");

        let plugin_ids: Vec<String> = {
            let instances = self.instances.read().await;
            instances.keys().cloned().collect()
        };

        for plugin_id in plugin_ids {
            if let Err(e) = self.dispose_plugin_async(&plugin_id).await {
                log_error!("清理插件 {} 失败: {}", plugin_id, e);
            }
        }

        log_info!("所有插件异步清理完成");
    }
}

impl Default for AsyncPluginManager {
    fn default() -> Self {
        Self::new()
    }
}
