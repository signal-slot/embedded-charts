//! Styling system for charts.
//!
//! This module provides comprehensive styling capabilities for charts including
//! color palettes, themes, line styles, and typography. All styling is designed
//! to work efficiently on embedded systems with limited resources.
//!
//! ## Color System
//!
//! ### Color Palettes
//! Pre-defined color palettes optimized for different display types:
//! ```rust
//! use embedded_charts::prelude::*;
//!
//! // Professional color palette (when color-support feature is enabled)
//! #[cfg(feature = "color-support")]
//! let colors = quick::professional_colors();
//!
//! // Nature-inspired palette
//! #[cfg(feature = "color-support")]
//! let nature_colors = quick::nature_colors();
//!
//! // Ocean-themed palette
//! #[cfg(feature = "color-support")]
//! let ocean_colors = quick::ocean_colors();
//!
//! // Custom palette
//! let mut custom_palette: ColorPalette<Rgb565, 8> = ColorPalette::new();
//! custom_palette.add_color(Rgb565::BLUE)?;
//! custom_palette.add_color(Rgb565::RED)?;
//! custom_palette.add_color(Rgb565::GREEN)?;
//! # Ok::<(), embedded_charts::error::ChartError>(())
//! ```
//!
//! ### Color Utilities
//! Advanced color manipulation and interpolation:
//! ```rust
//! use embedded_charts::prelude::*;
//! use embedded_graphics::pixelcolor::Rgb565;
//!
//! // Color interpolation for gradients (if color-support feature enabled)
//! let start_color = Rgb565::BLUE;
//! let end_color = Rgb565::RED;
//! #[cfg(feature = "color-support")]
//! let interpolated = Rgb565::interpolate(start_color, end_color, 0.5);
//!
//! // Color contrasting
//! #[cfg(feature = "color-support")]
//! let contrasting = ColorUtils::contrasting_color(Rgb565::BLUE);
//! ```
//!
//! ## Theme System
//!
//! Complete styling themes for different use cases:
//! ```rust
//! use embedded_charts::prelude::*;
//!
//! // Light theme for bright displays
//! let light_theme = quick::light_theme();
//!
//! // Dark theme for OLED displays
//! let dark_theme = quick::dark_theme();
//!
//! // Vibrant theme for colorful displays
//! let vibrant_theme = quick::vibrant_theme();
//!
//! // Cyberpunk theme for modern aesthetics
//! let cyberpunk_theme = quick::cyberpunk_theme();
//!
//! // Theme provides colors for chart styling
//! let bg_color = dark_theme.background;
//! let text_color = dark_theme.text;
//! ```
//!
//! ### Custom Themes
//! Create custom themes for specific requirements:
//! ```rust
//! use embedded_charts::prelude::*;
//! use embedded_graphics::pixelcolor::Rgb565;
//!
//! // Use predefined themes or access theme colors
//! let custom_bg = Rgb565::BLACK;
//! let custom_text = Rgb565::WHITE;
//! let custom_grid = Rgb565::new(8, 8, 8);
//! let custom_primary = Rgb565::CYAN;
//! let custom_secondary = Rgb565::MAGENTA;
//! ```
//!
//! ## Line Styling
//!
//! Comprehensive line styling options:
//! ```rust
//! use embedded_charts::prelude::*;
//! use embedded_graphics::pixelcolor::Rgb565;
//!
//! // Solid line style
//! let solid_line = LineStyle {
//!     color: Rgb565::BLUE,
//!     pattern: LinePattern::Solid,
//!     cap: LineCap::Round,
//!     join: LineJoin::Round,
//!     width: 2,
//! };
//!
//! // Dashed line style
//! let dashed_line = LineStyle {
//!     color: Rgb565::RED,
//!     pattern: LinePattern::Dashed,
//!     cap: LineCap::Square,
//!     join: LineJoin::Miter,
//!     width: 1,
//! };
//!
//! // Dotted line style
//! let dotted_line = LineStyle {
//!     color: Rgb565::GREEN,
//!     pattern: LinePattern::Dotted,
//!     cap: LineCap::Round,
//!     join: LineJoin::Round,
//!     width: 1,
//! };
//! ```
//!
//! ### Line Patterns
//! Available line patterns:
//! - `LinePattern::Solid` - Continuous line
//! - `LinePattern::Dashed(dash_length, gap_length)` - Dashed line
//! - `LinePattern::Dotted(dot_spacing)` - Dotted line
//!
//! ### Line Caps and Joins
//! Line ending and connection styles:
//! - `LineCap::Butt` - Square end, flush with line
//! - `LineCap::Round` - Rounded end
//! - `LineCap::Square` - Square end, extended beyond line
//! - `LineJoin::Miter` - Sharp corner
//! - `LineJoin::Round` - Rounded corner
//! - `LineJoin::Bevel` - Beveled corner
//!
//! ## Fill Styles
//!
//! Area filling options for charts:
//! ```rust
//! use embedded_charts::prelude::*;
//! use embedded_graphics::pixelcolor::Rgb565;
//!
//! // Solid fill
//! let solid_fill = FillStyle {
//!     color: Rgb565::BLUE,
//!     pattern: FillPattern::Solid,
//! };
//!
//! // Different color fill
//! let red_fill = FillStyle {
//!     color: Rgb565::RED,
//!     pattern: FillPattern::Solid,
//! };
//!
//! // Using convenience method
//! let green_fill = FillStyle::solid(Rgb565::GREEN);
//! ```
//!
//! ## Border Styles
//!
//! Customizable border styling:
//! ```rust
//! use embedded_charts::prelude::*;
//! use embedded_graphics::pixelcolor::Rgb565;
//!
//! let line_style = LineStyle::solid(Rgb565::BLACK).width(2);
//! let border_style = BorderStyle::rounded(line_style, 5);
//! ```
//!
//! ## Typography (feature: "fonts")
//!
//! Text styling and font management:
//! ```rust,no_run
//! # #[cfg(feature = "fonts")]
//! # {
//! use embedded_charts::prelude::*;
//! use embedded_graphics::pixelcolor::Rgb565;
//!
//! let text_style = TextStyle {
//!     font: Font::Medium,
//!     color: Rgb565::BLACK,
//!     size: 12,
//!     alignment: TextAlignment::Center,
//! };
//!
//! // Apply to chart title
//! let config = chart_config! {
//!     title: "My Chart",
//!     title_style: text_style,
//! };
//! # }
//! ```
//!
//! ## Color Palettes
//!
//! ### Built-in Palettes
//! - `professional_palette()` - Professional business colors
//! - `pastel_palette()` - Soft, muted colors
//! - `vibrant_palette()` - Bright, energetic colors
//! - `nature_palette()` - Earth and nature tones
//! - `ocean_palette()` - Blue and aqua tones
//! - `sunset_palette()` - Warm orange and red tones
//! - `cyberpunk_palette()` - Neon and electric colors
//! - `minimal_palette()` - Simple black, white, and gray
//! - `retro_palette()` - Vintage-inspired colors
//!
//! ### Palette Usage
//! ```rust
//! use embedded_charts::prelude::*;
//!
//! // Example with a simple palette
//! let mut palette: ColorPalette<Rgb565, 8> = ColorPalette::new();
//! palette.add_color(Rgb565::BLUE)?;
//! palette.add_color(Rgb565::RED)?;
//! palette.add_color(Rgb565::GREEN)?;
//!
//! let primary_color = palette.get_color(0).unwrap_or(Rgb565::WHITE);
//! let secondary_color = palette.get_color(1).unwrap_or(Rgb565::WHITE);
//!
//! // Cycle through colors
//! for i in 0..3 {
//!     let color = palette.get_color(i % palette.len()).unwrap_or(Rgb565::WHITE);
//!     // Apply color to series...
//! }
//! # Ok::<(), embedded_charts::error::ChartError>(())
//! ```
//!
//! ## Performance Considerations
//!
//! - **Memory Efficient**: All styling uses minimal memory footprint
//! - **Compile-time Optimization**: Many style calculations are done at compile time
//! - **Feature Gating**: Advanced styling features can be disabled to save space
//! - **Display Optimization**: Styles are optimized for different display types
//!
//! ## Display-Specific Optimizations
//!
//! ### OLED Displays
//! - Use dark themes to save power
//! - Avoid large filled areas
//! - Use high contrast colors
//!
//! ### E-Paper Displays
//! - Use monochrome or limited color palettes
//! - Optimize for slow refresh rates
//! - Use high contrast patterns
//!
//! ### TFT Displays
//! - Take advantage of full color range
//! - Use vibrant themes for better visibility
//! - Optimize for fast refresh rates

pub mod colors;
pub mod fonts;
pub mod line;
pub mod themes;

pub use colors::*;
pub use fonts::*;
pub use line::*;
pub use themes::*;
