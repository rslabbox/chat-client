use plugin_interfaces::{log_info, log_warn};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use walkdir::WalkDir;

use crate::plugins::config::{DownloadConfig, PlatformDownload, PluginConfig};

/// 可用插件信息（来自插件仓库）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailablePluginInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub keywords: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_client_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_client_version: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub platform: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download: Option<DownloadConfig>,
}

/// 插件下载结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginDownloadResult {
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installed_path: Option<String>,
}

#[derive(Debug)]
pub struct PluginRepository;

impl PluginRepository {
    pub fn new() -> Self {
        Self
    }

    /// 扫描可用插件列表（从插件仓库目录）
    pub fn scan_available_plugins(&self) -> Vec<AvailablePluginInfo> {
        let mut plugins = Vec::new();

        // 获取插件仓库目录
        let repo_dir = self.get_plugin_repository_directory();

        if !repo_dir.exists() {
            log_warn!("Plugin repository directory does not exist: {:?}", repo_dir);
            return plugins;
        }

        log_info!("Scanning plugin repository: {:?}", repo_dir);

        // 扫描插件仓库目录
        for entry in WalkDir::new(&repo_dir)
            .min_depth(1)
            .max_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_dir() {
                if let Some(plugin_info) = self.load_available_plugin_from_directory(entry.path()) {
                    plugins.push(plugin_info);
                }
            }
        }

