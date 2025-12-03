//! Searching and finding similar patches
//! params:
//! patch_size (8*8), search_window(39*39), max_patches_per_group(~16)

use std::cmp::Ordering;
use crate::error::ImageProcessingError;
use crate::Margin;

use zune_image::{
    image::Image,
    codecs::bmp::zune_core::colorspace::ColorSpace,
};

/// Find the search window whose center is the reference block in *Img*.
/// Note that the center of the search window is not always the reference block due to image borders.
pub fn search_window(
    img: &Image, 
    ref_point: (usize, usize),
    block_size: usize,
    window_size: usize
) -> Result<Margin, ImageProcessingError> {
    // Ensure block size is smaller than window size
    if block_size >= window_size {
        return Err(ImageProcessingError::InvalidParameter(
            "Invalid Image size, block size must be smaller than window size"
        ));
    }

    let (img_width, img_height) = img.dimensions();
    
    // Calculate left/top coordinates (may shift at borders)
    let half_diff = (window_size as i32 - block_size as i32) / 2;
    let mut left = ref_point.0 as i32 - half_diff;
    let mut top = ref_point.1 as i32 - half_diff;
    
    // Clamp to image bounds
    if left < 0 { left = 0; }
    if top < 0 { top = 0; }
    
    let mut right = left + window_size as i32;
    let mut bottom = top + window_size as i32;
    
    if right > img_width as i32 {
        right = img_width as i32;
        left = right - window_size as i32;
        if left < 0 { left = 0; }
    }
    
    if bottom > img_height as i32 {
        bottom = img_height as i32;
        top = bottom - window_size as i32;
        if top < 0 { top = 0; }
    }

    Ok(Margin::new((left, top), (right, bottom)))
}

#[derive(Debug, Clone)]
pub struct Patch {
    pub top_left: (usize, usize),
    pub data: Vec<f32>, // Patch data, e.g. flattened block
}

#[derive(Debug)]
struct MatchedPatch {
    pub patch: Patch,
    pub distance: f32,
}

/// Extract a patch (block) from the image at the specified top-left coordinate
/// The patch contains all YCbCr channels flattened into a single array.
/// [Y data][Cb data][Cr data]
pub fn extract_patch(img: &Image, top_left: (usize, usize), block_size: usize, ignore_alpha: bool) -> Option<Patch> {
    let (width, height) = img.dimensions();

    // Don't go out of image bounds
    if top_left.0 + block_size > width || top_left.1 + block_size > height {
        return None;
    }

    // Get the flattened image data
    let data = img.flatten_to_u8();
    if data.is_empty() {
        return None;
    }

    // Get number of channels from colorspace
    let colorspace = img.colorspace();
    let channels_count = match colorspace {
        ColorSpace::RGB => 3,
        ColorSpace::RGBA => if ignore_alpha { 3 } else { 4 },
        ColorSpace::YCbCr => 3,
        ColorSpace::YCCK => 4,
        ColorSpace::CMYK => 4,
        ColorSpace::Luma => 1,
        ColorSpace::LumaA => if ignore_alpha { 1 } else { 2 },
        _ => 3, // default fallback
    };

    let mut patch_data = Vec::with_capacity(block_size * block_size * channels_count);

    // Extract patch data for each channel
    for channel_idx in 0..channels_count.min(data.len()) {
        let channel_data = &data[channel_idx];
        for y in 0..block_size {
            for x in 0..block_size {
                let pixel_x = top_left.0 + x;
                let pixel_y = top_left.1 + y;
                let idx = pixel_y * width + pixel_x;
                
                if idx < channel_data.len() {
                    patch_data.push(channel_data[idx] as f32);
                } else {
                    return None; // out of bounds
                }
            }
        }
    }
    
    Some(Patch { top_left, data: patch_data })
}

/// Compute the L2 (Euclidean) distance between two patches
pub fn l2_patch_distance(a: &Patch, b: &Patch) -> f32 {
    a.data.iter()
        .zip(&b.data)
        .map(|(x, y)| (x - y) * (x - y))
        .sum::<f32>()
}

/// Find the most similar patches to the reference patch inside its search window.
/// Returns up to max_patches_per_group patches sorted by similarity.
pub fn find_similar_patches(
    img: &Image,
    ref_point: (usize, usize),
    block_size: usize,
    window_size: usize,
    max_patches_per_group: usize,
    ignore_alpha: bool,
) -> Result<Vec<Patch>, ImageProcessingError> {
    // 1. Get the search window for the reference patch
    let margin = search_window(img, ref_point, block_size, window_size)?;

    // 2. Extract the reference patch
    let reference_patch = extract_patch(img, ref_point, block_size, ignore_alpha)
        .ok_or(ImageProcessingError::Other("Reference patch invalid"))?;
    
    // 3. For every possible patch in the search window, compute similarity to the reference patch
    let mut candidates: Vec<MatchedPatch> = Vec::new();
    
    let start_y = margin.top_left.1.max(0) as usize;
    let start_x = margin.top_left.0.max(0) as usize;
    let end_y = (margin.bottom_right.1 as usize).saturating_sub(block_size);
    let end_x = (margin.bottom_right.0 as usize).saturating_sub(block_size);
    
    for y in start_y..=end_y {
        for x in start_x..=end_x {
            if let Some(patch) = extract_patch(img, (x, y), block_size, ignore_alpha) {
                let dist = l2_patch_distance(&reference_patch, &patch);
                candidates.push(MatchedPatch { patch, distance: dist });
            }
        }
    }
    
    // 4. Sort patches by increasing distance (most similar first)
    candidates.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap_or(Ordering::Equal));

    // 5. Take the top max_patches_per_group patches
    let matched_patches = candidates.into_iter()
        .take(max_patches_per_group)
        .map(|mp| mp.patch)
        .collect();

    Ok(matched_patches)
}