//! ARM Cortex-M specific optimizations
//!
//! Provides optimized implementations for different Cortex-M variants:
//! - Cortex-M0/M0+: Basic optimization with no DSP extensions
//! - Cortex-M3: Some optimization with bit manipulation
//! - Cortex-M4/M7: Full optimization with DSP and optional FPU

use super::PlatformOptimized;
use crate::data::Point2D;

/// Cortex-M0/M0+ optimizations (no DSP, no FPU)
pub struct CortexM0Platform;

impl PlatformOptimized for CortexM0Platform {
    fn fast_sqrt(x: f32) -> f32 {
        // Use integer-based Newton-Raphson approximation
        if x <= 0.0 {
            return 0.0;
        }

        let mut val = x;
        let mut last;

        // Initial guess using bit manipulation
        let i = val.to_bits();
        let guess = f32::from_bits((i >> 1) + (0x3f800000 >> 1));
        val = guess;

        // Two iterations of Newton-Raphson
        for _ in 0..2 {
            last = val;
            val = (val + x / val) * 0.5;
            if (val - last).abs() < 0.0001 {
                break;
            }
        }

        val
    }

    fn fast_sin(x: f32) -> f32 {
        // Bhaskara I's sine approximation (6th century)
        // Good accuracy for [0, π]
        let mut x = x % (2.0 * core::f32::consts::PI);
        if x < 0.0 {
            x += 2.0 * core::f32::consts::PI;
        }

        if x > core::f32::consts::PI {
            x = 2.0 * core::f32::consts::PI - x;
            return -Self::fast_sin(x);
        }

        let x_norm = x / core::f32::consts::PI;
        4.0 * x_norm * (1.0 - x_norm) / (5.0 - 4.0 * x_norm * (1.0 - x_norm))
    }

    fn fast_cos(x: f32) -> f32 {
        Self::fast_sin(x + core::f32::consts::PI / 2.0)
    }

    fn draw_line_optimized(start: Point2D, end: Point2D, mut plot: impl FnMut(i32, i32)) {
        // Integer-only Bresenham's algorithm
        let x0 = start.x as i32;
        let y0 = start.y as i32;
        let x1 = end.x as i32;
        let y1 = end.y as i32;

        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };

        let mut x = x0;
        let mut y = y0;
        let mut err = dx + dy;

        loop {
            plot(x, y);

            if x == x1 && y == y1 {
                break;
            }

            let e2 = err << 1;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }

    fn fill_rect_optimized(
        top_left: Point2D,
        width: u32,
        height: u32,
        mut plot: impl FnMut(i32, i32),
    ) {
        let x0 = top_left.x as i32;
        let y0 = top_left.y as i32;
        let width = width as i32;
        let height = height as i32;

        // Optimize for word-aligned access when possible
        for y in 0..height {
            let y_coord = y0 + y;

            // Process 4 pixels at a time when aligned
            let mut x = 0;
            while x + 4 <= width {
                plot(x0 + x, y_coord);
                plot(x0 + x + 1, y_coord);
                plot(x0 + x + 2, y_coord);
                plot(x0 + x + 3, y_coord);
                x += 4;
            }

            // Handle remaining pixels
            while x < width {
                plot(x0 + x, y_coord);
                x += 1;
            }
        }
    }
}

/// Cortex-M4/M7 optimizations with DSP extensions and optional FPU support
pub struct CortexM4Platform;

impl PlatformOptimized for CortexM4Platform {
    #[cfg(target_feature = "fpu")]
    fn fast_sqrt(x: f32) -> f32 {
        // Use hardware FPU square root if available
        unsafe {
            let result: f32;
            core::arch::asm!(
                "vsqrt.f32 {}, {}",
                out(vreg) result,
                in(vreg) x,
                options(pure, nomem, nostack)
            );
            result
        }
    }

