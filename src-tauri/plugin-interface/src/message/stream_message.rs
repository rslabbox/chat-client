use crate::{log_info, send_to_frontend, PluginHandler};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

/// 流式传输错误类型
#[derive(Debug, Clone)]
pub enum StreamError {
    SendFailed,
    InvalidStreamId,
    StreamNotFound,
    StreamAlreadyEnded,
    InvalidState,
}

impl std::fmt::Display for StreamError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StreamError::SendFailed => write!(f, "Failed to send message to frontend"),
            StreamError::InvalidStreamId => write!(f, "Invalid stream ID"),
            StreamError::StreamNotFound => write!(f, "Stream not found"),
            StreamError::StreamAlreadyEnded => write!(f, "Stream already ended"),
            StreamError::InvalidState => write!(f, "Invalid stream state"),
        }
    }
}

impl std::error::Error for StreamError {}

/// 流状态
#[derive(Debug, Clone, PartialEq)]
pub enum StreamStatus {
    Active,
    Paused,
    Finalizing,
    Completed,
    Error,
    Cancelled,
}

/// 流信息
#[derive(Debug, Clone)]
pub struct StreamInfo {
    pub id: String,
    pub plugin_id: String,
    pub stream_type: String,
    pub status: StreamStatus,
    pub created_at: u64,
    pub metadata: Option<String>,
}

/// 流式消息基础结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamMessageWrapper {
    pub r#type: String,
    pub plugin_id: String,
    pub data: StreamMessageData,
    pub timestamp: u64,
}

/// 流式消息数据联合体
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StreamMessageData {
    Start(StreamStartData),
    Data(StreamDataData),
    End(StreamEndData),
    Control(StreamControlData),
}

/// 流开始消息数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamStartData {
    pub stream_id: String,
    pub stream_type: String,
    pub metadata: Option<String>,
}

/// 流数据消息数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamDataData {
    pub stream_id: String,
    pub chunk: String,
    pub is_final: bool,
}

/// 流结束消息数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamEndData {
    pub stream_id: String,
    pub success: bool,
    pub error: Option<String>,
}

/// 流控制消息数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamControlData {
    pub stream_id: String,
}

/// 全局流管理器
static STREAM_MANAGER: std::sync::LazyLock<Arc<Mutex<HashMap<String, StreamInfo>>>> =
    std::sync::LazyLock::new(|| Arc::new(Mutex::new(HashMap::new())));

/// 生成唯一的流ID
fn generate_stream_id() -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("stream_{}", timestamp)
}

/// 发送流式消息到前端
fn send_stream_message_to_frontend(
    plugin_id: &str,
    message_type: &str,
    data: StreamMessageData,
) -> bool {
    let wrapper = StreamMessageWrapper {
        r#type: message_type.to_string(),
        plugin_id: plugin_id.to_string(),
        data,
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64,
    };

    match serde_json::to_string(&wrapper) {
        Ok(payload) => send_to_frontend("plugin-stream", &payload),
        Err(_) => false,
    }
}

/// 插件流式消息发送器
pub trait PluginStreamMessage {
    /// 开始流式传输，返回流ID
    fn send_message_stream_start(
        &self,
        stream_type: &str,
        metadata: Option<&str>,
    ) -> Result<String, StreamError>;

    /// 发送流式数据块
    fn send_message_stream(
        &self,
        stream_id: &str,
        chunk: &str,
        is_final: bool,
    ) -> Result<(), StreamError>;

    /// 结束流式传输
    fn send_message_stream_end(
        &self,
        stream_id: &str,
        success: bool,
        error_msg: Option<&str>,
    ) -> Result<(), StreamError>;

    /// 暂停流式传输
    fn send_message_stream_pause(&self, stream_id: &str) -> Result<(), StreamError>;

    /// 恢复流式传输
    fn send_message_stream_resume(&self, stream_id: &str) -> Result<(), StreamError>;

    /// 取消流式传输
    fn send_message_stream_cancel(&self, stream_id: &str) -> Result<(), StreamError>;

    /// 获取流状态
    fn get_stream_status(&self, stream_id: &str) -> Option<StreamStatus>;

    /// 列出活跃的流
    fn list_active_streams(&self) -> Vec<String>;

    /// 批量发送流式数据
    fn send_message_stream_batch(
        &self,
        stream_id: &str,
        chunks: &[&str],
    ) -> Result<(), StreamError>;
}

