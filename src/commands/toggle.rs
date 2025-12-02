use crate::config::Config;
use anyhow::Result;

pub async fn toggle_theme(config: &mut Config, config_path: &std::path::PathBuf) -> Result<()> {
    // Determine what theme to toggle to
    let target_theme = match (
        &config.settings.current_theme,
        &config.settings.previous_theme,
    ) {
        // If we have both current and previous, toggle to previous
        (Some(_current), Some(previous)) => previous.clone(),

        // If we only have current but no previous, fall back to default toggle behavior
        (Some(current), None) => {
            // Try to find a different theme that's not the current one
            let available_themes: Vec<_> = config.themes.keys().collect();
            if available_themes.len() > 1 {
                available_themes
                    .iter()
                    .find(|&&name| name != current)
                    .map(|&name| name.clone())
                    .unwrap_or_else(|| config.settings.default_theme.clone())
            } else {
                println!("Error: Cannot toggle - only one theme available");
                return Ok(());
            }
        }

        // If no current theme is set, use default
        (None, _) => config.settings.default_theme.clone(),
    };

    // Check if target theme exists
    if !config.themes.contains_key(&target_theme) {
        println!(
            "Error: Target theme '{}' not found in configuration",
            target_theme
        );
        return Ok(());
    }

    println!("Toggling to theme: {}", target_theme);
    super::set::set_theme(config, config_path, &target_theme).await?;
    Ok(())
}
