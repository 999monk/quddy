//! The capture module, responsible for taking screenshots.

pub mod screenshot;
use crate::config::CaptureConfig;
use anyhow::Result;
use image::DynamicImage;

pub fn select_and_capture_region(config: &CaptureConfig) -> Result<DynamicImage> {
    screenshot::capture_with_tool(config)
}
