//! # Embedded Charts
//!
//! A production-ready, no_std graph framework for embedded systems using embedded-graphics.
//!
//! This library provides comprehensive chart types (line, bar, pie, scatter, gauge), axes, grids,
//! legends, real-time data streaming capabilities, and customizable styling while maintaining
//! memory efficiency and performance suitable for resource-constrained environments.
//!
//! ## Features
//!
//! - **Memory Efficient**: Static allocation with compile-time bounds, no heap usage
//! - **Performance Optimized**: Designed for real-time rendering on embedded systems
//! - **Flexible**: Plugin-like architecture for custom chart types
//! - **Easy to Use**: Fluent builder API with sensible defaults
//! - **std/no_std Compatible**: Full compatibility with both desktop and embedded environments
//! - **Rich Chart Types**: Line, bar, pie, gauge, and scatter charts with professional styling
//! - **Real-time Animation**: Streaming data with smooth transitions and configurable easing
//! - **Professional Styling**: Built-in themes, color palettes, and typography support
//!
//! ## Chart Types
//!
//! ### Line Charts
//! Multi-series line charts with markers, area filling, and smooth curves:
//! ```rust,no_run
//! # #[cfg(feature = "line")]
//! # {
//! # fn test() -> Result<(), embedded_charts::error::ChartError> {
//! use embedded_charts::prelude::*;
//! use embedded_graphics::pixelcolor::Rgb565;
//!
//! let chart = LineChart::builder()
//!     .line_color(Rgb565::BLUE)
//!     .line_width(2)
//!     .with_markers(MarkerStyle {
//!         shape: MarkerShape::Circle,
//!         size: 6,
//!         color: Rgb565::RED,
//!         visible: true,
//!     })
//!     .build()?;
//! # Ok(())
//! # }
//! # }
//! ```
//!
//! ### Bar Charts
//! Vertical and horizontal bar charts with stacking support:
//! ```rust,no_run
//! # #[cfg(feature = "bar")]
//! # fn test() -> Result<(), embedded_charts::error::ChartError> {
//! use embedded_charts::prelude::*;
//! use embedded_graphics::pixelcolor::Rgb565;
//!
//! let chart = BarChart::builder()
//!     .orientation(BarOrientation::Vertical)
//!     .bar_width(BarWidth::Fixed(20))
//!     .colors(&[Rgb565::GREEN])
//!     .build()?;
//! Ok(())
//! # }
//! ```
//!
//! ### Pie Charts
//! Full circle and donut charts with custom styling:
//! ```rust,no_run
//! # #[cfg(feature = "pie")]
//! # fn test() -> Result<(), embedded_charts::error::ChartError> {
//! use embedded_charts::prelude::*;
//! use embedded_graphics::pixelcolor::Rgb565;
//!
//! let chart = PieChart::builder()
//!     .donut(30) // Donut chart with inner radius of 30
//!     .colors(&[Rgb565::BLUE, Rgb565::RED, Rgb565::GREEN])
//!     .build()?;
//! Ok(())
//! # }
//! ```
//!
//! ### Gauge Charts
//! Semicircle gauges with threshold zones and custom indicators:
//! ```rust,no_run
//! # #[cfg(feature = "gauge")]
//! # fn test() -> Result<(), embedded_charts::error::ChartError> {
//! use embedded_charts::prelude::*;
//! use embedded_graphics::pixelcolor::Rgb565;
//!
//! let chart = GaugeChart::builder()
//!     .gauge_type(GaugeType::Semicircle)
//!     .value_range(0.0, 100.0)
//!     .add_threshold_zone(70.0, 100.0, Rgb565::RED)
//!     .build()?;
//! Ok(())
//! # }
//! ```
//!
//! ## Data Management
//!
//! ### Static Data Series
//! Fixed-capacity data storage for predictable memory usage:
//! ```rust
//! use embedded_charts::prelude::*;
//!
//! // Create a series with capacity for 256 points
//! let mut series: StaticDataSeries<Point2D, 256> = StaticDataSeries::new();
//! series.push(Point2D::new(0.0, 10.0))?;
//! series.push(Point2D::new(1.0, 20.0))?;
//!
//! // Create from tuples using the macro
//! let data = data_points![(0.0, 10.0), (1.0, 20.0), (2.0, 15.0)];
//! # Ok::<(), embedded_charts::error::DataError>(())
//! ```
//!
//! ### Multi-Series Data
//! Container for multiple data series with automatic color assignment:
//! ```rust
//! use embedded_charts::prelude::*;
//!
//! // Container for 8 series, 256 points each
//! let mut multi_series: MultiSeries<Point2D, 8, 256> = MultiSeries::new();
//!
//! let temp_data = data_points![(0.0, 22.5), (1.0, 23.1), (2.0, 24.2)];
//! let humidity_data = data_points![(0.0, 65.0), (1.0, 68.0), (2.0, 72.0)];
//!
//! multi_series.add_series(temp_data)?;
//! multi_series.add_series(humidity_data)?;
//! # Ok::<(), embedded_charts::error::DataError>(())
//! ```
//!
//! ## Professional Styling
//!
//! ### Themes and Color Palettes
//! Built-in themes optimized for different display types:
//! ```rust,no_run
//! # #[cfg(feature = "color-support")]
//! # {
//! use embedded_charts::prelude::*;
//!
//! // Professional color palettes
//! let colors = quick::professional_colors();
//! let nature_colors = quick::nature_colors();
//! let ocean_colors = quick::ocean_colors();
//!
//! // Complete themes
//! let light_theme = quick::light_theme();
//! let dark_theme = quick::dark_theme();
//! let cyberpunk_theme = quick::cyberpunk_theme();
//! # }
//! ```
//!
//! ### Chart Configuration
//! Fluent configuration with the `chart_config!` macro:
//! ```rust
//! use embedded_charts::prelude::*;
//! use embedded_graphics::pixelcolor::Rgb565;
//!
//! let config = chart_config! {
//!     title: "Temperature Monitor",
//!     background: Rgb565::WHITE,
//!     margins: constants::DEFAULT_MARGINS,
//!     grid: true,
//! };
//! ```
//!
//! ## Real-time Animation
//!
//! ### Streaming Data (requires "animations" feature)
//! ```rust,no_run
//! # #[cfg(feature = "animations")]
//! # {
//! use embedded_charts::prelude::*;
//! use embedded_graphics::{prelude::*, pixelcolor::Rgb565};
//!
//! // Sliding window for real-time data
//! let mut streaming_data: SlidingWindowSeries<Point2D, 100> =
//!     SlidingWindowSeries::new();
//!
//! // Add data points (automatically removes old ones)
//! let timestamp = 1.0;
//! let value = 25.0;
//! streaming_data.push(Point2D::new(timestamp, value));
//!
//! // Create a chart for rendering
//! let chart: LineChart<Rgb565> = LineChart::builder().build()?;
//! let config: ChartConfig<Rgb565> = ChartConfig::default();
//! let viewport = Rectangle::new(Point::zero(), Size::new(320, 240));
//! // chart.draw(&streaming_data, &config, viewport, &mut display)?;
//! # }
//! # Ok::<(), embedded_charts::error::ChartError>(())
//! ```
//!
//! ## System Optimization
//!
//! ### Feature Configuration
//! Choose the appropriate feature set for your target system's capabilities:
//!
//! ```toml
//! # Minimal configuration - Integer math only
//! [dependencies]
//! embedded-charts = {
//!     version = "0.1.0",
//!     default-features = false,
//!     features = ["integer-math"]
//! }
//!
//! # Balanced configuration - Fixed-point math with color support
//! [dependencies]
//! embedded-charts = {
//!     version = "0.1.0",
//!     default-features = false,
//!     features = ["fixed-point", "color-support"]
//! }
//!
//! # Full-featured configuration - All features enabled
//! [dependencies]
//! embedded-charts = {
//!     version = "0.1.0",
//!     default-features = false,
//!     features = ["floating-point", "animations", "color-support"]
//! }
//! ```
//!
//! ### no_std Usage
//! Complete example for embedded systems:
//! ```rust,ignore
//! #![no_std]
//!
//! use embedded_charts::prelude::*;
//! use embedded_graphics::{pixelcolor::Rgb565, prelude::*};
//!
//! fn render_sensor_chart() -> Result<(), embedded_charts::error::ChartError> {
//!     // Create data series with static allocation
//!     let mut sensor_data: StaticDataSeries<Point2D, 64> = StaticDataSeries::new();
//!     let _ = sensor_data.push(Point2D::new(0.0, 22.5));
//!     let _ = sensor_data.push(Point2D::new(1.0, 23.1));
//!
//!     // Create minimal chart for small displays
//!     let chart = LineChart::builder()
//!         .line_color(Rgb565::BLUE)
//!         .build()?;
//!
//!     // Render to embedded display
//!     let viewport = Rectangle::new(Point::zero(), Size::new(128, 64));
//!     // chart.draw(&sensor_data, chart.config(), viewport, &mut display)?;
//!     Ok(())
//! }
//!
//! fn main() {
//!     let _ = render_sensor_chart();
//! }
//! ```
//!
//! ## Complete Example
//!
//! Professional multi-series chart:
//! ```rust,ignore
//! use embedded_charts::prelude::*;
//! use embedded_graphics::{pixelcolor::Rgb565, prelude::*};
//!
//! // Create sample data
//! let temp_data = data_points![(0.0, 22.5), (1.0, 23.1), (2.0, 24.2), (3.0, 23.8)];
//! let humidity_data = data_points![(0.0, 65.0), (1.0, 68.0), (2.0, 72.0), (3.0, 70.0)];
//!
//! // Create multi-series container
//! let mut multi_series: MultiSeries<Point2D, 8, 256> = MultiSeries::new();
//! multi_series.add_series(temp_data)?;
//! multi_series.add_series(humidity_data)?;
//!
//! // Create a simple line chart
//! let chart = LineChart::builder()
//!     .line_color(Rgb565::BLUE)
//!     .build()?;
//!
//! // Configure the chart
//! let config: ChartConfig<Rgb565> = ChartConfig::default();
//!
//! // Render to display
//! let viewport = Rectangle::new(Point::zero(), Size::new(320, 240));
//! // chart.draw(&multi_series, &config, viewport, &mut display)?;
//! # Ok::<(), embedded_charts::error::ChartError>(())
//! ```
//!
//! ## Module Organization
//!
//! - [`chart`] - Chart implementations (line, bar, pie, gauge, scatter)
//! - [`data`] - Data series and point management
//! - [`fluent`] - Fluent API for easy chart creation
//! - [`style`] - Styling, themes, and color palettes
//! - [`axes`] - Axis configuration and rendering
//! - [`grid`] - Grid system for chart backgrounds
//! - [`legend`] - Legend positioning and styling
//! - [`animation`] - Real-time animations and transitions (feature-gated)
//! - [`render`] - Low-level rendering primitives
//! - [`layout`] - Chart layout and positioning
//! - [`memory`] - Memory management utilities
//! - [`time`] - Time abstraction for animations
//! - [`math`] - Mathematical operations abstraction
//! - [`error`] - Error types and handling
//! - [`prelude`] - Convenient re-exports for common usage
//!
//! For complete API documentation, see the [API Documentation](API_DOCUMENTATION.md).

