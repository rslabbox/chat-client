# Plugin Development Guide

## Overview

This guide provides comprehensive instructions for developing plugins within the chat-client framework. It covers architectural requirements, implementation patterns, and best practices derived from successful plugin development experiences.

## Core Architecture Requirements

### Plugin Handler Trait Implementation

All plugins must implement the `PluginHandler` trait, which defines the standard interface for plugin lifecycle management and system interaction.

```rust
impl PluginHandler for YourPlugin {
    fn on_mount(&mut self, plugin_ctx: &PluginInstanceContext) -> Result<(), Box<dyn std::error::Error>>
    fn on_connect(&mut self, plugin_ctx: &PluginInstanceContext) -> Result<(), Box<dyn std::error::Error>>
    fn on_disconnect(&mut self, plugin_ctx: &PluginInstanceContext) -> Result<(), Box<dyn std::error::Error>>
    fn on_dispose(&mut self, plugin_ctx: &PluginInstanceContext) -> Result<(), Box<dyn std::error::Error>>
    fn handle_message(&mut self, message: &str, plugin_ctx: &PluginInstanceContext) -> Result<String, Box<dyn std::error::Error>>
    fn update_ui(&mut self, ui: &mut egui::Ui)
}
```

### Dependency Management

- Declare all dependencies in `Cargo.toml` with compatible versions
- Pay special attention to async runtime, serialization libraries, and UI framework compatibility
- Common dependencies include: `tokio`, `serde`, `egui`, `chrono`
- Use appropriate feature flags for serialization libraries
- Necessary dependency: `plugin-interfaces = { git = "https://github.com/luodeb/plugin-interfaces.git" }`

### Required Imports

Import essential components from `plugin_interfaces`:

```rust
use plugin_interfaces::{
    // Core traits and types
    PluginHandler, PluginInstanceContext, PluginInterface,
    create_plugin_interface_from_handler,
    
    // UI components
    pluginui::{Context, Ui},
    
    // Logging functions (use these instead of external logging crates)
    log_error, log_warn, log_info, log_debug, log_trace,
};
```

### Configuration Management

Separate plugin configuration into two categories:
- **Static Configuration**: Plugin metadata and core settings (embedded in code)
- **User Configuration**: User-modifiable settings (external `.toml` file)

## Lifecycle Method Responsibilities

### on_mount Function
**Purpose**: Plugin initialization and resource allocation

**Responsibilities**:
- Create async runtime (`tokio::Runtime`)
- Initialize shared resources (Arc, Mutex)
- Set up initial state
- Allocate necessary system resources
- Scan for external devices or services (if applicable)

**Error Handling**: Must handle runtime creation failures gracefully

### on_connect Function
**Purpose**: Establish connections to external services or devices

**Critical Requirements**:
- Use synchronous waiting mechanism (`runtime.block_on`) to ensure connection results are properly captured
- Return actual connection status - never return `Ok(())` when connection fails
- Provide specific error messages for connection failures
- Validate connection prerequisites before attempting connection

**Common Pattern**:
```rust
let result = runtime.block_on(async move {
    self.connect_to_external_service().await
});

match result {
    Ok(_) => Ok(()),
    Err(e) => Err(e.into())
}
```

### on_disconnect Function
**Purpose**: Clean up connections and reset state

**Responsibilities**:
- Close all network connections, file handles, or device connections
- Reset connection state variables
- Clean up temporary resources
- Prevent resource leaks

### on_dispose Function
**Purpose**: Plugin cleanup and resource deallocation

**Responsibilities**:
- Save user configuration to persistent storage
- Terminate background tasks
- Release all system resources
- Clean up async runtime (if owned by plugin)

### handle_message Function
**Purpose**: Process user input and external messages

**Implementation Guidelines**:
- Check connection state before processing messages
- Provide appropriate feedback for different connection states
- Handle various message formats as required by plugin functionality
- Return meaningful response messages

### update_ui Function
**Purpose**: Render plugin user interface

**Design Principles**:
- Include only essential UI elements
- Avoid redundant control buttons (use lifecycle methods instead)
- Provide clear status indicators
- Implement configuration interfaces for user-modifiable settings
- Maintain responsive and intuitive user experience

