use std::path::Path;
use image::DynamicImage;
use crate::error::ImageProcessingError;
use zune_image::{image::Image, codecs::bmp::zune_core::colorspace::ColorSpace,};

pub fn load_dynamic_image<P: AsRef<Path>>(path: P) -> Result<DynamicImage, ImageProcessingError> {
    let img = image::open(path).map_err(|e| {
        ImageProcessingError::Other(Box::leak(format!("Error while loading image: {}", e).into_boxed_str()))
    })?;
    Ok(img)
}

pub fn dynamic_to_ycbcr(dynamic_img: &DynamicImage) -> Result<Image, ImageProcessingError> {
    let rgb_img = dynamic_img.to_rgb8();
    let (width, height) = rgb_img.dimensions();
    
    let zune_img = Image::from_u8(
        rgb_img.as_raw(),
        width as usize,
        height as usize,
        ColorSpace::RGB
    );
    if zune_img.frames_len() == 0{
        return Err(ImageProcessingError::ColorConversionError);
    };

    // Nota: zune-image potrebbe non supportare direttamente la conversione a YCbCr
    // In tal caso, lavoriamo direttamente in RGB
    Ok(zune_img)
}

pub fn ycbcr_to_dynamic(ycbcr_img: Image) -> Result<DynamicImage, ImageProcessingError> {
    let img_clone = ycbcr_img.clone();
    let (width, height) = img_clone.dimensions();
    let data = img_clone.flatten_to_u8();
    
    if data.is_empty() {
        return Err(ImageProcessingError::ColorConversionError);
    }
    
    // Prendi il primo canale
    let channel_data = data[0].clone();
    
    // Controlla se abbiamo abbastanza dati
    let expected_size = (width * height * 3) as usize;
    let actual_size = channel_data.len();
    
    let rgb_data = if actual_size >= expected_size {
        // I dati sono già in formato RGB
        channel_data
    } else if actual_size >= width * height {
        // I dati sono in scala di grigi, convertiamo a RGB
        let mut rgb = Vec::with_capacity(width * height * 3);
        for &gray in &channel_data {
            rgb.push(gray); // R
            rgb.push(gray); // G  
            rgb.push(gray); // B
        }
        rgb
    } else {
        return Err(ImageProcessingError::ColorConversionError);
    };
    
    Ok(DynamicImage::ImageRgb8(
        image::RgbImage::from_raw(width as u32, height as u32, rgb_data)
            .ok_or(ImageProcessingError::ColorConversionError)?
    ))
}