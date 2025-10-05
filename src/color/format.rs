//! conversion functions from rgb to YCbCr color space, if luminance_only is true working only on Y channel,
//! otherwise working on all channels.
use crate::Bm3d;

impl Bm3d {
    /// constructor for Bm3d struct
    pub fn new(
        sigma: f32,
        lambda: f32,
        patch_size: usize,
        search_window: usize,
        luminance_only: bool,
        mix: f32,
    ) -> Self {
        Self {
            sigma,
            lambda,
            patch_size,
            search_window,
            luminance_only,
            mix,
        }
    }

    /// conversion functions from rgb to YCbCr color space, if luminance_only is true working only on Y channel,
    /// otherwise working on all channels.
    #[inline(always)]
    pub fn rgb_to_ycbcr(&self, r: u8, g: u8, b: u8) -> Result<(u8, u8, u8), > {
        // Use integer arithmetic instead of float for maximum speed
        let r = r as i32;
        let g = g as i32;
        let b = b as i32;

        let y = ((66 * r + 129 * g + 25 * b + 128) >> 8) + 16;
        let cb = ((-38 * r - 74 * g + 112 * b + 128) >> 8) + 128;
        let cr = ((112 * r - 94 * g - 18 * b + 128) >> 8) + 128;
        
        if self.luminance_only {
            Ok((y.clamp(0, 255) as u8, 0, 0))
        } else {
            Ok((
                y.clamp(0, 255) as u8,
                cb.clamp(0, 255) as u8,
                cr.clamp(0, 255) as u8,
            ))
        }
    }
    
    #[inline(always)]
    pub fn ycbcr_to_rgb(&self, r: f32, g: f32, b: f32, y: f32) -> (u8, u8, u8) {
        let y = y as i32;
        let cb = g as i32;
        let cr = b as i32;

        let mut r = ((298 * y + 409 * cr + 128) >> 8) - 222;
        let mut g = ((298 * y - 100 * cb - 208 * cr + 128) >> 8) + 135;
        let mut b = ((298 * y + 516 * cb + 128) >> 8) - 276;

        r = r.clamp(0, 255);
        g = g.clamp(0, 255);
        b = b.clamp(0, 255);

        (r as u8, g as u8, b as u8)
    }
}
