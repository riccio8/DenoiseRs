/// wrapper function for BM3D denoising algorithmx

use crate::error::ImageProcessingError;
use crate::{Margin, Point, Bm3dImage, Bm3dParams};

use zune_image::{
    image::Image,
};
use ndarray::Array2;

use std::path::Path;


/// main denoise function accepting parameters
pub fn denoise(image_path: &Path, output_path: &Path, sigma: f64)  {
    todo!("Implement BM3D denoising algorithm")
}

//let ref_point = (
//(spdup_factor * i).min(basic_img_height - block_size - 1),
//(spdup_factor * j).min(basic_img_width - block_size - 1),
//);


fn search_window(img: &Image, 
    ref_point: (usize, usize),
    block_size: usize,
    window_size: usize) -> Result<Margin, ImageProcessingError>{
    
    if block_size >= window_size {
        Err(ImageProcessingError::InvalidParameter("Invalid Image size, block size must be smaller than window size"))
    } else {
        let mut margin = Array2::<f64>::zeros((2, 2));
        margin[[0,0]] = f64::max(0.0, (ref_point.0 as f64 + (block_size as f64 - window_size as f64) / 2.0));
        
        
         Ok(Margin::new((ref_point.0 as i32, ref_point.1 as i32), (block_size as i32, block_size as i32)))
                
    }
    
   
}