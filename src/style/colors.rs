//! Color utilities and palettes for charts.

use embedded_graphics::prelude::*;
use heapless::Vec;

/// Color palette for charts
#[derive(Debug, Clone)]
pub struct ColorPalette<C: PixelColor, const N: usize> {
    colors: Vec<C, N>,
    current_index: usize,
}

impl<C: PixelColor, const N: usize> ColorPalette<C, N> {
    /// Create a new empty color palette
    pub fn new() -> Self {
        Self {
            colors: Vec::new(),
            current_index: 0,
        }
    }

    /// Create a palette from a slice of colors
    pub fn from_colors(colors: &[C]) -> Result<Self, crate::error::DataError> {
        let mut palette = Self::new();
        for &color in colors {
            palette.add_color(color)?;
        }
        Ok(palette)
    }

    /// Add a color to the palette
    pub fn add_color(&mut self, color: C) -> Result<(), crate::error::DataError> {
        self.colors
            .push(color)
            .map_err(|_| crate::error::DataError::buffer_full("add color to palette", N))
    }

    /// Get the next color in the palette (cycles through)
    pub fn next_color(&mut self) -> Option<C> {
        if self.colors.is_empty() {
            return None;
        }

        let color = self.colors[self.current_index];
        self.current_index = (self.current_index + 1) % self.colors.len();
        Some(color)
    }

    /// Get a color by index
    pub fn get_color(&self, index: usize) -> Option<C> {
        self.colors.get(index).copied()
    }

    /// Get the number of colors in the palette
    pub fn len(&self) -> usize {
        self.colors.len()
    }

    /// Check if the palette is empty
    pub fn is_empty(&self) -> bool {
        self.colors.is_empty()
    }

    /// Reset the color index to the beginning
    pub fn reset(&mut self) {
        self.current_index = 0;
    }

    /// Get all colors as a slice
    pub fn as_slice(&self) -> &[C] {
        &self.colors
    }
}

impl<C: PixelColor, const N: usize> Default for ColorPalette<C, N> {
    fn default() -> Self {
        Self::new()
    }
}

/// Predefined color palettes for RGB565
#[cfg(feature = "color-support")]
pub mod rgb565_palettes {
    use super::*;
    use embedded_graphics::pixelcolor::Rgb565;

    /// Default color palette with modern, vibrant colors
    pub fn default_palette() -> ColorPalette<Rgb565, 8> {
        ColorPalette::from_colors(&[
            Rgb565::new(59 >> 3, 130 >> 2, 246 >> 3),  // Modern blue
            Rgb565::new(239 >> 3, 68 >> 2, 68 >> 3),   // Modern red
            Rgb565::new(34 >> 3, 197 >> 2, 94 >> 3),   // Emerald green
            Rgb565::new(245 >> 3, 158 >> 2, 11 >> 3),  // Amber
            Rgb565::new(147 >> 3, 51 >> 2, 234 >> 3),  // Purple
            Rgb565::new(6 >> 3, 182 >> 2, 212 >> 3),   // Cyan
            Rgb565::new(251 >> 3, 113 >> 2, 133 >> 3), // Rose
            Rgb565::new(168 >> 3, 85 >> 2, 247 >> 3),  // Violet
        ])
        .unwrap()
    }

    /// Professional color palette with sophisticated colors
    pub fn professional_palette() -> ColorPalette<Rgb565, 8> {
        ColorPalette::from_colors(&[
            Rgb565::new(30 >> 3, 58 >> 2, 138 >> 3),  // Navy blue
            Rgb565::new(185 >> 3, 28 >> 2, 28 >> 3),  // Dark red
            Rgb565::new(21 >> 3, 128 >> 2, 61 >> 3),  // Forest green
            Rgb565::new(217 >> 3, 119 >> 2, 6 >> 3),  // Orange
            Rgb565::new(88 >> 3, 28 >> 2, 135 >> 3),  // Indigo
            Rgb565::new(14 >> 3, 116 >> 2, 144 >> 3), // Teal
            Rgb565::new(120 >> 3, 53 >> 2, 15 >> 3),  // Brown
            Rgb565::new(75 >> 3, 85 >> 2, 99 >> 3),   // Slate gray
        ])
        .unwrap()
    }

