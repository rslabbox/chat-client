//! 组件引用模块
//!
//! 提供了类型安全的组件引用，确保只能调用适用于特定组件类型的方法

use super::{ComponentType, PluginUi};
use std::sync::{Arc, Mutex};

/// 基础组件引用，包含通用信息
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ComponentRef {
    pub(crate) id: String,
    pub(crate) ui: Arc<Mutex<PluginUi>>,
}

#[allow(dead_code)]
impl ComponentRef {
    /// 创建新的组件引用
    pub(crate) fn new(id: String, ui: Arc<Mutex<PluginUi>>) -> Self {
        Self { id, ui }
    }
}

/// 按钮组件引用，只能调用按钮相关的方法
#[derive(Debug, Clone)]
pub struct ButtonRef {
    pub(crate) id: String,
    pub(crate) ui: Arc<Mutex<PluginUi>>,
}

impl ButtonRef {
    /// 创建新的按钮引用
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
}

/// 文本输入框组件引用，只能调用文本框相关的方法
#[derive(Debug, Clone)]
pub struct TextFieldRef {
    pub(crate) id: String,
    pub(crate) ui: Arc<Mutex<PluginUi>>,
}

impl TextFieldRef {
    /// 创建新的文本框引用
    pub(crate) fn new(id: String, ui: Arc<Mutex<PluginUi>>) -> Self {
        Self { id, ui }
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

/// 文本显示组件引用，只能调用文本显示相关的方法
#[derive(Debug, Clone)]
pub struct TextRef {
    pub(crate) id: String,
    pub(crate) ui: Arc<Mutex<PluginUi>>,
}

impl TextRef {
    /// 创建新的文本显示引用
    pub(crate) fn new(id: String, ui: Arc<Mutex<PluginUi>>) -> Self {
        Self { id, ui }
    }

    /// 设置文本显示值
    pub fn set_value(&self, value: &str) -> bool {
        let mut ui = self.ui.lock().unwrap();
        if let Some(component) = ui.find_component_mut(&self.id) {
            match &mut component.component {
                ComponentType::Text { value: v } => {
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
}

/// 下拉选择框组件引用，只能调用下拉选择框相关的方法
#[derive(Debug, Clone)]
pub struct SelectRef {
    pub(crate) id: String,
    pub(crate) ui: Arc<Mutex<PluginUi>>,
}

impl SelectRef {
    /// 创建新的下拉选择框引用
    pub(crate) fn new(id: String, ui: Arc<Mutex<PluginUi>>) -> Self {
        Self { id, ui }
    }

    /// 设置下拉选择框的选项
    pub fn set_value(&self, options: Vec<&str>) -> bool {
        let mut ui = self.ui.lock().unwrap();
        if let Some(component) = ui.find_component_mut(&self.id) {
            match &mut component.component {
                ComponentType::Select { options: opts, .. } => {
                    *opts = options.into_iter().map(|s| s.to_string()).collect();
                    ui.notify_update();
                    true
                }
                _ => false,
            }
        } else {
            false
        }
    }

    /// 设置当前选中的值
    pub fn set_selected(&self, selected: Option<&str>) -> bool {
        let mut ui = self.ui.lock().unwrap();
        if let Some(component) = ui.find_component_mut(&self.id) {
            match &mut component.component {
                ComponentType::Select { selected: sel, .. } => {
                    *sel = selected.map(|s| s.to_string());
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

/// 容器组件引用，用于管理容器布局和子组件
#[derive(Debug, Clone)]
pub struct ContainerRef {
    pub(crate) id: String,
    pub(crate) ui: Arc<Mutex<PluginUi>>,
}

impl ContainerRef {
    /// 创建新的容器引用
    pub(crate) fn new(id: String, ui: Arc<Mutex<PluginUi>>) -> Self {
        Self { id, ui }
    }

    /// 添加子组件到容器
    pub fn add_child(&self, child: super::Component) -> bool {
        let mut ui = self.ui.lock().unwrap();
        if let Some(component) = ui.find_component_mut(&self.id) {
            match &mut component.component {
                ComponentType::Container { children, .. } => {
                    children.push(child);
                    ui.notify_update();
                    true
                }
                _ => false,
            }
        } else {
            false
        }
    }

    /// 移除指定索引的子组件
    pub fn remove_child(&self, index: usize) -> bool {
        let mut ui = self.ui.lock().unwrap();
        if let Some(component) = ui.find_component_mut(&self.id) {
            match &mut component.component {
                ComponentType::Container { children, .. } => {
                    if index < children.len() {
                        children.remove(index);
                        ui.notify_update();
                        true
                    } else {
                        false
                    }
                }
                _ => false,
            }
        } else {
            false
        }
    }

    /// 清空所有子组件
    pub fn clear_children(&self) -> bool {
        let mut ui = self.ui.lock().unwrap();
        if let Some(component) = ui.find_component_mut(&self.id) {
            match &mut component.component {
                ComponentType::Container { children, .. } => {
                    children.clear();
                    ui.notify_update();
                    true
                }
                _ => false,
            }
        } else {
            false
        }
    }

    /// 获取子组件数量
    pub fn child_count(&self) -> usize {
        let ui = self.ui.lock().unwrap();
        if let Some(component) = ui.find_component(&self.id) {
            match &component.component {
                ComponentType::Container { children, .. } => children.len(),
                _ => 0,
            }
        } else {
            0
        }
    }
}
