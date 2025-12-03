//! Accelerazione GPU per Intel con OpenCL semplificato

use std::ffi::CString;
use opencl3::{device, context, command_queue, kernel, buffer, types::CL_MEM_READ_ONLY, types::CL_MEM_WRITE_ONLY};

pub struct GpuAccelerator {
    context: context::Context,
    queue: command_queue::CommandQueue,
    device: device::Device,
}

impl GpuAccelerator {
    pub fn new() -> Result<Self, String> {
        println!("Initializing Intel GPU acceleration...");
        
        // Ottieni piattaforma Intel
        let platforms = match device::get_platforms() {
            Ok(platforms) => platforms,
            Err(e) => return Err(format!("Failed to get OpenCL platforms: {:?}", e)),
        };
        
        if platforms.is_empty() {
            return Err("No OpenCL platforms found".to_string());
        }
        
        // Cerca piattaforma Intel
        let mut intel_platform = None;
        for platform in &platforms {
            let name = platform.name().unwrap_or_default();
            if name.contains("Intel") || name.contains("INTEL") {
                println!("Found Intel platform: {}", name);
                intel_platform = Some(platform);
                break;
            }
        }
        
        let platform = intel_platform.ok_or("No Intel OpenCL platform found")?;
        
        // Ottieni dispositivi GPU Intel
        let devices = match device::get_devices(platform, device::DeviceType::GPU) {
            Ok(devices) => devices,
            Err(_) => {
                // Prova anche CPU se GPU non trovata
                println!("No GPU devices found, trying CPU...");
                device::get_devices(platform, device::DeviceType::CPU)
                    .map_err(|e| format!("Failed to get devices: {:?}", e))?
            }
        };
        
        if devices.is_empty() {
            return Err("No OpenCL devices found".to_string());
        }
        
        // Prendi il primo dispositivo (probabilmente Intel UHD Graphics)
        let device = devices[0];
        let device_name = device.name().unwrap_or_else(|_| "Unknown".to_string());
        println!("Using device: {}", device_name);
        
        // Crea contesto
        let context = match context::Context::from_device(&device) {
            Ok(ctx) => ctx,
            Err(e) => return Err(format!("Failed to create context: {:?}", e)),
        };
        
        // Crea coda di comandi
        let queue = match command_queue::CommandQueue::create_default_with_properties(&context, &device, 0) {
            Ok(q) => q,
            Err(e) => return Err(format!("Failed to create command queue: {:?}", e)),
        };
        
        println!("✅ GPU acceleration initialized successfully");
        Ok(Self { context, queue, device })
    }
    
    pub fn is_available(&self) -> bool {
        true
    }
    
    pub fn compute_patch_distances_simple(
        &self,
        img_data: &[f32],
        ref_patch: &[f32],
        width: usize,
        height: usize,
        patch_size: usize,
    ) -> Result<Vec<(usize, usize, f32)>, String> {
        // Per semplicità, usa CPU per ora
        // In una versione completa, implementa il kernel OpenCL qui
        println!("⚠️ GPU kernel not implemented, falling back to CPU computation");
        
        let total_patches = (width - patch_size + 1) * (height - patch_size + 1);
        let mut results = Vec::with_capacity(total_patches);
        
        // Calcolo semplice su CPU (placeholder)
        for y in 0..(height - patch_size + 1) {
            for x in 0..(width - patch_size + 1) {
                // Calcola distanza L2 semplice
                let mut dist = 0.0;
                for i in 0..patch_size * patch_size {
                    let img_idx = (y * width + x) + i;
                    if img_idx < img_data.len() && i < ref_patch.len() {
                        let diff = img_data[img_idx] - ref_patch[i];
                        dist += diff * diff;
                    }
                }
                results.push((x, y, dist));
            }
        }
        
        Ok(results)
    }
}