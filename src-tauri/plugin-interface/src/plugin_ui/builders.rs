//! 组件构建器模块
//!
//! 提供了创建各种UI组件的便捷方法

use super::{Component, ComponentRef, ComponentType, PluginUi};
use std::sync::{Arc, Mutex};

impl PluginUi {
    /// 创建按钮组件
    pub fn button<F>(
        ui_arc: &Arc<Mutex<Self>>,
        label: &str,
        icon: Option<&str>,
        enabled: bool,
        on_press: F,
    ) -> ComponentRef
    where
        F: Fn() + Send + Sync + 'static,
    {
        let id = format!("btn_{}", uuid::Uuid::new_v4());
        let action_id = id.clone();

        let mut ui = ui_arc.lock().unwrap();

        // 存储回调函数
        ui.actions.insert(
            action_id.clone(),
            Box::new(move |_| {
                on_press();
            }),
        );

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

        ComponentRef::new(id, Arc::clone(ui_arc))
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
        ui.actions.insert(
            action_id.clone(),
            Box::new(move |value| {
                on_submit(value);
            }),
        );

        let component = Component {
            id: id.clone(),
            component: ComponentType::TextField {
                hint: hint.to_string(),
                value: String::new(),
                action_id,
            },
        };

        ui.components.push(component);

        ComponentRef::new(id, Arc::clone(ui_arc))
    }
}