## Error Handling Best Practices

### Synchronous-Asynchronous Integration
- Use `runtime.block_on` for critical operations in lifecycle methods
- Avoid `spawn` for operations that need error propagation
- Ensure errors are properly captured and returned to the framework

### Error Type Standardization
- Define consistent error types using custom error enums
- Provide context-rich error messages
- Include sufficient debugging information
- Map external errors to plugin-specific error types

### State Consistency
- Maintain consistency between internal state and external reality
- Implement proper state validation
- Handle edge cases where external state changes unexpectedly

## Logging Best Practices

### Using Plugin Interface Logging Functions

All plugins must use the standardized logging functions provided by `plugin_interfaces` instead of external logging crates like `log` or `env_logger`. This ensures consistent log formatting and integration with the chat-client framework.

### Available Logging Macros

Import the logging macros from `plugin_interfaces`:

```rust
use plugin_interfaces::{log_error, log_warn, log_info, log_debug, log_trace};
```

### Logging Level Guidelines

**log_error!** - Critical errors that prevent plugin functionality
```rust
log_error!("Failed to connect to device: {}", error_message);
log_error!("[{}] Critical configuration error", plugin_name);
```

**log_warn!** - Warning conditions that should be noted but don't prevent operation
```rust
log_warn!("Configuration file not found, using defaults");
log_warn!("[{}] Connection timeout, retrying", plugin_name);
```

**log_info!** - General information about plugin operations
```rust
log_info!("[{}] Plugin mounted successfully", plugin_name);
log_info!("Connected to device: {}", device_name);
log_info!("Data sent: {} bytes", data_length);
```

**log_debug!** - Detailed information for debugging purposes
```rust
log_debug!("Processing message: {}", message);
log_debug!("Configuration loaded: {:?}", config);
```

**log_trace!** - Very detailed tracing information
```rust
log_trace!("Entering function: connect_device");
log_trace!("Raw data received: {:?}", raw_data);
```

### Logging Best Practices

**Context Information**: Always include plugin name and relevant context
```rust
let metadata = plugin_ctx.get_metadata();
log_info!("[{}] Operation completed successfully", metadata.name);
```

**Error Context**: Provide sufficient context for debugging
```rust
log_error!("[{}] Failed to parse configuration: {} at line {}", 
           plugin_name, error, line_number);
```

**Structured Logging**: Use consistent formatting for similar operations
```rust
// Connection events
log_info!("[{}] Connecting to {}", plugin_name, target);
log_info!("[{}] Connected successfully", plugin_name);
log_warn!("[{}] Connection failed: {}", plugin_name, error);

// Data operations
log_debug!("Sending data: {} bytes", data.len());
log_debug!("Received data: {} bytes", received.len());
```

**Performance Considerations**: Avoid expensive operations in log statements
```rust
// Good - simple formatting
log_debug!("Processing {} items", items.len());

// Avoid - expensive serialization in log statement
// log_debug!("Items: {:?}", expensive_serialize(&items));
```

**Security**: Never log sensitive information
```rust
// Good
log_info!("Authentication successful for user");

// Avoid
// log_info!("Password: {}", password);
// log_debug!("API key: {}", api_key);
```

## User Interface Guidelines

### Functional Separation
- UI components should focus on display and basic interaction
- Implement business logic through lifecycle methods
- Avoid complex logic processing in UI code

### Status Feedback Systems
- Provide clear connection status indicators
- Display operation progress when applicable
- Show error messages in user-friendly format
- Implement real-time status updates

### Configuration Interface Design
- Present only necessary configuration options in UI
- Use external configuration files for advanced settings
- Implement real-time validation for configuration changes
- Provide sensible default values

## Resource Management Strategies

### Async Runtime Management
- Create tokio runtime during plugin initialization
- Maintain runtime throughout plugin lifecycle
- Avoid multiple runtime creation
- Properly clean up runtime on disposal

### Memory and Handle Management
- Use thread-safe sharing mechanisms (Arc, Mutex)
- Implement proper resource cleanup in disposal methods
- Monitor for memory leaks during development
- Handle cross-async-task resource sharing correctly

