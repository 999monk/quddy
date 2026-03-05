use crate::config::OcrConfig;
use anyhow::{Context, Result};
use image::DynamicImage;
use leptess::{LepTess, Variable};

pub fn extract_text(img: &DynamicImage, config: &OcrConfig) -> Result<String> {
    let mut lt = LepTess::new(None, &config.language).context(
        "Failed to initialize Tesseract. Is it installed and are language data files present?",
    )?;

    lt.set_variable(Variable::TesseditPagesegMode, &config.psm_mode.to_string())?;

    lt.set_variable(
        Variable::TesseditCharWhitelist,
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789.,!?'-:;\"()[] ",
    )?;

    let mut buffer = std::io::Cursor::new(Vec::new());
    img.write_to(&mut buffer, image::ImageFormat::Png)
        .context("Failed to encode image to PNG for OCR")?;

    lt.set_image_from_mem(buffer.get_ref())?;

    let text = lt.get_utf8_text()?;

    if text.trim().is_empty() {
        anyhow::bail!("OCR engine did not detect any text in the selected region.");
    }

    Ok(normalize_whitespace(text.trim()))
}

fn normalize_whitespace(text: &str) -> String {
    text.split("\n\n")
        .map(|paragraph| {
            paragraph
                .lines()
                .map(str::trim)
                .filter(|line| !line.is_empty())
                .collect::<Vec<_>>()
                .join(" ")
        })
        .filter(|paragraph| !paragraph.is_empty())
        .collect::<Vec<_>>()
        .join("\n\n")
}
