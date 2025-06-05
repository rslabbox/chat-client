use crate::{send_to_frontend, PluginHandler};

pub fn send_message_to_frontend(plugin_id: &str, payload: &str) -> bool {
    send_to_frontend(
        "plugin-message-response",
        &format!(
            "{{\"plugin\": \"{}\", \"response\": \"{}\"}}",
            plugin_id, payload
        ),
    )
}

/// 插件消息发送器
pub trait PluginMessage {
    /// 向前端发送消息
    fn send_message_to_frontend(&self, payload: &str) -> bool;
}

impl<T: PluginHandler> PluginMessage for T {
    fn send_message_to_frontend(&self, payload: &str) -> bool {
        send_message_to_frontend(self.get_metadata().id.as_str(), payload)
    }
}
