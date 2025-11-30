//! ricomposition between original image and reconstructed image.
//! params:
//! -mix: mixing factor between original image and reconstructed image, blending

use crate::error::{Bm3dError, ImageProcessingError};

#[derive(Debug)]
pub enum Bm3dError {
    InvalidMixFactor(f64),
    DimensionMismatch { a: usize, b: usize },
    ImageProcessingError,
}

/// Reconstructs two images (f64 pixel vectors) using a blending factor.
/// 
/// # Parameters
/// - `original`: reference to the original image.
/// - `reconstructed`: reference to the reconstructed image.
/// - `mix`: blending factor between 0.0 and 1.0 (0: original only, 1: reconstructed only)
///
/// # Returns
/// - `Ok(image)`: image resulting from blending.
/// - `Err(Bm3dError)`: in case of size errors or other errors.
pub fn aggregate(
    original: &[f64],
    reconstructed: &[f64],
    mix: f64,
) -> Result<Vec<f64>, Bm3dError> {
    if mix < 0.0 || mix > 1.0 {
        return Err(Bm3dError::InvalidMixFactor(mix));
    }
    if original.len() != reconstructed.len() {
        return Err(Bm3dError::DimensionMismatch{ 
            a: original.len(), 
            b: reconstructed.len() 
        });
    }

    // Blending pixel per pixel
    let blended: Vec<f64> = original.iter()
        .zip(reconstructed.iter())
        .map(|(orig, recon)| (1.0 - mix) * orig + mix * recon)
        .collect();

    Ok(blended)
}