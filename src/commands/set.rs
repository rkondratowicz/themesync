use crate::adapter::ThemeAdapter;
use crate::adapters::AdapterRegistry;
use crate::config::Config;
use anyhow::Result;

async fn apply_theme_to_adapter(
    adapter: &dyn ThemeAdapter,
    theme_option: Option<&String>,
    app_display_name: &str,
) -> Result<()> {
    if adapter.is_available().await {
        if let Some(theme) = theme_option {
            match adapter.set_theme(theme).await {
                Ok(_) => println!("✓ Set {} theme to: {}", app_display_name, theme),
                Err(e) => println!("✗ Failed to set {} theme: {}", app_display_name, e),
            }
        }
    } else {
        println!("- {} not available", app_display_name);
    }
    Ok(())
}

pub async fn set_theme(
    config: &mut Config,
    config_path: &std::path::PathBuf,
    theme_name: &str,
) -> Result<()> {
    if !config.themes.contains_key(theme_name) {
        println!("Error: Theme '{}' not found in configuration", theme_name);
        return Ok(());
    }

    let theme_mapping = &config.themes[theme_name];
    let registry = AdapterRegistry::new();

    for adapter in registry.get_all_adapters() {
        let app_theme = theme_mapping.get(adapter.config_key());
        apply_theme_to_adapter(adapter.as_ref(), app_theme, adapter.app_name()).await?;
    }

    // Update theme state and save config
    config.update_theme_state(theme_name);
    config.save_to_file(config_path)?;

    Ok(())
}
