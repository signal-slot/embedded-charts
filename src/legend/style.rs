//! Legend styling configuration.

use embedded_graphics::prelude::*;

/// Complete legend styling configuration
#[derive(Debug, Clone)]
pub struct LegendStyle<C: PixelColor> {
    /// Text styling
    pub text: TextStyle<C>,
    /// Symbol styling
    pub symbol: SymbolStyle<C>,
    /// Background styling
    pub background: BackgroundStyle<C>,
    /// Spacing configuration
    pub spacing: SpacingStyle,
}

/// Text styling for legend labels
#[derive(Debug, Clone)]
pub struct TextStyle<C: PixelColor> {
    /// Text color
    pub color: C,
    /// Font size (in pixels)
    pub font_size: u32,
    /// Line height (in pixels)
    pub line_height: u32,
    /// Character width (for monospace estimation)
    pub char_width: u32,
    /// Maximum text width (for layout calculations)
    pub max_text_width: u32,
    /// Text alignment
    pub alignment: TextAlignment,
}

/// Symbol styling for legend entries
#[derive(Debug, Clone)]
pub struct SymbolStyle<C: PixelColor> {
    /// Default symbol size
    pub size: u32,
    /// Symbol border width
    pub border_width: u32,
    /// Default symbol color (used as fallback)
    pub default_color: C,
    /// Symbol rendering quality
    pub quality: SymbolQuality,
}

/// Background styling for the legend
#[derive(Debug, Clone)]
pub struct BackgroundStyle<C: PixelColor> {
    /// Background color (None for transparent)
    pub color: Option<C>,
    /// Border color (None for no border)
    pub border_color: Option<C>,
    /// Border width
    pub border_width: u32,
    /// Corner radius for rounded backgrounds
    pub corner_radius: u32,
    /// Background opacity (0-255, 255 = opaque)
    pub opacity: u8,
}

/// Spacing configuration for legend layout
#[derive(Debug, Clone)]
pub struct SpacingStyle {
    /// Width allocated for symbols
    pub symbol_width: u32,
    /// Gap between symbol and text
    pub symbol_text_gap: u32,
    /// Spacing between legend entries
    pub entry_spacing: u32,
    /// Padding around the entire legend
    pub padding: Padding,
    /// Margins outside the legend
    pub margins: Margins,
}

/// Text alignment options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAlignment {
    /// Left-aligned text
    Left,
    /// Center-aligned text
    Center,
    /// Right-aligned text
    Right,
}

/// Symbol rendering quality options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolQuality {
    /// Fast rendering with basic shapes
    Fast,
    /// Standard quality with anti-aliasing where possible
    Standard,
    /// High quality with smooth curves (may be slower)
    High,
}

/// Padding configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Padding {
    /// Top padding
    pub top: u32,
    /// Right padding
    pub right: u32,
    /// Bottom padding
    pub bottom: u32,
    /// Left padding
    pub left: u32,
}

/// Margins configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Margins {
    /// Top margin
    pub top: u32,
    /// Right margin
    pub right: u32,
    /// Bottom margin
    pub bottom: u32,
    /// Left margin
    pub left: u32,
}

impl<C: PixelColor> LegendStyle<C> {
    /// Create a new legend style with default values
    pub fn new() -> Self
    where
        C: From<embedded_graphics::pixelcolor::Rgb565>,
    {
        Self {
            text: TextStyle::new(),
            symbol: SymbolStyle::new(),
            background: BackgroundStyle::new(),
            spacing: SpacingStyle::new(),
        }
    }

    /// Create a minimal style for small displays
    pub fn minimal() -> Self
    where
        C: From<embedded_graphics::pixelcolor::Rgb565>,
    {
        Self {
            text: TextStyle::minimal(),
            symbol: SymbolStyle::minimal(),
            background: BackgroundStyle::minimal(),
            spacing: SpacingStyle::minimal(),
        }
    }

    /// Create a professional style
    pub fn professional() -> Self
    where
        C: From<embedded_graphics::pixelcolor::Rgb565>,
    {
        Self {
            text: TextStyle::professional(),
            symbol: SymbolStyle::professional(),
            background: BackgroundStyle::professional(),
            spacing: SpacingStyle::professional(),
        }
    }

