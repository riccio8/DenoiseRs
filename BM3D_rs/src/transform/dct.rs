//! second phase: implementation of 2D DCT
//! params:
//!  - DCT, 2D

// ### **Forward Transform (2D DCT)**
// - **`Dct2D::new(len)`** - Creates a planner for a specific length
// - **`dct_2d(&mut buffer)`** - Applies the 2D DCT in-place on the buffer
// - **`Dct2D::process(&self, input, output)`** - Performs DCT with separate input/output
// 
// ### **Inverse Transform (2D iDCT)**
// - **`IDct2D::new(len)`** - Creates a planner for the iDCT
// - **`idct_2d(&mut buffer)`** - Applies the 2D iDCT in-place
// - **`IDct2D::process(&self, input, output)`** - Performs iDCT with separate input/output
// 
// ### **Utility**
// - **`DctNum`** - Trait for supported numeric types (f32, f64)
// - **`scaled_dct2(&mut buffer)`** - Scaled DCT
// - **`scaled_idct2(&mut buffer)`** - Scaled iDCT
// 
// **Returns**: `()` - modifies buffers in-place
// 
// Inspired by:
// https://github.com/diegolrs/DCT2D-Digital-Image-Processing/blob/main/DCT2D_in_images.ipynb
// ---

use std::f64::consts::PI;
use rustdct::num_traits;
use ndarray::Array2;

/// dct1d implementation, will be called from the dct2d function, applying it to rows and columns
fn dct1d(matrix: &mut [f64], array_length: Option<usize>) -> Vec<f64> {
    let length = array_length.unwrap_or(matrix.len());
    let alpha = (2.0 / length as f64).sqrt();

    let cos_table = Array2::from_shape_fn((length, length), |(k, n)| {
        ((PI * (2.0 * n as f64 + 1.0) * k as f64) / (2.0 * length as f64)).cos()
    });

    (0..length)
        .map(|k| {
            let ck = if k == 0 { 1.0 / 2.0f64.sqrt() } else { 1.0 };
            let sum: f64 = (0..length)
                .map(|n| matrix[n] * cos_table[[k, n]])
                .sum();
            alpha * ck * sum
        })
        .collect()
}

/// inverse dct1d 
fn idct1d(matrix: &mut [f64], array_length: Option<usize>) -> Vec<f64> {
    let length = array_length.unwrap_or(matrix.len());
    let alpha = (2.0 / length as f64).sqrt();

    let cos_table = Array2::from_shape_fn((length, length), |(n, k)| {
        ((PI * (2.0 * n as f64 + 1.0) * k as f64) / (2.0 * length as f64)).cos()
    });
    
    (0..length)
        .map(|n| {
            (0..length)
                .map(|k| {
                    let ck = if k == 0 { 1.0 / 2.0f64.sqrt() } else { 1.0 };
                    ck * matrix[k] * cos_table[[n, k]]
                })
                .sum::<f64>() * alpha
        })
        .collect()
}
/// implementation of dct2d
pub fn dct2d(matrix: &mut Vec<Vec<f64>>,
    quant_rows: Option<usize>,
    quant_columns: Option<usize>) -> &mut Vec<Vec<f64>>{
    let rows = quant_rows.unwrap_or_else(|| matrix.len());
    let columns = quant_columns.unwrap_or_else(|| matrix[0].len());
        
    let _array2d = vec![
        vec![1.0, 2.0, 3.0],
        vec![4.0, 5.0, 6.0],
        vec![7.0, 8.0, 9.0],
    ];
    for row in matrix.iter_mut() {
        *row = dct1d(&mut*row.as_mut_slice(), None);
    }
    for j in 0..columns {

        let mut column: Vec<f64> = (0..rows)
            .map(|i| matrix[i][j])
            .collect();

        // Apply dct to first column
        let transformed = dct1d(&mut column.as_mut_slice(), None);

        for (i, val) in transformed.into_iter().enumerate() {
            matrix[i][j] = val;
        }
    }
    matrix
}
/// implementation fo inverse dct2d
pub fn idct2d(matrix: &mut Vec<Vec<f64>>,
    quant_rows: Option<usize>,
    quant_columns: Option<usize>) -> &mut Vec<Vec<f64>>{
    let rows = quant_rows.unwrap_or_else(|| matrix.len());
    let columns = quant_columns.unwrap_or_else(|| matrix[0].len());
        
    let _array2d = vec![
        vec![1.0, 2.0, 3.0],
        vec![4.0, 5.0, 6.0],
        vec![7.0, 8.0, 9.0],
    ];
    for row in matrix.iter_mut() {
        *row = idct1d(&mut*row.as_mut_slice(), None);
    }
    for j in 0..columns {
        let mut column: Vec<f64> = (0..rows)
            .map(|i| matrix[i][j])
            .collect();

        // Apply idct to first column
        let transformed = idct1d(&mut column.as_mut_slice(), None);

        for (i, val) in transformed.into_iter().enumerate() {
            matrix[i][j] = val;
        }
    }
    matrix
}




