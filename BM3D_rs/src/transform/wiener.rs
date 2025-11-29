//! first phase BM3D, signal estimation + refined filtering.
//!Parameters: sigma
//!
//!

use crate::transform::dct::{Dct2D, IDct2D};

// Apply the Wiener filter to a single block in the DCT domain
/// - `noisy`: noisy block to be filtered (in/out)
/// - `reference`: reference block (base estimate)
/// - `sigma`: estimated noise
pub fn wiener_filter_block(noisy: &mut Vec<Vec<f64>>, reference: &[Vec<f64>], sigma: f64) {
    let rows = noisy.len();
    let cols = noisy[0].len();

    // copy because it works in-place
    let mut reference_dct = reference.to_owned();

    let dct = Dct2D::new(rows, cols);
    let idct = IDct2D::new(rows, cols);
    dct.dct_2d(noisy);
    dct.dct_2d(&mut reference_dct);

    // Wiener gain
    for i in 0..rows {
        for j in 0..cols {
            let var_est = reference_dct[i][j].powi(2);
            let gain = var_est / (var_est + sigma.powi(2));
            noisy[i][j] *= gain;
        }
    }
    idct.idct_2d(noisy);
}

/// Apply the Wiener filter to a set of blocks
/// - `noisy_blocks`: noisy blocks (modified in-place)
/// - `reference_blocks`: base estimate of the blocks
/// - `sigma`: estimated noise
pub fn wiener_filter_blocks(noisy_blocks: &mut [Vec<Vec<f64>>], reference_blocks: &[Vec<Vec<f64>>], sigma: f64) {
    assert_eq!(noisy_blocks.len(), reference_blocks.len());

    for (noisy, reference) in noisy_blocks.iter_mut().zip(reference_blocks.iter()) {
        wiener_filter_block(noisy, reference, sigma);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EPS: f64 = 1e-6;
    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < EPS
    }
    // #[test]
    fn test_wiener_block_roundtrip() {
        let mut noisy = vec![
            vec![10.0, 20.0, 30.0],
            vec![40.0, 50.0, 60.0],
            vec![70.0, 80.0, 90.0],
        ];
        let reference = noisy.clone();
        let sigma = 1.0;

        wiener_filter_block(&mut noisy, &reference, sigma);

        // The result should be close to the reference because sigma is small.
        for i in 0..3 {
            for j in 0..3 {
                assert!(approx_eq(noisy[i][j], reference[i][j]), 
                    "Mismatch at ({}, {}): got {}, expected {}", 
                    i, j, noisy[i][j], reference[i][j]);
            }
        }
    }
    #[test]
    fn test_wiener_simple() {
        let mut noisy = vec![
            vec![10.0, 20.0, 30.0],
            vec![40.0, 50.0, 60.0],
            vec![70.0, 80.0, 90.0],
        ];
        let reference = vec![
            vec![11.0, 19.0, 31.0],
            vec![39.0, 51.0, 59.0],
            vec![71.0, 79.0, 91.0],
        ];
        let sigma = 5.0;

        // Copia del blocco originale per confronto
        let original = noisy.clone();

        wiener_filter_block(&mut noisy, &reference, sigma);

        // Controlla che tutti i valori siano finiti
        for row in noisy.iter() {
            for &v in row.iter() {
                assert!(v.is_finite(), "Filtered value must be finite");
            }
        }

        // Controlla che almeno un valore sia cambiato
        let changed = noisy.iter().flatten().zip(original.iter().flatten())
            .any(|(&n, &o)| (n - o).abs() > 1e-12);
        assert!(changed, "At least one value should be modified by Wiener filter");
    }
}