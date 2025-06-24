//! RISC-V specific optimizations
//!
//! Provides optimized implementations for RISC-V processors

use super::PlatformOptimized;
use crate::data::Point2D;

/// RISC-V platform optimizations
pub struct RiscVPlatform;

impl PlatformOptimized for RiscVPlatform {
    fn fast_sqrt(x: f32) -> f32 {
        // RISC-V often has hardware FPU support
        #[cfg(target_feature = "f")]
        {
            // If F extension is available, use hardware sqrt
            x.sqrt()
        }

        #[cfg(not(target_feature = "f"))]
        {
            // Fast approximation for RV32I without FPU
            if x <= 0.0 {
                return 0.0;
            }

            // Carmack's fast inverse square root adapted
            let threehalfs = 1.5f32;
            let x2 = x * 0.5f32;
            let mut i = x.to_bits();
            i = 0x5f375a86 - (i >> 1); // Magic constant tuned for better accuracy
            let mut y = f32::from_bits(i);

            // Two Newton-Raphson iterations
            y = y * (threehalfs - (x2 * y * y));
            y = y * (threehalfs - (x2 * y * y));

            x * y
        }
    }

    fn fast_sin(x: f32) -> f32 {
        // RISC-V optimized sine using Padé approximation
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

        // Padé [3,2] approximation
        let x2 = x * x;
        let num = x * (11.0 * x2 - 183.0);
        let den = x2 - 30.0;
        sign * (num / (6.0 * den))
    }

    fn fast_cos(x: f32) -> f32 {
        // Use identity cos(x) = sin(x + π/2)
        Self::fast_sin(x + core::f32::consts::PI / 2.0)
    }

    fn draw_line_optimized(start: Point2D, end: Point2D, mut plot: impl FnMut(i32, i32)) {
        // RISC-V optimized Bresenham with branch prediction hints
        let x0 = start.x as i32;
        let y0 = start.y as i32;
        let x1 = end.x as i32;
        let y1 = end.y as i32;

        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();

        // Use conditional moves when available to reduce branches
        let steep = dy > dx;

        if steep {
            // Swap coordinates for steep lines
            Self::draw_line_steep(x0, y0, x1, y1, plot);
        } else {
            // Normal line drawing
            Self::draw_line_shallow(x0, y0, x1, y1, plot);
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

        // RISC-V benefits from predictable memory access patterns
        // Use tiling for better cache performance
        const TILE_SIZE: i32 = 16;

        let full_tiles_x = width / TILE_SIZE;
        let full_tiles_y = height / TILE_SIZE;
        let remainder_x = width % TILE_SIZE;
        let remainder_y = height % TILE_SIZE;

        // Process full tiles
        for tile_y in 0..full_tiles_y {
            for tile_x in 0..full_tiles_x {
                let base_x = x0 + tile_x * TILE_SIZE;
                let base_y = y0 + tile_y * TILE_SIZE;

                for dy in 0..TILE_SIZE {
                    for dx in 0..TILE_SIZE {
                        plot(base_x + dx, base_y + dy);
                    }
                }
            }
        }

        // Process right edge
        if remainder_x > 0 {
            for tile_y in 0..full_tiles_y {
                let base_x = x0 + full_tiles_x * TILE_SIZE;
                let base_y = y0 + tile_y * TILE_SIZE;

                for dy in 0..TILE_SIZE {
                    for dx in 0..remainder_x {
                        plot(base_x + dx, base_y + dy);
                    }
                }
            }
        }

        // Process bottom edge
        if remainder_y > 0 {
            for tile_x in 0..full_tiles_x {
                let base_x = x0 + tile_x * TILE_SIZE;
                let base_y = y0 + full_tiles_y * TILE_SIZE;

                for dy in 0..remainder_y {
                    for dx in 0..TILE_SIZE {
                        plot(base_x + dx, base_y + dy);
                    }
                }
            }
        }

        // Process bottom-right corner
        if remainder_x > 0 && remainder_y > 0 {
            let base_x = x0 + full_tiles_x * TILE_SIZE;
            let base_y = y0 + full_tiles_y * TILE_SIZE;

            for dy in 0..remainder_y {
                for dx in 0..remainder_x {
                    plot(base_x + dx, base_y + dy);
                }
            }
        }
    }
}

impl RiscVPlatform {
    /// Optimized line drawing for shallow lines (dx > dy)
    fn draw_line_shallow(x0: i32, y0: i32, x1: i32, y1: i32, mut plot: impl FnMut(i32, i32)) {
        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };

        let mut err = dx >> 1;
        let mut x = x0;
        let mut y = y0;

        while x != x1 {
            plot(x, y);
            err -= dy;
            x += sx;

            // Branchless y increment
            let mask = (err >> 31) as i32;
            y += sy & mask;
            err += dx & mask;
        }
        plot(x1, y1);
    }

    /// Optimized line drawing for steep lines (dy > dx)
    fn draw_line_steep(x0: i32, y0: i32, x1: i32, y1: i32, mut plot: impl FnMut(i32, i32)) {
        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };

        let mut err = dy >> 1;
        let mut x = x0;
        let mut y = y0;

        while y != y1 {
            plot(x, y);
            err -= dx;
            y += sy;

            // Branchless x increment
            let mask = (err >> 31) as i32;
            x += sx & mask;
            err += dy & mask;
        }
        plot(x1, y1);
    }
}

/// RISC-V specific helper functions
#[cfg(target_arch = "riscv32")]
pub(crate) mod intrinsics {
    /// Population count (number of set bits)
    #[inline(always)]
    pub fn popcount(x: u32) -> u32 {
        x.count_ones()
    }

    /// Count trailing zeros
    #[inline(always)]
    pub fn ctz(x: u32) -> u32 {
        x.trailing_zeros()
    }
}
