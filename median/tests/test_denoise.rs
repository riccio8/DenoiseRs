use median::denoise;
use median::models::ColorSpace;
use std::path::Path;

#[test]
fn test_denoise_rgb8() {

    let input_path = "tests/noise.jpg";
    let output_path = "tests/output_test.png";

    assert!(Path::new(input_path).exists(), "Test file not found {}", input_path);

    let result = denoise(input_path, 3, ColorSpace::Rgb8);
    
    assert!(result.is_ok(), "Denoise function failed {:?}", result.err());
    result.unwrap().save(output_path).unwrap();
    assert!(Path::new(output_path).exists(), "Output file not created");
}
