use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 插件配置文件结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginConfig {
    pub plugin: PluginInfo,
    #[serde(default)]
    pub download: Option<DownloadConfig>,
    #[serde(default)]
    pub metadata: HashMap<String, toml::Value>,
}

/// 插件基本信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    pub id: String,
    #[serde(default)]
    pub disabled: bool,
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    #[serde(default)]
    pub avatar: Option<String>,
    #[serde(default)]
    pub homepage: Option<String>,
    #[serde(default)]
    pub repository: Option<String>,
    #[serde(default)]
    pub license: Option<String>,
    #[serde(default)]
    pub keywords: Vec<String>,
    #[serde(default)]
    pub min_client_version: Option<String>,
    #[serde(default)]
    pub max_client_version: Option<String>,
    #[serde(default)]
    pub platform: Vec<String>,
    #[serde(default)]
    pub dependencies: Vec<String>,
    #[serde(default)]
    pub library: String, // 动态库文件名
}

/// 下载配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadConfig {
    #[serde(default)]
    pub windows: Option<PlatformDownload>,
    #[serde(default)]
    pub macos: Option<PlatformDownload>,
    #[serde(default)]
    pub linux: Option<PlatformDownload>,
}

/// 平台特定下载信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformDownload {
    pub checksum: String,
    pub download_url: String,
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
