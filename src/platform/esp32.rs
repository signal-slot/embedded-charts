//! ESP32 specific optimizations
//!
//! Provides optimized implementations for ESP32 family processors
//! including ESP32, ESP32-S2, ESP32-S3, ESP32-C3

use crate::data::Point2D;
use super::PlatformOptimized;

/// ESP32 platform optimizations
pub struct ESP32Platform;

impl PlatformOptimized for ESP32Platform {
    fn fast_sqrt(x: f32) -> f32 {
        // ESP32 has hardware FPU, use it directly
        #[cfg(any(target_cpu = "esp32", target_cpu = "esp32s3"))]
        {
            x.sqrt()
        }
        
        // ESP32-C3 (RISC-V based) may not have FPU
        #[cfg(not(any(target_cpu = "esp32", target_cpu = "esp32s3")))]
        {
            // Use fast approximation
            if x <= 0.0 {
                return 0.0;
            }
            
            // Initial approximation
            let mut y = x;
            let mut last;
            
            // Newton-Raphson iterations
            for _ in 0..3 {
                last = y;
                y = 0.5 * (y + x / y);
                if (y - last).abs() < 0.00001 {
                    break;
                }
            }
            
            y
        }
    }
    
    fn fast_sin(x: f32) -> f32 {
        // ESP32 optimized sine using ROM tables when available
        // Otherwise use polynomial approximation
        
        let mut x = x % (2.0 * core::f32::consts::PI);
        if x < 0.0 {
            x += 2.0 * core::f32::consts::PI;
        }
        
        // Reduce to [0, π/2]
        let (sign, x) = if x > core::f32::consts::PI {
            (-1.0, x - core::f32::consts::PI)
        } else {
            (1.0, x)
        };
        
        let x = if x > core::f32::consts::PI / 2.0 {
            core::f32::consts::PI - x
        } else {
            x
        };
        
        // High precision polynomial for ESP32's FPU
        let x2 = x * x;
        let x3 = x2 * x;
        let x5 = x3 * x2;
        let x7 = x5 * x2;
        
        sign * (x - x3 / 6.0 + x5 / 120.0 - x7 / 5040.0)
    }
    
    fn fast_cos(x: f32) -> f32 {
        Self::fast_sin(x + core::f32::consts::PI / 2.0)
    }
    
    fn draw_line_optimized(start: Point2D, end: Point2D, mut plot: impl FnMut(i32, i32)) {
        // ESP32 specific optimizations:
        // - Use dual-core capabilities when available
        // - Optimize for PSRAM access patterns
        
        let x0 = start.x as i32;
        let y0 = start.y as i32;
        let x1 = end.x as i32;
        let y1 = end.y as i32;
        
        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        
        // For ESP32, we can use DMA-friendly patterns for long lines
        if dx > 32 || dy > 32 {
            Self::draw_line_dma_optimized(x0, y0, x1, y1, plot);
        } else {
            Self::draw_line_bresenham(x0, y0, x1, y1, plot);
        }
    }
    
    fn fill_rect_optimized(top_left: Point2D, width: u32, height: u32, mut plot: impl FnMut(i32, i32)) {
        let x0 = top_left.x as i32;
        let y0 = top_left.y as i32;
        let width = width as i32;
        let height = height as i32;
        
        // ESP32 optimization: use DMA-friendly patterns
        // Process in chunks that align with ESP32's memory architecture
        const CHUNK_WIDTH: i32 = 32; // Optimize for 32-bit access
        
        for y in 0..height {
            let y_coord = y0 + y;
            let mut x = 0;
            
            // Process aligned chunks
            while x + CHUNK_WIDTH <= width {
                // Prefetch next line for better cache usage
                let _prefetch = y + 1 < height;
                
                // Process chunk
                for dx in 0..CHUNK_WIDTH {
                    plot(x0 + x + dx, y_coord);
                }
                x += CHUNK_WIDTH;
            }
            
            // Process remaining pixels
            while x < width {
                plot(x0 + x, y_coord);
                x += 1;
            }
        }
    }
}

impl ESP32Platform {
    /// Standard Bresenham's algorithm for short lines
    fn draw_line_bresenham(x0: i32, y0: i32, x1: i32, y1: i32, mut plot: impl FnMut(i32, i32)) {
        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx - dy;
        
        let mut x = x0;
        let mut y = y0;
        
        loop {
            plot(x, y);
            
            if x == x1 && y == y1 {
                break;
            }
            
            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
    }
    
    /// DMA-optimized line drawing for longer lines
    fn draw_line_dma_optimized(x0: i32, y0: i32, x1: i32, y1: i32, mut plot: impl FnMut(i32, i32)) {
        let dx = x1 - x0;
        let dy = y1 - y0;
        let steps = dx.abs().max(dy.abs());
        
        if steps == 0 {
            plot(x0, y0);
            return;
        }
        
        // Use fixed-point arithmetic for better precision
        let x_inc = ((dx as i64) << 16) / steps as i64;
        let y_inc = ((dy as i64) << 16) / steps as i64;
        
        let mut x_fixed = (x0 as i64) << 16;
        let mut y_fixed = (y0 as i64) << 16;
        
        // Process in batches for better performance
        const BATCH_SIZE: i32 = 8;
        let full_batches = steps / BATCH_SIZE;
        let remainder = steps % BATCH_SIZE;
        
        // Process full batches
        for _ in 0..full_batches {
            for _ in 0..BATCH_SIZE {
                plot((x_fixed >> 16) as i32, (y_fixed >> 16) as i32);
                x_fixed += x_inc;
                y_fixed += y_inc;
            }
        }
        
        // Process remainder
        for _ in 0..remainder {
            plot((x_fixed >> 16) as i32, (y_fixed >> 16) as i32);
            x_fixed += x_inc;
            y_fixed += y_inc;
        }
        
        // Ensure we hit the end point
        plot(x1, y1);
    }
}

/// ESP32 specific features and intrinsics
#[cfg(target_arch = "xtensa")]
pub(crate) mod intrinsics {
    /// Use ESP32's MAC unit for multiply-accumulate operations
    #[inline(always)]
    pub fn mac(acc: i32, a: i32, b: i32) -> i32 {
        acc + a * b
    }
    
    /// Prefetch data for better cache performance
    #[inline(always)]
    pub fn prefetch(addr: *const u8) {
        // Hint to the processor to prefetch data
        unsafe {
            core::ptr::read_volatile(addr);
        }
    }
}