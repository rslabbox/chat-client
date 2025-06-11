use std::path::PathBuf;

pub fn get_plugin_repository_root() -> PathBuf {
    let home_dir = std::env::home_dir().unwrap_or_default();
    home_dir.join(".chat_client")
}

pub fn get_root_plugin_installed_directory() -> PathBuf {
    get_plugin_repository_root().join("installed_plugins")
}

pub fn get_plugins_directories() -> Vec<PathBuf> {
    let current_dir = std::env::current_dir().unwrap_or_default();
    let mut directories = Vec::new();

    // 添加 (pwd)/src-tauri/src/plugins 目录
    let src_plugins_dir = current_dir.join("src").join("plugins");
    directories.push(src_plugins_dir);

    // 添加 ~/.chat_client/installed_plugins 目录
    let installed_plugins_dir = get_root_plugin_installed_directory();
    directories.push(installed_plugins_dir);

    directories
}

pub fn get_repository_directory() -> PathBuf {
    get_plugin_repository_root().join("chat-client-plugin")
}

pub fn get_plugin_repository_directory() -> PathBuf {
    get_repository_directory().join("plugins")
}