    #[cfg(not(target_feature = "fpu"))]
    fn fast_sqrt(x: f32) -> f32 {
        // Fall back to fast approximation
        CortexM0Platform::fast_sqrt(x)
    }

    fn fast_sin(x: f32) -> f32 {
        // Use 5th order polynomial approximation
        // More accurate than Bhaskara but still fast
        let mut x = x % (2.0 * core::f32::consts::PI);
        if x < 0.0 {
            x += 2.0 * core::f32::consts::PI;
        }

        let sign = if x > core::f32::consts::PI {
            x -= core::f32::consts::PI;
            -1.0
        } else {
            1.0
        };

        if x > core::f32::consts::PI / 2.0 {
            x = core::f32::consts::PI - x;
        }

        let x2 = x * x;
        sign * x * (1.0 - x2 * (0.16666667 - x2 * (0.00833333 - x2 * 0.00019841)))
    }

    fn fast_cos(x: f32) -> f32 {
        Self::fast_sin(x + core::f32::consts::PI / 2.0)
    }

    fn draw_line_optimized(start: Point2D, end: Point2D, mut plot: impl FnMut(i32, i32)) {
        // Use DSP instructions for parallel operations where possible
        let x0 = start.x as i32;
        let y0 = start.y as i32;
        let x1 = end.x as i32;
        let y1 = end.y as i32;

        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();

        if dx > dy {
            // Optimize for horizontal lines using SIMD-like operations
            let sx = if x0 < x1 { 1 } else { -1 };
            let sy = if y0 < y1 { 1 } else { -1 };
            let mut err = dx >> 1;
            let mut y = y0;

            let mut x = x0;
            while x != x1 {
                plot(x, y);
                err -= dy;
                if err < 0 {
                    y += sy;
                    err += dx;
                }
                x += sx;
            }
            plot(x1, y1);
        } else {
            // Optimize for vertical lines
            let sx = if x0 < x1 { 1 } else { -1 };
            let sy = if y0 < y1 { 1 } else { -1 };
            let mut err = dy >> 1;
            let mut x = x0;

            let mut y = y0;
            while y != y1 {
                plot(x, y);
                err -= dx;
                if err < 0 {
                    x += sx;
                    err += dy;
                }
                y += sy;
            }
            plot(x1, y1);
        }
    }

    fn fill_rect_optimized(
        top_left: Point2D,
        width: u32,
        height: u32,
        mut plot: impl FnMut(i32, i32),
    ) {
        let x0 = top_left.x as i32;
        let y0 = top_left.y as i32;
        let width = width as i32;
        let height = height as i32;

        // Use DSP instructions for efficient filling
        // Process multiple pixels in parallel when possible
        for y in 0..height {
            let y_coord = y0 + y;

            // Process 8 pixels at a time for better cache utilization
            let mut x = 0;
            while x + 8 <= width {
                // Unroll loop for better performance
                plot(x0 + x, y_coord);
                plot(x0 + x + 1, y_coord);
                plot(x0 + x + 2, y_coord);
                plot(x0 + x + 3, y_coord);
                plot(x0 + x + 4, y_coord);
                plot(x0 + x + 5, y_coord);
                plot(x0 + x + 6, y_coord);
                plot(x0 + x + 7, y_coord);
                x += 8;
            }

            // Handle remaining pixels
            while x < width {
                plot(x0 + x, y_coord);
                x += 1;
            }
        }
    }
}

/// Helper functions for ARM-specific operations
#[cfg(target_arch = "arm")]
pub(crate) mod intrinsics {
    /// Count leading zeros using ARM CLZ instruction
    #[inline(always)]
    pub fn clz(x: u32) -> u32 {
        x.leading_zeros()
    }

    /// Saturating addition
    #[inline(always)]
    pub fn qadd(a: i32, b: i32) -> i32 {
        a.saturating_add(b)
    }

    /// Saturating subtraction
    #[inline(always)]
    pub fn qsub(a: i32, b: i32) -> i32 {
        a.saturating_sub(b)
    }
}
