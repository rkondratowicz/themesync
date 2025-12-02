use crate::adapter::{ThemeAdapter, ThemeError};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct HelixConfig {
    theme: Option<String>,
    #[serde(flatten)]
    other: HashMap<String, toml::Value>,
}

pub struct HelixAdapter {
    config_path: PathBuf,
}

impl Default for HelixAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl HelixAdapter {
    pub fn new() -> Self {
        let config_path = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("helix")
            .join("config.toml");

        Self { config_path }
    }

    fn read_config(&self) -> Result<HelixConfig, ThemeError> {
        if !self.config_path.exists() {
            return Ok(HelixConfig::default());
        }

        let contents = std::fs::read_to_string(&self.config_path).map_err(|e| ThemeError {
            message: format!("Failed to read Helix config: {}", e),
            app_name: "helix".to_string(),
        })?;

        if contents.trim().is_empty() {
            return Ok(HelixConfig::default());
        }

        let config: HelixConfig = toml::from_str(&contents).map_err(|e| ThemeError {
            message: format!("Failed to parse Helix config.toml: {}", e),
            app_name: "helix".to_string(),
        })?;

        Ok(config)
    }

    fn write_config(&self, config: &HelixConfig) -> Result<(), ThemeError> {
        if let Some(parent) = self.config_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| ThemeError {
                message: format!("Failed to create Helix config directory: {}", e),
                app_name: "helix".to_string(),
            })?;
        }

        let contents = toml::to_string_pretty(config).map_err(|e| ThemeError {
            message: format!("Failed to serialize Helix config: {}", e),
            app_name: "helix".to_string(),
        })?;

        std::fs::write(&self.config_path, contents).map_err(|e| ThemeError {
            message: format!("Failed to write Helix config.toml: {}", e),
            app_name: "helix".to_string(),
        })?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl ThemeAdapter for HelixAdapter {
    async fn set_theme(&self, theme: &str) -> Result<(), ThemeError> {
        let mut config = self.read_config()?;
        config.theme = Some(theme.to_string());
        self.write_config(&config)?;
        Ok(())
    }

    async fn get_theme(&self) -> Result<String, ThemeError> {
        let config = self.read_config()?;

        match config.theme {
            Some(theme) => Ok(theme),
            None => Ok("default".to_string()), // Helix default theme
        }
    }

    async fn is_available(&self) -> bool {
        // Check if Helix is installed by looking for the executable
        let helix_paths = [
            "/usr/local/bin/hx",
            "/usr/bin/hx",
            "/opt/homebrew/bin/hx",
            "/usr/local/bin/helix",
            "/usr/bin/helix",
        ];

        // First check common installation paths
        if helix_paths.iter().any(|path| PathBuf::from(path).exists()) {
            return true;
        }

        // Also try to run the command to see if it's in PATH
        Command::new("hx")
            .arg("--version")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    fn app_name(&self) -> &str {
        "Helix"
    }

    fn config_key(&self) -> &str {
        "helix"
    }

    fn default_themes(&self) -> HashMap<String, String> {
        let mut themes = HashMap::new();
        themes.insert("dark".to_string(), "onedark".to_string());
        themes.insert("light".to_string(), "ayu_light".to_string());
        themes
    }
}