impl<T: PluginHandler> PluginStreamMessage for T {
    fn send_message_stream_start(
        &self,
        stream_type: &str,
        metadata: Option<&str>,
    ) -> Result<String, StreamError> {
        let stream_id = generate_stream_id();
        let plugin_id = self.get_metadata().id.clone();

        log_info!("Starting stream: {} {}", stream_id, plugin_id);

        let data = StreamMessageData::Start(StreamStartData {
            stream_id: stream_id.clone(),
            stream_type: stream_type.to_string(),
            metadata: metadata.map(|s| s.to_string()),
        });

        if send_stream_message_to_frontend(&plugin_id, "stream_start", data) {
            // 记录流信息
            if let Ok(mut manager) = STREAM_MANAGER.lock() {
                let stream_info = StreamInfo {
                    id: stream_id.clone(),
                    plugin_id: plugin_id.clone(),
                    stream_type: stream_type.to_string(),
                    status: StreamStatus::Active,
                    created_at: SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                    metadata: metadata.map(|s| s.to_string()),
                };
                manager.insert(stream_id.clone(), stream_info);
            }
            Ok(stream_id)
        } else {
            Err(StreamError::SendFailed)
        }
    }

    fn send_message_stream(
        &self,
        stream_id: &str,
        chunk: &str,
        is_final: bool,
    ) -> Result<(), StreamError> {
        // 检查流是否存在且状态有效
        {
            let manager = STREAM_MANAGER
                .lock()
                .map_err(|_| StreamError::InvalidState)?;
            match manager.get(stream_id) {
                Some(stream_info) => match stream_info.status {
                    StreamStatus::Active | StreamStatus::Finalizing => {}
                    StreamStatus::Paused => return Err(StreamError::InvalidState),
                    StreamStatus::Completed | StreamStatus::Error | StreamStatus::Cancelled => {
                        return Err(StreamError::StreamAlreadyEnded);
                    }
                },
                None => return Err(StreamError::StreamNotFound),
            }
        }

        let plugin_id = self.get_metadata().id.clone();
        let data = StreamMessageData::Data(StreamDataData {
            stream_id: stream_id.to_string(),
            chunk: chunk.to_string(),
            is_final,
        });

        if send_stream_message_to_frontend(&plugin_id, "stream_data", data) {
            // 更新流状态
            if is_final {
                if let Ok(mut manager) = STREAM_MANAGER.lock() {
                    if let Some(stream_info) = manager.get_mut(stream_id) {
                        stream_info.status = StreamStatus::Finalizing;
                    }
                }
            }
            Ok(())
        } else {
            Err(StreamError::SendFailed)
        }
    }

    fn send_message_stream_end(
        &self,
        stream_id: &str,
        success: bool,
        error_msg: Option<&str>,
    ) -> Result<(), StreamError> {
        // 检查流是否存在
        {
            let manager = STREAM_MANAGER
                .lock()
                .map_err(|_| StreamError::InvalidState)?;
            if !manager.contains_key(stream_id) {
                return Err(StreamError::StreamNotFound);
            }
        }

        let plugin_id = self.get_metadata().id.clone();
        let data = StreamMessageData::End(StreamEndData {
            stream_id: stream_id.to_string(),
            success,
            error: error_msg.map(|s| s.to_string()),
        });

        if send_stream_message_to_frontend(&plugin_id, "stream_end", data) {
            // 更新流状态
            if let Ok(mut manager) = STREAM_MANAGER.lock() {
                if let Some(stream_info) = manager.get_mut(stream_id) {
                    stream_info.status = if success {
                        StreamStatus::Completed
                    } else {
                        StreamStatus::Error
                    };
                }
            }
            Ok(())
        } else {
            Err(StreamError::SendFailed)
        }
    }

    fn send_message_stream_pause(&self, stream_id: &str) -> Result<(), StreamError> {
        let mut manager = STREAM_MANAGER
            .lock()
            .map_err(|_| StreamError::InvalidState)?;
        match manager.get_mut(stream_id) {
            Some(stream_info) => {
                if stream_info.status == StreamStatus::Active {
                    stream_info.status = StreamStatus::Paused;

                    let plugin_id = self.get_metadata().id.clone();
                    let data = StreamMessageData::Control(StreamControlData {
                        stream_id: stream_id.to_string(),
                    });

                    if send_stream_message_to_frontend(&plugin_id, "stream_pause", data) {
                        Ok(())
                    } else {
                        // 回滚状态
                        stream_info.status = StreamStatus::Active;
                        Err(StreamError::SendFailed)
                    }
                } else {
                    Err(StreamError::InvalidState)
                }
            }
            None => Err(StreamError::StreamNotFound),
        }
    }

