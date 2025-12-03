use std::path::PathBuf;
use bm3d_rs::denoise;
use clap::Parser;

/// BM3D Denoising Tool
#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "BM3D Image Denoising Tool with configurable parameters",
    long_about = r#"
BM3D (Block-Matching and 3D Filtering) Image Denoising Tool

Example usage:
  bm3d --input noisy.jpg --output clean.jpg --sigma 25.0
  bm3d --input noisy.jpg --output clean.jpg --sigma 25.0 --window-size 39 --max-matches 16 --step-size 3
  bm3d --input noisy.jpg --output clean.jpg --sigma 25.0 --fast-params --max-dimension 1024
"#
)]
struct Args {
    /// Input image path
    #[arg(short, long, value_name = "FILE")]
    input: PathBuf,
    
    /// Output image path
    #[arg(short, long, value_name = "FILE")]
    output: PathBuf,
    
    /// Noise sigma value (higher = more aggressive denoising)
    #[arg(short, long, default_value_t = 25.0, value_name = "FLOAT")]
    sigma: f64,
    
    /// Block size (patch size in pixels)
    #[arg(long, default_value_t = 8, value_name = "SIZE")]
    block_size: usize,
    
    /// Search window size (area to search for similar patches)
    #[arg(long, default_value_t = 21, value_name = "SIZE")]
    window_size: usize,
    
    /// Maximum number of similar patches to find
    #[arg(long, default_value_t = 8, value_name = "COUNT")]
    max_matches: usize,
    
    /// Step size between reference blocks (higher = faster)
    #[arg(long, default_value_t = 8, value_name = "STEP")]
    step_size: usize,
    
    /// Maximum image dimension for processing (0 = no resize)
    #[arg(long, default_value_t = 512, value_name = "PIXELS")]
    max_dimension: usize,
    
    /// Use optimized parameters for speed (overrides other parameters)
    #[arg(long, default_value_t = false)]
    fast_params: bool,
    
    /// Use high quality parameters (overrides other parameters)
    #[arg(long, default_value_t = false)]
    high_quality: bool,
    
    /// Verbose output with progress information
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
    
    /// Estimate processing time without actually denoising
    #[arg(long, default_value_t = false)]
    estimate_only: bool,
}

fn main() {
    let args = Args::parse();
    
    // Validazione input
    if !args.input.exists() {
        eprintln!("❌ Error: Input file '{}' does not exist", args.input.display());
        std::process::exit(1);
    }
    
    if args.sigma <= 0.0 {
        eprintln!("❌ Error: Sigma must be positive (got {})", args.sigma);
        std::process::exit(1);
    }
    
    if args.block_size < 4 || args.block_size > 32 {
        eprintln!("❌ Error: Block size must be between 4 and 32 (got {})", args.block_size);
        std::process::exit(1);
    }
    
    if args.window_size < args.block_size {
        eprintln!("❌ Error: Window size ({}) must be >= block size ({})", args.window_size, args.block_size);
        std::process::exit(1);
    }
    
    if args.step_size < 1 {
        eprintln!("❌ Error: Step size must be >= 1 (got {})", args.step_size);
        std::process::exit(1);
    }
    

    
    // Configurazione parametri
    let (block_size, window_size, max_matches, step_size, max_dimension) = if args.fast_params {
    println!("⚠️ Using FAST parameters (optimized for speed)");
    (8, 21, 4, 16, 512)  // ← CAMBIA QUI: 8 invece di 16, 21 invece di 15
    }else if args.high_quality {
        println!("⚠️ Using HIGH QUALITY parameters (slower but better)");
        (8, 39, 16, 3, if args.max_dimension > 0 { args.max_dimension } else { 2048 })
    } else {
        (args.block_size, args.window_size, args.max_matches, args.step_size, args.max_dimension)
    };
    
    // Stampa configurazione
    println!("╔════════════════════════════════════════════════╗");
    println!("║              BM3D Denoising Tool               ║");
    println!("╚════════════════════════════════════════════════╝");
    println!("");
    println!("📊 Configuration:");
    println!("  Input:          {}", args.input.display());
    println!("  Output:         {}", args.output.display());
    println!("  Sigma:          {}", args.sigma);
    println!("");
    println!("⚙️ Parameters:");
    println!("  Block size:     {} px", block_size);
    println!("  Window size:    {} px", window_size);
    println!("  Max matches:    {}", max_matches);
    println!("  Step size:      {}", step_size);
    println!("  Max dimension:  {}", if max_dimension > 0 { 
        format!("{} px", max_dimension) 
    } else { 
        "Original".to_string() 
    });
    println!("");
    
    if args.estimate_only {
        estimate_processing_time(&args.input, block_size, window_size, step_size, max_dimension);
        return;
    }
    
    if args.verbose {
        println!("🚀 Starting denoising process...");
    }
    
    // Chiama la funzione denoise con i parametri
    // Prima dobbiamo creare una versione di denoise che accetta parametri
    match denoise(
        &args.input,
        &args.output,
        args.sigma,
        block_size,
        window_size,
        max_matches,
        step_size,
        max_dimension as u32,
    ) {
        Ok(_) => {
            println!("");
            println!("✅ Denoising completed successfully!");
            println!("📁 Output saved to: {}", args.output.display());
            
            // Mostra informazioni sul file di output
            if let Ok(metadata) = std::fs::metadata(&args.output) {
                let size_kb = metadata.len() / 1024;
                println!("📦 File size: {} KB", size_kb);
            }
        }
        Err(e) => {
            eprintln!("");
            eprintln!("❌ Error: {}", e);
            eprintln!("");
            eprintln!("💡 Troubleshooting tips:");
            eprintln!("  1. Check if input image is corrupted");
            eprintln!("  2. Try with --fast-params for faster processing");
            eprintln!("  3. Reduce --max-dimension (e.g., 256)");
            eprintln!("  4. Increase --step-size (e.g., 16)");
            eprintln!("  5. Reduce --window-size (e.g., 15)");
            std::process::exit(1);
        }
    }
}

