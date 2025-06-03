use plugin_interface::{
    create_plugin_interface_from_handler, create_plugin_ui, log_info, log_warn,
    send_message_to_frontend, ContainerRef, PluginHandler, PluginInterface, PluginMetadata,
    PluginUi, SelectRef, TextRef,
};
use std::sync::{Arc, Mutex};

// 包含构建时生成的元数据
include!(concat!(env!("OUT_DIR"), "/plugin_metadata.rs"));

/// 示例插件实现
pub struct ExamplePlugin {
    name: String,
    ui: Arc<Mutex<PluginUi>>,
    text_value: Arc<Mutex<String>>,
    text_display: Option<TextRef>,
    select_ref: Option<SelectRef>,
    container_ref: Option<ContainerRef>,
}

impl ExamplePlugin {
    pub fn new() -> Self {
        let ui = create_plugin_ui!();

        let mut instance = Self {
            name: PLUGIN_NAME.to_string(),
            ui: Arc::clone(&ui),
            text_value: Arc::new(Mutex::new(String::new())),
            text_display: None,
            select_ref: None,
            container_ref: None,
        };

        // 在创建完实例后初始化UI
        instance.init_ui();
        instance
    }

    fn init_ui(&mut self) {
        let ui_clone = Arc::clone(&self.ui);

        // 创建一个文本显示组件
        let text_display = PluginUi::text(&ui_clone, "欢迎使用示例插件！");
        self.text_display = Some(text_display);

        // 创建一个文本框
        let text_value_clone = Arc::clone(&self.text_value);
        // let text_display_clone = self.text_display.clone();
        let _text_field = PluginUi::textfield(&ui_clone, "输入内容", move |value| {
            if let Ok(mut text) = text_value_clone.lock() {
                *text = value.clone();
            }
            log_info!("收到文本输入: {}", value);
        });

        // _text_field.set_enabled(true); // 这行会编译错误，因为TextFieldRef没有set_enabled方法
        _text_field.set_value("默认值"); // 这行可以编译，因为TextFieldRef有set_value方法

        let text_value_clone = Arc::clone(&self.text_value);
        let _scan_button = PluginUi::button(&ui_clone, "打招呼", None, true, move || {
            let text = text_value_clone.lock().unwrap();
            send_message_to_frontend!("hi, {}", *text);
        });
        // _scan_button.set_enabled(false);

        // 创建下拉选择框
        let select_ref = PluginUi::select(
            &ui_clone,
            vec!["选项1", "选项2", "选项3"],
            move |selected| {
                log_info!("用户选择了: {}", selected);
                send_message_to_frontend!("您选择了: {}", selected);
            },
        );
        self.select_ref = Some(select_ref);

        // 创建水平容器示例（行布局）
        let container_ref = PluginUi::build_horizontal_container(&ui_clone)
            .add_select(vec!["选项1", "选项2", "选项3"], move |selected| {
                log_info!("用户选择了: {}", selected);
                send_message_to_frontend!("您选择了: {}", selected);
            })
            .add_button("右侧按钮", None, true, || {
                log_info!("容器中的按钮被点击了！");
                send_message_to_frontend!("容器按钮被点击");
            })
            .build();
        self.container_ref = Some(container_ref);
    }
}

impl PluginHandler for ExamplePlugin {
    fn on_mount(&self) -> Result<(), Box<dyn std::error::Error>> {
        log_info!("[{}] Plugin mount successfully", self.name);

        Ok(())
    }

    fn on_dispose(&self) -> Result<(), Box<dyn std::error::Error>> {
        log_info!("[{}] Plugin disposed successfully", self.name);
        Ok(())
    }

    fn on_connect(&self) -> Result<(), Box<dyn std::error::Error>> {
        log_info!("[{}] Connected", self.name);
        Ok(())
    }

    fn on_disconnect(&self) -> Result<(), Box<dyn std::error::Error>> {
        log_warn!("[{}] Disconnected", self.name);
        Ok(())
    }

    fn handle_message(&self, message: &str) -> Result<String, Box<dyn std::error::Error>> {
        log_info!("[{}] Received message: {}", self.name, message);

        let response = format!("Echo from {}: {}", self.name, message);

        // 向前端发送响应
        send_message_to_frontend!("收到消息: {}", message);
        Ok(response)
    }

    fn get_metadata(&self) -> PluginMetadata {
        get_plugin_metadata()
    }

    fn get_ui(&self) -> Arc<Mutex<PluginUi>> {
        Arc::clone(&self.ui)
    }
}

/// 创建插件实例的导出函数
#[no_mangle]
pub extern "C" fn create_plugin() -> *mut PluginInterface {
    let plugin = ExamplePlugin::new();
    let handler: Box<dyn PluginHandler> = Box::new(plugin);
    create_plugin_interface_from_handler(handler)
}

/// 销毁插件实例的导出函数
#[no_mangle]
pub extern "C" fn destroy_plugin(interface: *mut PluginInterface) {
    if !interface.is_null() {
        unsafe {
            ((*interface).destroy)((*interface).plugin_ptr);
            let _ = Box::from_raw(interface);
        }
    }
}