    /// Pastel color palette for gentle, soothing appearance
    pub fn pastel_palette() -> ColorPalette<Rgb565, 8> {
        ColorPalette::from_colors(&[
            Rgb565::new(147 >> 3, 197 >> 2, 253 >> 3), // Sky blue
            Rgb565::new(252 >> 3, 165 >> 2, 165 >> 3), // Light pink
            Rgb565::new(167 >> 3, 243 >> 2, 208 >> 3), // Mint green
            Rgb565::new(254 >> 3, 215 >> 2, 170 >> 3), // Peach
            Rgb565::new(196 >> 3, 181 >> 2, 253 >> 3), // Lavender
            Rgb565::new(165 >> 3, 243 >> 2, 252 >> 3), // Light cyan
            Rgb565::new(254 >> 3, 202 >> 2, 202 >> 3), // Light coral
            Rgb565::new(253 >> 3, 230 >> 2, 138 >> 3), // Light yellow
        ])
        .unwrap()
    }

    /// Vibrant color palette for energetic designs
    pub fn vibrant_palette() -> ColorPalette<Rgb565, 8> {
        ColorPalette::from_colors(&[
            Rgb565::new(236 >> 3, 72 >> 2, 153 >> 3),  // Hot pink
            Rgb565::new(14 >> 3, 165 >> 2, 233 >> 3),  // Sky blue
            Rgb565::new(16 >> 3, 185 >> 2, 129 >> 3),  // Teal green
            Rgb565::new(245 >> 3, 101 >> 2, 101 >> 3), // Coral
            Rgb565::new(168 >> 3, 85 >> 2, 247 >> 3),  // Electric purple
            Rgb565::new(251 >> 3, 191 >> 2, 36 >> 3),  // Bright yellow
            Rgb565::new(220 >> 3, 38 >> 2, 127 >> 3),  // Deep pink
            Rgb565::new(6 >> 3, 182 >> 2, 212 >> 3),   // Bright cyan
        ])
        .unwrap()
    }

    /// Nature-inspired color palette with earth tones
    pub fn nature_palette() -> ColorPalette<Rgb565, 8> {
        ColorPalette::from_colors(&[
            Rgb565::new(34 >> 3, 139 >> 2, 34 >> 3),  // Forest green
            Rgb565::new(139 >> 3, 69 >> 2, 19 >> 3),  // Saddle brown
            Rgb565::new(107 >> 3, 142 >> 2, 35 >> 3), // Olive green
            Rgb565::new(218 >> 3, 165 >> 2, 32 >> 3), // Goldenrod
            Rgb565::new(72 >> 3, 187 >> 2, 120 >> 3), // Medium sea green
            Rgb565::new(160 >> 3, 82 >> 2, 45 >> 3),  // Sienna
            Rgb565::new(85 >> 3, 107 >> 2, 47 >> 3),  // Dark olive green
            Rgb565::new(205 >> 3, 133 >> 2, 63 >> 3), // Peru
        ])
        .unwrap()
    }

    /// Ocean-inspired color palette with blue tones
    pub fn ocean_palette() -> ColorPalette<Rgb565, 8> {
        ColorPalette::from_colors(&[
            Rgb565::new(30 >> 3, 144 >> 2, 255 >> 3),  // Dodger blue
            Rgb565::new(0 >> 3, 191 >> 2, 255 >> 3),   // Deep sky blue
            Rgb565::new(72 >> 3, 209 >> 2, 204 >> 3),  // Medium turquoise
            Rgb565::new(32 >> 3, 178 >> 2, 170 >> 3),  // Light sea green
            Rgb565::new(95 >> 3, 158 >> 2, 160 >> 3),  // Cadet blue
            Rgb565::new(70 >> 3, 130 >> 2, 180 >> 3),  // Steel blue
            Rgb565::new(123 >> 3, 104 >> 2, 238 >> 3), // Medium slate blue
            Rgb565::new(25 >> 3, 25 >> 2, 112 >> 3),   // Midnight blue
        ])
        .unwrap()
    }