    /// Create a compact style for space-constrained environments
    pub fn compact() -> Self
    where
        C: From<embedded_graphics::pixelcolor::Rgb565>,
    {
        Self {
            text: TextStyle::compact(),
            symbol: SymbolStyle::compact(),
            background: BackgroundStyle::compact(),
            spacing: SpacingStyle::compact(),
        }
    }
}

impl<C: PixelColor> TextStyle<C> {
    /// Create a new text style with default values
    pub fn new() -> Self
    where
        C: From<embedded_graphics::pixelcolor::Rgb565>,
    {
        Self {
            color: C::from(embedded_graphics::pixelcolor::Rgb565::BLACK),
            font_size: 12,
            line_height: 16,
            char_width: 6,
            max_text_width: 120,
            alignment: TextAlignment::Left,
        }
    }

    /// Create a minimal text style
    pub fn minimal() -> Self
    where
        C: From<embedded_graphics::pixelcolor::Rgb565>,
    {
        Self {
            color: C::from(embedded_graphics::pixelcolor::Rgb565::BLACK),
            font_size: 8,
            line_height: 10,
            char_width: 4,
            max_text_width: 60,
            alignment: TextAlignment::Left,
        }
    }

    /// Create a professional text style
    pub fn professional() -> Self
    where
        C: From<embedded_graphics::pixelcolor::Rgb565>,
    {
        Self {
            color: C::from(embedded_graphics::pixelcolor::Rgb565::new(8, 16, 8)), // Dark gray
            font_size: 14,
            line_height: 18,
            char_width: 7,
            max_text_width: 150,
            alignment: TextAlignment::Left,
        }
    }

    /// Create a compact text style
    pub fn compact() -> Self
    where
        C: From<embedded_graphics::pixelcolor::Rgb565>,
    {
        Self {
            color: C::from(embedded_graphics::pixelcolor::Rgb565::BLACK),
            font_size: 10,
            line_height: 12,
            char_width: 5,
            max_text_width: 80,
            alignment: TextAlignment::Left,
        }
    }
}

impl<C: PixelColor> SymbolStyle<C> {
    /// Create a new symbol style with default values
    pub fn new() -> Self
    where
        C: From<embedded_graphics::pixelcolor::Rgb565>,
    {
        Self {
            size: 16,
            border_width: 1,
            default_color: C::from(embedded_graphics::pixelcolor::Rgb565::BLUE),
            quality: SymbolQuality::Standard,
        }
    }

    /// Create a minimal symbol style
    pub fn minimal() -> Self
    where
        C: From<embedded_graphics::pixelcolor::Rgb565>,
    {
        Self {
            size: 8,
            border_width: 0,
            default_color: C::from(embedded_graphics::pixelcolor::Rgb565::BLUE),
            quality: SymbolQuality::Fast,
        }
    }

    /// Create a professional symbol style
    pub fn professional() -> Self
    where
        C: From<embedded_graphics::pixelcolor::Rgb565>,
    {
        Self {
            size: 20,
            border_width: 2,
            default_color: C::from(embedded_graphics::pixelcolor::Rgb565::new(14, 28, 14)), // Steel blue
            quality: SymbolQuality::High,
        }
    }

    /// Create a compact symbol style
    pub fn compact() -> Self
    where
        C: From<embedded_graphics::pixelcolor::Rgb565>,
    {
        Self {
            size: 12,
            border_width: 1,
            default_color: C::from(embedded_graphics::pixelcolor::Rgb565::BLUE),
            quality: SymbolQuality::Standard,
        }
    }
}

impl<C: PixelColor> BackgroundStyle<C> {
    /// Create a new background style with default values
    pub fn new() -> Self {
        Self {
            color: None, // Transparent by default
            border_color: None,
            border_width: 0,
            corner_radius: 0,
            opacity: 255,
        }
    }

    /// Create a minimal background style
    pub fn minimal() -> Self {
        Self {
            color: None,
            border_color: None,
            border_width: 0,
            corner_radius: 0,
            opacity: 255,
        }
    }

