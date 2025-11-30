use std::path::Path;
use crate::{
    Bm3dParams, Parameters, ParamValue,
    utils::metrics::{load_dynamic_image, dynamic_to_ycbcr, ycbcr_to_dynamic},
    threshold::hard::hard_threshold,
    transform::dct::{Dct2D, IDct2D},
    transform::wiener::wiener_filter_blocks,
    blocks::match_b::Patch,
    error::ImageProcessingError,
};

/// Funzione principale BM3D denoise
pub fn denoise(image_path: &Path, output_path: &Path, sigma: f64) -> Result<(), ImageProcessingError> {
    // 1. Inizializza i parametri BM3D
    let mut params = Bm3dParams::new();
    params.set(Parameters::Sigma, ParamValue::F64(sigma));

    // 2. Carica immagine e converti in YCbCr
    let dyn_img = load_dynamic_image(image_path)?;
    let ycbcr_img = dynamic_to_ycbcr(&dyn_img)?;
    

    // 3. Parametri di patch
    let block_size = if let ParamValue::I32(bs) = params.get(&Parameters::Step1BlockSize).unwrap() { *bs as usize } else { 8 };
    let window_size = if let ParamValue::I32(ws) = params.get(&Parameters::Step1WindowSize).unwrap() { *ws as usize } else { 39 };
    let max_match = if let ParamValue::I32(mm) = params.get(&Parameters::Step1MaxMatch).unwrap() { *mm as usize } else { 16 };
    let step = if let ParamValue::I32(st) = params.get(&Parameters::Step1SpeedupFactor).unwrap() { *st as usize } else { 3 };
    let ignore_alpha = true;

    let (width, height) = ycbcr_img.dimensions();

    // 4. Estrazione blocchi simili - lavoriamo solo sul canale Y (luminanza)
    let mut grouped_blocks: Vec<Vec<Patch>> = Vec::new();
    for y in (0..height ).step_by(step) {
        for x in (0..width ).step_by(step) {
            let patches = crate::blocks::match_b::find_similar_patches(
                &ycbcr_img,
                (x, y),
                block_size,
                window_size,
                max_match,
                ignore_alpha,
            )?;
            grouped_blocks.push(patches);
        }
    }

    // 5. Hard thresholding (fase 1)
    let dct = Dct2D::new(block_size, block_size);
    let idct = IDct2D::new(block_size, block_size);
    let lambda_2d = if let ParamValue::F64(l) = params.get(&Parameters::Lamb2D).unwrap() { *l } else { 2.0 };
    let threshold = lambda_2d * sigma;

    // Convertiamo i blocchi nel formato atteso da wiener_filter_blocks
    let mut step1_blocks: Vec<Vec<Vec<f64>>> = Vec::new();

    for group in grouped_blocks.iter() {
        let mut filtered_group: Vec<Vec<f64>> = Vec::new();
        for patch in group.iter() {
            // Converti patch in 2D f64
            let mut block_2d: Vec<Vec<f64>> = patch.data
                .chunks(block_size)
                .map(|r| r.iter().map(|&v| v as f64).collect())
                .collect();

            // DCT 2D
            dct.dct_2d(&mut block_2d);

            // Hard thresholding
            hard_threshold(&mut block_2d, threshold);

            // iDCT 2D
            idct.idct_2d(&mut block_2d);

            // Appiattisci il blocco 2D in 1D per wiener_filter_blocks
            let flattened_block: Vec<f64> = block_2d.into_iter().flatten().collect();
            filtered_group.push(flattened_block);
        }
        step1_blocks.push(filtered_group);
    }

    // 6. Wiener filtering (fase 2)
    let mut step2_blocks = step1_blocks.clone();
    wiener_filter_blocks(&mut step2_blocks, &step1_blocks, sigma);

    // 7. Ricostruzione dell'immagine dai blocchi filtrati
    // Nota: Questa parte è semplificata - in un'implementazione completa dovresti:
    // - Aggregare i blocchi sovrapposti
    // - Ricostruire l'immagine canale Y
    // - Combinare con i canali CbCr originali
    
    // Per ora, salviamo l'immagine originale come placeholder
    let dyn_result = ycbcr_to_dynamic(ycbcr_img)?;
    dyn_result.save(output_path).map_err(|_| ImageProcessingError::Other("Error while saving"))?;

    Ok(())
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;


    #[test]
    fn test_denoise() {
        let input_path = Path::new("C:/Users/ricci/rust/denoise/BM3D_rs/src/foto.jpg");
        let output_path = Path::new("C:/Users/ricci/rust/denoise/BM3D_rs/foto_denoised.jpg");
        let sigma = 15.0;

        // Caricamento forzando RGB
        let dyn_img = match image::open(input_path) {
            Ok(img) => img.to_rgb8(), // forza RGB, scarta eventuale alpha
            Err(e) => {
                eprintln!("Errore caricando immagine: {:?}", e);
                return;
            }
        };
        
        // Chiamata alla funzione denoise che accetta DynamicImage
        match denoise(&input_path, &output_path, sigma) {
            Ok(_) => println!("Denoising completato! Output salvato in {:?}", output_path),
            Err(e) => eprintln!("Errore durante il denoise: {:?}", e),
        }
    }
}
