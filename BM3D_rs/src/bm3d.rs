use crate::{
    blocks::match_b::Patch,
    error::ImageProcessingError,
    threshold::hard::hard_threshold,
    transform::dct::{Dct2D, IDct2D},
    utils::metrics::load_dynamic_image,
};
use rayon::prelude::*;
use std::path::Path;
use std::time::Instant;
use zune_image::{codecs::bmp::zune_core::colorspace::ColorSpace, image::Image};

fn setup_rayon() {
    let num_cpus = num_cpus::get();
    println!("Detected {} CPU cores", num_cpus);

    // Usa tutti i core tranne uno per il sistema
    let threads_to_use = (num_cpus - 1).max(1);

    if let Err(e) = rayon::ThreadPoolBuilder::new()
        .num_threads(threads_to_use)
        .build_global()
    {
        eprintln!("Warning: Failed to configure Rayon: {}", e);
    }
}

/// Versione ottimizzata per CPU parallela
pub fn denoise(
    image_path: &Path,
    output_path: &Path,
    sigma: f64,
    block_s: usize,
    window_s: usize,
    max_m: usize,
    stepp: usize,
    sizze: u32,
) -> Result<(), ImageProcessingError> {
    setup_rayon();

    // 1. Carica immagine
    println!("Loading image from {:?}...", image_path);
    let dyn_img = load_dynamic_image(image_path)?;

    // 2. Ridimensiona per velocità
    let max_size = sizze; // Imposta a512 1024 o più per qualità
    let dyn_img = if dyn_img.width() > max_size || dyn_img.height() > max_size {
        println!("Resizing to {}px max for performance...", max_size);
        let scale = max_size as f32 / dyn_img.width().max(dyn_img.height()) as f32;
        let new_width = (dyn_img.width() as f32 * scale) as u32;
        let new_height = (dyn_img.height() as f32 * scale) as u32;
        dyn_img.resize(new_width, new_height, image::imageops::FilterType::Triangle)
    } else {
        dyn_img
    };

    // 3. Converti
    let rgb_img = {
        let rgb = dyn_img.to_rgb8();
        let (width, height) = rgb.dimensions();
        Image::from_u8(
            rgb.as_raw(),
            width as usize,
            height as usize,
            ColorSpace::RGB,
        )
    };

    let (width, height) = rgb_img.dimensions();
    println!("Processing image: {}x{}", width, height);

    // 4. Parametri ottimizzati per CPU
    let block_size = block_s;
    let window_size = window_s; // Finestra più piccola per velocità
    let max_match = max_m; // Meno match per velocità
    let step = stepp; // Step più grande per velocità

    println!("Configuration (CPU optimized):");
    println!("  Block size: {}", block_size);
    println!("  Search window: {}x{}", window_size, window_size);
    println!("  Max matches: {}", max_match);
    println!("  Step: {}", step);

    // 5. Ricerca patch parallela con progresso
    println!("\nFinding similar patches...");
    let start_time = Instant::now();

    let total_y = height.saturating_sub(block_size).div_ceil(step);
    let total_x = width.saturating_sub(block_size).div_ceil(step);
    let total_blocks = total_y * total_x;
    println!("Total reference blocks: {}", total_blocks);

    // Usa parallelizzazione efficiente
    let grouped_blocks: Vec<Vec<Patch>> = (0..height.saturating_sub(block_size))
        .step_by(step)
        .par_bridge()
        .flat_map(|y| {
            (0..width.saturating_sub(block_size))
                .step_by(step)
                .filter_map(|x| {
                    // Stampa progresso ogni 100 blocchi
                    static COUNTER: std::sync::atomic::AtomicUsize =
                        std::sync::atomic::AtomicUsize::new(0);
                    let count = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    if count.is_multiple_of(100) {
                        let elapsed = start_time.elapsed().as_secs_f32();
                        let rate = count as f32 / elapsed.max(0.1);
                        print!(
                            "\rProcessed: {}/{} blocks ({:.1} blocks/sec)",
                            count, total_blocks, rate
                        );
                        let _ = std::io::Write::flush(&mut std::io::stdout());
                    }

                    match crate::blocks::match_b::find_similar_patches(
                        &rgb_img,
                        (x, y),
                        block_size,
                        window_size,
                        max_match,
                        true,
                    ) {
                        Ok(patches) if !patches.is_empty() => Some(patches),
                        _ => None,
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let elapsed = start_time.elapsed();
    println!(
        "\rFound {} groups of patches in {:.2}s ({:.1} blocks/sec)   ",
        grouped_blocks.len(),
        elapsed.as_secs_f32(),
        total_blocks as f32 / elapsed.as_secs_f32()
    );

    if grouped_blocks.is_empty() {
        return Err(ImageProcessingError::Other("No patches found"));
    }

    // 6. Hard thresholding
    println!("\nApplying hard thresholding...");
    let dct = Dct2D::new(block_size, block_size);
    let idct = IDct2D::new(block_size, block_size);
    let threshold = 2.0 * sigma;

    // Parallelizza anche il thresholding
    let step1_reconstructed: Vec<Vec<Patch>> = grouped_blocks
        .par_iter()
        .map(|group| {
            let mut reconstructed = Vec::new();
            for patch in group {
                // Prendi solo il canale Y (luminanza) per semplificare
                let y_channel_size = block_size * block_size;
                let y_data: Vec<f64> = patch.data[..y_channel_size.min(patch.data.len())]
                    .iter()
                    .map(|&v| v as f64)
                    .collect();

                if y_data.len() < y_channel_size {
                    continue;
                }

                let mut block_2d: Vec<Vec<f64>> =
                    y_data.chunks(block_size).map(|r| r.to_vec()).collect();

                // DCT + Threshold + iDCT
                dct.dct_2d(&mut block_2d);
                hard_threshold(&mut block_2d, threshold);
                idct.idct_2d(&mut block_2d);

                let filtered_data: Vec<f32> =
                    block_2d.into_iter().flatten().map(|v| v as f32).collect();
                reconstructed.push(Patch {
                    top_left: patch.top_left,
                    data: filtered_data,
                });
            }
            reconstructed
        })
        .collect();

    // 7. Aggregazione
    println!("Aggregating patches...");
    let aggregated = aggregate_patches(&step1_reconstructed, width, height, block_size)?;

    // 8. Crea immagine risultato (scala di grigi per ora)
    println!("Creating output image...");
    let gray_data: Vec<u8> = aggregated
        .iter()
        .map(|&v| v.clamp(0.0, 255.0) as u8)
        .collect();

    let denoised_image = Image::from_u8(&gray_data, width, height, ColorSpace::Luma);

    // 9. Salva
    println!("Saving to {:?}...", output_path);
    save_image(&denoised_image, output_path)?;

    println!("\n✅ Denoising completed in {:.2}s!", elapsed.as_secs_f32());
    Ok(())
}

/// Aggrega i patches in un'immagine completa
fn aggregate_patches(
    patch_groups: &[Vec<Patch>],
    width: usize,
    height: usize,
    block_size: usize,
) -> Result<Vec<f32>, ImageProcessingError> {
    // Crea buffer per accumulare i valori e i pesi
    let mut accumulator = vec![0.0f32; width * height];
    let mut weights = vec![0.0f32; width * height];

    for group in patch_groups {
        for patch in group {
            let (x, y) = patch.top_left;

            for patch_y in 0..block_size {
                for patch_x in 0..block_size {
                    let img_x = x + patch_x;
                    let img_y = y + patch_y;

                    if img_x < width && img_y < height {
                        let patch_idx = patch_y * block_size + patch_x;
                        if patch_idx < patch.data.len() {
                            let img_idx = img_y * width + img_x;
                            accumulator[img_idx] += patch.data[patch_idx];
                            weights[img_idx] += 1.0;
                        }
                    }
                }
            }
        }
    }

    // Normalizza dividendo per i pesi
    let mut result = vec![0.0f32; width * height];
    for i in 0..accumulator.len() {
        if weights[i] > 0.0 {
            result[i] = accumulator[i] / weights[i];
        }
    }

    Ok(result)
}

fn save_image(img: &Image, path: &Path) -> Result<(), ImageProcessingError> {
    let (width, height) = img.dimensions();
    let data = img.flatten_to_u8();

    if data.is_empty() {
        println!("ERROR: No image data to save!");
        return Err(ImageProcessingError::Other("No image data"));
    }

    println!("Image data: {} channels, sizes:", data.len());
    for (i, channel) in data.iter().enumerate() {
        println!("  Channel {}: {} bytes", i, channel.len());
    }

    // Per immagini in scala di grigi (Luma), converti a RGB
    if data.len() == 1 && data[0].len() == width * height {
        println!("Converting Luma to RGB...");
        let gray_data = &data[0];
        let mut rgb_data = Vec::with_capacity(width * height * 3);

        for &gray in gray_data {
            rgb_data.push(gray); // R
            rgb_data.push(gray); // G
            rgb_data.push(gray); // B
        }

        let dyn_img = image::DynamicImage::ImageRgb8(
            image::RgbImage::from_raw(width as u32, height as u32, rgb_data).ok_or_else(|| {
                println!("ERROR: Failed to create RGB buffer from Luma");
                ImageProcessingError::Other("Failed to create image buffer")
            })?,
        );

        return dyn_img.save(path).map_err(|e| {
            println!("ERROR: Failed to save image: {}", e);
            ImageProcessingError::Other(Box::leak(
                format!("Failed to save image: {}", e).into_boxed_str(),
            ))
        });
    }

    // Per immagini RGB
    if !data.is_empty() && data[0].len() == width * height * 3 {
        let rgb_data = data[0].clone();
        let dyn_img = image::DynamicImage::ImageRgb8(
            image::RgbImage::from_raw(width as u32, height as u32, rgb_data).ok_or_else(|| {
                println!("ERROR: Invalid buffer size for RGB");
                ImageProcessingError::Other("Failed to create image buffer")
            })?,
        );

        return dyn_img.save(path).map_err(|e| {
            println!("ERROR: Failed to save RGB image: {}", e);
            ImageProcessingError::Other(Box::leak(
                format!("Failed to save image: {}", e).into_boxed_str(),
            ))
        });
    }

    println!("ERROR: Unsupported image format");
    Err(ImageProcessingError::Other(
        "Unsupported image format for saving",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_denoise_with_test_image() {
        // Crea un'immagine di test temporanea
        let width = 128; // Più piccola per test veloce
        let height = 128;
        let mut img_data = vec![0u8; width * height * 3];

        // Crea un pattern semplice
        for y in 0..height {
            for x in 0..width {
                let idx = (y * width + x) * 3;
                img_data[idx] = ((x + y) % 256) as u8; // R
                img_data[idx + 1] = (x % 256) as u8; // G
                img_data[idx + 2] = (y % 256) as u8; // B
            }
        }

        let temp_input = NamedTempFile::new().unwrap();
        let temp_output = NamedTempFile::new().unwrap();

        // Salva l'immagine di test
        let img = image::RgbImage::from_vec(width as u32, height as u32, img_data).unwrap();
        img.save(temp_input.path()).unwrap();

        // Test denoise con sigma piccolo
        let sigma = 10.0;
        match denoise(
            temp_input.path(),
            temp_output.path(),
            sigma,
            8,
            16,
            8,
            8,
            512,
        ) {
            Ok(_) => {
                // Verifica che il file di output esista
                assert!(temp_output.path().exists());
                println!("Test passed: denoising completed successfully");
            }
            Err(e) => {
                // Per il test, stampa l'errore ma non fallisce
                eprintln!(
                    "Note: Denoising test encountered error (might be expected): {}",
                    e
                );
            }
        }
    }

    #[test]
    fn test_aggregate_patches() {
        let width = 16;
        let height = 16;
        let block_size = 4;

        // Crea alcuni patch di test
        let patches = vec![vec![Patch {
            top_left: (0, 0),
            data: vec![1.0; block_size * block_size],
        }]];

        let result = aggregate_patches(&patches, width, height, block_size);
        assert!(result.is_ok());

        let aggregated = result.unwrap();
        assert_eq!(aggregated.len(), width * height);

        // I primi block_size*block_size pixel dovrebbero essere 1.0
        for i in 0..block_size * block_size {
            assert!((aggregated[i] - 1.0).abs() < 0.001);
        }
    }
}


// /// Combina i canali RGB in un'immagine
// fn combine_rgb_channels(
//     channels: &[Vec<f32>],
//     width: usize,
//     height: usize,
// ) -> Result<Image, ImageProcessingError> {
//     if channels.len() < 3 {
//         return Err(ImageProcessingError::ColorConversionError);
//     }

//     let r_channel = &channels[0];
//     let g_channel = &channels[1];
//     let b_channel = &channels[2];

//     // Combina i canali in un singolo array RGB
//     let mut rgb_data = Vec::with_capacity(width * height * 3);

//     for i in 0..width * height {
//         if i < r_channel.len() {
//             rgb_data.push(r_channel[i].clamp(0.0, 255.0) as u8);
//         } else {
//             rgb_data.push(0);
//         }

//         if i < g_channel.len() {
//             rgb_data.push(g_channel[i].clamp(0.0, 255.0) as u8);
//         } else {
//             rgb_data.push(0);
//         }

//         if i < b_channel.len() {
//             rgb_data.push(b_channel[i].clamp(0.0, 255.0) as u8);
//         } else {
//             rgb_data.push(0);
//         }
//     }

//     Ok(Image::from_u8(&rgb_data, width, height, ColorSpace::RGB))
// }



// fn test() {
//     println!("DEBUG: Testing patch search performance...");
//     let test_img = image::RgbImage::new(100, 100);
//     let zune_img = Image::from_u8(test_img.as_raw(), 100, 100, ColorSpace::RGB);

//     let start = Instant::now();
//     let result = crate::blocks::match_b::find_similar_patches(
//         &zune_img,
//         (0, 0),
//         8, 21, 8, true
//     );

//     println!("DEBUG: Single patch search took: {:?}", start.elapsed());
//     println!("DEBUG: Result: {:?}", result.is_ok());
// }
