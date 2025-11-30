//! Searching and finding similar patches
//! params:
//! patch_size (8*8), search_window(39*39), max_patches_per_group(~16)

use std::cmp::Ordering;
use crate::error::ImageProcessingError;
use crate::Margin;

use zune_image::{
    image::Image,
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

    // Calculate left/top coordinates (may shift at borders)
    let mut left = f64::max(0.0, ref_point.0 as f64 + (block_size as f64 - window_size as f64) / 2.0);
    let mut top  = f64::max(0.0, ref_point.1 as f64 + (block_size as f64 - window_size as f64) / 2.0);
    let mut right = left + window_size as f64;
    let mut bottom = top + window_size as f64;
    
    // Clamp window if at image border
    if right >= img.dimensions().0 as f64 {
        right = (img.dimensions().0 - 1) as f64;
        left = right - window_size as f64;
    }
    if bottom >= img.dimensions().1 as f64 {
        bottom = (img.dimensions().1 - 1) as f64;
        top = bottom - window_size as f64; 
    }

    Ok(Margin {
        top_left: (left as i32, top as i32),
        bottom_right: (right as i32, bottom as i32),
    })
}

pub struct Patch {
    pub top_left: (usize, usize),
    pub data: Vec<f32>, // Patch data, e.g. flattened block
}

pub struct MatchedPatch {
    pub patch: Patch,
    pub distance: f32,
}

/// Extract a patch (block) from the image at the specified top-left coordinate
/// The patch contains all YCbCr channels flattened into a single array.
/// [Y data][Cb data][Cr data]
pub fn extract_patch(img: &Image, top_left: (usize, usize), block_size: usize, ignore_alpha: bool) -> Option<Patch> {

    let (width, height ) = img.dimensions();

    // Don't go out of image bounds
    if top_left.0 + block_size > width || top_left.1 + block_size > height {
        return None;
    }

    // Get number of channels from channels_ref()
    let channels: usize = img.channels_ref(ignore_alpha).len();

    let mut data = Vec::with_capacity(block_size * block_size * channels);

    // Get references to channels, set ignore_alpha as needed
    let channels = img.channels_ref(ignore_alpha);


for ch in channels {
        let ch_data = unsafe {ch.alias()}; // &[u8] (or &[T])
        for y in 0..block_size {
            for x in 0..block_size {
                let xx = top_left.0 + x;
                let yy = top_left.1 + y;
                let idx = yy * width + xx;
                if idx >= ch.len() {
                    return None; // out of bounds protection
                }
                data.push(ch_data[idx] as f32);
            }
        }
    }
    Some(Patch { top_left, data })
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
    let reference_patch = extract_patch(img, ref_point, block_size, ignore_alpha )
        .ok_or(ImageProcessingError::Other("Reference patch invalid"))?;
    
    // 3. For every possible patch in the search window, compute similarity to the reference patch
    let mut candidates: Vec<MatchedPatch> = Vec::new();
    for y in margin.top_left.1 as usize ..= (margin.bottom_right.1 as usize).saturating_sub(block_size) {
        for x in margin.top_left.0 as usize ..= (margin.bottom_right.0 as usize).saturating_sub(block_size) {
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