//! 插件UI容器模块
//!
//! 提供了PluginUi结构体，作为插件UI系统的核心容器

use super::Component;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// 插件UI容器
pub struct PluginUi {
    pub(crate) components: Vec<Component>,
    pub(crate) actions: HashMap<String, Box<dyn Fn(String) + Send + Sync + 'static>>,
    pub(crate) on_update: Option<Box<dyn Fn() + Send + Sync + 'static>>,
}

impl std::fmt::Debug for PluginUi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PluginUi")
            .field("components", &self.components)
            .field("actions_count", &self.actions.len())
            .field("has_update_callback", &self.on_update.is_some())
            .finish()
    }
}

impl Clone for PluginUi {
    fn clone(&self) -> Self {
        Self {
            components: self.components.clone(),
            actions: HashMap::new(), // 不能克隆函数指针，创建空的HashMap
            on_update: None,         // 不能克隆函数指针，设为None
        }
    }
}

impl Default for PluginUi {
    fn default() -> Self {
        Self {
            components: Vec::new(),
            actions: HashMap::new(),
            on_update: None,
        }
    }
}

impl PluginUi {
    /// 创建新的UI容器
    pub fn new() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self {
            components: Vec::new(),
            actions: HashMap::new(),
            on_update: None,
        }))
    }

    /// 创建新的UI容器并自动设置更新回调
    pub fn new_with_plugin_id(plugin_id: &str) -> Arc<Mutex<Self>> {
        let ui = Arc::new(Mutex::new(Self {
            components: Vec::new(),
            actions: HashMap::new(),
            on_update: None,
        }));

        // 自动设置UI更新回调
        let plugin_id = plugin_id.to_string();
        {
            let mut ui_guard = ui.lock().unwrap();
            ui_guard.set_update_callback(move || {
                // 当UI状态更新时，发送事件到前端
                crate::api::send_to_frontend("plugin-ui-updated", &format!(r#"{{"plugin": "{}"}}"#, plugin_id));
            });
        }

        ui
    }

    /// 设置UI更新回调
    pub fn set_update_callback<F>(&mut self, callback: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.on_update = Some(Box::new(callback));
    }

    /// 通知UI已更新
    pub(crate) fn notify_update(&self) {
        if let Some(callback) = &self.on_update {
            callback();
        }
    }

    /// 查找组件（可变引用）
    pub(crate) fn find_component_mut(&mut self, id: &str) -> Option<&mut Component> {
        self.components.iter_mut().find(|c| c.id == id)
    }

    /// 查找组件（不可变引用）
    pub(crate) fn find_component(&self, id: &str) -> Option<&Component> {
        self.components.iter().find(|c| c.id == id)
    }

    /// 处理UI事件
    pub fn handle_event(&self, component_id: &str, value: &str) -> bool {
        if let Some(action) = self.actions.get(component_id) {
            action(value.to_string());
            true
        } else {
            false
        }
    }

    /// 获取所有组件（用于序列化发送到前端）
    pub fn get_components(&self) -> Vec<Component> {
        self.components.clone()
    }

    /// 清空所有组件
    pub fn clear_components(&mut self) {
        self.components.clear();
        self.actions.clear();
    }
}

// 为了序列化，实现Serialize和Deserialize
impl Serialize for PluginUi {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.components.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for PluginUi {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let components = Vec::<Component>::deserialize(deserializer)?;
        Ok(Self {
            components,
            actions: HashMap::new(),
            on_update: None,
        })
    }
}
