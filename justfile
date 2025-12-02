# Themesync Development Commands
# Run `just --list` to see all available commands

# Show all available commands
default:
    @just --list

# Check compilation without building
check:
    cargo check

# Format code (required for CI)
fmt:
    cargo fmt --all

# Check formatting without changing files
fmt-check:
    cargo fmt --all -- --check

# Run linter (must pass with no warnings for CI)
lint:
    cargo clippy -- -D warnings

# Run all tests
test:
    cargo test

# Run tests with output
test-verbose:
    cargo test -- --nocapture

# Run specific test
test-one TEST_NAME:
    cargo test {{TEST_NAME}}

# Build for development
build:
    cargo build

# Build optimized release binary
release:
    cargo build --release

# Run complete CI check locally
ci: check fmt-check lint test
    @echo "✅ All CI checks passed!"

# Run CLI command in development mode
dev +ARGS:
    cargo run -- {{ARGS}}

# Install development dependencies (Just itself)
setup:
    @echo "Installing just command runner..."
    cargo install just
    @echo "✅ Setup complete! Run 'just --list' to see available commands"

# Clean build artifacts
clean:
    cargo clean

# Watch for changes and run checks
watch:
    @echo "Watching for changes... (requires cargo-watch: cargo install cargo-watch)"
    cargo watch -x check -x test