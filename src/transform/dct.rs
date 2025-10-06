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
use rustdct::{DctPlanner, Dct2D, IDct2D};