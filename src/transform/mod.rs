//! Wrapper for transform functions
//! 

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
// 
// ### **Biorthogonal Wavelets**
// - **`Transform::from_wavelet(Wavelet::bior)`** - Creates a transform from a biorthogonal wavelet
// - **`Wavelet::new_biorthogonal(1, 3)`** - bior1.3, bior2.2, bior3.1, etc.
// 
// ### **Forward 2D Transform**
// - **`transform_2d(&mut data, width, height, &transform, decomp_lvls)`**
// - **`forward(&mut data, width, height, levels)`** - Simplified version
// 
// ### **Inverse 2D Transform**
// - **`inverse_2d(&mut data, width, height, &transform, decomp_lvls)`**
// - **`inverse(&mut data, width, height, levels)`** - Simplified version
// 
// ### **Wavelet Utilities**
// - **`get_biorthogonal_wavelet()`** - Predefined biorthogonal wavelets
// - **`decomposition_steps(width, height, levels)`** - Computes decomposition dimensions
// 
// **Returns**: `Result<(), TransformError>` - modifies buffer in-place
// 
// ---
// 
// 
// ### **Constructing Biorthogonal Wavelets**
// - **`Wavelet::biorthogonal(vanishing_moments_decomposition, vanishing_moments_reconstruction)`**
// - Example: `bior2.2` → `Wavelet::biorthogonal(2, 2)`
// 
// - **`Wavelet::bior1_3()`** - bior1.3
// - **`Wavelet::bior2_2()`** - bior2.2  
// - **`Wavelet::bior3_1()`** - bior3.1
// - **`Wavelet::bior3_3()`** - bior3.3
// - **`Wavelet::bior3_5()`** - bior3.5
// - **`Wavelet::bior4_4()`** - bior4.4
// 
// **Parameters**: `decomp_lvls` = number of decomposition levels (3-5)
// 
// ---
// 
// ## ⚙️ **Typical Processing Order**
// 1. **Apply Wavelet Transform (bior2.2)** → e.g. `forward()`
// 2. **Apply 2D DCT** → e.g. `dct_2d()`
// 
