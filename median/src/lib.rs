pub mod models;

use image::{ImageReader, RgbImage, RgbaImage, GrayImage};
use imageproc::filter::median_filter;
use crate::models::ColorSpace;

pub fn denoise(path: &str, out: &str, kernel: u32, color_space: ColorSpace) -> Result<(), String> {

    let img = ImageReader::open(path)
        .map_err(|_| "Error opening image")?
        .decode()
        .map_err(|_| "Error decoding image")?;

    match color_space {
        ColorSpace::Rgb8
        | ColorSpace::Rgb16
        | ColorSpace::Rgb32 => {
            let img_buf: RgbImage = img.to_rgb8();
            let img_denoised = median_filter(&img_buf, kernel, kernel);
            img_denoised.save(out).map_err(|_| "Error saving image")?;
        }

        ColorSpace::Rgba8
        | ColorSpace::Rgba16
        | ColorSpace::Rgba32 => {
            let img_buf: RgbaImage = img.to_rgba8();
            let img_denoised = median_filter(&img_buf, kernel, kernel);
            img_denoised.save(out).map_err(|_| "Error saving image")?;
        }

        ColorSpace::Luma8
        | ColorSpace::Luma16
        | ColorSpace::Luma32 => {
            let img_buf: GrayImage = img.to_luma8();
            let img_denoised = median_filter(&img_buf, kernel, kernel);
            img_denoised.save(out).map_err(|_| "Error saving image")?;
        }
        
        ColorSpace::LumaAlpha8
        | ColorSpace::LumaAlpha16
        | ColorSpace::LumaAlpha32 => {
            let img_buf = img.to_luma_alpha8();
            let img_denoised = median_filter(&img_buf, kernel, kernel);
            img_denoised.save(out).map_err(|_| "Error saving image")?;
        }
    }

    Ok(())
}
    