        plugins
    }

    /// 下载并安装插件
    pub async fn download_plugin(&self, plugin_id: &str) -> PluginDownloadResult {
        log_info!("开始下载插件: {}", plugin_id);

        // 首先查找插件信息
        let available_plugins = self.scan_available_plugins();
        let plugin_info = match available_plugins.iter().find(|p| p.id == plugin_id) {
            Some(info) => info,
            None => {
                return PluginDownloadResult {
                    success: false,
                    message: format!("插件 {} 未找到", plugin_id),
                    plugin_id: Some(plugin_id.to_string()),
                    installed_path: None,
                };
            }
        };

        // 获取当前平台的下载信息
        let platform_download = match self.get_platform_download_info(&plugin_info.download) {
            Some(download) => download,
            None => {
                return PluginDownloadResult {
                    success: false,
                    message: format!("插件 {} 不支持当前平台", plugin_id),
                    plugin_id: Some(plugin_id.to_string()),
                    installed_path: None,
                };
            }
        };

        // 执行下载
        match self
            .download_and_install_plugin(plugin_info, &platform_download)
            .await
        {
            Ok(installed_path) => PluginDownloadResult {
                success: true,
                message: format!("插件 {} 下载安装成功", plugin_info.name),
                plugin_id: Some(plugin_id.to_string()),
                installed_path: Some(installed_path),
            },
            Err(error) => PluginDownloadResult {
                success: false,
                message: format!("下载插件失败: {}", error),
                plugin_id: Some(plugin_id.to_string()),
                installed_path: None,
            },
        }
    }

    /// 获取插件仓库目录
    fn get_plugin_repository_directory(&self) -> PathBuf {
        let current_dir = std::env::current_dir().unwrap_or_default();

        // 根据当前目录判断是否在src-tauri目录中
        if current_dir.ends_with("src-tauri") {
            current_dir
                .parent()
                .unwrap()
                .join("plugins")
                .join("chat-client-plugin")
                .join("plugins")
        } else {
            current_dir
                .join("plugins")
                .join("chat-client-plugin")
                .join("plugins")
        }
    }

    /// 从目录加载可用插件信息
    fn load_available_plugin_from_directory(
        &self,
        plugin_dir: &std::path::Path,
    ) -> Option<AvailablePluginInfo> {
        let config_path = plugin_dir.join("config.toml");

        if !config_path.exists() {
            return None;
        }

        match PluginConfig::from_file(&config_path) {
            Ok(config) => Some(AvailablePluginInfo {
                id: config.plugin.id,
                name: config.plugin.name,
                version: config.plugin.version,
                description: config.plugin.description,
                author: config.plugin.author,
                avatar: config.plugin.avatar,
                homepage: config.plugin.homepage,
                repository: config.plugin.repository,
                license: config.plugin.license,
                keywords: config.plugin.keywords,
                min_client_version: config.plugin.min_client_version,
                max_client_version: config.plugin.max_client_version,
                platform: config.plugin.platform,
                dependencies: config.plugin.dependencies,
                download: config.download,
            }),
            Err(e) => {
                log_warn!("Failed to load plugin config from {:?}: {}", config_path, e);
                None
            }
        }
    }

    /// 获取当前平台的下载信息
    fn get_platform_download_info<'a>(
        &self,
        download_config: &'a Option<DownloadConfig>,
    ) -> Option<&'a PlatformDownload> {
        let download_config = download_config.as_ref()?;

        if cfg!(target_os = "windows") {
            download_config.windows.as_ref()
        } else if cfg!(target_os = "macos") {
            download_config.macos.as_ref()
        } else if cfg!(target_os = "linux") {
            download_config.linux.as_ref()
        } else {
            None
        }
    }

    /// 下载并安装插件
    async fn download_and_install_plugin(
        &self,
        plugin_info: &AvailablePluginInfo,
        platform_download: &PlatformDownload,
    ) -> Result<String, String> {
        // 创建HTTP客户端
        let client = reqwest::Client::new();

        // 下载文件
        log_info!("正在下载: {}", platform_download.download_url);
        let response = client
            .get(&platform_download.download_url)
            .send()
            .await
            .map_err(|e| format!("下载失败: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("下载失败，HTTP状态码: {}", response.status()));
        }

        let file_data = response
            .bytes()
            .await
            .map_err(|e| format!("读取下载数据失败: {}", e))?;

        // 获取目标安装目录
        let install_dir = self.get_plugin_install_directory();
        std::fs::create_dir_all(&install_dir).map_err(|e| format!("创建插件目录失败: {}", e))?;

        // 创建插件专用目录
        let plugin_dir = install_dir.join(&plugin_info.id);
        if plugin_dir.exists() {
            std::fs::remove_dir_all(&plugin_dir)
                .map_err(|e| format!("删除现有插件目录失败: {}", e))?;
        }
        std::fs::create_dir_all(&plugin_dir).map_err(|e| format!("创建插件目录失败: {}", e))?;

        // 生成文件名：平台前缀
        let platform_prefix = if cfg!(target_os = "windows") {
            ""
        } else if cfg!(target_os = "macos") || cfg!(target_os = "linux") {
            "lib"
        } else {
            ""
        };

        // 生成文件名：id-version.平台后缀
        let file_extension = if cfg!(target_os = "windows") {
            "dll"
        } else if cfg!(target_os = "macos") {
            "dylib"
        } else {
            "so"
        };

        let library_filename = format!(
            "{}{}-{}.{}",
            platform_prefix, plugin_info.id, plugin_info.version, file_extension
        );
        let library_path = plugin_dir.join(&library_filename);

        log_info!("保存动态链接库到: {:?}", library_path);

        // 保存动态链接库文件
        std::fs::write(&library_path, &file_data)
            .map_err(|e| format!("保存动态链接库失败: {}", e))?;

        // 复制config.toml文件
        let source_config_path = self
            .get_plugin_repository_directory()
            .join(&plugin_info.id)
            .join("config.toml");
        let target_config_path = plugin_dir.join("config.toml");

        if source_config_path.exists() {
            std::fs::copy(&source_config_path, &target_config_path)
                .map_err(|e| format!("复制配置文件失败: {}", e))?;
            log_info!(
                "配置文件已复制: {:?} -> {:?}",
                source_config_path,
                target_config_path
            );
        } else {
            log_warn!("源配置文件不存在: {:?}", source_config_path);

            // 如果源配置文件不存在，创建一个基本的配置文件
            let basic_config = format!(
                r#"[plugin]
id = "{}"
disabled = false
name = "{}"
version = "{}"
description = "{}"
author = "{}"
library = "{}"
"#,
                plugin_info.id,
                plugin_info.name,
                plugin_info.version,
                plugin_info.description,
                plugin_info.author,
                format!("{}-{}", plugin_info.id, plugin_info.version)
            );

            std::fs::write(&target_config_path, basic_config)
                .map_err(|e| format!("创建配置文件失败: {}", e))?;
            log_info!("已创建基本配置文件: {:?}", target_config_path);
        }

        log_info!("插件 {} 安装完成: {:?}", plugin_info.name, plugin_dir);

        // 触发插件扫描以更新插件列表
        log_info!("触发插件重新扫描");

        Ok(plugin_dir.to_string_lossy().to_string())
    }

    /// 获取插件安装目录
    fn get_plugin_install_directory(&self) -> PathBuf {
        let current_dir = std::env::current_dir().unwrap_or_default();

        // 根据当前目录判断是否在src-tauri目录中
        if current_dir.ends_with("src-tauri") {
            current_dir.parent().unwrap().join("plugins")
        } else {
            current_dir.join("plugins")
        }
    }
}
