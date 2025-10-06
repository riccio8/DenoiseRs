//! Wrapper for transform functions
//! 

// ---
// 
// ## ⚙️ **Typical Processing Order**
// 1. **Apply Wavelet Transform (bior2.2)** → e.g. `forward()`
// 2. **Apply 2D DCT** → e.g. `dct_2d()`
// 
