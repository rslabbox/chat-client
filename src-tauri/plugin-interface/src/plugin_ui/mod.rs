//! 插件UI组件模块
//! 
//! 提供了一套简单的UI组件系统，允许插件定义自己的UI界面

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// UI组件类型枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ComponentType {
    Button {
        label: String,
        icon: Option<String>,
        enabled: bool,
        #[serde(skip)]
        action_id: String,
    },
    TextField {
        hint: String,
        value: String,
        #[serde(skip)]
        action_id: String,
    },
    // 可以扩展更多组件类型
}

/// 单个UI组件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    pub id: String,
    pub component: ComponentType,
}

/// 组件引用，用于更新组件状态
#[derive(Debug, Clone)]
pub struct ComponentRef {
    id: String,
    ui: Arc<Mutex<PluginUi>>,
}

impl ComponentRef {
    /// 设置按钮标签
    pub fn set_label(&self, label: &str) -> bool {
        let mut ui = self.ui.lock().unwrap();
        if let Some(component) = ui.find_component_mut(&self.id) {
            match &mut component.component {
                ComponentType::Button { label: l, .. } => {
                    *l = label.to_string();
                    ui.notify_update();
                    true
                },
                _ => false,
            }
        } else {
            false
        }
    }
    
    /// 设置按钮启用状态
    pub fn set_enabled(&self, enabled: bool) -> bool {
        let mut ui = self.ui.lock().unwrap();
        if let Some(component) = ui.find_component_mut(&self.id) {
            match &mut component.component {
                ComponentType::Button { enabled: e, .. } => {
                    *e = enabled;
                    ui.notify_update();
                    true
                },
                _ => false,
            }
        } else {
            false
        }
    }
    
    /// 设置文本框值
    pub fn set_value(&self, value: &str) -> bool {
        let mut ui = self.ui.lock().unwrap();
        if let Some(component) = ui.find_component_mut(&self.id) {
            match &mut component.component {
                ComponentType::TextField { value: v, .. } => {
                    *v = value.to_string();
                    ui.notify_update();
                    true
                },
                _ => false,
            }
        } else {
            false
        }
    }
    
    /// 设置文本框提示
    pub fn set_hint(&self, hint: &str) -> bool {
        let mut ui = self.ui.lock().unwrap();
        if let Some(component) = ui.find_component_mut(&self.id) {
            match &mut component.component {
                ComponentType::TextField { hint: h, .. } => {
                    *h = hint.to_string();
                    ui.notify_update();
                    true
                },
                _ => false,
            }
        } else {
            false
        }
    }
}

/// 插件UI容器
pub struct PluginUi {
    components: Vec<Component>,
    actions: HashMap<String, Box<dyn Fn(String) + Send + Sync + 'static>>,
    on_update: Option<Box<dyn Fn() + Send + Sync + 'static>>,
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
            on_update: None, // 不能克隆函数指针，设为None
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
    
    /// 设置UI更新回调
    pub fn set_update_callback<F>(&mut self, callback: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.on_update = Some(Box::new(callback));
    }
    
    /// 通知UI已更新
    fn notify_update(&self) {
        if let Some(callback) = &self.on_update {
            callback();
        }
    }
    
    /// 查找组件
    fn find_component_mut(&mut self, id: &str) -> Option<&mut Component> {
        self.components.iter_mut().find(|c| c.id == id)
    }

    /// 创建按钮组件
    pub fn button<F>(ui_arc: &Arc<Mutex<Self>>, label: &str, icon: Option<&str>, enabled: bool, on_press: F) -> ComponentRef
    where
        F: Fn() + Send + Sync + 'static,
    {
        let id = format!("btn_{}", uuid::Uuid::new_v4());
        let action_id = id.clone();
        
        let mut ui = ui_arc.lock().unwrap();
        
        // 存储回调函数
        ui.actions.insert(action_id.clone(), Box::new(move |_| {
            on_press();
        }));
        
        let component = Component {
            id: id.clone(),
            component: ComponentType::Button {
                label: label.to_string(),
                icon: icon.map(|s| s.to_string()),
                enabled,
                action_id,
            },
        };
        
        ui.components.push(component);
        
        ComponentRef {
            id,
            ui: Arc::clone(ui_arc),
        }
    }
    
    /// 创建文本输入框组件
    pub fn textfield<F>(ui_arc: &Arc<Mutex<Self>>, hint: &str, on_submit: F) -> ComponentRef
    where
        F: Fn(String) + Send + Sync + 'static,
    {
        let id = format!("txt_{}", uuid::Uuid::new_v4());
        let action_id = id.clone();
        
        let mut ui = ui_arc.lock().unwrap();
        
        // 存储回调函数
        ui.actions.insert(action_id.clone(), Box::new(move |value| {
            on_submit(value);
        }));
        
        let component = Component {
            id: id.clone(),
            component: ComponentType::TextField {
                hint: hint.to_string(),
                value: String::new(),
                action_id,
            },
        };
        
        ui.components.push(component);
        
        ComponentRef {
            id,
            ui: Arc::clone(ui_arc),
        }
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
