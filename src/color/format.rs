//! conversion functions from rgb to YCbCr color space, if luminance_only is true working only on Y channel,
//! otherwise working on all channels.
use crate::Bm3d;
use crate::error::ImageProcessingError;
use image::{DynamicImage, ImageBuffer, Rgb};
use zune_image::{
    image::Image, 
    codecs::bmp::zune_core::colorspace::ColorSpace, 
};


impl Bm3d {
    /// constructor for Bm3d struct
    pub fn new(
        sigma: f32,
        lambda: f32,
        patch_size: usize,
        search_window: usize,
        luminance_only: bool,
        mix: f32,
    ) -> Self {
        Self {
            sigma,
            lambda,
            patch_size,
            search_window,
            luminance_only,
            mix,
        }
    }
    
    /// Convert a DynamicImage to YCbCr color space using Zune Image library.
    pub fn convert_dynamic_to_ycbcr(dynamic_img: &DynamicImage) -> Result<Image, ImageProcessingError> {
        let rgb_img = dynamic_img.to_rgb8();
        let (width, height) = rgb_img.dimensions();
        
        let mut zune_img = Image::from_u8(
            &rgb_img,
            width as usize, 
            height as usize,
            ColorSpace::RGB
        );

        zune_img.convert_color(ColorSpace::YCbCr).map_err(|_| ImageProcessingError::ColorConversionError)?;
        Ok(zune_img)
    }

    /// Convert a DynamicImage to Zune Image format using Zune Image library.
    pub fn dynamic_to_zune(dynamic_img: &image::DynamicImage) -> Image {
        let rgb_img = dynamic_img.to_rgb8();
        let (width, height) = rgb_img.dimensions();
        
        Image::from_u8(
            &rgb_img,  // &[u8] default from rgb image
            width as usize,
            height as usize,
            ColorSpace::RGB
        )
    }
    
    /// Convert a Vec<u8> to Zune Image format using Zune Image library.
    pub fn vec_to_zune(
        data: Vec<u8>,
        width: usize, 
        height: usize,
        colorspace: ColorSpace
    ) -> Image {
        Image::from_u8(&data, width, height, colorspace)
    }
    
    pub fn convert_ycbcr_to_dynamic(zune_img: &Image) -> DynamicImage {
        let mut img_clone = zune_img.clone();
        
        img_clone.convert_color(ColorSpace::RGB).unwrap();
        
        let (width, height) = img_clone.dimensions();
        let data = img_clone.flatten_to_u8().remove(0); 
        
        DynamicImage::ImageRgb8(
            ImageBuffer::from_raw(width as u32, height as u32, data)
                .expect("Invalid dimensions")
        )
    }

    // will implement conversion functions from rgb to YCbCr color space manually
}