use plugin_interfaces::{log_info, log_warn};
use serde::{Deserialize, Serialize};
use std::{fs, io::Cursor};
use walkdir::WalkDir;
use zip::ZipArchive;

use crate::plugins::{config::{DownloadConfig, PlatformDownload, PluginConfig}, directories::{get_plugin_repository_directory, get_plugin_repository_root, get_root_plugin_installed_directory}};

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

/// 下载GitHub仓库的响应结构
#[derive(serde::Serialize)]
pub struct DownloadResponse {
    pub success: bool,
    pub message: String,
    pub download_path: Option<String>,
}

struct RepoInfo {
    owner: String,
    name: String,
}

const GITHUB_PLUGIN_REPO: &str = "https://github.com/luodeb/chat-client-plugin";

/// 从GitHub URL提取仓库信息
fn extract_repo_info(url: &str) -> Result<RepoInfo, String> {
    // 移除可能的.git后缀和尾部斜杠
    let clean_url = url.trim_end_matches('/').trim_end_matches(".git");

    // 分割URL获取路径部分
    let parts: Vec<&str> = clean_url.split('/').collect();

    if parts.len() < 5 || parts[2] != "github.com" {
        return Err("无效的GitHub仓库URL格式".to_string());
    }

    let owner = parts[3].to_string();
    let name = parts[4].to_string();

    if owner.is_empty() || name.is_empty() {
        return Err("无法从URL中提取仓库所有者或名称".to_string());
    }

    Ok(RepoInfo { owner, name })
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
        let repo_dir = get_plugin_repository_directory();

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
        let install_dir = get_root_plugin_installed_directory();
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
        let source_config_path = get_plugin_repository_directory()
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

    pub async fn download_github_repo(&self) -> Result<DownloadResponse, String> {
        let repo_url: String = GITHUB_PLUGIN_REPO.to_string();
        // 验证URL格式
        if !repo_url.starts_with("https://github.com/") {
            return Ok(DownloadResponse {
                success: false,
                message: "无效的GitHub仓库URL".to_string(),
                download_path: None,
            });
        }

        // 从URL提取仓库信息
        let repo_info = extract_repo_info(&repo_url)?;
        let zip_url = format!(
            "https://github.com/{}/{}/archive/refs/heads/main.zip",
            repo_info.owner, repo_info.name
        );

        // 获取 home_dir/.chat_client
        let plugins_dir = get_plugin_repository_root();

        // 确保plugins目录不存在
        if !plugins_dir.exists() {
            fs::create_dir_all(&plugins_dir).map_err(|e| format!("无法创建plugins目录: {}", e))?;
        }

        // 下载ZIP文件
        let client = reqwest::Client::new();
        let response = client
            .get(&zip_url)
            .send()
            .await
            .map_err(|e| format!("下载失败: {}", e))?;

        if !response.status().is_success() {
            return Ok(DownloadResponse {
                success: false,
                message: format!("下载失败，HTTP状态码: {}", response.status()),
                download_path: None,
            });
        }

        let zip_data = response
            .bytes()
            .await
            .map_err(|e| format!("读取下载数据失败: {}", e))?;

        // 解压ZIP文件
        let cursor = Cursor::new(zip_data);
        let mut archive = ZipArchive::new(cursor).map_err(|e| format!("无法打开ZIP文件: {}", e))?;

        // 目标目录路径
        let target_dir = plugins_dir.join(&repo_info.name);

        // 如果目标目录已存在，先删除
        if target_dir.exists() {
            fs::remove_dir_all(&target_dir).map_err(|e| format!("无法删除现有目录: {}", e))?;
        }

        // 解压文件
        for i in 0..archive.len() {
            let mut file = archive
                .by_index(i)
                .map_err(|e| format!("无法读取ZIP文件条目: {}", e))?;

            let outpath = match file.enclosed_name() {
                Some(path) => {
                    // 移除ZIP文件中的根目录前缀（通常是 repo-name-main/）
                    let path_components: Vec<_> = path.components().collect();
                    if path_components.len() > 1 {
                        let relative_path: std::path::PathBuf =
                            path_components[1..].iter().collect();
                        target_dir.join(relative_path)
                    } else {
                        continue; // 跳过根目录本身
                    }
                }
                None => continue,
            };

            if file.name().ends_with('/') {
                // 创建目录
                fs::create_dir_all(&outpath)
                    .map_err(|e| format!("无法创建目录 {:?}: {}", outpath, e))?;
            } else {
                // 创建文件
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        fs::create_dir_all(p)
                            .map_err(|e| format!("无法创建父目录 {:?}: {}", p, e))?;
                    }
                }

                let mut outfile = fs::File::create(&outpath)
                    .map_err(|e| format!("无法创建文件 {:?}: {}", outpath, e))?;

                std::io::copy(&mut file, &mut outfile)
                    .map_err(|e| format!("无法写入文件 {:?}: {}", outpath, e))?;
            }
        }

        Ok(DownloadResponse {
            success: true,
            message: format!("成功下载并解压仓库到: {:?}", target_dir),
            download_path: Some(target_dir.to_string_lossy().to_string()),
        })
    }
}
