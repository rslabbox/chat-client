use chat_client_lib::plugins::PluginLoader;

fn main() {
    println!("Testing plugin system...");
    
    let loader = PluginLoader::new();
    let plugins = loader.scan_plugins();
    
    println!("Found {} plugins:", plugins.len());
    for plugin in plugins {
        println!("  - ID: {}", plugin.id);
        println!("    Name: {}", plugin.name);
        println!("    Version: {}", plugin.version);
        println!("    Description: {}", plugin.description);
        if let Some(author) = &plugin.author {
            println!("    Author: {}", author);
        }
        if let Some(library_path) = &plugin.library_path {
            println!("    Library: {}", library_path);
        }
        println!("    Config: {}", plugin.config_path);
        println!("    Disabled: {}", plugin.disabled);
        println!();
    }
}
