use serde::{Deserialize, Serialize};

/// 插件元数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    pub id: String,
    pub disabled: bool,
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: Option<String>,
    pub library_path: Option<String>, // 动态库文件路径
    pub config_path: String,          // 配置文件路径
}
