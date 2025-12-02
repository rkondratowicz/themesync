use crate::adapters::AdapterRegistry;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub themes: HashMap<String, HashMap<String, String>>,
    pub apps: HashMap<String, AppConfig>,
    pub settings: Settings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub enabled: bool,
    pub path: Option<PathBuf>,
    pub method: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub default_theme: String,
    pub backup_configs: bool,
    pub parallel_execution: bool,
    pub current_theme: Option<String>,
    pub previous_theme: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self::new_with_defaults()
    }
}

impl Config {
    pub fn new_with_defaults() -> Self {
        let registry = AdapterRegistry::new();
        let mut themes = HashMap::new();
        let mut apps = HashMap::new();

        // Create a map of all theme names that any adapter supports
        let mut all_theme_names = HashSet::new();
        for adapter in registry.get_all_adapters() {
            for theme_name in adapter.default_themes().keys() {
                all_theme_names.insert(theme_name.clone());
            }
        }

        // For each global theme name, collect mappings from all adapters
        for theme_name in &all_theme_names {
            let mut theme_mapping = HashMap::new();
            for adapter in registry.get_all_adapters() {
                if let Some(app_theme) = adapter.default_themes().get(theme_name) {
                    theme_mapping.insert(adapter.config_key().to_string(), app_theme.clone());
                }
            }
            themes.insert(theme_name.clone(), theme_mapping);
        }

        // Create app configs for each adapter
        for adapter in registry.get_all_adapters() {
            apps.insert(
                adapter.config_key().to_string(),
                AppConfig {
                    enabled: true,
                    path: None, // Let adapters auto-detect their paths
                    method: "auto".to_string(),
                },
            );
        }

        Config {
            themes,
            apps,
            settings: Settings {
                default_theme: "dark".to_string(),
                backup_configs: true,
                parallel_execution: true,
                current_theme: None,
                previous_theme: None,
            },
        }
    }
    pub fn load_from_file(path: &PathBuf) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }

        let contents = std::fs::read_to_string(path)?;
        let config: Config = serde_yaml::from_str(&contents)?;
        Ok(config)
    }

    pub fn save_to_file(&self, path: &PathBuf) -> Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let contents = serde_yaml::to_string(self)?;
        std::fs::write(path, contents)?;
        Ok(())
    }

    pub fn update_theme_state(&mut self, new_theme: &str) {
        self.settings.previous_theme = self.settings.current_theme.clone();
        self.settings.current_theme = Some(new_theme.to_string());
    }

    pub fn get_config_path() -> PathBuf {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".config")
            .join("themesync")
            .join("config.yaml")
    }
}
