use std::path::PathBuf;
use clap::Parser;
use bm3d_rs::denoise;

/// BM3D Denoising Tool
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input image path
    #[arg(short, long)]
    input: PathBuf,
    
    /// Output image path
    #[arg(short, long)]
    output: PathBuf,
    
    /// Noise sigma value
    #[arg(short, long, default_value_t = 15.0)]
    sigma: f64,
    
    /// Verbose output
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();
    
    if args.verbose {
        println!("BM3D Denoising Tool");
        println!("Input: {:?}", args.input);
        println!("Output: {:?}", args.output);
        println!("Sigma: {}", args.sigma);
    }
    
    match denoise(&args.input, &args.output, args.sigma) {
        Ok(_) => {
            println!("✅ Denoising completed successfully!");
        }
        Err(e) => {
            eprintln!("❌ Error: {}", e);
            std::process::exit(1);
        }
    }
}