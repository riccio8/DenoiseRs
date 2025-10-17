//! second phase: implementation of 2D DCT using 'rustdct'
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


fn dct1d(matrix: &mut [f64],
    array_lenght: Option<usize>) -> Vec<f64> {
    let length: usize = match array_lenght {
        Some(len) => len,       
        None => matrix.len(),   
    };
    let alpha = (2.0 / length as f64).sqrt();
    let cos_table: Vec<Vec<f64>> = (0..length)
        .map(|k| {
            (0..length)
                .map(|n| ((PI * (2.0 * n as f64 + 1.0) * k as f64) / (2.0 * length as f64)).cos())
                .collect()
        })
        .collect();
    let result: Vec<f64> = (0..length)
        .map(|k| {
            let ck = if k == 0 { 1.0 / 2.0f64.sqrt() } else { 1.0 };
            let sum: f64 = (0..length)
                .map(|n| matrix[n] * cos_table[k][n])
                .sum();
            alpha * ck * sum
        })
        .collect();
    result
}
