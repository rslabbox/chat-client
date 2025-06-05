use serde::{Deserialize, Serialize};
use std::os::raw::c_char;

/// 插件元数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    pub id: String,
    pub disabled: bool,
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: Option<String>,
    pub library_path: Option<String>, // 动态库文件路径
    pub config_path: String,          // 配置文件路径
}

/// FFI安全的插件元数据结构
/// 使用C风格的字符串指针
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PluginMetadataFFI {
    pub id: *const c_char,
    pub disabled: bool,
    pub name: *const c_char,
    pub description: *const c_char,
    pub version: *const c_char,
    pub author: *const c_char, // 如果为null表示None
    pub library_path: *const c_char, // 如果为null表示None
    pub config_path: *const c_char,
}

impl PluginMetadata {
    /// 转换为FFI安全的结构
    /// 注意：调用者需要负责释放返回的字符串内存
    pub fn to_ffi(&self) -> PluginMetadataFFI {
        use std::ffi::CString;

        let id = CString::new(self.id.clone()).unwrap().into_raw();
        let name = CString::new(self.name.clone()).unwrap().into_raw();
        let description = CString::new(self.description.clone()).unwrap().into_raw();
        let version = CString::new(self.version.clone()).unwrap().into_raw();
        let config_path = CString::new(self.config_path.clone()).unwrap().into_raw();

        let author = if let Some(ref author) = self.author {
            CString::new(author.clone()).unwrap().into_raw()
        } else {
            std::ptr::null()
        };

        let library_path = if let Some(ref path) = self.library_path {
            CString::new(path.clone()).unwrap().into_raw()
        } else {
            std::ptr::null()
        };

        PluginMetadataFFI {
            id,
            disabled: self.disabled,
            name,
            description,
            version,
            author,
            library_path,
            config_path,
        }
    }
}

/// 释放FFI元数据结构中的字符串内存
/// 必须在不再使用PluginMetadataFFI时调用
pub unsafe fn free_plugin_metadata_ffi(metadata: PluginMetadataFFI) {
    use std::ffi::CString;

    if !metadata.id.is_null() {
        let _ = CString::from_raw(metadata.id as *mut c_char);
    }
    if !metadata.name.is_null() {
        let _ = CString::from_raw(metadata.name as *mut c_char);
    }
    if !metadata.description.is_null() {
        let _ = CString::from_raw(metadata.description as *mut c_char);
    }
    if !metadata.version.is_null() {
        let _ = CString::from_raw(metadata.version as *mut c_char);
    }
    if !metadata.config_path.is_null() {
        let _ = CString::from_raw(metadata.config_path as *mut c_char);
    }
    if !metadata.author.is_null() {
        let _ = CString::from_raw(metadata.author as *mut c_char);
    }
    if !metadata.library_path.is_null() {
        let _ = CString::from_raw(metadata.library_path as *mut c_char);
    }
}
