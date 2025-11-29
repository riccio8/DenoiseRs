## 2D DCT/iDCT API Overview

This module provides a complete implementation of the 2D Discrete Cosine Transform (DCT-II) and its inverse (iDCT).
It includes ergonomic high-level wrappers, lower-level direct functions, and JPEG-style scaled variants.
Everything operates on `Vec<Vec<f64>>` blocks and can be used for image processing, compression experiments, or denoising pipelines such as BM3D.

### High-Level API

The primary entry points are the `Dct2D` and `IDct2D` structs. They wrap the transform logic in a clean, practical interface:

```rust
let mut block = /* your 2D data */;

let dct = Dct2D::new(rows, cols);
dct.dct_2d(&mut block);          // in-place forward transform

let idct = IDct2D::new(rows, cols);
idct.idct_2d(&mut block);        // in-place inverse transform
```

If you prefer not to mutate the input, each wrapper also provides a `process()` method:

```rust
dct.process(&input, &mut output);
idct.process(&input, &mut output);
```

This is the recommended interface for most applications.

### Low-Level API

For more direct control, there are free functions:

```rust
dct2d(&mut buffer, Some(rows), Some(cols));
idct2d(&mut buffer, Some(rows), Some(cols));
```

These expose the raw transform steps without additional scaling.
They’re useful when integrating into custom pipelines or experimenting with coefficients.

### Scaled JPEG-Style Variants

Two convenience functions provide the normalization typically used in JPEG-like workflows:

```rust
scaled_dct2(&mut block);
scaled_idct2(&mut block);
```

These apply DCT/iDCT with the proper √N scaling factors so that the transform is orthonormal.
Use them if you need compatibility with standard DCT conventions or normalized frequency coefficients.

---

## Typical Workflow

A common pattern for block-based processing (e.g., denoising, compression) looks like this:

1. Extract or define an `N×M` block of floating-point data.
2. Apply the forward DCT:

   ```rust
   dct.dct_2d(&mut block);
   ```
3. Operate in the frequency domain
   (thresholding, weighting, quantization, filtering…).
4. Apply the inverse transform:

   ```rust
   idct.idct_2d(&mut block);
   ```
5. Reinsert or combine the reconstructed block in your image or algorithm.

This mirrors standard signal-processing workflows and integrates cleanly with BM3D stage-1 and stage-2 transforms.