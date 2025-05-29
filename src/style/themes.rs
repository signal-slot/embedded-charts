//! Color themes for charts.

use embedded_graphics::prelude::*;

/// A color theme for charts
#[derive(Debug, Clone)]
pub struct Theme<C: PixelColor> {
    /// Background color
    pub background: C,
    /// Primary color for data
    pub primary: C,
    /// Secondary color for data
    pub secondary: C,
    /// Text color
    pub text: C,
    /// Grid color
    pub grid: C,
    /// Accent color for highlights
    pub accent: C,
    /// Success/positive color
    pub success: C,
    /// Warning color
    pub warning: C,
    /// Error/danger color
    pub error: C,
}

impl<C: PixelColor> Theme<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    /// Create a light theme with clean, modern colors
    pub fn light() -> Self {
        Self {
            background: embedded_graphics::pixelcolor::Rgb565::WHITE.into(),
            primary: embedded_graphics::pixelcolor::Rgb565::new(59 >> 3, 130 >> 2, 246 >> 3).into(), // Modern blue
            secondary: embedded_graphics::pixelcolor::Rgb565::new(239 >> 3, 68 >> 2, 68 >> 3)
                .into(), // Modern red
            text: embedded_graphics::pixelcolor::Rgb565::new(17 >> 3, 24 >> 2, 39 >> 3).into(), // Dark gray
            grid: embedded_graphics::pixelcolor::Rgb565::new(229 >> 3, 231 >> 2, 235 >> 3).into(), // Light gray
            accent: embedded_graphics::pixelcolor::Rgb565::new(147 >> 3, 51 >> 2, 234 >> 3).into(), // Purple
            success: embedded_graphics::pixelcolor::Rgb565::new(34 >> 3, 197 >> 2, 94 >> 3).into(), // Green
            warning: embedded_graphics::pixelcolor::Rgb565::new(245 >> 3, 158 >> 2, 11 >> 3).into(), // Amber
            error: embedded_graphics::pixelcolor::Rgb565::new(239 >> 3, 68 >> 2, 68 >> 3).into(), // Red
        }
    }

    /// Create a dark theme with modern, eye-friendly colors
    pub fn dark() -> Self {
        Self {
            background: embedded_graphics::pixelcolor::Rgb565::new(17 >> 3, 24 >> 2, 39 >> 3)
                .into(), // Dark blue-gray
            primary: embedded_graphics::pixelcolor::Rgb565::new(96 >> 3, 165 >> 2, 250 >> 3).into(), // Bright blue
            secondary: embedded_graphics::pixelcolor::Rgb565::new(251 >> 3, 113 >> 2, 133 >> 3)
                .into(), // Soft red
            text: embedded_graphics::pixelcolor::Rgb565::new(248 >> 3, 250 >> 2, 252 >> 3).into(), // Off-white
            grid: embedded_graphics::pixelcolor::Rgb565::new(55 >> 3, 65 >> 2, 81 >> 3).into(), // Medium gray
            accent: embedded_graphics::pixelcolor::Rgb565::new(168 >> 3, 85 >> 2, 247 >> 3).into(), // Bright purple
            success: embedded_graphics::pixelcolor::Rgb565::new(52 >> 3, 211 >> 2, 153 >> 3).into(), // Emerald
            warning: embedded_graphics::pixelcolor::Rgb565::new(251 >> 3, 191 >> 2, 36 >> 3).into(), // Yellow
            error: embedded_graphics::pixelcolor::Rgb565::new(248 >> 3, 113 >> 2, 113 >> 3).into(), // Soft red
        }
    }

    /// Create a vibrant theme with energetic colors
    pub fn vibrant() -> Self {
        Self {
            background: embedded_graphics::pixelcolor::Rgb565::new(255 >> 3, 251 >> 2, 235 >> 3)
                .into(), // Warm white
            primary: embedded_graphics::pixelcolor::Rgb565::new(236 >> 3, 72 >> 2, 153 >> 3).into(), // Hot pink
            secondary: embedded_graphics::pixelcolor::Rgb565::new(14 >> 3, 165 >> 2, 233 >> 3)
                .into(), // Sky blue
            text: embedded_graphics::pixelcolor::Rgb565::new(30 >> 3, 41 >> 2, 59 >> 3).into(), // Dark blue
            grid: embedded_graphics::pixelcolor::Rgb565::new(254 >> 3, 215 >> 2, 170 >> 3).into(), // Peach
            accent: embedded_graphics::pixelcolor::Rgb565::new(168 >> 3, 85 >> 2, 247 >> 3).into(), // Electric purple
            success: embedded_graphics::pixelcolor::Rgb565::new(16 >> 3, 185 >> 2, 129 >> 3).into(), // Teal green
            warning: embedded_graphics::pixelcolor::Rgb565::new(245 >> 3, 101 >> 2, 101 >> 3)
                .into(), // Coral
            error: embedded_graphics::pixelcolor::Rgb565::new(220 >> 3, 38 >> 2, 127 >> 3).into(), // Deep pink
        }
    }

    /// Create a pastel theme with soft, calming colors
    pub fn pastel() -> Self {
        Self {
            background: embedded_graphics::pixelcolor::Rgb565::new(253 >> 3, 253 >> 2, 253 >> 3)
                .into(), // Almost white
            primary: embedded_graphics::pixelcolor::Rgb565::new(147 >> 3, 197 >> 2, 253 >> 3)
                .into(), // Soft blue
            secondary: embedded_graphics::pixelcolor::Rgb565::new(252 >> 3, 165 >> 2, 165 >> 3)
                .into(), // Soft pink
            text: embedded_graphics::pixelcolor::Rgb565::new(75 >> 3, 85 >> 2, 99 >> 3).into(), // Muted gray
            grid: embedded_graphics::pixelcolor::Rgb565::new(243 >> 3, 244 >> 2, 246 >> 3).into(), // Very light gray
            accent: embedded_graphics::pixelcolor::Rgb565::new(196 >> 3, 181 >> 2, 253 >> 3).into(), // Lavender
            success: embedded_graphics::pixelcolor::Rgb565::new(167 >> 3, 243 >> 2, 208 >> 3)
                .into(), // Mint green
            warning: embedded_graphics::pixelcolor::Rgb565::new(254 >> 3, 215 >> 2, 170 >> 3)
                .into(), // Peach
            error: embedded_graphics::pixelcolor::Rgb565::new(254 >> 3, 202 >> 2, 202 >> 3).into(), // Light coral
        }
    }

    /// Create a nature-inspired theme with earth tones
    pub fn nature() -> Self {
        Self {
            background: embedded_graphics::pixelcolor::Rgb565::new(249 >> 3, 250 >> 2, 251 >> 3)
                .into(), // Off-white
            primary: embedded_graphics::pixelcolor::Rgb565::new(34 >> 3, 139 >> 2, 34 >> 3).into(), // Forest green
            secondary: embedded_graphics::pixelcolor::Rgb565::new(139 >> 3, 69 >> 2, 19 >> 3)
                .into(), // Saddle brown
            text: embedded_graphics::pixelcolor::Rgb565::new(41 >> 3, 37 >> 2, 36 >> 3).into(), // Dark brown
            grid: embedded_graphics::pixelcolor::Rgb565::new(229 >> 3, 229 >> 2, 229 >> 3).into(), // Light gray
            accent: embedded_graphics::pixelcolor::Rgb565::new(107 >> 3, 142 >> 2, 35 >> 3).into(), // Olive green
            success: embedded_graphics::pixelcolor::Rgb565::new(72 >> 3, 187 >> 2, 120 >> 3).into(), // Medium sea green
            warning: embedded_graphics::pixelcolor::Rgb565::new(218 >> 3, 165 >> 2, 32 >> 3).into(), // Goldenrod
            error: embedded_graphics::pixelcolor::Rgb565::new(178 >> 3, 34 >> 2, 34 >> 3).into(), // Fire brick
        }
    }

    /// Create an ocean-inspired theme with blue tones
    pub fn ocean() -> Self {
        Self {
            background: embedded_graphics::pixelcolor::Rgb565::new(240 >> 3, 249 >> 2, 255 >> 3)
                .into(), // Alice blue
            primary: embedded_graphics::pixelcolor::Rgb565::new(30 >> 3, 144 >> 2, 255 >> 3).into(), // Dodger blue
            secondary: embedded_graphics::pixelcolor::Rgb565::new(0 >> 3, 191 >> 2, 255 >> 3)
                .into(), // Deep sky blue
            text: embedded_graphics::pixelcolor::Rgb565::new(25 >> 3, 25 >> 2, 112 >> 3).into(), // Midnight blue
            grid: embedded_graphics::pixelcolor::Rgb565::new(230 >> 3, 230 >> 2, 250 >> 3).into(), // Lavender
            accent: embedded_graphics::pixelcolor::Rgb565::new(72 >> 3, 209 >> 2, 204 >> 3).into(), // Medium turquoise
            success: embedded_graphics::pixelcolor::Rgb565::new(32 >> 3, 178 >> 2, 170 >> 3).into(), // Light sea green
            warning: embedded_graphics::pixelcolor::Rgb565::new(255 >> 3, 215 >> 2, 0 >> 3).into(), // Gold
            error: embedded_graphics::pixelcolor::Rgb565::new(220 >> 3, 20 >> 2, 60 >> 3).into(), // Crimson
        }
    }

    /// Create a sunset theme with warm gradient colors
    pub fn sunset() -> Self {
        Self {
            background: embedded_graphics::pixelcolor::Rgb565::new(255 >> 3, 248 >> 2, 240 >> 3)
                .into(), // Seashell
            primary: embedded_graphics::pixelcolor::Rgb565::new(255 >> 3, 99 >> 2, 71 >> 3).into(), // Tomato
            secondary: embedded_graphics::pixelcolor::Rgb565::new(255 >> 3, 165 >> 2, 0 >> 3)
                .into(), // Orange
            text: embedded_graphics::pixelcolor::Rgb565::new(139 >> 3, 69 >> 2, 19 >> 3).into(), // Saddle brown
            grid: embedded_graphics::pixelcolor::Rgb565::new(255 >> 3, 228 >> 2, 196 >> 3).into(), // Bisque
            accent: embedded_graphics::pixelcolor::Rgb565::new(255 >> 3, 20 >> 2, 147 >> 3).into(), // Deep pink
            success: embedded_graphics::pixelcolor::Rgb565::new(255 >> 3, 215 >> 2, 0 >> 3).into(), // Gold
            warning: embedded_graphics::pixelcolor::Rgb565::new(255 >> 3, 140 >> 2, 0 >> 3).into(), // Dark orange
            error: embedded_graphics::pixelcolor::Rgb565::new(178 >> 3, 34 >> 2, 34 >> 3).into(), // Fire brick
        }
    }

    /// Create a cyberpunk theme with neon colors
    pub fn cyberpunk() -> Self {
        Self {
            background: embedded_graphics::pixelcolor::Rgb565::new(13 >> 3, 13 >> 2, 13 >> 3)
                .into(), // Very dark gray
            primary: embedded_graphics::pixelcolor::Rgb565::new(0 >> 3, 255 >> 2, 127 >> 3).into(), // Spring green (changed from cyan)
            secondary: embedded_graphics::pixelcolor::Rgb565::new(255 >> 3, 0 >> 2, 255 >> 3)
                .into(), // Magenta
            text: embedded_graphics::pixelcolor::Rgb565::new(0 >> 3, 255 >> 2, 255 >> 3).into(), // Cyan (moved from primary)
            grid: embedded_graphics::pixelcolor::Rgb565::new(64 >> 3, 64 >> 2, 64 >> 3).into(), // Dark gray
            accent: embedded_graphics::pixelcolor::Rgb565::new(255 >> 3, 255 >> 2, 0 >> 3).into(), // Yellow
            success: embedded_graphics::pixelcolor::Rgb565::new(50 >> 3, 205 >> 2, 50 >> 3).into(), // Lime green
            warning: embedded_graphics::pixelcolor::Rgb565::new(255 >> 3, 165 >> 2, 0 >> 3).into(), // Orange
            error: embedded_graphics::pixelcolor::Rgb565::new(255 >> 3, 69 >> 2, 0 >> 3).into(), // Red orange
        }
    }

    /// Create a minimal theme with subtle colors
    pub fn minimal() -> Self {
        Self {
            background: embedded_graphics::pixelcolor::Rgb565::new(250 >> 3, 250 >> 2, 250 >> 3)
                .into(), // Very light gray
            primary: embedded_graphics::pixelcolor::Rgb565::new(55 >> 3, 65 >> 2, 81 >> 3).into(), // Slate gray
            secondary: embedded_graphics::pixelcolor::Rgb565::new(107 >> 3, 114 >> 2, 128 >> 3)
                .into(), // Slate gray
            text: embedded_graphics::pixelcolor::Rgb565::new(31 >> 3, 41 >> 2, 55 >> 3).into(), // Dark slate gray
            grid: embedded_graphics::pixelcolor::Rgb565::new(241 >> 3, 245 >> 2, 249 >> 3).into(), // Very light blue
            accent: embedded_graphics::pixelcolor::Rgb565::new(99 >> 3, 102 >> 2, 241 >> 3).into(), // Indigo
            success: embedded_graphics::pixelcolor::Rgb565::new(16 >> 3, 185 >> 2, 129 >> 3).into(), // Emerald
            warning: embedded_graphics::pixelcolor::Rgb565::new(245 >> 3, 158 >> 2, 11 >> 3).into(), // Amber
            error: embedded_graphics::pixelcolor::Rgb565::new(239 >> 3, 68 >> 2, 68 >> 3).into(), // Red
        }
    }

    /// Create a retro theme with vintage colors
    pub fn retro() -> Self {
        Self {
            background: embedded_graphics::pixelcolor::Rgb565::new(245 >> 3, 245 >> 2, 220 >> 3)
                .into(), // Beige
            primary: embedded_graphics::pixelcolor::Rgb565::new(205 >> 3, 92 >> 2, 92 >> 3).into(), // Indian red
            secondary: embedded_graphics::pixelcolor::Rgb565::new(218 >> 3, 165 >> 2, 32 >> 3)
                .into(), // Goldenrod
            text: embedded_graphics::pixelcolor::Rgb565::new(139 >> 3, 69 >> 2, 19 >> 3).into(), // Saddle brown
            grid: embedded_graphics::pixelcolor::Rgb565::new(222 >> 3, 184 >> 2, 135 >> 3).into(), // Burlywood
            accent: embedded_graphics::pixelcolor::Rgb565::new(160 >> 3, 82 >> 2, 45 >> 3).into(), // Sienna
            success: embedded_graphics::pixelcolor::Rgb565::new(107 >> 3, 142 >> 2, 35 >> 3).into(), // Olive drab
            warning: embedded_graphics::pixelcolor::Rgb565::new(255 >> 3, 140 >> 2, 0 >> 3).into(), // Dark orange
            error: embedded_graphics::pixelcolor::Rgb565::new(178 >> 3, 34 >> 2, 34 >> 3).into(), // Fire brick
        }
    }
}
