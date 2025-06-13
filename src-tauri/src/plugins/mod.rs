pub mod config;
pub mod directories;
pub mod loader;
pub mod manager;
pub mod repository;

pub use config::{DownloadConfig, PlatformDownload, PluginConfig, PluginInfo};
pub use loader::PluginLoader;
pub use manager::PluginManager;
pub use plugin_interfaces::{
    CreatePluginFn, DestroyPluginFn, PluginHandler, PluginMetadata, CREATE_PLUGIN_SYMBOL,
    DESTROY_PLUGIN_SYMBOL,
};
pub use repository::{
    AvailablePluginInfo, DownloadResponse, PluginDownloadResult, PluginRepository,
};
