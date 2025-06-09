use serde::{Deserialize, Serialize};

/// 插件配置文件结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginConfig {
    pub plugin: PluginInfo,
    #[serde(default)]
    pub metadata: std::collections::HashMap<String, toml::Value>,
}

/// 插件基本信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    pub id: String,
    pub disabled: bool,
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: Option<String>,
    pub library: String, // 动态库文件名
}

impl PluginConfig {
    /// 从 TOML 文件加载插件配置
    pub fn from_file<P: AsRef<std::path::Path>>(
        path: P,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config: PluginConfig = toml::from_str(&content)?;
        Ok(config)
    }
}