#![cfg_attr(feature = "no_std", no_std)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

// Conditional std imports
#[cfg(feature = "std")]
extern crate std;

#[cfg(all(feature = "no_std", not(feature = "std")))]
extern crate alloc;

// Math abstraction layer - always available
pub mod math;

// Core modules
pub mod chart;
pub mod data;
pub mod fluent;
pub mod layout;
pub mod render;
pub mod style;

// Grid system
pub mod grid;

// Optional modules based on features
#[cfg(feature = "animations")]
pub mod animation;

// Time abstraction layer
pub mod time;

pub mod axes;

pub mod legend;

// Memory management utilities
pub mod memory;

// Convenience re-exports
pub mod prelude;

// Error types
pub mod error;

// Re-export commonly used types
pub use embedded_graphics;
pub use heapless;

// Re-export math types for convenience
pub use math::{Math, Number};

/// Current version of the library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Library configuration and feature detection
pub mod config {
    /// Check if std is available
    pub const fn has_std() -> bool {
        cfg!(feature = "std")
    }

    /// Check if floating-point math is available
    pub const fn has_floating_point() -> bool {
        cfg!(feature = "floating-point")
    }

    /// Check if fixed-point math is available
    pub const fn has_fixed_point() -> bool {
        cfg!(feature = "fixed-point")
    }

