use image::{DynamicImage, ImageBuffer, Luma, imageops};

pub fn preprocess_for_ocr(img: DynamicImage) -> DynamicImage {
    let mut luma_img: ImageBuffer<Luma<u8>, Vec<u8>> = img.to_luma8();

    if luma_img.width() < 800 {
        luma_img = imageops::resize(
            &luma_img,
            luma_img.width() * 2,
            luma_img.height() * 2,
            imageops::FilterType::Lanczos3,
        );
    }

    let contrasted_img = imageops::contrast(&luma_img, 20.0);

    DynamicImage::ImageLuma8(contrasted_img)
}
