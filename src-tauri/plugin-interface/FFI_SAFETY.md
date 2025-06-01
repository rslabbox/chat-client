# FFI 安全性解决方案

## 问题描述

之前的插件接口使用了 `dyn PluginHandler` trait 对象，这在 FFI (Foreign Function Interface) 中是不安全的，因为：

1. **trait 对象不是 FFI 安全的**：trait 对象在 C 语言中没有等价物
2. **内存布局不确定**：trait 对象的内存布局在不同编译器版本间可能不同
3. **ABI 不稳定**：Rust 的 trait 对象 ABI 不保证稳定性

## 解决方案

我们实现了一个 FFI 安全的插件接口系统：

### 1. FFI 安全的数据结构

```rust
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
```

### 2. FFI 安全的元数据结构

```rust
#[repr(C)]
pub struct PluginMetadataFFI {
    pub id: *const c_char,
    pub disabled: bool,
    pub name: *const c_char,
    pub description: *const c_char,
    pub version: *const c_char,
    pub author: *const c_char,
    pub library_path: *const c_char,
    pub config_path: *const c_char,
}
```

### 3. 转换函数

提供了 `create_plugin_interface_from_handler` 函数来将 trait 对象转换为 FFI 安全的接口：

```rust
pub fn create_plugin_interface_from_handler(
    handler: Box<dyn PluginHandler>
) -> *mut PluginInterface
```

## 使用方法

### 插件开发者

```rust
#[no_mangle]
pub extern "C" fn create_plugin() -> *mut PluginInterface {
    let plugin = MyPlugin::new();
    let handler: Box<dyn PluginHandler> = Box::new(plugin);
    create_plugin_interface_from_handler(handler)
}

#[no_mangle]
pub extern "C" fn destroy_plugin(interface: *mut PluginInterface) {
    if !interface.is_null() {
        unsafe {
            ((*interface).destroy)((*interface).plugin_ptr);
            let _ = Box::from_raw(interface);
        }
    }
}
```

### 主程序

主程序现在使用 FFI 安全的接口调用插件方法：

```rust
// 1. 首先初始化插件（设置回调函数）
let callbacks = create_host_callbacks();
let init_result = unsafe { ((*handler).initialize)((*handler).plugin_ptr, callbacks) };
if init_result != 0 {
    return Err("插件初始化失败");
}

// 2. 然后调用其他插件方法
let result = unsafe { ((*handler).on_mount)((*handler).plugin_ptr) };
if result == 0 {
    // 成功
} else {
    // 失败
}
```

### 回调函数系统

`set_host_callbacks` 函数仍然保留并正常工作：

```rust
// 在插件的 initialize 方法中
fn initialize(&self, callbacks: HostCallbacks) -> Result<(), Box<dyn std::error::Error>> {
    set_host_callbacks(callbacks).map_err(|_| "Failed to set host callbacks")?;
    Ok(())
}
```

这确保了插件可以使用主程序提供的回调函数，如日志记录、前端通信等。

## 优势

1. **FFI 安全**：所有跨 FFI 边界的类型都是 C 兼容的
2. **ABI 稳定**：使用 `#[repr(C)]` 确保内存布局稳定
3. **向后兼容**：插件开发者仍然可以使用熟悉的 trait 接口
4. **内存安全**：正确管理内存分配和释放
5. **错误处理**：使用整数返回码进行错误处理

## 注意事项

1. **内存管理**：插件和主程序都需要正确管理内存
2. **字符串处理**：使用 C 风格字符串，需要手动管理内存
3. **错误处理**：返回码约定：0 表示成功，非 0 表示失败
4. **线程安全**：确保插件在多线程环境中的安全性

这个解决方案消除了 FFI 安全警告，同时保持了插件系统的易用性和功能完整性。
