# Themesync
[![CI](https://github.com/rkondratowicz/themesync/actions/workflows/ci.yml/badge.svg)](https://github.com/rkondratowicz/themesync/actions/workflows/ci.yml)

A command-line tool for synchronizing theme changes across multiple applications simultaneously. Switch from light to dark mode (or any custom theme) across your entire development environment with a single command.

## Features

- 🎨 **Multi-app theme switching** - Change themes across VS Code, terminals, and more with one command
- ⚡ **Fast execution** - Built in Rust for minimal overhead and quick theme switches
- 🔧 **Extensible** - Plugin architecture allows adding support for new applications
- 📁 **Configuration-driven** - YAML-based configuration with sensible defaults
- 🌓 **Smart toggle** - Toggle between current and previously used themes
- 📊 **Status monitoring** - See current theme state across all applications

## Installation

### From Source

```bash
git clone https://github.com/your-username/themesync.git
cd themesync
cargo build --release
```

The binary will be available at `target/release/themesync`.

### Package Managers (Coming Soon)

- Cargo: `cargo install themesync`
- Homebrew: `brew install themesync`

## Usage

### Basic Commands

```bash
# Switch all configured apps to dark theme
themesync set dark

# Switch all configured apps to light theme
themesync set light

# Toggle between current and previously used theme
themesync toggle

# Show current theme status across all apps
themesync status

# List available themes
themesync themes list

# List configured applications
themesync apps list
```

### Toggle Behavior

The `toggle` command intelligently switches themes based on your usage history:

- **First time**: Switches to the default theme
- **After setting themes**: Toggles between the current theme and the most recently used theme
- **Smart fallback**: If no previous theme exists, finds another available theme to toggle to

Example workflow:
```bash
# Set initial theme
themesync set dark

# Switch to another theme  
themesync set light

# Toggle back to previous theme (dark)
themesync toggle

# Toggle again switches back to light
themesync toggle
```

### Configuration

Themesync uses a YAML configuration file located at `~/.config/themesync/config.yaml`. The configuration is automatically created with defaults on first run.

#### Example Configuration

```yaml
themes:
  dark:
    vscode: "Dracula"
    helix: "onedark"
    ghostty: "tokyonight"
  light:
    vscode: "GitHub Light"
    helix: "ayu_light"
    ghostty: "catppuccin-latte"

apps:
  vscode:
    enabled: true
    method: "auto"
  helix:
    enabled: true
    method: "auto"
  ghostty:
    enabled: true
    method: "auto"

settings:
  default_theme: "dark"
  backup_configs: true
  parallel_execution: true
  current_theme: "light"    # Automatically tracked
  previous_theme: "dark"    # Automatically tracked
```

## Supported Applications

### Currently Supported

- **Visual Studio Code** - Direct manipulation of `settings.json`
- **Helix** - Direct manipulation of `config.toml`
- **Ghostty** - Direct config file modification

## Development

### Prerequisites

- Rust 1.70+ 
- Cargo
- [Just](https://github.com/casey/just) command runner

### Setup

```bash
# Install Just command runner
cargo install just

# Or use the setup command after cloning
just setup
```

### Development Commands

We use [Just](https://github.com/casey/just) as a command runner to simplify development workflows:

```bash
# Show all available commands
just --list

# Common development workflows
just check           # Check compilation without building
just build           # Build for development
just release         # Build optimized release binary
just test            # Run all tests
just test-one NAME   # Run specific test
just test-verbose    # Run tests with output

# Code quality (required for CI)
just fmt             # Format code
just fmt-check       # Check formatting without changing files
just lint            # Run linter with no warnings allowed
just ci              # Run complete CI check locally

# Development usage
just dev status        # cargo run -- status
just dev "set dark"    # cargo run -- set dark
just dev "themes list" # cargo run -- themes list

# Utilities
just clean           # Clean build artifacts
just watch           # Watch for changes and run checks (requires cargo-watch)
```

### Legacy Commands

Raw cargo commands are still available but Just commands are preferred:

```bash
cargo build         # Use: just build
cargo test          # Use: just test  
cargo run -- status # Use: just dev status
```

## Architecture

Themesync is built with a modular architecture:

- **Adapter Registry System**: Automatic discovery and management of application adapters
- **Dynamic Configuration**: Theme mappings are generated automatically from adapter metadata
- **Async Support**: Built on Tokio for efficient concurrent theme switching
- **Self-Contained Adapters**: Each adapter provides its own configuration and theme mappings
- **Error Handling**: Comprehensive error handling with graceful degradation

### Adding New Adapters

Thanks to the automatic adapter discovery system, adding support for a new application is incredibly simple:

**Step 1:** Create the adapter file in `src/adapters/newapp.rs`:

```rust
use crate::adapter::{ThemeAdapter, ThemeError};
use std::collections::HashMap;

pub struct NewAppAdapter;

impl NewAppAdapter {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl ThemeAdapter for NewAppAdapter {
    async fn set_theme(&self, theme: &str) -> Result<(), ThemeError> {
        // Implementation for setting theme in the app
        Ok(())
    }

    async fn get_theme(&self) -> Result<String, ThemeError> {
        // Implementation for getting current theme
        Ok("current-theme".to_string())
    }

    async fn is_available(&self) -> bool {
        // Check if the app is installed
        true
    }

    fn app_name(&self) -> &str {
        "New App"
    }

    fn config_key(&self) -> &str {
        "newapp"  // Key used in config files
    }

    fn default_themes(&self) -> HashMap<String, String> {
        let mut themes = HashMap::new();
        themes.insert("dark".to_string(), "newapp-dark-theme".to_string());
        themes.insert("light".to_string(), "newapp-light-theme".to_string());
        themes
    }
}
```

**Step 2:** Add the adapter to the registry in `src/adapters/mod.rs`:

```rust
// Add the module
pub mod newapp;
pub use newapp::NewAppAdapter;

// Add to the registry in AdapterRegistry::new()
Arc::new(NewAppAdapter::new()),
```

**That's it!** The adapter will automatically:
- Appear in `themesync apps list`
- Have its themes included in `themesync themes list` 
- Be used by `themesync set <theme>` commands
- Generate appropriate default configuration

No need to modify any command files or configuration structures.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

### Areas for Contribution

- New application adapters
- Bug fixes and improvements
- Documentation
- Testing
- Package manager integration

## Acknowledgments

- Inspired by the need for seamless theme switching across development environments
- Built with Rust's excellent ecosystem including Clap, Tokio, and Serde