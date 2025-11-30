/// wrapper function for BM3D denoising algorithmx

use crate::error::ImageProcessingError;
use crate::Margin;

use zune_image::{
    image::Image,
};

use std::path::Path;


/// main denoise function accepting parameters
pub fn denoise(image_path: &Path, output_path: &Path, sigma: f64)  {
    todo!("Implement BM3D denoising algorithm")
}

//let ref_point = (
//(spdup_factor * i).min(basic_img_height - block_size - 1),
//(spdup_factor * j).min(basic_img_width - block_size - 1),
//);
