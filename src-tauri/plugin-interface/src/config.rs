use std::fs;

/// 插件配置结构
#[derive(Debug, Clone)]
pub struct PluginConfig {
    pub id: String,
    pub disabled: bool,
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: Option<String>,
}

impl PluginConfig {
    /// 从config.toml文件读取配置
    pub fn from_file() -> Result<Self, Box<dyn std::error::Error>> {
        // 读取当前目录下的config.toml文件
        let config_content = fs::read_to_string("config.toml")?;

        // 解析TOML配置
        let config: toml::Value = toml::from_str(&config_content)?;

        // 提取插件信息
        let plugin = config.get("plugin")
            .ok_or("Missing [plugin] section in config.toml")?;

        let id = plugin.get("id")
            .and_then(|v| v.as_str())
            .ok_or("Missing 'id' in [plugin] section")?
            .to_string();

        let disabled = plugin.get("disabled")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let name = plugin.get("name")
            .and_then(|v| v.as_str())
            .ok_or("Missing 'name' in [plugin] section")?
            .to_string();

        let description = plugin.get("description")
            .and_then(|v| v.as_str())
            .ok_or("Missing 'description' in [plugin] section")?
            .to_string();

        let version = plugin.get("version")
            .and_then(|v| v.as_str())
            .ok_or("Missing 'version' in [plugin] section")?
            .to_string();

        let author = plugin.get("author")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        Ok(PluginConfig {
            id,
            disabled,
            name,
            description,
            version,
            author,
        })
    }
}
