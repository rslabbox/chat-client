use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // 读取配置文件
    let config_path = "config.toml";
    let config_content = fs::read_to_string(config_path)
        .expect("Failed to read config.toml");
    
    // 解析TOML配置
    let config: toml::Value = toml::from_str(&config_content)
        .expect("Failed to parse config.toml");
    
    // 提取插件信息
    let plugin = config.get("plugin")
        .expect("Missing [plugin] section in config.toml");
    
    let id = plugin.get("id")
        .and_then(|v| v.as_str())
        .expect("Missing 'id' in [plugin] section");
    
    let disabled = plugin.get("disabled")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    
    let name = plugin.get("name")
        .and_then(|v| v.as_str())
        .expect("Missing 'name' in [plugin] section");
    
    let description = plugin.get("description")
        .and_then(|v| v.as_str())
        .expect("Missing 'description' in [plugin] section");
    
    let version = plugin.get("version")
        .and_then(|v| v.as_str())
        .expect("Missing 'version' in [plugin] section");
    
    let author = plugin.get("author")
        .and_then(|v| v.as_str());
    
    let library = plugin.get("library")
        .and_then(|v| v.as_str())
        .expect("Missing 'library' in [plugin] section");
    
    // 生成Rust代码
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("plugin_metadata.rs");

    let generated_code = format!(
        r#"
// 自动生成的插件元数据常量
pub const PLUGIN_ID: &str = "{}";
pub const PLUGIN_DISABLED: bool = {};
pub const PLUGIN_NAME: &str = "{}";
pub const PLUGIN_DESCRIPTION: &str = "{}";
pub const PLUGIN_VERSION: &str = "{}";
pub const PLUGIN_AUTHOR: Option<&str> = {};
pub const PLUGIN_LIBRARY: &str = "{}";

// 生成PluginMetadata的函数
pub fn get_plugin_metadata() -> plugin_interface::PluginMetadata {{
    plugin_interface::PluginMetadata {{
        id: PLUGIN_ID.to_string(),
        disabled: PLUGIN_DISABLED,
        name: PLUGIN_NAME.to_string(),
        description: PLUGIN_DESCRIPTION.to_string(),
        version: PLUGIN_VERSION.to_string(),
        author: PLUGIN_AUTHOR.map(|s| s.to_string()),
        library_path: None, // 运行时设置
        config_path: "".to_string(), // 运行时设置
    }}
}}


"#,
        id,
        disabled,
        name,
        description,
        version,
        if author.is_some() { format!("Some(\"{}\")", author.unwrap()) } else { "None".to_string() },
        library
    );
    
    fs::write(&dest_path, generated_code)
        .expect("Failed to write generated plugin metadata");
    
    // 告诉Cargo当config.toml改变时重新构建
    println!("cargo:rerun-if-changed=config.toml");
}
