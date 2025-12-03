//! conversion functions from rgb to YCbCr color space
use crate::{Bm3dParams, Bm3dImage};
use crate::error::ImageProcessingError;
use image::{DynamicImage, ImageBuffer};
use zune_image::{
    image::Image, 
    codecs::bmp::zune_core::colorspace::ColorSpace,
};

impl Bm3dImage {
    /// constructor for Bm3dImage struct
    pub fn new(
        image: DynamicImage,
        params: Bm3dParams,
    ) -> Self {
        Self {
            image,
            params,
        }
    }
    
    /// Convert a DynamicImage to RGB format using Zune Image library.
    pub fn convert_dynamic_to_rgb(dynamic_img: &DynamicImage) -> Result<Image, ImageProcessingError> {
        let rgb_img = dynamic_img.to_rgb8();
        let (width, height) = rgb_img.dimensions();
        
        Ok(Image::from_u8(
            rgb_img.as_raw(),
            width as usize, 
            height as usize,
            ColorSpace::RGB
        ))
    }
    
    /// Convert Image to DynamicImage
    pub fn convert_to_dynamic(zune_img: &Image) -> Result<DynamicImage, ImageProcessingError> {
        let (width, height) = zune_img.dimensions();
        let data = zune_img.flatten_to_u8();
        
        if data.is_empty() {
            return Err(ImageProcessingError::ColorConversionError);
        }
        
        // Prendi il primo canale
        let channel_data = &data[0];
        
        // Se abbiamo 3 canali (RGB), usa tutti, altrimenti usa solo il primo
        let rgb_data = if data.len() >= 3 {
            // Combina i canali RGB
            let mut combined = Vec::with_capacity(width * height * 3);
            for i in 0..width * height {
                if i < data[0].len() { combined.push(data[0][i]); } else { combined.push(0); }
                if i < data[1].len() { combined.push(data[1][i]); } else { combined.push(0); }
                if i < data[2].len() { combined.push(data[2][i]); } else { combined.push(0); }
            }
            combined
        } else {
            // Solo un canale (luminanza), duplica per RGB
            let mut rgb = Vec::with_capacity(width * height * 3);
            for &gray in channel_data {
                rgb.push(gray); // R
                rgb.push(gray); // G  
                rgb.push(gray); // B
            }
            rgb
        };
        
        Ok(DynamicImage::ImageRgb8(
            ImageBuffer::from_raw(width as u32, height as u32, rgb_data)
                .ok_or(ImageProcessingError::ColorConversionError)?
        ))
    }
}