//! The translation module, responsible for translating text via external APIs.

pub mod google;

use crate::config::TranslationConfig;
use anyhow::Result;

pub fn translate_text(text: &str, config: &TranslationConfig) -> Result<String> {
    google::translate(text, config)
}
