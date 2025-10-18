use median::denoise;
use median::models::ColorSpace;
use std::path::Path;

#[test]
fn test_denoise_rgb8() {

    let input_path = "tests/noise.jpg";
    let output_path = "tests/output_test.png";

    assert!(Path::new(input_path).exists(), "File di test non trovato: {}", input_path);

    let result = denoise(input_path, output_path, 3, ColorSpace::Rgb8);

    assert!(result.is_ok(), "La funzione denoise ha fallito: {:?}", result.err());

    assert!(Path::new(output_path).exists(), "File di output non creato");
}
