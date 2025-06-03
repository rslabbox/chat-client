//! 组件构建器模块
//!
//! 提供了创建各种UI组件的便捷方法

use super::{Component, ComponentType, PluginUi, ContainerLayout};
use super::component_ref::{ButtonRef, TextFieldRef, TextRef, SelectRef, ContainerRef};
use std::sync::{Arc, Mutex};

impl PluginUi {
    /// 创建按钮组件
    pub fn button<F>(
        ui_arc: &Arc<Mutex<Self>>,
        label: &str,
        icon: Option<&str>,
        enabled: bool,
        on_press: F,
    ) -> ButtonRef
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

        ButtonRef::new(id, Arc::clone(ui_arc))
    }

    /// 创建文本输入框组件
    pub fn textfield<F>(ui_arc: &Arc<Mutex<Self>>, hint: &str, on_submit: F) -> TextFieldRef
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

        TextFieldRef::new(id, Arc::clone(ui_arc))
    }

    /// 创建文本显示组件
    pub fn text(ui_arc: &Arc<Mutex<Self>>, value: &str) -> TextRef {
        let id = format!("text_{}", uuid::Uuid::new_v4());

        let mut ui = ui_arc.lock().unwrap();

        let component = Component {
            id: id.clone(),
            component: ComponentType::Text {
                value: value.to_string(),
            },
        };

        ui.components.push(component);

        TextRef::new(id, Arc::clone(ui_arc))
    }

    /// 创建下拉选择框组件
    pub fn select<F>(
        ui_arc: &Arc<Mutex<Self>>,
        options: Vec<&str>,
        on_select: F,
    ) -> SelectRef
    where
        F: Fn(String) + Send + Sync + 'static,
    {
        let id = format!("select_{}", uuid::Uuid::new_v4());
        let action_id = id.clone();

        let mut ui = ui_arc.lock().unwrap();

        // 存储回调函数
        ui.actions.insert(
            action_id.clone(),
            Box::new(move |value| {
                on_select(value);
            }),
        );

        let component = Component {
            id: id.clone(),
            component: ComponentType::Select {
                options: options.into_iter().map(|s| s.to_string()).collect(),
                selected: None,
                action_id,
            },
        };

        ui.components.push(component);

        SelectRef::new(id, Arc::clone(ui_arc))
    }

    /// 创建容器组件
    pub fn container(ui_arc: &Arc<Mutex<Self>>, layout: ContainerLayout) -> ContainerRef {
        let id = format!("container_{}", uuid::Uuid::new_v4());

        let mut ui = ui_arc.lock().unwrap();

        let component = Component {
            id: id.clone(),
            component: ComponentType::Container {
                layout,
                children: Vec::new(),
            },
        };

        ui.components.push(component);

        ContainerRef::new(id, Arc::clone(ui_arc))
    }

    /// 创建水平容器（行布局）
    pub fn horizontal_container(ui_arc: &Arc<Mutex<Self>>) -> ContainerRef {
        Self::container(ui_arc, ContainerLayout::Horizontal)
    }

    /// 创建垂直容器（列布局）
    pub fn vertical_container(ui_arc: &Arc<Mutex<Self>>) -> ContainerRef {
        Self::container(ui_arc, ContainerLayout::Vertical)
    }

    /// 创建网格容器
    pub fn grid_container(ui_arc: &Arc<Mutex<Self>>, columns: u32) -> ContainerRef {
        Self::container(ui_arc, ContainerLayout::Grid { columns })
    }
}

/// 容器构建器 - 提供便捷的方法来构建容器和子组件
pub struct ContainerBuilder {
    container_ref: ContainerRef,
    ui: Arc<Mutex<PluginUi>>,
}

impl ContainerBuilder {
    /// 创建新的容器构建器
    pub fn new(container_ref: ContainerRef, ui: Arc<Mutex<PluginUi>>) -> Self {
        Self { container_ref, ui }
    }

    /// 添加按钮子组件
    pub fn add_button<F>(self, label: &str, icon: Option<&str>, enabled: bool, on_press: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        let _button_ref = PluginUi::button(&self.ui, label, icon, enabled, on_press);

        // 从UI中移除刚创建的按钮组件，然后添加到容器中
        let button_component = {
            let mut ui = self.ui.lock().unwrap();
            ui.components.pop().unwrap() // 刚创建的组件在最后
        };

        self.container_ref.add_child(button_component);
        self
    }

    /// 添加文本子组件
    pub fn add_text(self, value: &str) -> Self {
        let _text_ref = PluginUi::text(&self.ui, value);

        // 从UI中移除刚创建的文本组件，然后添加到容器中
        let text_component = {
            let mut ui = self.ui.lock().unwrap();
            ui.components.pop().unwrap() // 刚创建的组件在最后
        };

        self.container_ref.add_child(text_component);
        self
    }

