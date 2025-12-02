use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ThemeError {
    pub message: String,
    pub app_name: String,
}

impl std::fmt::Display for ThemeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Theme error for {}: {}", self.app_name, self.message)
    }
}

impl std::error::Error for ThemeError {}

#[async_trait::async_trait]
pub trait ThemeAdapter: Send + Sync {
    async fn set_theme(&self, theme: &str) -> Result<(), ThemeError>;
    async fn get_theme(&self) -> Result<String, ThemeError>;
    async fn is_available(&self) -> bool;
    fn app_name(&self) -> &str;

    /// Returns the key used for this adapter in config files
    fn config_key(&self) -> &str;

    /// Returns default theme mappings for this adapter
    /// Maps global theme names to app-specific theme names
    fn default_themes(&self) -> HashMap<String, String>;
}