/// Trait markers for permitted numeric types
pub trait DctNum: num_traits::Float {}
impl DctNum for f32 {}
impl DctNum for f64 {}

/// Ergonomic wrapper for DCT 2D
pub struct Dct2D {
    rows: usize,
    cols: usize,
}

impl Dct2D {
    /// new instance creator of dct2d
    pub fn new(rows: usize, cols: usize) -> Self {
        Self { rows, cols }
    }

    /// DCT in-place
    pub fn dct_2d(&self, buffer: &mut Vec<Vec<f64>>) {
        dct2d(buffer, Some(self.rows), Some(self.cols));
    }

    /// iDCT with separated in-out
    pub fn process(&self, input: &Vec<Vec<f64>>, output: &mut Vec<Vec<f64>>) {
        *output = input.clone();
        self.dct_2d(output);
    }
}

/// Ergonomic wrapper for iDCT 2D
pub struct IDct2D {
    rows: usize,
    cols: usize,
}

impl IDct2D {
    /// new instance creator of inverste dct2d
    pub fn new(rows: usize, cols: usize) -> Self {
        Self { rows, cols }
    }

    /// iDCT in-place
    pub fn idct_2d(&self, buffer: &mut Vec<Vec<f64>>) {
        idct2d(buffer, Some(self.rows), Some(self.cols));
    }

    /// iDCT with separated in-out
    pub fn process(&self, input: &Vec<Vec<f64>>, output: &mut Vec<Vec<f64>>) {
        *output = input.clone();
        self.idct_2d(output);
    }
}

// -------------------------------------------------------------
// Scaled versions
// -------------------------------------------------------------
/// orthonormalized versiion
pub fn scaled_dct2(buffer: &mut Vec<Vec<f64>>) {
    let rows = buffer.len();
    let cols = buffer[0].len();
    dct2d(buffer, Some(rows), Some(cols));

    // scale size for ortho
    let scale = 1.0 / (rows as f64).sqrt() / (cols as f64).sqrt();
    for r in buffer.iter_mut() {
        for c in r.iter_mut() {
            *c *= scale;
        }
    }
}
/// orthonormalized
pub fn scaled_idct2(buffer: &mut Vec<Vec<f64>>) {
    let rows = buffer.len();
    let cols = buffer[0].len();

    // Undo pre-scaling
    let scale = (rows as f64).sqrt() * (cols as f64).sqrt();
    for r in buffer.iter_mut() {
        for c in r.iter_mut() {
            *c *= scale;
        }
    }

    idct2d(buffer, Some(rows), Some(cols));
}



#[cfg(test)]
mod tests {
    use super::*;

    // Threshold for floating point comparisons
    const EPS: f64 = 1e-6;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < EPS
    }

    #[test]
    fn test_dct2d_idct2d_roundtrip() {
        let mut input = vec![
            vec![10.0, 20.0, 30.0],
            vec![40.0, 50.0, 60.0],
            vec![70.0, 80.0, 90.0],
        ];

        let original = input.clone();

        let dct = Dct2D::new(3, 3);
        dct.dct_2d(&mut input);

        let idct = IDct2D::new(3, 3);
        idct.idct_2d(&mut input);

        for i in 0..3 {
            for j in 0..3 {
                assert!(
                    approx_eq(input[i][j], original[i][j]),
                    "Mismatch at ({}, {}): got {}, expected {}",
                    i, j, input[i][j], original[i][j]
                );
            }
        }
    }

    #[test]
    fn test_process_api() {
        let input = vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0],
        ];

        let mut out_dct = vec![vec![0.0; 3]; 3];
        let mut out_roundtrip = vec![vec![0.0; 3]; 3];

        let dct = Dct2D::new(3, 3);
        dct.process(&input, &mut out_dct);

        let idct = IDct2D::new(3, 3);
        idct.process(&out_dct, &mut out_roundtrip);

        for i in 0..3 {
            for j in 0..3 {
                assert!(
                    approx_eq(out_roundtrip[i][j], input[i][j]),
                    "Mismatch after process() roundtrip at ({}, {})",
                    i, j
                );
            }
        }
    }
}