/// Stima il tempo di processing basato sui parametri
fn estimate_processing_time(
    input_path: &PathBuf,
    block_size: usize,
    window_size: usize,
    step_size: usize,
    max_dimension: usize,
) {
    println!("⏱️  Estimating processing time...");
    
    // Prova a ottenere le dimensioni dell'immagine
    if let Ok(img) = image::open(input_path) {
        
        let orig_w = img.width();
        let orig_h = img.height();
        
        // Calcola dimensioni di lavoro
        let (work_w, work_h) = if max_dimension > 0 && (orig_w > max_dimension as u32 || orig_h > max_dimension as u32) {
            let scale = max_dimension as f32 / orig_w.max(orig_h) as f32;
            let new_w = (orig_w as f32 * scale) as u32;
            let new_h = (orig_h as f32 * scale) as u32;
            (new_w, new_h)
        } else {
            (orig_w, orig_h)
        };
        
        // Calcola numero di blocchi
        let blocks_x = (work_w as usize).saturating_sub(block_size) / step_size + 1;
        let blocks_y = (work_h as usize).saturating_sub(block_size) / step_size + 1;
        let total_blocks = blocks_x * blocks_y;
        
        // Calcola operazioni per blocco
        let positions_per_block = window_size * window_size;
        let ops_per_block = positions_per_block * block_size * block_size * 3; // 3 canali RGB
        
        // Stima tempo (empirica)
        let ops_per_second = 50_000_000.0; // Operazioni al secondo stimate
        let estimated_seconds = (total_blocks as f64 * ops_per_block as f64) / ops_per_second;
        
        println!("");
        println!("📈 Estimation:");
        println!("  Original size:      {} x {}", orig_w, orig_h);
        println!("  Working size:       {} x {}", work_w, work_h);
        println!("  Total blocks:       {}", total_blocks);
        println!("  Positions/block:    {}", positions_per_block);
        println!("  Operations/block:   {:.1}M", ops_per_block as f64 / 1_000_000.0);
        println!("  Total operations:   {:.1}B", (total_blocks as f64 * ops_per_block as f64) / 1_000_000_000.0);
        println!("");
        println!("⏱️  Estimated time:");
        
        if estimated_seconds < 60.0 {
            println!("  About {:.1} seconds", estimated_seconds);
        } else if estimated_seconds < 3600.0 {
            println!("  About {:.1} minutes", estimated_seconds / 60.0);
        } else {
            println!("  About {:.1} hours", estimated_seconds / 3600.0);
        }
        
        println!("");
        println!("💡 Suggestions:");
        
        if estimated_seconds > 300.0 {
            println!("  ⚠️  This will take a long time!");
            println!("  Try: --fast-params or --max-dimension 256");
        } else if estimated_seconds > 60.0 {
            println!("  ⏳ This will take a few minutes");
            println!("  Consider: --step-size {}", step_size * 2);
        } else {
            println!("  🚀 This should be relatively fast");
        }
    } else {
        eprintln!("❌ Could not open image for estimation");
    }
}