### Configuration Persistence
- Implement automatic configuration saving and loading
- Handle configuration file read/write errors gracefully
- Provide configuration migration for version updates
- Validate configuration data on load

## Development and Testing Standards

### Dependency Compatibility
- Ensure all dependency versions are mutually compatible
- Use appropriate serde features (e.g., `#[serde(skip)]` for non-serializable fields)
- Test compilation across different dependency combinations

### Code Quality Requirements
- Maintain clean code structure
- Remove unused code and duplicate implementations
- Use appropriate comments for complex logic
- Follow consistent naming conventions

### Functional Validation Process
- Test plugin loading and initialization
- Verify connection handling with various scenarios
- Test message processing with different input types
- Validate configuration persistence across restarts
- Test error handling for edge cases

## Common Implementation Patterns

### Logging Usage in Lifecycle Methods
```rust
fn on_mount(&mut self, plugin_ctx: &PluginInstanceContext) -> Result<(), Box<dyn std::error::Error>> {
    let metadata = plugin_ctx.get_metadata();
    log_info!("[{}] Plugin mounting", metadata.name);
    
    match Runtime::new() {
        Ok(runtime) => {
            self.runtime = Some(Arc::new(runtime));
            log_info!("[{}] Runtime initialized successfully", metadata.name);
        }
        Err(e) => {
            log_error!("[{}] Failed to initialize runtime: {}", metadata.name, e);
            return Err(e.into());
        }
    }
    
    log_info!("[{}] Plugin mounted successfully", metadata.name);
    Ok(())
}

fn on_connect(&mut self, plugin_ctx: &PluginInstanceContext) -> Result<(), Box<dyn std::error::Error>> {
    let metadata = plugin_ctx.get_metadata();
    log_info!("[{}] Attempting connection", metadata.name);
    
    // Connection logic with appropriate logging
    match self.establish_connection().await {
        Ok(_) => {
            log_info!("[{}] Connection established successfully", metadata.name);
            Ok(())
        }
        Err(e) => {
            log_warn!("[{}] Connection failed: {}", metadata.name, e);
            Err(e.into())
        }
    }
}
```

### Connection State Management
```rust
// Use Arc<Mutex<bool>> for thread-safe state sharing
let is_connected = Arc::new(Mutex::new(false));

// Update state in connection methods
*self.is_connected.lock().await = true;

// Check state in message handling
let connected = *self.is_connected.lock().await;
if connected {
    // Process message
} else {
    // Return not connected error
}
```

### Configuration Structure
```rust
// Separate user config from plugin metadata
#[derive(Serialize, Deserialize, Clone)]
struct UserConfig {
    // User-modifiable settings
}

impl Default for UserConfig {
    fn default() -> Self {
        // Provide sensible defaults
    }
}
```

### Error Handling Pattern
```rust
// Define plugin-specific error types
#[derive(Debug)]
enum PluginError {
    ConnectionFailed(String),
    ConfigurationError(String),
    OperationTimeout(String),
}

impl std::error::Error for PluginError {}
impl std::fmt::Display for PluginError {
    // Implement user-friendly error messages
}
```

## Security and Performance Considerations

### Resource Limits
- Implement timeouts for external operations
- Limit resource consumption (memory, connections)
- Handle resource exhaustion gracefully

### Data Validation
- Validate all external input
- Sanitize configuration data
- Implement bounds checking for numeric values

### Thread Safety
- Use appropriate synchronization primitives
- Avoid data races in shared state
- Implement proper cleanup for background tasks

## Plugin Packaging and Distribution

### File Structure
```
your-plugin/
├── Cargo.toml          # Dependencies and metadata
├── config.toml         # Plugin description
├── user.toml           # User configuration template
├── src/
│   ├── lib.rs          # Main plugin implementation
│   ├── config.rs       # Configuration management
│   ├── error.rs        # Error type definitions
│   └── client.rs       # External service client
└── README.md           # Plugin documentation
```

### Build Requirements
- Ensure compilation produces correct dynamic library format
- Test plugin loading in target environment
- Verify all dependencies are properly linked
- Validate plugin interface compatibility

This guide serves as a comprehensive reference for plugin development, ensuring consistency, reliability, and maintainability across all plugins in the system.
