//! Chart types and implementations.
//!
//! This module provides all chart types supported by the embedded-charts library.
//! Each chart type is feature-gated to allow fine-grained control over binary size
//! and memory usage.
//!
//! ## Available Chart Types
//!
//! ### Line Charts (feature: "line")
//! Multi-series line charts with markers, area filling, and smooth curves:
//! ```rust
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
//! # Ok::<(), embedded_charts::error::ChartError>(())
//! ```
//!
//! ### Bar Charts (feature: "bar")
//! Vertical and horizontal bar charts with customizable spacing:
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
//!     .spacing(5)
//!     .build()?;
//! Ok(())
//! # }
//! ```
//!
//! ### Pie Charts (feature: "pie")
//! Full circle and donut charts with custom slice styling:
//! ```rust,no_run
//! # #[cfg(feature = "pie")]
//! # fn test() -> Result<(), embedded_charts::error::ChartError> {
//! use embedded_charts::prelude::*;
//! use embedded_graphics::pixelcolor::Rgb565;
//!
//! let chart = PieChart::builder()
//!     .donut(30) // Donut chart with inner radius of 30
//!     .start_angle(0.0)
//!     .colors(&[Rgb565::BLUE, Rgb565::RED, Rgb565::GREEN])
//!     .build()?;
//! Ok(())
//! # }
//! ```
//!
//! ### Scatter Charts (feature: "scatter")
//! Data point visualization with clustering and collision detection:
//! ```rust,no_run
//! # #[cfg(feature = "scatter")]
//! # fn test() -> Result<(), embedded_charts::error::ChartError> {
//! use embedded_charts::prelude::*;
//! use embedded_graphics::pixelcolor::Rgb565;
//!
//! let chart = ScatterChart::builder()
//!     .point_shape(PointShape::Circle)
//!     .point_size(8)
//!     .point_color(Rgb565::BLUE)
//!     .with_collision_detection(CollisionSettings {
//!         enabled: true,
//!         min_distance: 5,
//!         strategy: CollisionStrategy::Hide,
//!     })
//!     .build()?;
//! Ok(())
//! # }
//! ```
//!
//! ### Gauge Charts (feature: "gauge")
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
//!     .needle_style(NeedleShape::Arrow, Rgb565::BLACK, 0.8, 2)
//!     .build()?;
//! Ok(())
//! # }
//! ```
//!
//! ### Stacked Charts (feature: "stacked-charts")
//! Stacked bar and line charts for comparative data visualization:
//! ```rust,no_run
//! # #[cfg(feature = "stacked-charts")]
//! # fn test() -> Result<(), embedded_charts::error::ChartError> {
//! use embedded_charts::prelude::*;
//! use embedded_graphics::pixelcolor::Rgb565;
//!
//! let stacked_data: StackedData<Point2D, 256> = StackedData::new();
//! // Add data series...
//!
//! let chart: AnimatedStackedBarChart<Rgb565> = AnimatedStackedBarChart::builder()
//!     .bar_width(StackedBarWidth::Fixed(25))
//!     .spacing(5)
//!     .build()?;
//! Ok(())
//! # }
//! ```
//!
//! ## Chart Traits
//!
//! All charts implement the core [`Chart`] trait, which provides:
//! - `draw()` - Render the chart to a display target
//! - `required_size()` - Calculate minimum size requirements
//! - `data_bounds()` - Get data bounds for scaling
//!
//! Additional traits provide extended functionality:
//! - [`StylableChart`] - Apply custom styling
//! - [`AxisChart`] - Configure chart axes
//! - [`LegendChart`] - Add legends
//! - [`AnimatedChart`] - Animation support (feature-gated)
//! - [`StreamingChart`] - Real-time data streaming (feature-gated)
//!
//! ## Builder Pattern
//!
//! All charts use the builder pattern for fluent configuration:
//! ```rust,no_run
//! # fn test() -> Result<(), embedded_charts::error::ChartError> {
//! use embedded_charts::prelude::*;
//! use embedded_graphics::pixelcolor::Rgb565;
//!
//! let chart = LineChart::builder()
//!     .line_color(Rgb565::BLUE)
//!     .line_width(2)
//!     .build()?;
//! Ok(())
//! # }
//! ```

#[cfg(feature = "bar")]
pub mod bar;
#[cfg(feature = "line")]
pub mod line;
#[cfg(feature = "pie")]
pub mod pie;
pub mod traits;

#[cfg(feature = "scatter")]
pub mod scatter;

#[cfg(feature = "gauge")]
pub mod gauge;

#[cfg(feature = "stacked-charts")]
pub mod stacked;

#[cfg(feature = "custom")]
pub mod custom;

#[cfg(feature = "bar")]
pub use bar::*;
#[cfg(feature = "line")]
pub use line::*;
#[cfg(feature = "pie")]
pub use pie::*;
pub use traits::*;

#[cfg(feature = "scatter")]
pub use scatter::*;

#[cfg(feature = "gauge")]
pub use gauge::*;

#[cfg(feature = "stacked-charts")]
pub use stacked::*;

#[cfg(feature = "custom")]
pub use custom::*;
