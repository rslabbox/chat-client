//! 组件引用模块
//!
//! 提供了ComponentRef结构体，用于更新组件状态

use super::{ComponentType, PluginUi};
use std::sync::{Arc, Mutex};

/// 组件引用，用于更新组件状态
#[derive(Debug, Clone)]
pub struct ComponentRef {
    pub(crate) id: String,
    pub(crate) ui: Arc<Mutex<PluginUi>>,
}

impl ComponentRef {
    /// 创建新的组件引用
    pub(crate) fn new(id: String, ui: Arc<Mutex<PluginUi>>) -> Self {
        Self { id, ui }
    }

    /// 设置按钮标签
    pub fn set_label(&self, label: &str) -> bool {
        let mut ui = self.ui.lock().unwrap();
        if let Some(component) = ui.find_component_mut(&self.id) {
            match &mut component.component {
                ComponentType::Button { label: l, .. } => {
                    *l = label.to_string();
                    ui.notify_update();
                    true
                }
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
                }
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
                }
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
                }
                _ => false,
            }
        } else {
            false
        }
    }
}
