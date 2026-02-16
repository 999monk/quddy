//! The OCR module, which orchestrates image pre-processing and text extraction.

pub mod preprocessor;
pub mod tesseract;

use crate::config::OcrConfig;
use anyhow::Result;
use image::DynamicImage;

pub fn perform_ocr(raw_image: DynamicImage, config: &OcrConfig) -> Result<String> {
    let processed_image = preprocessor::preprocess_for_ocr(raw_image);

    let text = tesseract::extract_text(&processed_image, config)?;

    Ok(text)
}
