use crate::adapter::{ThemeAdapter, ThemeError};
use anyhow::Result;
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;

pub struct GhosttyAdapter {
    config_path: PathBuf,
}

impl Default for GhosttyAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl GhosttyAdapter {
    pub fn new() -> Self {
        let config_path = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("ghostty")
            .join("config");

        Self { config_path }
    }

    fn read_config(&self) -> Result<String, ThemeError> {
        if !self.config_path.exists() {
            return Ok(String::new());
        }

        std::fs::read_to_string(&self.config_path).map_err(|e| ThemeError {
            message: format!("Failed to read Ghostty config: {}", e),
            app_name: "ghostty".to_string(),
        })
    }

    fn write_config(&self, content: &str) -> Result<(), ThemeError> {
        if let Some(parent) = self.config_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| ThemeError {
                message: format!("Failed to create Ghostty config directory: {}", e),
                app_name: "ghostty".to_string(),
            })?;
        }

        std::fs::write(&self.config_path, content).map_err(|e| ThemeError {
            message: format!("Failed to write Ghostty config: {}", e),
            app_name: "ghostty".to_string(),
        })?;

        Ok(())
    }

    pub fn parse_config_lines(config: &str) -> Vec<(String, String)> {
        config
            .lines()
            .filter_map(|line| {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    return None;
                }

                if let Some((key, value)) = line.split_once('=') {
                    Some((key.trim().to_string(), value.trim().to_string()))
                } else {
                    None
                }
            })
            .collect()
    }

    fn format_config_lines(lines: &[(String, String)]) -> String {
        lines
            .iter()
            .map(|(key, value)| format!("{} = {}", key, value))
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn update_theme_in_config(config: &str, new_theme: &str) -> String {
        let mut lines = Self::parse_config_lines(config);

        // Remove existing theme line
        lines.retain(|(key, _)| key != "theme");

        // Add new theme line
        lines.push(("theme".to_string(), new_theme.to_string()));

        Self::format_config_lines(&lines)
    }

    pub fn extract_theme_from_config(config: &str) -> Option<String> {
        Self::parse_config_lines(config)
            .into_iter()
            .find(|(key, _)| key == "theme")
            .map(|(_, value)| value)
    }
}

#[async_trait::async_trait]
impl ThemeAdapter for GhosttyAdapter {
    async fn set_theme(&self, theme: &str) -> Result<(), ThemeError> {
        let current_config = self.read_config()?;
        let updated_config = Self::update_theme_in_config(&current_config, theme);
        self.write_config(&updated_config)?;
        Ok(())
    }

    async fn get_theme(&self) -> Result<String, ThemeError> {
        let config = self.read_config()?;

        match Self::extract_theme_from_config(&config) {
            Some(theme) => Ok(theme),
            None => Ok("default".to_string()), // Ghostty default
        }
    }

    async fn is_available(&self) -> bool {
        // Check if Ghostty is installed by looking for the executable
        let ghostty_paths = [
            "/Applications/Ghostty.app/Contents/MacOS/ghostty",
            "/usr/local/bin/ghostty",
            "/usr/bin/ghostty",
            "/opt/homebrew/bin/ghostty",
        ];

        // First check common installation paths
        if ghostty_paths
            .iter()
            .any(|path| PathBuf::from(path).exists())
        {
            return true;
        }

        // Also try to run the command to see if it's in PATH
        Command::new("ghostty")
            .arg("--version")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    fn app_name(&self) -> &str {
        "Ghostty"
    }

    fn config_key(&self) -> &str {
        "ghostty"
    }

    fn default_themes(&self) -> HashMap<String, String> {
        let mut themes = HashMap::new();
        themes.insert("dark".to_string(), "tokyonight".to_string());
        themes.insert("light".to_string(), "catppuccin-latte".to_string());
        themes
    }
}
