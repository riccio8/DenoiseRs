pub mod models;

use image::{DynamicImage, ImageReader, RgbImage, RgbaImage, GrayImage};
use imageproc::filter::median_filter;
use crate::models::ColorSpace;

pub fn denoise(
    path: &str,
    kernel: u32,
    color_space: ColorSpace,
) -> Result<DynamicImage, String> {
    let img = ImageReader::open(path)
        .map_err(|_| "Error opening image")?
        .decode()
        .map_err(|_| "Error decoding image")?;

    let img_denoised = match color_space {
        ColorSpace::Rgb8
        | ColorSpace::Rgb16
        | ColorSpace::Rgb32 => {
            let img_buf: RgbImage = img.to_rgb8();
            let filtered = median_filter(&img_buf, kernel, kernel);
            DynamicImage::ImageRgb8(filtered)
        }

        ColorSpace::Rgba8
        | ColorSpace::Rgba16
        | ColorSpace::Rgba32 => {
            let img_buf: RgbaImage = img.to_rgba8();
            let filtered = median_filter(&img_buf, kernel, kernel);
            DynamicImage::ImageRgba8(filtered)
        }

        ColorSpace::Luma8
        | ColorSpace::Luma16
        | ColorSpace::Luma32 => {
            let img_buf: GrayImage = img.to_luma8();
            let filtered = median_filter(&img_buf, kernel, kernel);
            DynamicImage::ImageLuma8(filtered)
        }

        ColorSpace::LumaAlpha8
        | ColorSpace::LumaAlpha16
        | ColorSpace::LumaAlpha32 => {
            let img_buf = img.to_luma_alpha8();
            let filtered = median_filter(&img_buf, kernel, kernel);
            DynamicImage::ImageLumaA8(filtered)
        }
    };
    Ok(img_denoised)
}
