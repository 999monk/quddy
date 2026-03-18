use crate::config::CaptureConfig;
use anyhow::{Context, Result};
use image::{DynamicImage, open as open_image};
use std::process::Command;
use tempfile::NamedTempFile;

pub fn capture_with_tool(config: &CaptureConfig) -> Result<DynamicImage> {
    // Create a temporary file with a .png extension that persists after close
    let temp_file = NamedTempFile::with_suffix(".png")
        .context("Failed to create temporary file for screenshot")?;

    let temp_path = temp_file.path().to_path_buf();

    // Keep the file alive until we're done reading it
    let _temp_file = temp_file.keep().context("Failed to keep temporary file")?;

    let maim_status = Command::new(&config.selector_tool)
        .args(["-s", "-u", temp_path.to_str().context("Invalid temp path")?])
        .status()
        .with_context(|| {
            format!(
                "Failed to execute `{}`. Is it installed and in your PATH?",
                config.selector_tool
            )
        })?;

    if !maim_status.success() {
        // Clean up temp file on failure
        let _ = std::fs::remove_file(&temp_path);
        anyhow::bail!("`maim` command failed. It may have been cancelled.");
    }

    let image = open_image(&temp_path)
        .with_context(|| format!("Failed to open temporary capture file at {:?}", temp_path))?;

    // Clean up temp file after successful read
    let _ = std::fs::remove_file(&temp_path);

    Ok(image)
}
