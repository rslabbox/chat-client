use crate::metadata::PluginMetadataFFI;
use crate::callbacks::HostCallbacks;
use std::os::raw::c_char;

/// FFI安全的插件接口
/// 使用C风格的函数指针而不是trait对象
#[repr(C)]
pub struct PluginInterface {
    pub plugin_ptr: *mut std::ffi::c_void,
    pub initialize: unsafe extern "C" fn(*mut std::ffi::c_void, HostCallbacks) -> i32,
    pub on_mount: unsafe extern "C" fn(*mut std::ffi::c_void) -> i32,
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

    unsafe extern "C" fn on_mount_wrapper(ptr: *mut std::ffi::c_void) -> i32 {
        let handler = &*(ptr as *mut Box<dyn crate::handler::PluginHandler>);
        match handler.on_mount() {
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