    /// Sunset-inspired color palette with warm tones
    pub fn sunset_palette() -> ColorPalette<Rgb565, 8> {
        ColorPalette::from_colors(&[
            Rgb565::new(255 >> 3, 99 >> 2, 71 >> 3),  // Tomato
            Rgb565::new(255 >> 3, 165 >> 2, 0 >> 3),  // Orange
            Rgb565::new(255 >> 3, 215 >> 2, 0 >> 3),  // Gold
            Rgb565::new(255 >> 3, 20 >> 2, 147 >> 3), // Deep pink
            Rgb565::new(255 >> 3, 140 >> 2, 0 >> 3),  // Dark orange
            Rgb565::new(220 >> 3, 20 >> 2, 60 >> 3),  // Crimson
            Rgb565::new(255 >> 3, 69 >> 2, 0 >> 3),   // Red orange
            Rgb565::new(178 >> 3, 34 >> 2, 34 >> 3),  // Fire brick
        ])
        .unwrap()
    }

    /// Cyberpunk-inspired color palette with neon colors
    pub fn cyberpunk_palette() -> ColorPalette<Rgb565, 8> {
        ColorPalette::from_colors(&[
            Rgb565::new(0 >> 3, 255 >> 2, 255 >> 3),  // Cyan
            Rgb565::new(255 >> 3, 0 >> 2, 255 >> 3),  // Magenta
            Rgb565::new(0 >> 3, 255 >> 2, 127 >> 3),  // Spring green
            Rgb565::new(255 >> 3, 255 >> 2, 0 >> 3),  // Yellow
            Rgb565::new(50 >> 3, 205 >> 2, 50 >> 3),  // Lime green
            Rgb565::new(255 >> 3, 165 >> 2, 0 >> 3),  // Orange
            Rgb565::new(255 >> 3, 69 >> 2, 0 >> 3),   // Red orange
            Rgb565::new(138 >> 3, 43 >> 2, 226 >> 3), // Blue violet
        ])
        .unwrap()
    }

    /// High contrast palette for accessibility
    pub fn high_contrast_palette() -> ColorPalette<Rgb565, 6> {
        ColorPalette::from_colors(&[
            Rgb565::BLACK,
            Rgb565::WHITE,
            Rgb565::new(255 >> 3, 0 >> 2, 0 >> 3),   // Pure red
            Rgb565::new(0 >> 3, 0 >> 2, 255 >> 3),   // Pure blue
            Rgb565::new(0 >> 3, 255 >> 2, 0 >> 3),   // Pure green
            Rgb565::new(255 >> 3, 255 >> 2, 0 >> 3), // Pure yellow
        ])
        .unwrap()
    }

    /// Monochrome palette using different shades of gray
    pub fn monochrome_palette() -> ColorPalette<Rgb565, 8> {
        ColorPalette::from_colors(&[
            Rgb565::BLACK,
            Rgb565::new(32 >> 3, 32 >> 2, 32 >> 3), // Very dark gray
            Rgb565::new(64 >> 3, 64 >> 2, 64 >> 3), // Dark gray
            Rgb565::new(96 >> 3, 96 >> 2, 96 >> 3), // Medium dark gray
            Rgb565::new(128 >> 3, 128 >> 2, 128 >> 3), // Gray
            Rgb565::new(160 >> 3, 160 >> 2, 160 >> 3), // Medium light gray
            Rgb565::new(192 >> 3, 192 >> 2, 192 >> 3), // Light gray
            Rgb565::WHITE,
        ])
        .unwrap()
    }

    /// Minimal palette with subtle, sophisticated colors
    pub fn minimal_palette() -> ColorPalette<Rgb565, 6> {
        ColorPalette::from_colors(&[
            Rgb565::new(55 >> 3, 65 >> 2, 81 >> 3),    // Slate gray
            Rgb565::new(107 >> 3, 114 >> 2, 128 >> 3), // Slate gray
            Rgb565::new(148 >> 3, 163 >> 2, 184 >> 3), // Light slate gray
            Rgb565::new(99 >> 3, 102 >> 2, 241 >> 3),  // Indigo
            Rgb565::new(16 >> 3, 185 >> 2, 129 >> 3),  // Emerald
            Rgb565::new(239 >> 3, 68 >> 2, 68 >> 3),   // Red
        ])
        .unwrap()
    }

