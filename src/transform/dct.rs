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
// ---

use std::f64::consts::PI;


fn dct2d(matrix: &Vec<Vec<i32>>, row: usize, column: usize) {
    // Creiamo dct[row][column], non column x row
    let mut dct: Vec<Vec<f64>> = vec![vec![0.0; column]; row];

    let mut ci;
    let mut cj;
    let mut dct1;
    let mut sum;

    for i in 0..row {
        for j in 0..column {
            if i == 0 {
                ci = 1.0 / (row as f64).sqrt();
            } else {
                ci = (2.0 / row as f64).sqrt();
            }
            if j == 0 {
                cj = 1.0 / (column as f64).sqrt();
            } else {
                cj = (2.0 / column as f64).sqrt();
            }

            sum = 0.0;
            for k in 0..row {
                for l in 0..column {
                    dct1 = matrix[k][l] as f64
                        * ((2*k+1) as f64 * i as f64 * PI / (2.0 * row as f64)).cos()
                        * ((2*l+1) as f64 * j as f64 * PI / (2.0 * column as f64)).cos();
                    sum += dct1;
                }
            }
            dct[i][j] = ci * cj * sum;
        }
    }
    for r in dct.iter() {
        for val in r.iter() {
            print!("{:.6}\t", val);
        }
        println!();
    }
}
