use std::path::Path;
use image::DynamicImage;
use crate::Bm3dImage;
use crate::error::ImageProcessingError;
use zune_image::image::Image;


pub fn load_dynamic_image<P: AsRef<Path>>(path: P) -> Result<DynamicImage, ImageProcessingError> {
    let img = image::open(path).map_err(|_| ImageProcessingError::Other("Error while loading image"))?;
    Ok(img)
}

pub fn dynamic_to_ycbcr(dynamic_img: &DynamicImage) -> Result<Image, ImageProcessingError> {
    let img:Image = Bm3dImage::convert_dynamic_to_ycbcr(dynamic_img)?;
    Ok(img)
}


pub fn ycbcr_to_dynamic(ycbcr_img: Image) -> Result<DynamicImage, ImageProcessingError> {
    let img:DynamicImage = Bm3dImage::convert_ycbcr_to_dynamic(ycbcr_img).map_err(|_| ImageProcessingError::Other("Error while saving image"))?;
    Ok(img)

}