    /// Retro palette with vintage-inspired colors
    pub fn retro_palette() -> ColorPalette<Rgb565, 8> {
        ColorPalette::from_colors(&[
            Rgb565::new(205 >> 3, 92 >> 2, 92 >> 3),   // Indian red
            Rgb565::new(218 >> 3, 165 >> 2, 32 >> 3),  // Goldenrod
            Rgb565::new(107 >> 3, 142 >> 2, 35 >> 3),  // Olive drab
            Rgb565::new(160 >> 3, 82 >> 2, 45 >> 3),   // Sienna
            Rgb565::new(188 >> 3, 143 >> 2, 143 >> 3), // Rosy brown
            Rgb565::new(222 >> 3, 184 >> 2, 135 >> 3), // Burlywood
            Rgb565::new(139 >> 3, 69 >> 2, 19 >> 3),   // Saddle brown
            Rgb565::new(205 >> 3, 133 >> 2, 63 >> 3),  // Peru
        ])
        .unwrap()
    }
}

/// Color interpolation utilities
pub trait ColorInterpolation<C: PixelColor> {
    /// Interpolate between two colors
    ///
    /// # Arguments
    /// * `from` - Starting color
    /// * `to` - Ending color
    /// * `t` - Interpolation factor (0.0 = from, 1.0 = to)
    fn interpolate(from: C, to: C, t: f32) -> C;

    /// Create a gradient between multiple colors
    ///
    /// # Arguments
    /// * `colors` - Array of colors to interpolate between
    /// * `steps` - Number of steps in the gradient
    fn gradient(colors: &[C], steps: usize) -> Vec<C, 256>;
}

/// RGB565 color interpolation implementation
#[cfg(feature = "color-support")]
impl ColorInterpolation<embedded_graphics::pixelcolor::Rgb565>
    for embedded_graphics::pixelcolor::Rgb565
{
    fn interpolate(from: Self, to: Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);

        // Extract RGB components
        let from_r = (from.into_storage() >> 11) & 0x1F;
        let from_g = (from.into_storage() >> 5) & 0x3F;
        let from_b = from.into_storage() & 0x1F;

        let to_r = (to.into_storage() >> 11) & 0x1F;
        let to_g = (to.into_storage() >> 5) & 0x3F;
        let to_b = to.into_storage() & 0x1F;

        // Interpolate each component
        let r = (from_r as f32 + (to_r as f32 - from_r as f32) * t) as u16;
        let g = (from_g as f32 + (to_g as f32 - from_g as f32) * t) as u16;
        let b = (from_b as f32 + (to_b as f32 - from_b as f32) * t) as u16;

        // Combine back into RGB565
        Self::new((r & 0x1F) as u8, (g & 0x3F) as u8, (b & 0x1F) as u8)
    }

    fn gradient(colors: &[Self], steps: usize) -> Vec<Self, 256> {
        let mut result = Vec::new();

        if colors.is_empty() || steps == 0 {
            return result;
        }

        if colors.len() == 1 {
            for _ in 0..steps.min(256) {
                let _ = result.push(colors[0]);
            }
            return result;
        }

        let segments = colors.len() - 1;
        let steps_per_segment = steps / segments;
        let remaining_steps = steps % segments;

        for segment in 0..segments {
            let segment_steps = steps_per_segment + if segment < remaining_steps { 1 } else { 0 };

            for step in 0..segment_steps {
                if result.len() >= 256 {
                    break;
                }

                let t = if segment_steps > 1 {
                    step as f32 / (segment_steps - 1) as f32
                } else {
                    0.0
                };

                let color = Self::interpolate(colors[segment], colors[segment + 1], t);
                let _ = result.push(color);
            }
        }

        result
    }
}

/// Color utility functions
pub struct ColorUtils;

