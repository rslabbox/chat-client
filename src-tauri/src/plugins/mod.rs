pub mod config;
pub mod loader;
pub mod manager;
pub mod repository;

pub use config::{PluginConfig, PluginInfo, DownloadConfig, PlatformDownload};
pub use loader::PluginLoader;
pub use manager::PluginManager;
pub use repository::{PluginRepository, AvailablePluginInfo, PluginDownloadResult};
pub use plugin_interfaces::{
    CreatePluginFn, DestroyPluginFn, PluginHandler, PluginMetadata, CREATE_PLUGIN_SYMBOL,
    DESTROY_PLUGIN_SYMBOL,
};
