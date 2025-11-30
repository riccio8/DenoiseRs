//! Hard thresholding of DCT coefficients
//! Parameters: lambda, sigma

/// Apply hard thresholding to a 2D block in-place.
/// All coefficients with absolute value less than `threshold` are set to zero.
pub fn hard_threshold(block: &mut Vec<Vec<f64>>, threshold: f64) {
    for row in block.iter_mut() {
        for val in row.iter_mut() {
            if val.abs() < threshold {
                *val = 0.0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hard_threshold() {
        let mut block = vec![
            vec![0.5, 2.0, -1.0],
            vec![3.0, -0.2, 0.0],
        ];
        let threshold = 1.0;
        hard_threshold(&mut block, threshold);

        let expected = vec![
            vec![0.0, 2.0, -1.0],
            vec![3.0, 0.0, 0.0],
        ];
        assert_eq!(block, expected);
    }
}
