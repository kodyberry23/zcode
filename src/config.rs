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
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub general: GeneralConfig,
    #[serde(default)]
    pub keybindings: KeybindingsConfig,
    #[serde(default)]
    pub display: DisplayConfig,
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
}
