use crate::{send_to_frontend, PluginHandler};
use serde_json::json;

/// 消息类型枚举
#[derive(Debug, Clone)]
pub enum MessageType {
    /// 普通消息
    Normal,
    /// 成功消息
    Success,
    /// 警告消息
    Warning,
    /// 错误消息
    Error,
    /// 信息消息
    Info,
}

impl MessageType {
    fn as_str(&self) -> &'static str {
        match self {
            MessageType::Normal => "normal",
            MessageType::Success => "success",
            MessageType::Warning => "warning",
            MessageType::Error => "error",
            MessageType::Info => "info",
        }
    }
}

/// 发送消息到前端（新协议）
pub fn send_message_to_frontend(plugin_id: &str, content: &str, message_type: MessageType) -> bool {
    let payload = json!({
        "type": "plugin_message",
        "plugin_id": plugin_id,
        "content": content,
        "message_type": message_type.as_str(),
        "timestamp": std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
    });

    send_to_frontend("plugin-message", &payload.to_string())
}

/// 发送消息到前端（兼容旧协议）
pub fn send_message_to_frontend_legacy(plugin_id: &str, payload: &str) -> bool {
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
    /// 向前端发送消息（新协议）
    fn send_message_to_frontend(&self, content: &str) -> bool {
        self.send_message_to_frontend_typed(content, MessageType::Normal)
    }

    /// 向前端发送带类型的消息
    fn send_message_to_frontend_typed(&self, content: &str, message_type: MessageType) -> bool;

    /// 向前端发送成功消息
    fn send_success_message(&self, content: &str) -> bool {
        self.send_message_to_frontend_typed(content, MessageType::Success)
    }

    /// 向前端发送错误消息
    fn send_error_message(&self, content: &str) -> bool {
        self.send_message_to_frontend_typed(content, MessageType::Error)
    }

    /// 向前端发送警告消息
    fn send_warning_message(&self, content: &str) -> bool {
        self.send_message_to_frontend_typed(content, MessageType::Warning)
    }

    /// 向前端发送信息消息
    fn send_info_message(&self, content: &str) -> bool {
        self.send_message_to_frontend_typed(content, MessageType::Info)
    }
}

impl<T: PluginHandler> PluginMessage for T {
    fn send_message_to_frontend_typed(&self, content: &str, message_type: MessageType) -> bool {
        send_message_to_frontend(self.get_metadata().id.as_str(), content, message_type)
    }
}


