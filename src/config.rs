//! Configuration management
//!
//! Loads and manages user configuration from `~/.config/zcode/config.toml`.
//!
//! Configuration sections:
//! - **general**: Default provider, backup behavior, confirmation settings
//! - **display**: Line numbers, syntax highlighting, color scheme
//! - **keybindings**: Custom key bindings for all modes

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub general: GeneralConfig,
    #[serde(default)]
    pub keybindings: KeybindingsConfig,
    #[serde(default)]
    pub display: DisplayConfig,
    #[serde(default)]
    pub providers: HashMap<String, ProviderConfig>,
}

/// Configuration for a specific AI provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    /// Whether this provider is enabled (default: true)
    #[serde(default = "default_true")]
    pub enabled: bool,
    /// Optional custom path to the provider's CLI tool
    pub path: Option<String>,
    /// Optional custom name for the provider
    pub name: Option<String>,
    /// Optional parser type (unified_diff, code_blocks, json, regex)
    pub parser: Option<String>,
}

impl Default for ProviderConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            path: None,
            name: None,
            parser: None,
        }
    }
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GeneralConfig {
    pub default_provider: Option<String>,
    pub create_backups: bool,
    pub confirm_before_apply: bool,
    pub context_lines: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KeybindingsConfig {
    pub next_hunk: String,
    pub prev_hunk: String,
    pub accept_hunk: String,
    pub reject_hunk: String,
    pub apply_changes: String,
    pub quit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisplayConfig {
    pub show_line_numbers: bool,
    pub syntax_highlighting: bool,
    pub color_scheme: String,
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path();

        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            Ok(toml::from_str(&content)?)
        } else {
            Ok(Self::default())
        }
    }

    pub fn config_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("zcode")
            .join("config.toml")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.general.default_provider, None);
        assert!(!config.general.create_backups);
        assert!(!config.general.confirm_before_apply);
    }

    #[test]
    fn test_general_config_default() {
        let general = GeneralConfig::default();
        assert_eq!(general.default_provider, None);
        assert!(!general.create_backups);
        assert!(!general.confirm_before_apply);
        assert_eq!(general.context_lines, 0);
    }

    #[test]
    fn test_display_config_default() {
        let display = DisplayConfig::default();
        assert!(!display.show_line_numbers);
        assert!(!display.syntax_highlighting);
        assert_eq!(display.color_scheme, String::new());
    }

    #[test]
    fn test_keybindings_config_default() {
        let keybindings = KeybindingsConfig::default();
        assert_eq!(keybindings.next_hunk, String::new());
        assert_eq!(keybindings.prev_hunk, String::new());
    }

    #[test]
    fn test_config_roundtrip() {
        let config = Config::default();

        // Serialize and deserialize
        let toml_str = toml::to_string(&config).unwrap();
        let deserialized: Config = toml::from_str(&toml_str).unwrap();

        // Verify structure is preserved
        assert_eq!(
            deserialized.general.default_provider,
            config.general.default_provider
        );
    }

    #[test]
    fn test_config_minimal_toml() {
        let toml_str = ""; // Empty config should work
        let config: Config = toml::from_str(toml_str).unwrap();
        // Verify defaults are applied
        assert_eq!(config.general.default_provider, None);
    }

    #[test]
    fn test_provider_config() {
        let toml_str = r#"
[providers.claude]
enabled = true
path = "/opt/homebrew/bin/claude"

[providers.custom_ai]
enabled = true
name = "My Custom AI"
path = "/usr/local/bin/my-ai"
parser = "unified_diff"
"#;
        let config: Config = toml::from_str(toml_str).unwrap();

        // Check Claude config
        let claude = config.providers.get("claude").unwrap();
        assert!(claude.enabled);
        assert_eq!(claude.path, Some("/opt/homebrew/bin/claude".to_string()));

        // Check custom AI config
        let custom = config.providers.get("custom_ai").unwrap();
        assert!(custom.enabled);
        assert_eq!(custom.name, Some("My Custom AI".to_string()));
        assert_eq!(custom.path, Some("/usr/local/bin/my-ai".to_string()));
        assert_eq!(custom.parser, Some("unified_diff".to_string()));
    }

    #[test]
    fn test_provider_config_defaults() {
        let provider = ProviderConfig::default();
        assert!(provider.enabled); // Should default to true
        assert_eq!(provider.path, None);
        assert_eq!(provider.name, None);
        assert_eq!(provider.parser, None);
    }

    #[test]
    #[ignore] // Only run with --ignored flag
    fn test_which_finds_claude() {
        // This test verifies that the which crate can find Claude on the system
        // It's ignored by default because it depends on the system having Claude installed
        use which::which;

        match which("claude") {
            Ok(path) => {
                println!("✓ Found Claude at: {}", path.display());
                assert!(path.exists());
            }
            Err(e) => {
                panic!("Claude not found in PATH. Error: {}. Make sure Claude is installed and in PATH.", e);
            }
        }
    }
}
