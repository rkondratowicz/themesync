use crate::adapter::ThemeAdapter;
use crate::adapters::AdapterRegistry;
use crate::config::Config;
use anyhow::Result;

async fn show_adapter_availability(
    adapter: &dyn ThemeAdapter,
    app_display_name: &str,
) -> Result<()> {
    let status = if adapter.is_available().await {
        "Available"
    } else {
        "Not available"
    };
    println!("  {}: {}", app_display_name, status);
    Ok(())
}

pub fn list_themes(config: &Config) -> Result<()> {
    let registry = AdapterRegistry::new();

    println!("Available themes:");
    for (name, mapping) in &config.themes {
        println!("  {}:", name);
        for adapter in registry.get_all_adapters() {
            if let Some(app_theme) = mapping.get(adapter.config_key()) {
                println!("    {}: {}", adapter.app_name(), app_theme);
            }
        }
    }
    Ok(())
}

pub async fn list_apps(_config: &Config) -> Result<()> {
    let registry = AdapterRegistry::new();

    println!("Available applications:");

    for adapter in registry.get_all_adapters() {
        show_adapter_availability(adapter.as_ref(), adapter.app_name()).await?;
    }

    Ok(())
}
