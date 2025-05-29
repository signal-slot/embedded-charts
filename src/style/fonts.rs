//! Font support for text rendering in charts.

use embedded_graphics::prelude::*;

/// Font configuration for text rendering
#[derive(Debug, Clone)]
pub struct FontConfig<C: PixelColor> {
    /// Text color
    pub color: C,
    /// Font size
    pub size: u32,
}

impl<C: PixelColor> FontConfig<C> {
    /// Create a new font configuration
    pub fn new(color: C, size: u32) -> Self {
        Self { color, size }
    }
}

impl<C: PixelColor> Default for FontConfig<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    fn default() -> Self {
        Self {
            color: embedded_graphics::pixelcolor::Rgb565::BLACK.into(),
            size: 12,
        }
    }
}
