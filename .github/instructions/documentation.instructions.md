---
applyTo: '**'
---

# Documentation Standards

## Required Updates

**Update README.md when:**
- Adding CLI commands, new app support, or features
- Changing command syntax or config format
- Modifying installation or usage instructions

**Update project-info.instructions.md when:**
- Changing `ThemeAdapter` trait, adapter registration, or core architecture
- Adding dependencies or build workflow changes

## Code Documentation

**Always document:**
- Public APIs (`pub fn`, `pub struct`, `pub enum`)
- Trait definitions
- Non-obvious logic or platform-specific behavior

**Never document:**
- Self-explanatory code
- Simple getters/setters
- Obvious implementations

**Inline comments for:**
- Platform-specific behavior
- Non-obvious workarounds
- TODO/FIXME (with context)

```rust
// macOS uses ~/Library/Application Support, Linux uses ~/.config
let config_dir = if cfg!(target_os = "macos") {
    home.join("Library/Application Support")
} else {
    home.join(".config")
};
```

## New Adapter Template

```rust
//! Adapter for [App Name] theme synchronization via [config file/mechanism].
//!
//! # Configuration Locations
//! - macOS: `~/.config/app/config.ext`
//! - Linux: `~/.config/app/config.ext`
//!
//! # Default Themes
//! - dark: theme-name
//! - light: theme-name
```

## Style Guidelines

- Present tense, active voice, concise
- Include usage examples for new features
- Use code fences: ```rust, ```bash, ```yaml
- Use backticks for inline code: `VsCodeAdapter`
- No commented-out code or contextless TODOs
