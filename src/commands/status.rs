use crate::adapter::ThemeAdapter;
use crate::adapters::AdapterRegistry;
use crate::config::Config;
use anyhow::Result;

async fn show_adapter_status(adapter: &dyn ThemeAdapter, app_display_name: &str) -> Result<()> {
    if adapter.is_available().await {
        match adapter.get_theme().await {
            Ok(theme) => println!("  {}: {}", app_display_name, theme),
            Err(e) => println!("  {}: Error - {}", app_display_name, e),
        }
    } else {
        println!("  {}: Not available", app_display_name);
    }
    Ok(())
}

pub async fn show_status(_config: &Config) -> Result<()> {
    let registry = AdapterRegistry::new();

    println!("Theme Status:");

    for adapter in registry.get_all_adapters() {
        show_adapter_status(adapter.as_ref(), adapter.app_name()).await?;
    }

    Ok(())
}
