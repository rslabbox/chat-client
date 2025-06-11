use reqwest;
use std::fs;
use std::io::Cursor;
use zip::ZipArchive;

/// 下载GitHub仓库的响应结构
#[derive(serde::Serialize)]
pub struct DownloadResponse {
    pub success: bool,
    pub message: String,
    pub download_path: Option<String>,
}

const GITHUB_PLUGIN_REPO: &str = "https://github.com/luodeb/chat-client-plugin";

/// 下载GitHub仓库到plugins目录
#[tauri::command]
pub async fn download_github_repo() -> Result<DownloadResponse, String> {
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

    // 获取当前工作目录并构建plugins目录路径
    let current_dir = std::env::current_dir().map_err(|e| format!("无法获取当前目录: {}", e))?;

    // 根据当前目录判断是否在src-tauri目录中
    let plugins_dir = if current_dir.ends_with("src-tauri") {
        current_dir.parent().unwrap().join("plugins")
    } else {
        current_dir.join("plugins")
    };

    // 确保plugins目录存在
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
                    let relative_path: std::path::PathBuf = path_components[1..].iter().collect();
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
                    fs::create_dir_all(p).map_err(|e| format!("无法创建父目录 {:?}: {}", p, e))?;
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

/// 仓库信息结构
struct RepoInfo {
    owner: String,
    name: String,
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_repo_info() {
        let url = "https://github.com/luodeb/chat-client-plugin";
        let info = extract_repo_info(url).unwrap();
        assert_eq!(info.owner, "luodeb");
        assert_eq!(info.name, "chat-client-plugin");
    }

    #[test]
    fn test_extract_repo_info_with_git_suffix() {
        let url = "https://github.com/luodeb/chat-client-plugin.git";
        let info = extract_repo_info(url).unwrap();
        assert_eq!(info.owner, "luodeb");
        assert_eq!(info.name, "chat-client-plugin");
    }

    #[test]
    fn test_extract_repo_info_with_trailing_slash() {
        let url = "https://github.com/luodeb/chat-client-plugin/";
        let info = extract_repo_info(url).unwrap();
        assert_eq!(info.owner, "luodeb");
        assert_eq!(info.name, "chat-client-plugin");
    }

    #[test]
    fn test_invalid_url() {
        let url = "https://invalid-url.com/repo";
        assert!(extract_repo_info(url).is_err());
    }
}
