pub mod adapter;
pub mod adapters;
pub mod config;

pub use adapter::*;
pub use adapters::*;
pub use config::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert!(config.themes.contains_key("dark"));
        assert!(config.themes.contains_key("light"));
        assert_eq!(config.settings.default_theme, "dark");

        // Test that adapter themes are included in the dynamic mapping
        let dark_theme = &config.themes["dark"];
        assert_eq!(dark_theme.get("helix").unwrap(), "onedark");
        assert_eq!(dark_theme.get("vscode").unwrap(), "Dracula");
        assert_eq!(dark_theme.get("ghostty").unwrap(), "tokyonight");

        let light_theme = &config.themes["light"];
        assert_eq!(light_theme.get("helix").unwrap(), "ayu_light");
        assert_eq!(light_theme.get("vscode").unwrap(), "GitHub Light");
        assert_eq!(light_theme.get("ghostty").unwrap(), "catppuccin-latte");
    }

    #[test]
    fn test_vscode_adapter_creation() {
        let adapter = VsCodeAdapter::new();
        assert_eq!(adapter.app_name(), "VS Code");
        assert_eq!(adapter.config_key(), "vscode");

        let themes = adapter.default_themes();
        assert_eq!(themes.get("dark").unwrap(), "Dracula");
        assert_eq!(themes.get("light").unwrap(), "GitHub Light");
    }

    #[test]
    fn test_ghostty_adapter_creation() {
        let adapter = GhosttyAdapter::new();
        assert_eq!(adapter.app_name(), "Ghostty");
        assert_eq!(adapter.config_key(), "ghostty");

        let themes = adapter.default_themes();
        assert_eq!(themes.get("dark").unwrap(), "tokyonight");
        assert_eq!(themes.get("light").unwrap(), "catppuccin-latte");
    }

    #[test]
    fn test_helix_adapter_creation() {
        let adapter = HelixAdapter::new();
        assert_eq!(adapter.app_name(), "Helix");
        assert_eq!(adapter.config_key(), "helix");

        let themes = adapter.default_themes();
        assert_eq!(themes.get("dark").unwrap(), "onedark");
        assert_eq!(themes.get("light").unwrap(), "ayu_light");
    }

    #[test]
    fn test_ghostty_config_parsing() {
        let config = "font-size = 17\ntheme = tokyonight\nwindow-width = 80";
        let lines = GhosttyAdapter::parse_config_lines(config);

        assert_eq!(lines.len(), 3);
        assert!(lines.iter().any(|(k, v)| k == "theme" && v == "tokyonight"));
        assert!(lines.iter().any(|(k, v)| k == "font-size" && v == "17"));
    }

    #[test]
    fn test_theme_extraction() {
        let config = "font-size = 17\ntheme = catppuccin-latte\nwindow-width = 80";
        let theme = GhosttyAdapter::extract_theme_from_config(config);
        assert_eq!(theme, Some("catppuccin-latte".to_string()));
    }

    #[test]
    fn test_theme_update() {
        let config = "font-size = 17\ntheme = old-theme\nwindow-width = 80";
        let updated = GhosttyAdapter::update_theme_in_config(config, "new-theme");

        assert!(updated.contains("theme = new-theme"));
        assert!(!updated.contains("old-theme"));
        assert!(updated.contains("font-size = 17"));
    }

    #[test]
    fn test_adapter_registry() {
        let registry = AdapterRegistry::new();
        let adapters = registry.get_all_adapters();

        // Should have all three adapters
        assert_eq!(adapters.len(), 3);

        // Test that we have adapters with expected config keys
        let config_keys: Vec<&str> = adapters.iter().map(|a| a.config_key()).collect();
        assert!(config_keys.contains(&"vscode"));
        assert!(config_keys.contains(&"helix"));
        assert!(config_keys.contains(&"ghostty"));
    }
}
