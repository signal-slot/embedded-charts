//! Platform-specific optimizations for embedded systems
//!
//! This module provides optimized implementations for different embedded platforms:
//! - ARM Cortex-M series (M0/M3/M4/M7)
//! - RISC-V 32/64-bit
//! - ESP32 family

use crate::data::Point2D;

// Platform detection and configuration
#[cfg(target_arch = "arm")]
pub mod arm;

#[cfg(target_arch = "riscv32")]
pub mod riscv;

#[cfg(target_arch = "xtensa")]
pub mod esp32;

/// Trait for platform-specific optimized operations
/// 
/// This trait provides optimized implementations of common mathematical
/// and drawing operations for different embedded platforms.
pub trait PlatformOptimized {
    /// Fast square root implementation
    fn fast_sqrt(x: f32) -> f32;
    
    /// Fast sine approximation
    fn fast_sin(x: f32) -> f32;
    
    /// Fast cosine approximation
    fn fast_cos(x: f32) -> f32;
    
    /// Optimized line drawing
    fn draw_line_optimized(start: Point2D, end: Point2D, plot: impl FnMut(i32, i32));
    
    /// Optimized rectangle filling
    fn fill_rect_optimized(top_left: Point2D, width: u32, height: u32, plot: impl FnMut(i32, i32));
}

/// Get the platform-specific implementation
pub fn get_platform() -> impl PlatformOptimized {
    #[cfg(all(target_arch = "arm", target_feature = "dsp"))]
    return arm::CortexM4Platform;
    
    #[cfg(all(target_arch = "arm", not(target_feature = "dsp")))]
    return arm::CortexM0Platform;
    
    #[cfg(target_arch = "riscv32")]
    return riscv::RiscVPlatform;
    
    #[cfg(target_arch = "xtensa")]
    return esp32::ESP32Platform;
    
    #[cfg(not(any(target_arch = "arm", target_arch = "riscv32", target_arch = "xtensa")))]
    return GenericPlatform;
}

/// Generic fallback implementation for platforms without specific optimizations
pub struct GenericPlatform;

impl PlatformOptimized for GenericPlatform {
    fn fast_sqrt(x: f32) -> f32 {
        // Fast inverse square root approximation
        let i = x.to_bits();
        let i = 0x5f3759df - (i >> 1);
        let y = f32::from_bits(i);
        let y = y * (1.5 - 0.5 * x * y * y);
        x * y
    }
    
    fn fast_sin(x: f32) -> f32 {
        // Normalize to [0, 2π]
        let mut x = x % (2.0 * core::f32::consts::PI);
        if x < 0.0 {
            x += 2.0 * core::f32::consts::PI;
        }
        
        // Reduce to [0, π]
        let sign = if x > core::f32::consts::PI {
            x -= core::f32::consts::PI;
            -1.0
        } else {
            1.0
        };
        
        // Reduce to [0, π/2]
        if x > core::f32::consts::PI / 2.0 {
            x = core::f32::consts::PI - x;
        }
        
        // Use a more accurate polynomial approximation
        let x2 = x * x;
        sign * x * (1.0 - x2 * (0.16666667 - x2 * (0.00833333 - x2 * 0.0001984)))
    }
    
    fn fast_cos(x: f32) -> f32 {
        // cos(x) = sin(x + π/2)
        Self::fast_sin(x + core::f32::consts::PI / 2.0)
    }
    
    fn draw_line_optimized(start: Point2D, end: Point2D, mut plot: impl FnMut(i32, i32)) {
        // Bresenham's line algorithm
        let mut x0 = start.x as i32;
        let mut y0 = start.y as i32;
        let x1 = end.x as i32;
        let y1 = end.y as i32;
        
        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx - dy;
        
        loop {
            plot(x0, y0);
            
            if x0 == x1 && y0 == y1 {
                break;
            }
            
            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x0 += sx;
            }
            if e2 < dx {
                err += dx;
                y0 += sy;
            }
        }
    }
    
    fn fill_rect_optimized(top_left: Point2D, width: u32, height: u32, mut plot: impl FnMut(i32, i32)) {
        let x0 = top_left.x as i32;
        let y0 = top_left.y as i32;
        
        for y in 0..height as i32 {
            for x in 0..width as i32 {
                plot(x0 + x, y0 + y);
            }
        }
    }
}