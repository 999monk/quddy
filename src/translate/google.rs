use crate::config::TranslationConfig;
use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde_json::Value;

/// Translates text using the unofficial Google Translate API.
pub fn translate(text: &str, config: &TranslationConfig) -> Result<String> {
    let encoded_text = urlencoding::encode(text);

    let url = format!(
        "https://translate.googleapis.com/translate_a/single?client=gtx&sl={}&tl={}&dt=t&q={}",
        config.source_lang, config.target_lang, encoded_text
    );

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(config.timeout_secs))
        .build()
        .context("Failed to build HTTP client")?;

    let response = client
        .get(&url)
        .send()
        .context("HTTP request to Google Translate API failed. Check internet connection.")?;

    if !response.status().is_success() {
        anyhow::bail!(
            "Google Translate API returned a non-success status: {}",
            response.status()
        );
    }

    let json: Value = response
        .json()
        .context("Failed to parse JSON response from Google Translate")?;

    let translation = json[0]
        .as_array()
        .context("Unexpected JSON structure in Google Translate API response")?;

    let full_translation: String = translation
        .iter()
        .filter_map(|part| part[0].as_str())
        .collect::<Vec<_>>()
        .join("");

    Ok(full_translation)
}
