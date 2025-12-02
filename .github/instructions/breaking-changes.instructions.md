---
applyTo: '**'
---

# Breaking Changes Protocol

## Versioning
- Pre-1.0: Breaking changes increment MINOR (0.4.0 → 0.5.0)
- Post-1.0: SemVer MAJOR.MINOR.PATCH

## What Breaks Compatibility

**Breaking changes:**
- Adding required `ThemeAdapter` methods without default implementations
- Changing `ThemeAdapter` method signatures or return types
- Modifying YAML config structure, renaming keys, changing value types
- Removing CLI commands/flags or changing syntax
- Removing/changing public API signatures

**Safe changes:**
- Adding optional config fields with `#[serde(default)]`
- Adding new CLI commands/flags
- Adding trait methods with default implementations
- Internal refactoring

## Making Breaking Changes

### ThemeAdapter Trait Protection
Always provide default implementations for new methods:

```rust
// ✅ Safe
pub trait ThemeAdapter {
    fn new_method(&self) -> String {
        "default".to_string()
    }
}

// ❌ Breaks all adapters
pub trait ThemeAdapter {
    fn new_method(&self) -> String;
}
```

If changing signatures:
1. Add new method with default
2. Deprecate old: `#[deprecated(since = "0.5.0", note = "Use new_method()")]`
3. Remove in next major version

### Config Compatibility

**Adding fields - always use defaults:**
```rust
#[derive(Deserialize)]
pub struct Settings {
    #[serde(default = "default_value")]
    pub new_field: bool,
    
    #[serde(default)]
    pub optional: Option<Config>,
}
```

**Renaming fields - provide aliases:**
```rust
#[serde(alias = "old_name")]
pub new_name: String,
```

**Config migration pattern:**
```rust
pub fn load_from_file(path: &PathBuf) -> Result<Self> {
    // Try new format, fall back to old + migrate
    Self::load_new_format(path)
        .or_else(|_| OldConfig::load(path).map(Self::migrate_from_old))
        .or_else(|_| Ok(Self::default()))
}
```

### Required Actions

1. Update all adapters: `grep -r "impl ThemeAdapter" src/adapters/`
2. Update docs: README.md, MIGRATION.md, doc comments
3. Test: `just ci` + manual testing with old configs
4. Version bump and release notes with ⚠️ warnings
