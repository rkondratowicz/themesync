use crate::adapter::{ThemeAdapter, ThemeError};
use anyhow::Result;
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::path::PathBuf;

pub struct VsCodeAdapter {
    settings_path: PathBuf,
}

impl Default for VsCodeAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl VsCodeAdapter {
    pub fn new() -> Self {
        let settings_path = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("Code")
            .join("User")
            .join("settings.json");

        Self { settings_path }
    }

    fn read_settings(&self) -> Result<Map<String, Value>, ThemeError> {
        if !self.settings_path.exists() {
            return Ok(Map::new());
        }

        let contents = std::fs::read_to_string(&self.settings_path).map_err(|e| ThemeError {
            message: format!("Failed to read settings.json: {}", e),
            app_name: "vscode".to_string(),
        })?;

        if contents.trim().is_empty() {
            return Ok(Map::new());
        }

        let value: Value = serde_json::from_str(&contents).map_err(|e| ThemeError {
            message: format!("Failed to parse settings.json: {}", e),
            app_name: "vscode".to_string(),
        })?;

        match value {
            Value::Object(map) => Ok(map),
            _ => Err(ThemeError {
                message: "settings.json is not a JSON object".to_string(),
                app_name: "vscode".to_string(),
            }),
        }
    }

    fn write_settings(&self, settings: &Map<String, Value>) -> Result<(), ThemeError> {
        if let Some(parent) = self.settings_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| ThemeError {
                message: format!("Failed to create settings directory: {}", e),
                app_name: "vscode".to_string(),
            })?;
        }

        let contents = serde_json::to_string_pretty(settings).map_err(|e| ThemeError {
            message: format!("Failed to serialize settings: {}", e),
            app_name: "vscode".to_string(),
        })?;

        std::fs::write(&self.settings_path, contents).map_err(|e| ThemeError {
            message: format!("Failed to write settings.json: {}", e),
            app_name: "vscode".to_string(),
        })?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl ThemeAdapter for VsCodeAdapter {
    async fn set_theme(&self, theme: &str) -> Result<(), ThemeError> {
        let mut settings = self.read_settings()?;

        settings.insert(
            "workbench.colorTheme".to_string(),
            Value::String(theme.to_string()),
        );

        self.write_settings(&settings)?;
        Ok(())
    }

    async fn get_theme(&self) -> Result<String, ThemeError> {
        let settings = self.read_settings()?;

        match settings.get("workbench.colorTheme") {
            Some(Value::String(theme)) => Ok(theme.clone()),
            _ => Ok("Default Dark+".to_string()), // VS Code default
        }
    }

    async fn is_available(&self) -> bool {
        // Check if VS Code is installed by looking for the executable
        let vscode_paths = [
            "/Applications/Visual Studio Code.app/Contents/Resources/app/bin/code",
            "/usr/local/bin/code",
            "/usr/bin/code",
        ];

        vscode_paths.iter().any(|path| PathBuf::from(path).exists())
    }

    fn app_name(&self) -> &str {
        "VS Code"
    }

    fn config_key(&self) -> &str {
        "vscode"
    }

    fn default_themes(&self) -> HashMap<String, String> {
        let mut themes = HashMap::new();
        themes.insert("dark".to_string(), "Dracula".to_string());
        themes.insert("light".to_string(), "GitHub Light".to_string());
        themes
    }
}
