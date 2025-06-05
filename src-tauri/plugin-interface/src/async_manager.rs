#[cfg(feature = "async")]
use crate::handler::AsyncPluginHandler;
#[cfg(feature = "async")]
use crate::metadata::PluginMetadata;
#[cfg(feature = "async")]
use std::collections::HashMap;
#[cfg(feature = "async")]
use std::sync::Arc;

#[cfg(feature = "async")]
use tokio::sync::{Mutex, RwLock};
#[cfg(feature = "async")]
use tokio::task::JoinHandle;

#[cfg(feature = "async")]
/// 异步插件实例
pub struct AsyncPluginInstance {
    pub metadata: PluginMetadata,
    pub handler: Arc<Mutex<dyn AsyncPluginHandler>>,
    pub runtime_handle: Option<JoinHandle<()>>,
    pub is_mounted: bool,
    pub is_connected: bool,
}

#[cfg(feature = "async")]
/// 异步插件管理器
/// 使用tokio运行时管理插件生命周期
pub struct AsyncPluginManager {
    instances: Arc<RwLock<HashMap<String, AsyncPluginInstance>>>,
    current_plugin: Arc<Mutex<Option<String>>>,
}

#[cfg(feature = "async")]
impl AsyncPluginManager {
    pub fn new() -> Self {
        Self {
            instances: Arc::new(RwLock::new(HashMap::new())),
            current_plugin: Arc::new(Mutex::new(None)),
        }
    }

    /// 异步挂载插件
    pub async fn mount_plugin_async(
        &self,
        plugin_id: &str,
        handler: Arc<Mutex<dyn AsyncPluginHandler>>,
        metadata: PluginMetadata,
    ) -> Result<String, String> {
        // 先卸载当前插件
        let current_plugin_id = {
            let current = self.current_plugin.lock().await;
            current.clone()
        };

        if let Some(current_id) = current_plugin_id {
            if current_id != plugin_id {
                if let Err(e) = self.dispose_plugin_async(&current_id).await {
                    eprintln!("Failed to dispose current plugin: {}", e);
                }
            }
        }

        let mut instances = self.instances.write().await;

        // 如果插件已经存在且已挂载，直接返回成功
        if let Some(instance) = instances.get(plugin_id) {
            if instance.is_mounted {
                *self.current_plugin.lock().await = Some(plugin_id.to_string());
                return Ok(format!("插件 {} 已经挂载", instance.metadata.name));
            }
        }

        // 调用异步挂载
        {
            let mut handler_guard = handler.lock().await;
            handler_guard.on_mount_async(&metadata).await
                .map_err(|e| format!("插件挂载失败: {}", e))?;
        }

        // 启动插件运行时
        let runtime_handle = {
            let handler_clone = handler.clone();
            Some(tokio::spawn(async move {
                let handler_guard = handler_clone.lock().await;
                if let Err(e) = handler_guard.start_runtime().await {
                    eprintln!("Plugin runtime failed: {}", e);
                }
            }))
        };

        // 创建插件实例
        let instance = AsyncPluginInstance {
            metadata: metadata.clone(),
            handler,
            runtime_handle,
            is_mounted: true,
            is_connected: false,
        };

        instances.insert(plugin_id.to_string(), instance);
        *self.current_plugin.lock().await = Some(plugin_id.to_string());

        Ok(format!("插件 {} 异步挂载成功", metadata.name))
    }

    /// 异步卸载插件
    pub async fn dispose_plugin_async(&self, plugin_id: &str) -> Result<String, String> {
        let mut instances = self.instances.write().await;

        if let Some(mut instance) = instances.remove(plugin_id) {
            if !instance.is_mounted {
                return Ok(format!("插件 {} 已经卸载", instance.metadata.name));
            }

            // 先断开连接
            if instance.is_connected {
                let handler_guard = instance.handler.lock().await;
                let _ = handler_guard.on_disconnect_async().await;
                instance.is_connected = false;
            }

            // 停止运行时
            if let Some(handle) = instance.runtime_handle.take() {
                handle.abort();
            }

            // 调用异步卸载
            {
                let handler_guard = instance.handler.lock().await;
                handler_guard.stop_runtime().await
                    .map_err(|e| format!("停止插件运行时失败: {}", e))?;
                handler_guard.on_dispose_async().await
                    .map_err(|e| format!("插件卸载失败: {}", e))?;
            }

            instance.is_mounted = false;

            // 如果这是当前插件，清除当前插件状态
            let mut current = self.current_plugin.lock().await;
            if current.as_ref() == Some(&plugin_id.to_string()) {
                *current = None;
            }

            Ok(format!("插件 {} 异步卸载成功", instance.metadata.name))
        } else {
            Err(format!("插件 {} 未找到", plugin_id))
        }
    }

    /// 异步连接插件
    pub async fn connect_plugin_async(&self, plugin_id: &str) -> Result<String, String> {
        let instances = self.instances.read().await;

        if let Some(instance) = instances.get(plugin_id) {
            if !instance.is_mounted {
                return Err(format!("插件 {} 未挂载", instance.metadata.name));
            }

            if instance.is_connected {
                return Ok(format!("插件 {} 已经连接", instance.metadata.name));
            }

            let handler_guard = instance.handler.lock().await;
            handler_guard.on_connect_async().await
                .map_err(|e| format!("插件连接失败: {}", e))?;

            // 注意：这里需要获取可变引用来更新状态
            // 在实际实现中，可能需要重新设计数据结构
            Ok(format!("插件 {} 异步连接成功", instance.metadata.name))
        } else {
            Err(format!("插件 {} 未找到", plugin_id))
        }
    }

    /// 异步处理消息
    pub async fn handle_message_async(&self, plugin_id: &str, message: &str) -> Result<String, String> {
        let instances = self.instances.read().await;

        if let Some(instance) = instances.get(plugin_id) {
            if !instance.is_mounted || !instance.is_connected {
                return Err(format!("插件 {} 未准备好", instance.metadata.name));
            }

            let handler_guard = instance.handler.lock().await;
            handler_guard.handle_message_async(message).await
                .map_err(|e| format!("消息处理失败: {}", e))
        } else {
            Err(format!("插件 {} 未找到", plugin_id))
        }
    }

    /// 获取当前插件ID
    pub async fn get_current_plugin_id(&self) -> Option<String> {
        let current = self.current_plugin.lock().await;
        current.clone()
    }

    /// 列出所有插件
    pub async fn list_plugins(&self) -> Vec<PluginMetadata> {
        let instances = self.instances.read().await;
        instances.values().map(|instance| instance.metadata.clone()).collect()
    }
}

#[cfg(feature = "async")]
impl Default for AsyncPluginManager {
    fn default() -> Self {
        Self::new()
    }
}
