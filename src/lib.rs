//! bm3d_rs implementation

#![warn(non_snake_case)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::nursery)]
#![deny(clippy::todo)]

use image::DynamicImage;

/// wrapper for color operations
pub mod color;

/// wrapper for block operations
pub mod blocks;

/// wrapper for transform operations
pub mod transform;

/// wrapper for threshold operations
pub mod threshold;

/// wrapper for utility operations
pub mod utils;

/// wrapper for BM3D operations
pub mod bm3d;

/// public api for BM3D denoise operations
pub use bm3d::denoise;

/// public api for bm3d errors
pub mod error;

/// parameters for BM3D denoise operations
#[derive(Debug, Clone, Copy, PartialEq,  Default)]
pub struct Bm3dParams {
    /// estimated noise level
    pub sigma: f32,
    /// threshold weight
    pub lambda: f32,
    /// default 8
    pub patch_size: usize,
    /// default 39
    pub search_window: usize,
    /// true/false
    pub luminance_only: bool,
    /// 0.0–1.0 if float, 0-255 if integer
    pub mix: f32,
}


#[derive(Clone, PartialEq, Debug, Default)]
/// BM3D image wrapper
pub struct Bm3dImage {
    /// image descriptor
    image: &DynamicImage,
    /// image parameters for denoise
    params: Bm3dParams,
}