    /// Check if integer-only math is being used
    pub const fn has_integer_math() -> bool {
        cfg!(feature = "integer-math")
    }

    /// Check if animations are available
    pub const fn has_animations() -> bool {
        cfg!(feature = "animations")
    }

    /// Get the math backend name
    pub const fn math_backend() -> &'static str {
        #[cfg(feature = "floating-point")]
        return "floating-point";

        #[cfg(all(feature = "libm-math", not(feature = "floating-point")))]
        return "libm";

        #[cfg(all(
            feature = "fixed-point",
            not(any(feature = "floating-point", feature = "libm-math"))
        ))]
        return "fixed-point";

        #[cfg(all(
            feature = "cordic-math",
            not(any(
                feature = "floating-point",
                feature = "libm-math",
                feature = "fixed-point"
            ))
        ))]
        return "cordic";

        #[cfg(all(
            feature = "integer-math",
            not(any(
                feature = "floating-point",
                feature = "libm-math",
                feature = "fixed-point",
                feature = "cordic-math"
            ))
        ))]
        return "integer";

        #[cfg(not(any(
            feature = "floating-point",
            feature = "libm-math",
            feature = "fixed-point",
            feature = "cordic-math",
            feature = "integer-math"
        )))]
        return "default-floating-point";
    }

    /// Get the system configuration category based on enabled features
    pub const fn system_category() -> &'static str {
        #[cfg(feature = "integer-math")]
        {
            "minimal"
        }
        #[cfg(all(feature = "fixed-point", not(feature = "integer-math")))]
        {
            "balanced"
        }
        #[cfg(all(
            feature = "floating-point",
            not(any(feature = "integer-math", feature = "fixed-point"))
        ))]
        {
            "full-featured"
        }
        #[cfg(not(any(
            feature = "integer-math",
            feature = "fixed-point",
            feature = "floating-point"
        )))]
        {
            "default"
        }
    }
}
