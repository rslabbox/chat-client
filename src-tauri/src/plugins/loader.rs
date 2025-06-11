use plugin_interfaces::{log_info, log_warn, PluginMetadata};
use walkdir::WalkDir;

use crate::plugins::{config::PluginConfig, directories::get_plugins_directories};

#[derive(Debug)]
pub struct PluginLoader;

impl PluginLoader {
    pub fn new() -> Self {
        Self
    }

    /// 扫描并返回插件列表
    /// 同时扫描 (pwd)/plugins 和 (pwd)/src-tauri/src/plugins 目录下的所有包含 config.toml 的子目录
    pub fn scan_plugins(&self) -> Vec<PluginMetadata> {
        let mut plugins = Vec::new();

        // 获取要扫描的插件目录列表
        let plugin_directories = get_plugins_directories();

        // 扫描每个插件目录
        for plugins_dir in plugin_directories {
            if !plugins_dir.exists() {
                log_warn!("Plugins directory does not exist: {:?}", plugins_dir);
                continue;
            }

            log_info!("Scanning plugins directory: {:?}", plugins_dir);

            // 扫描当前插件目录
            for entry in WalkDir::new(&plugins_dir)
                .min_depth(1)
                .max_depth(1)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                if entry.file_type().is_dir() {
                    if let Some(plugin_metadata) = self.load_plugin_from_directory(entry.path()) {
                        if plugin_metadata.disabled {
                            continue;
                        }

                        // 检查是否已经存在相同ID的插件，避免重复加载
                        if !plugins
                            .iter()
                            .any(|p: &PluginMetadata| p.id == plugin_metadata.id)
                        {
                            plugins.push(plugin_metadata);
                        } else {
                            log_warn!(
                                "Plugin with ID '{}' already loaded, skipping duplicate",
                                plugin_metadata.id
                            );
                        }
                    }
                }
            }
        }

        plugins
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
                    id: config.plugin.id,
                    disabled: config.plugin.disabled, // 默认启用，后续可以从配置中读取
                    name: config.plugin.name,
                    description: config.plugin.description,
                    version: config.plugin.version,
                    author: Some(config.plugin.author),
                    library_path,
                    config_path: config_path.to_string_lossy().to_string(),
                    instance_id: None,
                })
            }
            Err(e) => {
                log_warn!("Failed to load plugin config from {:?}: {}", config_path, e);
                None
            }
        }
    }

    /// 查找动态库文件
    fn find_library_file(
        &self,
        plugin_dir: &std::path::Path,
        library_name: &str,
    ) -> Option<String> {
        // 判断是哪个平台 windows / macos / linux
        let library_name_dylib = if cfg!(target_os = "windows") {
            format!("{}.dll", library_name)
        } else if cfg!(target_os = "macos") {
            format!("lib{}.dylib", library_name)
        } else {
            format!("lib{}.so", library_name)
        };

        // 直接在插件目录中查找
        let direct_path = plugin_dir.join(&library_name_dylib);
        if direct_path.exists() {
            return Some(direct_path.to_string_lossy().to_string());
        }

        // 在插件目录的 target/debug 目录中查找（开发环境）
        let debug_path = plugin_dir
            .join("target")
            .join("debug")
            .join(&library_name_dylib);
        if debug_path.exists() {
            return Some(debug_path.to_string_lossy().to_string());
        }

        // 在插件目录的 target/release 目录中查找（开发环境）
        let target_path = plugin_dir
            .join("target")
            .join("release")
            .join(&library_name_dylib);
        if target_path.exists() {
            return Some(target_path.to_string_lossy().to_string());
        }

        // 在工作空间的 target/release 目录中查找
        let workspace_release_path = std::env::current_dir()
            .unwrap_or_default()
            .join("target")
            .join("release")
            .join(&library_name_dylib);
        if workspace_release_path.exists() {
            return Some(workspace_release_path.to_string_lossy().to_string());
        }

        // 在工作空间的 target/debug 目录中查找
        let workspace_debug_path = std::env::current_dir()
            .unwrap_or_default()
            .join("target")
            .join("debug")
            .join(&library_name_dylib);
        if workspace_debug_path.exists() {
            return Some(workspace_debug_path.to_string_lossy().to_string());
        }

        None
    }
}

impl Default for PluginLoader {
    fn default() -> Self {
        Self::new()
    }
}
