---
applyTo: '**'
---

# Themesync Project Architecture

## Overview
Rust CLI for synchronizing themes across VS Code, Helix, and Ghostty with a single command.

## Tech Stack
- Rust 2021, `tokio` async runtime, `clap` CLI, `serde` serialization, `async-trait`
- Build: `just` command runner
- CI: `cargo fmt`, `cargo clippy`, `cargo test` (run `just ci` before pushing)

## Architecture: Self-Describing Adapter Registry

**Key Innovation:** Adapters auto-register via metadata (`config_key()`, `default_themes()`), eliminating manual wiring in commands.

```
src/
├── adapter.rs        # ThemeAdapter trait
├── adapters/mod.rs   # AdapterRegistry (ONLY place to register adapters)
├── adapters/*.rs     # vscode, helix, ghostty implementations
├── config.rs         # Dynamic YAML config generation
└── commands/*.rs     # set, toggle, status, list (no adapter references)
```

### ThemeAdapter Trait
```rust
#[async_trait::async_trait]
pub trait ThemeAdapter: Send + Sync {
    async fn set_theme(&self, theme: &str) -> Result<(), ThemeError>;
    async fn get_theme(&self) -> Result<String, ThemeError>;
    async fn is_available(&self) -> bool;
    fn app_name(&self) -> &str;
    fn config_key(&self) -> &str;  // "vscode", "helix"
    fn default_themes(&self) -> HashMap<String, String>;  // "dark" -> "Dracula"
}
```

### Config Structure
- Location: `~/.config/themesync/config.yaml`
- Format: `HashMap<theme_name, HashMap<config_key, app_theme>>`
- Example: `{"dark": {"vscode": "Dracula", "helix": "onedark"}}`
- Auto-generated from adapter metadata in `Config::new_with_defaults()`
- Tracks `current_theme`/`previous_theme` in `Settings`, persists after changes

## Adding New Adapters (2-File Process)

**1. Create `src/adapters/newapp.rs`:**
```rust
use crate::adapter::{ThemeAdapter, ThemeError};
use std::collections::HashMap;

pub struct NewAppAdapter;

#[async_trait::async_trait]
impl ThemeAdapter for NewAppAdapter {
    async fn set_theme(&self, theme: &str) -> Result<(), ThemeError> { Ok(()) }
    async fn get_theme(&self) -> Result<String, ThemeError> { Ok("theme".to_string()) }
    async fn is_available(&self) -> bool { true }
    fn app_name(&self) -> &str { "New App" }
    fn config_key(&self) -> &str { "newapp" }
    fn default_themes(&self) -> HashMap<String, String> {
        [("dark", "dark-theme"), ("light", "light-theme")]
            .iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
    }
}
```

**2. Register in `src/adapters/mod.rs`:**
```rust
pub mod newapp;
pub use newapp::NewAppAdapter;

// In AdapterRegistry::new()
Arc::new(NewAppAdapter::new()),
```

**Auto-integration:** Appears in all commands, config generation, theme listings.

## Config File Handling
- **VS Code:** JSON with `serde_json::Map` to preserve unknown fields
- **Helix:** TOML with `#[serde(flatten)]` to preserve unknown fields
- **Ghostty:** Custom key=value parser preserving non-theme settings

## Error Handling
- Graceful degradation: Continue with other adapters if one fails
- User-friendly: `✓` success, `✗` failure, `-` unavailable
- Non-blocking: Unavailable apps don't fail commands

## Development Commands
```bash
just ci               # Full CI check before push
just test            # Run tests
just dev "set dark"  # Test CLI commands
```

## Code Guidelines
- Use `AdapterRegistry::get_all_adapters()`, never instantiate adapters in commands
- Check `is_available()` before operations
- Preserve unknown config fields in adapters
- Config auto-creates with defaults if missing