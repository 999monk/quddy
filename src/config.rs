//! Configuration structures for the application.

use crate::constants::config_dir;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Config {
    #[serde(default)]
    pub ocr: OcrConfig,
    #[serde(default)]
    pub translation: TranslationConfig,
    #[serde(default)]
    pub capture: CaptureConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OcrConfig {
    pub language: String,
    pub psm_mode: u8,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TranslationConfig {
    pub source_lang: String,
    pub target_lang: String,
    pub timeout_secs: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CaptureConfig {
    pub selector_tool: String,
}

impl Default for OcrConfig {
    fn default() -> Self {
        Self {
            language: "eng".to_string(),
            psm_mode: 6,
        }
    }
}

impl Default for TranslationConfig {
    fn default() -> Self {
        Self {
            source_lang: "en".to_string(),
            target_lang: "es".to_string(),
            timeout_secs: 10,
        }
    }
}

impl Default for CaptureConfig {
    fn default() -> Self {
        Self {
            selector_tool: "maim".to_string(),
        }
    }
}

impl Config {
    /// Loads the configuration from the user's config directory (~/.config/quddy/config.toml).
    /// If the file doesn't exist, it creates it with default values and returns it.
    pub fn load() -> Result<Self> {
        let config_path = Self::path()?;

        if !config_path.exists() {
            let default_config = Config::default();
            default_config
                .save()
                .context("Failed to save default configuration")?;
            return Ok(default_config);
        }

        let config_str = fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read config file at {:?}", config_path))?;

        let config: Config = toml::from_str(&config_str)
            .with_context(|| format!("Failed to parse config file at {:?}", config_path))?;

        Ok(config)
    }

    /// Saves the current configuration to the config file.
    pub fn save(&self) -> Result<()> {
        let config_path = Self::path()?;
        let config_dir = config_path
            .parent()
            .context("Config path has no parent directory")?;

        fs::create_dir_all(config_dir).with_context(|| {
            format!(
                "Failed to create configuration directory at {:?}",
                config_dir
            )
        })?;

        let config_str =
            toml::to_string_pretty(self).context("Failed to serialize configuration")?;

        fs::write(&config_path, config_str)
            .with_context(|| format!("Failed to write configuration file to {:?}", config_path))?;

        Ok(())
    }

    /// Returns the path to the config file.
    pub fn path() -> Result<PathBuf> {
        Ok(config_dir().join("config.toml"))
    }
}
