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
//! ```rust
//! use embedded_charts::prelude::*;
//! use embedded_graphics::pixelcolor::Rgb565;
//!
//! let chart = BarChart::builder()
//!     .orientation(BarOrientation::Vertical)
//!     .bar_width(BarWidth::Fixed(20))
//!     .bar_color(Rgb565::GREEN)
//!     .spacing(5)
//!     .build()?;
//! # Ok::<(), embedded_charts::error::ChartError>(())
//! ```
//!
//! ### Pie Charts (feature: "pie")
//! Full circle and donut charts with custom slice styling:
//! ```rust
//! use embedded_charts::prelude::*;
//! use embedded_graphics::pixelcolor::Rgb565;
//!
//! let chart = PieChart::builder()
//!     .center_style(CenterStyle::Hollow(30)) // Donut chart
//!     .slice_spacing(2)
//!     .start_angle(0.0)
//!     .build()?;
//! # Ok::<(), embedded_charts::error::ChartError>(())
//! ```
//!
//! ### Scatter Charts (feature: "scatter")
//! Data point visualization with clustering and collision detection:
//! ```rust
//! use embedded_charts::prelude::*;
//! use embedded_graphics::pixelcolor::Rgb565;
//!
//! let chart = ScatterChart::builder()
//!     .point_style(PointStyle {
//!         shape: PointShape::Circle,
//!         size: SizeMapping::Fixed(8),
//!         color: ColorMapping::Single(Rgb565::BLUE),
//!     })
//!     .collision_detection(true)
//!     .build()?;
//! # Ok::<(), embedded_charts::error::ChartError>(())
//! ```
//!
//! ### Gauge Charts (feature: "gauge")
//! Semicircle gauges with threshold zones and custom indicators:
//! ```rust
//! use embedded_charts::prelude::*;
//! use embedded_graphics::pixelcolor::Rgb565;
//!
//! let chart = GaugeChart::builder()
//!     .gauge_type(GaugeType::Semicircle)
//!     .value_range(ValueRange::new(0.0, 100.0))
//!     .add_threshold_zone(ThresholdZone {
//!         min: 70.0,
//!         max: 100.0,
//!         color: Rgb565::RED,
//!     })
//!     .needle_style(NeedleStyle {
//!         shape: NeedleShape::Arrow,
//!         color: Rgb565::BLACK,
//!         width: 2,
//!     })
//!     .build()?;
//! # Ok::<(), embedded_charts::error::ChartError>(())
//! ```
//!
//! ### Stacked Charts (feature: "stacked-charts")
//! Stacked bar and line charts for comparative data visualization:
//! ```rust,no_run
//! use embedded_charts::prelude::*;
//! use embedded_graphics::pixelcolor::Rgb565;
//!
//! let stacked_data = StackedData::new();
//! // Add data series...
//!
//! let chart = AnimatedStackedBarChart::builder()
//!     .bar_width(StackedBarWidth::Fixed(25))
//!     .spacing(5)
//!     .build()?;
//! # Ok::<(), embedded_charts::error::ChartError>(())
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
//! ```rust
//! use embedded_charts::prelude::*;
//! use embedded_graphics::pixelcolor::Rgb565;
//!
//! let chart = LineChart::builder()
//!     .line_color(Rgb565::BLUE)
//!     .line_width(2)
//!     .margins(constants::DEFAULT_MARGINS)
//!     .with_grid(GridSystem::builder()
//!         .horizontal_linear(GridSpacing::Fixed(10.0))
//!         .build()?)
//!     .build()?;
//! # Ok::<(), embedded_charts::error::ChartError>(())
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
