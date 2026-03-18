use crate::config::CaptureConfig;
use anyhow::{Context, Result};
use image::{DynamicImage, open as open_image};
use std::process::Command;

pub fn capture_with_tool(config: &CaptureConfig) -> Result<DynamicImage> {
    let temp_path = std::env::temp_dir().join("quddy_capture.png");

    let status = Command::new(&config.selector_tool)
        .args(["-s", "-u", temp_path.to_str().context("Invalid temp path")?])
        .status()
        .with_context(|| {
            format!(
                "Failed to execute `{}`. Is it installed and in your PATH?",
                config.selector_tool
            )
        })?;

    if !status.success() {
        let _ = std::fs::remove_file(&temp_path);
        anyhow::bail!(
            "`{}` command failed. It may have been cancelled.",
            config.selector_tool
        );
    }

    let image = open_image(&temp_path)
        .with_context(|| format!("Failed to open temporary capture file at {:?}", temp_path))?;

    let _ = std::fs::remove_file(&temp_path);

    Ok(image)
}
