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
//! // Professional color palette (8 colors)
//! let colors = quick::professional_colors();
//!
//! // Nature-inspired palette
//! let nature_colors = quick::nature_colors();
//!
//! // Ocean-themed palette
//! let ocean_colors = quick::ocean_colors();
//!
//! // Custom palette
//! let mut custom_palette = ColorPalette::new();
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
//! // Color interpolation for gradients
//! let start_color = Rgb565::BLUE;
//! let end_color = Rgb565::RED;
//! let interpolated = ColorUtils::interpolate(start_color, end_color, 0.5);
//!
//! // Color brightness adjustment
//! let darker = ColorUtils::darken(Rgb565::BLUE, 0.3);
//! let lighter = ColorUtils::lighten(Rgb565::BLUE, 0.3);
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
//! // Apply theme to chart
//! chart.apply_theme(&dark_theme);
//! ```
//!
//! ### Custom Themes
//! Create custom themes for specific requirements:
//! ```rust
//! use embedded_charts::prelude::*;
//! use embedded_graphics::pixelcolor::Rgb565;
//!
//! let custom_theme = Theme::builder()
//!     .background_color(Rgb565::BLACK)
//!     .text_color(Rgb565::WHITE)
//!     .grid_color(Rgb565::new(8, 8, 8))
//!     .primary_color(Rgb565::CYAN)
//!     .secondary_color(Rgb565::MAGENTA)
//!     .build();
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
//!     pattern: LinePattern::Solid,
//!     cap: LineCap::Round,
//!     join: LineJoin::Round,
//!     width: 2,
//! };
//!
//! // Dashed line style
//! let dashed_line = LineStyle {
//!     pattern: LinePattern::Dashed(5, 3), // 5px dash, 3px gap
//!     cap: LineCap::Square,
//!     join: LineJoin::Miter,
//!     width: 1,
//! };
//!
//! // Dotted line style
//! let dotted_line = LineStyle {
//!     pattern: LinePattern::Dotted(2), // 2px spacing
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
//!     pattern: FillPattern::Solid(Rgb565::BLUE),
//!     opacity: 255, // Fully opaque
//! };
//!
//! // Semi-transparent fill
//! let transparent_fill = FillStyle {
//!     pattern: FillPattern::Solid(Rgb565::BLUE),
//!     opacity: 128, // 50% transparent
//! };
//!
//! // Gradient fill (if supported)
//! let gradient_fill = FillStyle {
//!     pattern: FillPattern::Gradient {
//!         start_color: Rgb565::BLUE,
//!         end_color: Rgb565::WHITE,
//!         direction: GradientDirection::Vertical,
//!     },
//!     opacity: 255,
//! };
//! ```
//!
//! ## Border Styles
//!
//! Customizable border styling:
//! ```rust
//! use embedded_charts::prelude::*;
//! use embedded_graphics::pixelcolor::Rgb565;
//!
//! let border_style = BorderStyle {
//!     color: Rgb565::BLACK,
//!     width: 2,
//!     pattern: LinePattern::Solid,
//!     radius: 5, // Rounded corners
//! };
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
//! // Get a color from palette
//! let palette = quick::professional_colors();
//! let primary_color = palette.get_color(0)?; // First color
//! let secondary_color = palette.get_color(1)?; // Second color
//!
//! // Cycle through colors for multi-series
//! for (i, series) in multi_series.iter().enumerate() {
//!     let color = palette.get_color(i % palette.len())?;
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
