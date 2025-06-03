//! 插件UI组件测试模块

#[cfg(test)]
mod tests {
    use super::super::{ComponentType, PluginUi, ContainerLayout};

    #[test]
    fn test_text_component_creation() {
        let ui = PluginUi::new();
        let _text_ref = PluginUi::text(&ui, "测试文本");
        
        // 验证组件是否被正确创建
        let ui_guard = ui.lock().unwrap();
        assert_eq!(ui_guard.components.len(), 1);
        
        let component = &ui_guard.components[0];
        match &component.component {
            ComponentType::Text { value } => {
                assert_eq!(value, "测试文本");
            }
            _ => panic!("Expected Text component"),
        }
    }

    #[test]
    fn test_text_component_set_value() {
        let ui = PluginUi::new();
        let text_ref = PluginUi::text(&ui, "初始文本");
        
        // 更新文本值
        let success = text_ref.set_value("更新后的文本");
        assert!(success);
        
        // 验证文本是否被正确更新
        let ui_guard = ui.lock().unwrap();
        let component = &ui_guard.components[0];
        match &component.component {
            ComponentType::Text { value } => {
                assert_eq!(value, "更新后的文本");
            }
            _ => panic!("Expected Text component"),
        }
    }

    #[test]
    fn test_type_safety() {
        let ui = PluginUi::new();
        let text_ref = PluginUi::text(&ui, "测试文本");
        let button_ref = PluginUi::button(&ui, "按钮", None, true, || {});
        let textfield_ref = PluginUi::textfield(&ui, "提示", |_| {});
        let select_ref = PluginUi::select(&ui, vec!["选项1", "选项2"], |_| {});

        // 现在每个组件引用只能调用自己类型的方法
        // 这些调用在编译时就会被检查，不正确的调用会导致编译错误

        // Text组件只能调用set_value
        let success = text_ref.set_value("新文本");
        assert!(success);

        // Button组件只能调用set_label和set_enabled
        let success = button_ref.set_label("新标签");
        assert!(success);
        let success = button_ref.set_enabled(false);
        assert!(success);

        // TextField组件只能调用set_value和set_hint
        let success = textfield_ref.set_value("新值");
        assert!(success);
        let success = textfield_ref.set_hint("新提示");
        assert!(success);

        // Select组件只能调用set_value和set_selected
        let success = select_ref.set_value(vec!["新选项1", "新选项2", "新选项3"]);
        assert!(success);
        let success = select_ref.set_selected(Some("新选项2"));
        assert!(success);

        // Container组件测试
        let _container_ref = PluginUi::build_horizontal_container(&ui)
            .add_text("左侧")
            .add_button("右侧", None, true, || {})
            .build();
    }

    #[test]
    fn test_container() {
        let ui = PluginUi::new();

        // 创建水平容器
        let _container_ref = PluginUi::build_horizontal_container(&ui)
            .add_text("测试文本")
            .add_button("测试按钮", Some("refresh"), true, || {
                println!("按钮被点击");
            })
            .build();

        // 验证组件已添加
        let ui_guard = ui.lock().unwrap();
        let components = ui_guard.get_components();
        assert_eq!(components.len(), 1);

        // 验证是容器组件
        match &components[0].component {
            ComponentType::Container { layout, children } => {
                // 验证布局类型
                match layout {
                    ContainerLayout::Horizontal => {},
                    _ => panic!("Expected Horizontal layout"),
                }

                // 验证子组件数量
                assert_eq!(children.len(), 2);

                // 验证第一个子组件是文本
                match &children[0].component {
                    ComponentType::Text { value } => {
                        assert_eq!(value, "测试文本");
                    }
                    _ => panic!("Expected Text component"),
                }

                // 验证第二个子组件是按钮
                match &children[1].component {
                    ComponentType::Button { label, icon, enabled, .. } => {
                        assert_eq!(label, "测试按钮");
                        assert_eq!(icon, &Some("refresh".to_string()));
                        assert!(enabled);
                    }
                    _ => panic!("Expected Button component"),
                }
            }
            _ => panic!("Expected Container component"),
        }
    }

