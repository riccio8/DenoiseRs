//! construction of 3D models (patch stacking)
use image::DynamicImage;
use std::cmp::max;

use crate::{Bm3dImage, Bm3dParams};
use crate::error::ImageProcessingError;

/// Divide an image into smaller patches.
/// (0,0), (2,0), (4,0) ...
pub fn divide_image(img: &mut Bm3dImage, params: Bm3dParams) -> Result<Vec<DynamicImage>, ImageProcessingError> {
    let width = img.image.width();
    let height = img.image.height();

    
    let mut patches = Vec::new();
    if width == 0 || height == 0 {
        return Err(ImageProcessingError::InvalidParameter("Invalid image size"));
    }
    
    for y in (0..height).step_by(params.patch_size / 4) {
        for x in (0..width).step_by(params.patch_size / 4) {
             if x + params.patch_size as u32 > width || y + params.patch_size as u32 > height {
                 let missing_x = max(0, x + params.patch_size as u32 - width);
                 let missing_y = max(0, y + params.patch_size as u32 - height);
                 // for padding, missing pixels have to be filled with the nearest pixel value, now is just zero padding
                 let patch = img.image.crop(x, y, params.patch_size as u32 - missing_x, params.patch_size as u32 - missing_y);
                 patches.push(patch);
             }
             else{
                let patch = img.image.crop(x, y, params.patch_size as u32, params.patch_size as u32);
                patches.push(patch);
             }
        }
    }
    
    Ok(patches)
}
