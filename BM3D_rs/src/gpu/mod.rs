use ocl::{Platform, Device, Context, Queue, Program, Buffer, Kernel};

pub struct GpuAccelerator {
    context: Context,
    queue: Queue,
    program: Program,
}

impl GpuAccelerator {
    pub fn new() -> Result<Self, String> {
        println!("Initializing GPU acceleration...");
        
        let platform = Platform::default();
        println!("Using OpenCL platform: {:?}", platform.name());
        
        let devices = match Device::list_all(platform) {
            Ok(devices) => devices,
            Err(e) => return Err(format!("Failed to list devices: {}", e)),
        };
        
        if devices.is_empty() {
            return Err("No OpenCL devices found".to_string());
        }
        
        println!("Found {} GPU devices:", devices.len());
        for (i, device) in devices.iter().enumerate() {
            let name = device.name().unwrap_or_else(|_| "Unknown".to_string());
            let device_type = device.device_type().unwrap_or_default();
            let is_gpu = device_type.contains(ocl::flags::DeviceType::GPU);
            println!("  {}: {} ({})", i, name, if is_gpu { "GPU" } else { "CPU" });
        }
        
        // Cerca GPU Intel
        let intel_device = devices.iter()
            .find(|d| {
                let is_gpu = d.device_type()
                    .map(|t| t.contains(ocl::flags::DeviceType::GPU))
                    .unwrap_or(false);
                let name = d.name().unwrap_or_default();
                is_gpu && (name.contains("Intel") || name.contains("UHD") || name.contains("HD Graphics"))
            })
            .or_else(|| devices.first());
        
        let device = match intel_device {
            Some(d) => d,
            None => return Err("No compatible Intel GPU found".to_string()),
        };
        
        println!("Using device: {:?}", device.name());
        
        let context = match Context::builder()
            .devices(device.clone())
            .platform(platform)
            .build()
        {
            Ok(ctx) => ctx,
            Err(e) => return Err(format!("Failed to create context: {}", e)),
        };
        
        let queue = match Queue::new(&context, *device, None) {
            Ok(q) => q,
            Err(e) => return Err(format!("Failed to create queue: {}", e)),
        };
        
        // Kernel OpenCL ottimizzato
        let kernel_source = r#"
            __kernel void patch_distance(
                __global const float* img_data,
                __global const float* ref_patch,
                __global float* distances,
                const int width,
                const int height,
                const int patch_size,
                const int channels,
                const int start_x,
                const int start_y,
                const int end_x,
                const int end_y
            ) {
                int gid = get_global_id(0);
                int search_width = end_x - start_x + 1;
                int search_height = end_y - start_y + 1;
                int total_positions = search_width * search_height;
                
                if (gid >= total_positions) return;
                
                int rel_y = gid / search_width;
                int rel_x = gid % search_width;
                int patch_x = start_x + rel_x;
                int patch_y = start_y + rel_y;
                
                // Verifica bounds
                if (patch_x + patch_size > width || patch_y + patch_size > height) {
                    distances[gid] = 1e10f; // Distanza molto grande
                    return;
                }
                
                float dist = 0.0f;
                
                for (int ch = 0; ch < channels; ch++) {
                    for (int y = 0; y < patch_size; y++) {
                        int img_row_start = (patch_y + y) * width * channels + ch;
                        int ref_row_start = y * patch_size * channels + ch;
                        
                        for (int x = 0; x < patch_size; x++) {
                            int img_idx = img_row_start + (patch_x + x) * channels;
                            int ref_idx = ref_row_start + x * channels;
                            
                            float diff = img_data[img_idx] - ref_patch[ref_idx];
                            dist += diff * diff;
                        }
                    }
                }
                
                distances[gid] = dist;
            }
        "#;
        
        let program = match Program::builder()
            .source(kernel_source)
            .devices(device)
            .build(&context)
        {
            Ok(prog) => prog,
            Err(e) => return Err(format!("Failed to build program: {}", e)),
        };
        
        println!("GPU acceleration initialized successfully");
        Ok(Self { context, queue, program })
    }
    
    pub fn find_similar_patches_gpu(
        &self,
        img_data: &[f32],
        ref_patch: &[f32],
        width: usize,
        height: usize,
        patch_size: usize,
        channels: usize,
        max_results: usize,
    ) -> Result<Vec<(usize, usize, f32)>, String> {
        // Per ora cerca in tutta l'immagine
        let start_x = 0;
        let start_y = 0;
        let end_x = width.saturating_sub(patch_size);
        let end_y = height.saturating_sub(patch_size);
        
        let search_width = end_x - start_x + 1;
        let search_height = end_y - start_y + 1;
        let total_positions = search_width * search_height;
        
        if total_positions == 0 {
            return Ok(Vec::new());
        }
        
        // Crea buffer GPU
        let img_buffer = Buffer::<f32>::builder()
            .queue(self.queue.clone())
            .flags(ocl::flags::MEM_READ_ONLY)
            .len(img_data.len())
            .copy_host_slice(img_data)
            .build()
            .map_err(|e| format!("Failed to create image buffer: {}", e))?;
        
        let ref_buffer = Buffer::<f32>::builder()
            .queue(self.queue.clone())
            .flags(ocl::flags::MEM_READ_ONLY)
            .len(ref_patch.len())
            .copy_host_slice(ref_patch)
            .build()
            .map_err(|e| format!("Failed to create reference buffer: {}", e))?;
        
        let dist_buffer = Buffer::<f32>::builder()
            .queue(self.queue.clone())
            .flags(ocl::flags::MEM_WRITE_ONLY)
            .len(total_positions)
            .build()
            .map_err(|e| format!("Failed to create distance buffer: {}", e))?;
        
        // Crea kernel
        let kernel = Kernel::builder()
            .program(&self.program)
            .name("patch_distance")
            .queue(self.queue.clone())
            .global_work_size(total_positions)
            .local_work_size(256.min(total_positions))
            .arg(&img_buffer)
            .arg(&ref_buffer)
            .arg(&dist_buffer)
            .arg(&(width as i32))
            .arg(&(height as i32))
            .arg(&(patch_size as i32))
            .arg(&(channels as i32))
            .arg(&(start_x as i32))
            .arg(&(start_y as i32))
            .arg(&(end_x as i32))
            .arg(&(end_y as i32))
            .build()
            .map_err(|e| format!("Failed to build kernel: {}", e))?;
        
        // Esegui kernel
        unsafe {
            kernel.enq().map_err(|e| format!("Failed to enqueue kernel: {}", e))?;
        }
        
        // Leggi risultati
        let mut distances = vec![0.0f32; total_positions];
        dist_buffer.read(&mut distances).enq()
            .map_err(|e| format!("Failed to read results: {}", e))?;
        
        // Trova i migliori match
        let mut results: Vec<(usize, usize, f32)> = Vec::new();
        
        for (i, &dist) in distances.iter().enumerate() {
            let rel_y = i / search_width;
            let rel_x = i % search_width;
            let patch_x = start_x + rel_x;
            let patch_y = start_y + rel_y;
            
            if patch_x <= end_x && patch_y <= end_y {
                results.push((patch_x, patch_y, dist));
            }
        }
        
        // Ordina per distanza
        results.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
        
        // Prendi i primi max_results
        Ok(results.into_iter().take(max_results).collect())
    }
}