    fn send_message_stream_resume(&self, stream_id: &str) -> Result<(), StreamError> {
        let mut manager = STREAM_MANAGER
            .lock()
            .map_err(|_| StreamError::InvalidState)?;
        match manager.get_mut(stream_id) {
            Some(stream_info) => {
                if stream_info.status == StreamStatus::Paused {
                    stream_info.status = StreamStatus::Active;

                    let plugin_id = self.get_metadata().id.clone();
                    let data = StreamMessageData::Control(StreamControlData {
                        stream_id: stream_id.to_string(),
                    });

                    if send_stream_message_to_frontend(&plugin_id, "stream_resume", data) {
                        Ok(())
                    } else {
                        // 回滚状态
                        stream_info.status = StreamStatus::Paused;
                        Err(StreamError::SendFailed)
                    }
                } else {
                    Err(StreamError::InvalidState)
                }
            }
            None => Err(StreamError::StreamNotFound),
        }
    }

    fn send_message_stream_cancel(&self, stream_id: &str) -> Result<(), StreamError> {
        let mut manager = STREAM_MANAGER
            .lock()
            .map_err(|_| StreamError::InvalidState)?;
        match manager.get_mut(stream_id) {
            Some(stream_info) => match stream_info.status {
                StreamStatus::Active | StreamStatus::Paused | StreamStatus::Finalizing => {
                    stream_info.status = StreamStatus::Cancelled;

                    let plugin_id = self.get_metadata().id.clone();
                    let data = StreamMessageData::Control(StreamControlData {
                        stream_id: stream_id.to_string(),
                    });

                    if send_stream_message_to_frontend(&plugin_id, "stream_cancel", data) {
                        Ok(())
                    } else {
                        Err(StreamError::SendFailed)
                    }
                }
                _ => Err(StreamError::InvalidState),
            },
            None => Err(StreamError::StreamNotFound),
        }
    }

    fn get_stream_status(&self, stream_id: &str) -> Option<StreamStatus> {
        if let Ok(manager) = STREAM_MANAGER.lock() {
            manager.get(stream_id).map(|info| info.status.clone())
        } else {
            None
        }
    }

    fn list_active_streams(&self) -> Vec<String> {
        if let Ok(manager) = STREAM_MANAGER.lock() {
            let plugin_id = self.get_metadata().id.clone();
            manager
                .iter()
                .filter(|(_, info)| {
                    info.plugin_id == plugin_id
                        && matches!(
                            info.status,
                            StreamStatus::Active | StreamStatus::Paused | StreamStatus::Finalizing
                        )
                })
                .map(|(id, _)| id.clone())
                .collect()
        } else {
            Vec::new()
        }
    }

    fn send_message_stream_batch(
        &self,
        stream_id: &str,
        chunks: &[&str],
    ) -> Result<(), StreamError> {
        // 检查流是否存在且状态有效
        {
            let manager = STREAM_MANAGER
                .lock()
                .map_err(|_| StreamError::InvalidState)?;
            match manager.get(stream_id) {
                Some(stream_info) => match stream_info.status {
                    StreamStatus::Active | StreamStatus::Finalizing => {}
                    StreamStatus::Paused => return Err(StreamError::InvalidState),
                    StreamStatus::Completed | StreamStatus::Error | StreamStatus::Cancelled => {
                        return Err(StreamError::StreamAlreadyEnded);
                    }
                },
                None => return Err(StreamError::StreamNotFound),
            }
        }

        let plugin_id = self.get_metadata().id.clone();

        for (i, chunk) in chunks.iter().enumerate() {
            let is_final = i == chunks.len() - 1;
            let data = StreamMessageData::Data(StreamDataData {
                stream_id: stream_id.to_string(),
                chunk: chunk.to_string(),
                is_final,
            });

            if !send_stream_message_to_frontend(&plugin_id, "stream_data", data) {
                return Err(StreamError::SendFailed);
            }
        }

        // 更新流状态
        if !chunks.is_empty() {
            if let Ok(mut manager) = STREAM_MANAGER.lock() {
                if let Some(stream_info) = manager.get_mut(stream_id) {
                    stream_info.status = StreamStatus::Finalizing;
                }
            }
        }

        Ok(())
    }
}
