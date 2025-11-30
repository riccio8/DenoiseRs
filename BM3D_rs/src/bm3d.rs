//! wrapper function for BM3D denoising algorithm


use crate::error::ImageProcessingError;
use crate::*;

use zune_image::{
    image::Image,
};

use std::path::Path;


/// main denoise function accepting parameters
pub fn denoise(image_path: &Path, output_path: &Path, sigma: f64)  {
    todo!("Implement BM3D denoising algorithm")
}