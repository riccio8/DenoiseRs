//! BM3D_rs implementation

#![warn(non_snake_case)]    
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::nursery)]
#![deny(clippy::todo)]

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
