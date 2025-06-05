use crate::metadata::PluginMetadataFFI;
use crate::callbacks::HostCallbacks;
use std::os::raw::c_char;

/// FFI安全的插件接口
/// 使用C风格的函数指针而不是trait对象
#[repr(C)]
pub struct PluginInterface {
    pub plugin_ptr: *mut std::ffi::c_void,
    pub initialize: unsafe extern "C" fn(*mut std::ffi::c_void, HostCallbacks) -> i32,
    pub update_ui: unsafe extern "C" fn(*mut std::ffi::c_void, *const std::ffi::c_void, *mut std::ffi::c_void) -> i32,
    pub on_mount: unsafe extern "C" fn(*mut std::ffi::c_void, PluginMetadataFFI) -> i32,
    pub on_dispose: unsafe extern "C" fn(*mut std::ffi::c_void) -> i32,
    pub on_connect: unsafe extern "C" fn(*mut std::ffi::c_void) -> i32,
    pub on_disconnect: unsafe extern "C" fn(*mut std::ffi::c_void) -> i32,
    pub handle_message: unsafe extern "C" fn(*mut std::ffi::c_void, *const c_char, *mut *mut c_char) -> i32,
    pub get_metadata: unsafe extern "C" fn(*mut std::ffi::c_void) -> PluginMetadataFFI,
    pub destroy: unsafe extern "C" fn(*mut std::ffi::c_void),
}

/// 插件创建函数类型
/// 返回FFI安全的插件接口
pub type CreatePluginFn = unsafe extern "C" fn() -> *mut PluginInterface;

/// 插件销毁函数类型
/// 销毁插件接口
pub type DestroyPluginFn = unsafe extern "C" fn(*mut PluginInterface);

/// 插件导出符号名称
pub const CREATE_PLUGIN_SYMBOL: &[u8] = b"create_plugin";
pub const DESTROY_PLUGIN_SYMBOL: &[u8] = b"destroy_plugin";

/// 从PluginHandler trait对象创建FFI安全的插件接口
/// 这个函数帮助插件开发者将trait对象转换为FFI安全的接口
pub fn create_plugin_interface_from_handler(
    handler: Box<dyn crate::handler::PluginHandler>
) -> *mut PluginInterface {
    use std::ffi::{CStr, CString};

    let handler_ptr = Box::into_raw(Box::new(handler)) as *mut std::ffi::c_void;

    // 定义FFI安全的函数包装器
    unsafe extern "C" fn initialize_wrapper(ptr: *mut std::ffi::c_void, callbacks: HostCallbacks) -> i32 {
        let handler = &*(ptr as *mut Box<dyn crate::handler::PluginHandler>);
        match handler.initialize(callbacks) {
            Ok(_) => 0,
            Err(_) => -1,
        }
    }

    unsafe extern "C" fn update_ui_wrapper(
        ptr: *mut std::ffi::c_void,
        ctx_ptr: *const std::ffi::c_void,
        ui_ptr: *mut std::ffi::c_void
    ) -> i32 {
        let handler = &mut *(ptr as *mut Box<dyn crate::handler::PluginHandler>);
        let ctx = &*(ctx_ptr as *const crate::pluginui::Context);
        let ui = &mut *(ui_ptr as *mut crate::pluginui::Ui);

        handler.update_ui(ctx, ui);
        0
    }

    unsafe extern "C" fn on_mount_wrapper(ptr: *mut std::ffi::c_void, metadata_ffi: PluginMetadataFFI) -> i32 {
        let handler = &mut *(ptr as *mut Box<dyn crate::handler::PluginHandler>);

        // Convert FFI metadata back to Rust struct
        let metadata = crate::metadata::PluginMetadata {
            id: if !metadata_ffi.id.is_null() {
                CStr::from_ptr(metadata_ffi.id).to_string_lossy().to_string()
            } else {
                String::new()
            },
            disabled: metadata_ffi.disabled,
            name: if !metadata_ffi.name.is_null() {
                CStr::from_ptr(metadata_ffi.name).to_string_lossy().to_string()
            } else {
                String::new()
            },
            description: if !metadata_ffi.description.is_null() {
                CStr::from_ptr(metadata_ffi.description).to_string_lossy().to_string()
            } else {
                String::new()
            },
            version: if !metadata_ffi.version.is_null() {
                CStr::from_ptr(metadata_ffi.version).to_string_lossy().to_string()
            } else {
                String::new()
            },
            author: if !metadata_ffi.author.is_null() {
                Some(CStr::from_ptr(metadata_ffi.author).to_string_lossy().to_string())
            } else {
                None
            },
            library_path: if !metadata_ffi.library_path.is_null() {
                Some(CStr::from_ptr(metadata_ffi.library_path).to_string_lossy().to_string())
            } else {
                None
            },
            config_path: if !metadata_ffi.config_path.is_null() {
                CStr::from_ptr(metadata_ffi.config_path).to_string_lossy().to_string()
            } else {
                String::new()
            },
        };

        match handler.on_mount(&metadata) {
            Ok(_) => 0,
            Err(_) => -1,
        }
    }

    unsafe extern "C" fn on_dispose_wrapper(ptr: *mut std::ffi::c_void) -> i32 {
        let handler = &*(ptr as *mut Box<dyn crate::handler::PluginHandler>);
        match handler.on_dispose() {
            Ok(_) => 0,
            Err(_) => -1,
        }
    }

    unsafe extern "C" fn on_connect_wrapper(ptr: *mut std::ffi::c_void) -> i32 {
        let handler = &*(ptr as *mut Box<dyn crate::handler::PluginHandler>);
        match handler.on_connect() {
            Ok(_) => 0,
            Err(_) => -1,
        }
    }

    unsafe extern "C" fn on_disconnect_wrapper(ptr: *mut std::ffi::c_void) -> i32 {
        let handler = &*(ptr as *mut Box<dyn crate::handler::PluginHandler>);
        match handler.on_disconnect() {
            Ok(_) => 0,
            Err(_) => -1,
        }
    }

    unsafe extern "C" fn handle_message_wrapper(
        ptr: *mut std::ffi::c_void,
        message: *const c_char,
        result: *mut *mut c_char
    ) -> i32 {
        let handler = &*(ptr as *mut Box<dyn crate::handler::PluginHandler>);
        let message_str = CStr::from_ptr(message).to_string_lossy();

        match handler.handle_message(&message_str) {
            Ok(response) => {
                let response_cstring = CString::new(response).unwrap();
                *result = response_cstring.into_raw();
                0
            }
            Err(_) => -1,
        }
    }

    unsafe extern "C" fn get_metadata_wrapper(ptr: *mut std::ffi::c_void) -> PluginMetadataFFI {
        let handler = &*(ptr as *mut Box<dyn crate::handler::PluginHandler>);
        let metadata = handler.get_metadata();
        metadata.to_ffi()
    }

    unsafe extern "C" fn destroy_wrapper(ptr: *mut std::ffi::c_void) {
        let _ = Box::from_raw(ptr as *mut Box<dyn crate::handler::PluginHandler>);
    }

    let interface = PluginInterface {
        plugin_ptr: handler_ptr,
        initialize: initialize_wrapper,
        update_ui: update_ui_wrapper,
        on_mount: on_mount_wrapper,
        on_dispose: on_dispose_wrapper,
        on_connect: on_connect_wrapper,
        on_disconnect: on_disconnect_wrapper,
        handle_message: handle_message_wrapper,
        get_metadata: get_metadata_wrapper,
        destroy: destroy_wrapper,
    };

    Box::into_raw(Box::new(interface))
}
