pub mod ghostty;
pub mod helix;
pub mod vscode;

pub use ghostty::GhosttyAdapter;
pub use helix::HelixAdapter;
pub use vscode::VsCodeAdapter;

use crate::adapter::ThemeAdapter;
use std::sync::Arc;

pub struct AdapterRegistry {
    adapters: Vec<Arc<dyn ThemeAdapter>>,
}

impl Default for AdapterRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl AdapterRegistry {
    pub fn new() -> Self {
        let adapters: Vec<Arc<dyn ThemeAdapter>> = vec![
            Arc::new(VsCodeAdapter::new()),
            Arc::new(GhosttyAdapter::new()),
            Arc::new(HelixAdapter::new()),
        ];

        Self { adapters }
    }

    pub fn get_all_adapters(&self) -> &Vec<Arc<dyn ThemeAdapter>> {
        &self.adapters
    }
}