    /// 添加文本框子组件
    pub fn add_textfield<F>(self, hint: &str, on_submit: F) -> Self
    where
        F: Fn(String) + Send + Sync + 'static,
    {
        let _textfield_ref = PluginUi::textfield(&self.ui, hint, on_submit);

        // 从UI中移除刚创建的文本框组件，然后添加到容器中
        let textfield_component = {
            let mut ui = self.ui.lock().unwrap();
            ui.components.pop().unwrap() // 刚创建的组件在最后
        };

        self.container_ref.add_child(textfield_component);
        self
    }

    /// 添加下拉选择框子组件
    pub fn add_select<F>(self, options: Vec<&str>, on_select: F) -> Self
    where
        F: Fn(String) + Send + Sync + 'static,
    {
        let _select_ref = PluginUi::select(&self.ui, options, on_select);

        // 从UI中移除刚创建的下拉选择框组件，然后添加到容器中
        let select_component = {
            let mut ui = self.ui.lock().unwrap();
            ui.components.pop().unwrap() // 刚创建的组件在最后
        };

        self.container_ref.add_child(select_component);
        self
    }

    /// 添加子容器
    pub fn add_container(self, layout: ContainerLayout) -> NestedContainerBuilder {
        // 创建子容器组件（不添加到主UI中）
        let child_id = format!("container_{}", uuid::Uuid::new_v4());
        let child_component = Component {
            id: child_id.clone(),
            component: ComponentType::Container {
                layout,
                children: Vec::new(),
            },
        };

        // 将子容器添加到父容器中
        self.container_ref.add_child(child_component);

        // 返回嵌套容器构建器，它可以修改父容器中的子容器
        NestedContainerBuilder::new(self.container_ref, child_id, self.ui)
    }

    /// 完成构建并返回容器引用
    pub fn build(self) -> ContainerRef {
        self.container_ref
    }
}

/// 嵌套容器构建器 - 用于构建嵌套在其他容器中的容器
pub struct NestedContainerBuilder {
    parent_container_ref: ContainerRef,
    child_container_id: String,
    ui: Arc<Mutex<PluginUi>>,
}

impl NestedContainerBuilder {
    /// 创建新的嵌套容器构建器
    pub fn new(parent_container_ref: ContainerRef, child_container_id: String, ui: Arc<Mutex<PluginUi>>) -> Self {
        Self {
            parent_container_ref,
            child_container_id,
            ui,
        }
    }

    /// 添加按钮到嵌套容器
    pub fn add_button<F>(self, label: &str, icon: Option<&str>, enabled: bool, on_press: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        let action_id = format!("{}_btn_{}", self.child_container_id, uuid::Uuid::new_v4());

        // 存储回调函数
        {
            let mut ui = self.ui.lock().unwrap();
            ui.actions.insert(
                action_id.clone(),
                Box::new(move |_| {
                    on_press();
                }),
            );
        }

        let button_component = Component {
            id: format!("{}_btn_{}", self.child_container_id, uuid::Uuid::new_v4()),
            component: ComponentType::Button {
                label: label.to_string(),
                icon: icon.map(|s| s.to_string()),
                enabled,
                action_id,
            },
        };

        // 将按钮添加到嵌套容器中
        self.add_child_to_nested_container(button_component);
        self
    }

    /// 添加文本到嵌套容器
    pub fn add_text(self, value: &str) -> Self {
        let text_component = Component {
            id: format!("{}_text_{}", self.child_container_id, uuid::Uuid::new_v4()),
            component: ComponentType::Text {
                value: value.to_string(),
            },
        };

        self.add_child_to_nested_container(text_component);
        self
    }

    /// 将子组件添加到嵌套容器中
    fn add_child_to_nested_container(&self, child: Component) {
        let mut ui = self.ui.lock().unwrap();
        if let Some(parent_component) = ui.find_component_mut(&self.parent_container_ref.id) {
            match &mut parent_component.component {
                ComponentType::Container { children, .. } => {
                    // 找到目标子容器并添加组件
                    for child_container in children.iter_mut() {
                        if child_container.id == self.child_container_id {
                            match &mut child_container.component {
                                ComponentType::Container { children: nested_children, .. } => {
                                    nested_children.push(child);
                                    ui.notify_update();
                                    return;
                                }
                                _ => {}
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    /// 完成构建并返回父容器引用
    pub fn build(self) -> ContainerRef {
        self.parent_container_ref
    }
}

impl PluginUi {
    /// 创建容器构建器 - 提供流畅的API
    pub fn build_horizontal_container(ui_arc: &Arc<Mutex<Self>>) -> ContainerBuilder {
        let container_ref = Self::horizontal_container(ui_arc);
        ContainerBuilder::new(container_ref, Arc::clone(ui_arc))
    }

    /// 创建垂直容器构建器
    pub fn build_vertical_container(ui_arc: &Arc<Mutex<Self>>) -> ContainerBuilder {
        let container_ref = Self::vertical_container(ui_arc);
        ContainerBuilder::new(container_ref, Arc::clone(ui_arc))
    }

    /// 创建网格容器构建器
    pub fn build_grid_container(ui_arc: &Arc<Mutex<Self>>, columns: u32) -> ContainerBuilder {
        let container_ref = Self::grid_container(ui_arc, columns);
        ContainerBuilder::new(container_ref, Arc::clone(ui_arc))
    }
}
