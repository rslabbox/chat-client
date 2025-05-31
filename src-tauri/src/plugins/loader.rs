use std::path::PathBuf;
use walkdir::WalkDir;
use plugin_interface::PluginMetadata;

use crate::plugins::config::PluginConfig;

pub struct PluginLoader;

impl PluginLoader {
    pub fn new() -> Self {
        Self
    }

    /// 扫描并返回插件列表
    /// 从 plugins 目录扫描所有包含 config.toml 的子目录
    pub fn scan_plugins(&self) -> Vec<PluginMetadata> {
        let mut plugins = Vec::new();

        // 获取插件目录路径
        let plugins_dir = self.get_plugins_directory();

        if !plugins_dir.exists() {
            eprintln!("Plugins directory does not exist: {:?}", plugins_dir);
            return plugins;
        }

        // 扫描插件目录
        for entry in WalkDir::new(&plugins_dir)
            .min_depth(1)
            .max_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_dir() {
                if let Some(plugin_metadata) = self.load_plugin_from_directory(entry.path()) {
                    plugins.push(plugin_metadata);
                }
            }
        }

        plugins
    }

    /// 获取插件目录路径
    fn get_plugins_directory(&self) -> PathBuf {
        // 在开发环境中，插件目录位于 src-tauri/src/plugins
        // 在生产环境中，可能位于应用程序目录下的 plugins 文件夹
        let current_dir = std::env::current_dir().unwrap_or_default();

        // 首先尝试开发环境路径（从 src-tauri 目录运行时）
        let dev_path_1 = current_dir.join("src").join("plugins");
        if dev_path_1.exists() {
            return dev_path_1;
        }

        // 然后尝试从项目根目录运行时的路径
        let dev_path_2 = current_dir.join("src-tauri").join("src").join("plugins");
        if dev_path_2.exists() {
            return dev_path_2;
        }

        // 最后尝试生产环境路径
        current_dir.join("plugins")
    }

    /// 从目录加载插件元数据
    fn load_plugin_from_directory(&self, plugin_dir: &std::path::Path) -> Option<PluginMetadata> {
        let config_path = plugin_dir.join("config.toml");

        if !config_path.exists() {
            return None;
        }

        match PluginConfig::from_file(&config_path) {
            Ok(config) => {
                let library_path = self.find_library_file(plugin_dir, &config.plugin.library);

                Some(PluginMetadata {
                    id: config.get_id(),
                    disabled: false, // 默认启用，后续可以从配置中读取
                    name: config.plugin.name,
                    description: config.plugin.description,
                    version: config.plugin.version,
                    author: config.plugin.author,
                    library_path,
                    config_path: config_path.to_string_lossy().to_string(),
                })
            }
            Err(e) => {
                eprintln!("Failed to load plugin config from {:?}: {}", config_path, e);
                None
            }
        }
    }

    /// 查找动态库文件
    fn find_library_file(&self, plugin_dir: &std::path::Path, library_name: &str) -> Option<String> {
        // 直接在插件目录中查找
        let direct_path = plugin_dir.join(library_name);
        if direct_path.exists() {
            return Some(direct_path.to_string_lossy().to_string());
        }

        // 在 target/release 目录中查找（开发环境）
        let target_path = plugin_dir.join("target").join("release").join(library_name);
        if target_path.exists() {
            return Some(target_path.to_string_lossy().to_string());
        }

        // 在 target/debug 目录中查找（开发环境）
        let debug_path = plugin_dir.join("target").join("debug").join(library_name);
        if debug_path.exists() {
            return Some(debug_path.to_string_lossy().to_string());
        }

        None
    }
}

impl Default for PluginLoader {
    fn default() -> Self {
        Self::new()
    }
}