    /// Create a professional background style
    pub fn professional() -> Self
    where
        C: From<embedded_graphics::pixelcolor::Rgb565>,
    {
        Self {
            color: Some(C::from(embedded_graphics::pixelcolor::Rgb565::new(
                31, 63, 31,
            ))), // Light gray
            border_color: Some(C::from(embedded_graphics::pixelcolor::Rgb565::new(
                16, 32, 16,
            ))), // Gray
            border_width: 1,
            corner_radius: 4,
            opacity: 240,
        }
    }

    /// Create a compact background style
    pub fn compact() -> Self {
        Self {
            color: None,
            border_color: None,
            border_width: 0,
            corner_radius: 0,
            opacity: 255,
        }
    }
}

impl SpacingStyle {
    /// Create a new spacing style with default values
    pub fn new() -> Self {
        Self {
            symbol_width: 20,
            symbol_text_gap: 8,
            entry_spacing: 4,
            padding: Padding::all(8),
            margins: Margins::all(4),
        }
    }

    /// Create a minimal spacing style
    pub fn minimal() -> Self {
        Self {
            symbol_width: 12,
            symbol_text_gap: 4,
            entry_spacing: 2,
            padding: Padding::all(2),
            margins: Margins::all(1),
        }
    }

    /// Create a professional spacing style
    pub fn professional() -> Self {
        Self {
            symbol_width: 24,
            symbol_text_gap: 12,
            entry_spacing: 8,
            padding: Padding::all(12),
            margins: Margins::all(8),
        }
    }

    /// Create a compact spacing style
    pub fn compact() -> Self {
        Self {
            symbol_width: 16,
            symbol_text_gap: 6,
            entry_spacing: 3,
            padding: Padding::all(4),
            margins: Margins::all(2),
        }
    }
}

impl Padding {
    /// Create uniform padding
    pub const fn all(value: u32) -> Self {
        Self {
            top: value,
            right: value,
            bottom: value,
            left: value,
        }
    }

    /// Create symmetric padding
    pub const fn symmetric(horizontal: u32, vertical: u32) -> Self {
        Self {
            top: vertical,
            right: horizontal,
            bottom: vertical,
            left: horizontal,
        }
    }

    /// Create custom padding
    pub const fn new(top: u32, right: u32, bottom: u32, left: u32) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }

    /// Get total horizontal padding
    pub const fn horizontal(&self) -> u32 {
        self.left + self.right
    }

    /// Get total vertical padding
    pub const fn vertical(&self) -> u32 {
        self.top + self.bottom
    }
}

impl Margins {
    /// Create uniform margins
    pub const fn all(value: u32) -> Self {
        Self {
            top: value,
            right: value,
            bottom: value,
            left: value,
        }
    }

    /// Create symmetric margins
    pub const fn symmetric(horizontal: u32, vertical: u32) -> Self {
        Self {
            top: vertical,
            right: horizontal,
            bottom: vertical,
            left: horizontal,
        }
    }

    /// Create custom margins
    pub const fn new(top: u32, right: u32, bottom: u32, left: u32) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }

    /// Get total horizontal margins
    pub const fn horizontal(&self) -> u32 {
        self.left + self.right
    }

    /// Get total vertical margins
    pub const fn vertical(&self) -> u32 {
        self.top + self.bottom
    }
}

// Default implementations
impl<C: PixelColor> Default for LegendStyle<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<C: PixelColor> Default for TextStyle<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<C: PixelColor> Default for SymbolStyle<C>
where
    C: From<embedded_graphics::pixelcolor::Rgb565>,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<C: PixelColor> Default for BackgroundStyle<C> {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for SpacingStyle {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for TextAlignment {
    fn default() -> Self {
        Self::Left
    }
}

impl Default for SymbolQuality {
    fn default() -> Self {
        Self::Standard
    }
}

impl Default for Padding {
    fn default() -> Self {
        Self::all(4)
    }
}

impl Default for Margins {
    fn default() -> Self {
        Self::all(2)
    }
}