impl ColorUtils {
    /// Convert a hex color string to RGB565 (basic implementation)
    #[cfg(feature = "color-support")]
    pub fn from_hex(hex: &str) -> Option<embedded_graphics::pixelcolor::Rgb565> {
        if hex.len() != 7 || !hex.starts_with('#') {
            return None;
        }

        let r = u8::from_str_radix(&hex[1..3], 16).ok()?;
        let g = u8::from_str_radix(&hex[3..5], 16).ok()?;
        let b = u8::from_str_radix(&hex[5..7], 16).ok()?;

        Some(embedded_graphics::pixelcolor::Rgb565::new(
            r >> 3,
            g >> 2,
            b >> 3,
        ))
    }

    /// Get a contrasting color (black or white) for the given color
    #[cfg(feature = "color-support")]
    pub fn contrasting_color(
        color: embedded_graphics::pixelcolor::Rgb565,
    ) -> embedded_graphics::pixelcolor::Rgb565 {
        // Calculate luminance using simplified formula
        let r = (color.into_storage() >> 11) & 0x1F;
        let g = (color.into_storage() >> 5) & 0x3F;
        let b = color.into_storage() & 0x1F;

        // Convert to 8-bit and calculate luminance
        let r8 = (r << 3) as f32;
        let g8 = (g << 2) as f32;
        let b8 = (b << 3) as f32;

        let luminance = 0.299 * r8 + 0.587 * g8 + 0.114 * b8;

        if luminance > 128.0 {
            embedded_graphics::pixelcolor::Rgb565::BLACK
        } else {
            embedded_graphics::pixelcolor::Rgb565::WHITE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "color-support")]
    use embedded_graphics::pixelcolor::Rgb565;

    #[test]
    fn test_color_palette_creation() {
        use embedded_graphics::pixelcolor::BinaryColor;
        let mut palette: ColorPalette<BinaryColor, 5> = ColorPalette::new();
        assert!(palette.is_empty());

        palette.add_color(BinaryColor::On).unwrap();
        palette.add_color(BinaryColor::Off).unwrap();

        assert_eq!(palette.len(), 2);
        assert!(!palette.is_empty());
    }

    #[test]
    fn test_color_palette_cycling() {
        use embedded_graphics::pixelcolor::BinaryColor;
        let mut palette: ColorPalette<BinaryColor, 3> = ColorPalette::new();
        palette.add_color(BinaryColor::On).unwrap();
        palette.add_color(BinaryColor::Off).unwrap();
        palette.add_color(BinaryColor::On).unwrap();

        assert_eq!(palette.next_color(), Some(BinaryColor::On));
        assert_eq!(palette.next_color(), Some(BinaryColor::Off));
        assert_eq!(palette.next_color(), Some(BinaryColor::On));
        assert_eq!(palette.next_color(), Some(BinaryColor::On)); // Should cycle back
    }

    #[cfg(feature = "color-support")]
    #[test]
    fn test_rgb565_interpolation() {
        let from = Rgb565::BLACK;
        let to = Rgb565::WHITE;

        let mid = Rgb565::interpolate(from, to, 0.5);
        // The exact value depends on the RGB565 representation
        assert_ne!(mid, from);
        assert_ne!(mid, to);

        let same_as_from = Rgb565::interpolate(from, to, 0.0);
        assert_eq!(same_as_from, from);

        let same_as_to = Rgb565::interpolate(from, to, 1.0);
        assert_eq!(same_as_to, to);
    }

    #[cfg(feature = "color-support")]
    #[test]
    fn test_default_palette() {
        let palette = rgb565_palettes::default_palette();
        assert_eq!(palette.len(), 8);
        // Test that it contains the expected modern blue and red colors
        assert_eq!(
            palette.get_color(0),
            Some(Rgb565::new(59 >> 3, 130 >> 2, 246 >> 3))
        ); // Modern blue
        assert_eq!(
            palette.get_color(1),
            Some(Rgb565::new(239 >> 3, 68 >> 2, 68 >> 3))
        ); // Modern red
    }

    #[cfg(feature = "color-support")]
    #[test]
    fn test_contrasting_color() {
        let black_contrast = ColorUtils::contrasting_color(Rgb565::BLACK);
        assert_eq!(black_contrast, Rgb565::WHITE);

        let white_contrast = ColorUtils::contrasting_color(Rgb565::WHITE);
        assert_eq!(white_contrast, Rgb565::BLACK);
    }
}