    #[test]
    fn test_nested_containers() {
        let ui = PluginUi::new();

        // 创建嵌套容器：垂直容器包含水平容器
        let _outer_container = PluginUi::build_vertical_container(&ui)
            .add_text("顶部文本")
            .add_container(ContainerLayout::Horizontal)
                .add_button("左按钮", None, true, || {})
                .add_button("右按钮", None, true, || {})
                .build();

        // 验证组件结构
        let ui_guard = ui.lock().unwrap();
        let components = ui_guard.get_components();
        assert_eq!(components.len(), 1);

        // 验证外层容器
        match &components[0].component {
            ComponentType::Container { layout, children } => {
                match layout {
                    ContainerLayout::Vertical => {},
                    _ => panic!("Expected Vertical layout"),
                }

                assert_eq!(children.len(), 2);

                // 验证第一个子组件是文本
                match &children[0].component {
                    ComponentType::Text { value } => {
                        assert_eq!(value, "顶部文本");
                    }
                    _ => panic!("Expected Text component"),
                }

                // 验证第二个子组件是水平容器
                match &children[1].component {
                    ComponentType::Container { layout: inner_layout, children: inner_children } => {
                        match inner_layout {
                            ContainerLayout::Horizontal => {},
                            _ => panic!("Expected Horizontal layout for inner container"),
                        }

                        assert_eq!(inner_children.len(), 2);

                        // 验证内层容器的两个按钮
                        for (i, child) in inner_children.iter().enumerate() {
                            match &child.component {
                                ComponentType::Button { label, .. } => {
                                    let expected_label = if i == 0 { "左按钮" } else { "右按钮" };
                                    assert_eq!(label, expected_label);
                                }
                                _ => panic!("Expected Button component"),
                            }
                        }
                    }
                    _ => panic!("Expected Container component"),
                }
            }
            _ => panic!("Expected Container component"),
        }
    }

    #[test]
    fn test_grid_container() {
        let ui = PluginUi::new();

        // 创建2列网格容器
        let _grid_container = PluginUi::build_grid_container(&ui, 2)
            .add_text("项目1")
            .add_text("项目2")
            .add_button("按钮1", None, true, || {})
            .add_button("按钮2", None, true, || {})
            .build();

        // 验证组件结构
        let ui_guard = ui.lock().unwrap();
        let components = ui_guard.get_components();
        assert_eq!(components.len(), 1);

        match &components[0].component {
            ComponentType::Container { layout, children } => {
                match layout {
                    ContainerLayout::Grid { columns } => {
                        assert_eq!(*columns, 2);
                    },
                    _ => panic!("Expected Grid layout"),
                }

                assert_eq!(children.len(), 4);
            }
            _ => panic!("Expected Container component"),
        }
    }

    #[test]
    fn test_mixed_components() {
        let ui = PluginUi::new();

        // 创建不同类型的组件
        let _text_ref = PluginUi::text(&ui, "文本组件");
        let _button_ref = PluginUi::button(&ui, "按钮", None, true, || {});
        let _textfield_ref = PluginUi::textfield(&ui, "提示", |_| {});

        // 验证组件数量
        let ui_guard = ui.lock().unwrap();
        assert_eq!(ui_guard.components.len(), 3);

        // 验证每个组件的类型
        let components = &ui_guard.components;
        match &components[0].component {
            ComponentType::Text { .. } => {},
            _ => panic!("Expected Text component at index 0"),
        }
        match &components[1].component {
            ComponentType::Button { .. } => {},
            _ => panic!("Expected Button component at index 1"),
        }
        match &components[2].component {
            ComponentType::TextField { .. } => {},
            _ => panic!("Expected TextField component at index 2"),
        }

        // 类型安全现在在编译时保证，不需要运行时测试
        // 如果尝试在错误的组件类型上调用方法，代码将无法编译
    }
}
