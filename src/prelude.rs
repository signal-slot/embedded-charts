//! Convenience re-exports for common types and traits.
//!
//! This module provides a comprehensive prelude that re-exports the most commonly used
//! types, traits, and functions from the embedded-charts library. By importing this
//! prelude, you get access to everything needed for typical chart creation and usage.
//!
//! # Usage
//!
//! ```rust
//! use embedded_charts::prelude::*;
//! use embedded_graphics::pixelcolor::Rgb565;
//!
//! // Now you have access to all common types and functions
//! let mut data: StaticDataSeries<Point2D, 256> = StaticDataSeries::new();
//! data.push(Point2D::new(0.0, 10.0))?;
//!
//! # #[cfg(feature = "line")]
//! let chart = LineChart::builder()
//!     .line_color(Rgb565::BLUE)
//!     .build()?;
//! # Ok::<(), embedded_charts::error::ChartError>(())
//! ```
//!
//! # What's Included
//!
//! ## Core Chart Types
//! - [`LineChart`], [`BarChart`], [`PieChart`], `ScatterChart`, `GaugeChart`
//! - Chart builders and style configurations
//! - Animation support (feature-gated)
//!
//! ## Data Management
//! - [`Point2D`], [`StaticDataSeries`], [`MultiSeries`]
//! - Data bounds calculation utilities
//! - Streaming data support (feature-gated)
//!
//! ## Styling and Themes
//! - Color palettes and themes
//! - Line styles, fill patterns, and borders
//! - Typography support (feature-gated)
//!
//! ## Layout and Rendering
//! - Chart layout and positioning
//! - Rendering primitives and utilities
//! - Memory management tools
//!
//! ## Utility Modules
//! - [`types`] - Common type aliases for convenience
//! - [`constants`] - Predefined constants for margins, spacing, etc.
//! - [`quick`] - Quick-start functions for common chart configurations
//!
//! ## Utility Macros
//! - [`data_points!`] - Create data series from tuples
//! - [`chart_config!`] - Fluent chart configuration syntax
//!
//! # Quick Start Examples
//!
//! ## Simple Line Chart
//! ```rust
//! # #[cfg(feature = "line")]
//! # {
//! # fn test() -> Result<(), embedded_charts::error::ChartError> {
//! use embedded_charts::prelude::*;
//! use embedded_graphics::pixelcolor::Rgb565;
//!
//! let data = data_points![(0.0, 10.0), (1.0, 20.0), (2.0, 15.0)];
//! let chart = quick::line_chart().build()?;
//! # Ok(())
//! # }
//! # }
//! ```
//!
//! ## Professional Styled Chart
//! ```rust
//! # #[cfg(feature = "line")]
//! # {
//! # fn test() -> Result<(), embedded_charts::error::ChartError> {
//! use embedded_charts::prelude::*;
//! use embedded_graphics::pixelcolor::Rgb565;
//!
//! let chart = quick::professional_line_chart()
//!     .with_markers(MarkerStyle::default())
//!     .build()?;
//!
//! let config = chart_config! {
//!     title: "Temperature Monitor",
//!     background: Rgb565::WHITE,
//!     margins: constants::DEFAULT_MARGINS,
//!     grid: true,
//! };
//! # Ok(())
//! # }
//! # }
//! ```
//!
//! ## Multi-Series Chart
//! ```rust
//! use embedded_charts::prelude::*;
//!
//! let mut multi_series: MultiSeries<Point2D, 8, 256> = MultiSeries::new();
//! let temp_data = data_points![(0.0, 22.5), (1.0, 23.1), (2.0, 24.2)];
//! let humidity_data = data_points![(0.0, 65.0), (1.0, 68.0), (2.0, 72.0)];
//!
//! multi_series.add_series(temp_data)?;
//! multi_series.add_series(humidity_data)?;
//! # Ok::<(), embedded_charts::error::DataError>(())
//! ```
//!
//! # Feature-Gated Exports
//!
//! Some exports are only available when specific features are enabled:
//!
//! - **animations**: Animation and streaming chart support
//! - **color-support**: Extended color palette functions
//! - **std**: Standard library time providers and error traits
//!
//! # Memory Efficiency
//!
//! All re-exported types are designed for embedded systems:
//! - Static allocation with compile-time bounds
//! - No heap usage in `no_std` environments
//! - Configurable memory usage through type parameters

// Math abstraction layer
pub use crate::math::{Math, Number, NumericConversion};

// Core traits
pub use crate::chart::traits::{
    Chart, ChartBuilder, ChartConfig, IncrementalChart, Margins, StylableChart,
};

#[cfg(feature = "animations")]
pub use crate::chart::traits::{AnimatedChart, StreamingChart};

pub use crate::chart::traits::{AxisChart, LegendChart};

// Legend types
pub use crate::legend::{
    BackgroundStyle, CompactLegend, CompactLegendBuilder, CustomLegend, CustomLegendBuilder,
    DefaultLegend, DefaultLegendEntry, DefaultLegendRenderer, Legend, LegendAlignment,
    LegendBuilder, LegendEntry, LegendEntryType, LegendMargins, LegendOrientation, LegendRenderer,
    LegendStyle, PositionCalculator, SpacingStyle, StandardLegend, StandardLegendBuilder,
    StandardLegendRenderer, SymbolStyle, TextStyle,
};

pub use crate::legend::types::{
    CompactLegendEntry, CustomLayoutParams, CustomLegendEntry, MarkerShape as LegendMarkerShape,
    MarkerStyle as LegendMarkerStyle, StandardLegendEntry, SymbolShape,
};

pub use crate::legend::position::LegendPosition as LegendPos;

// Axes types
pub use crate::axes::{
    AxisConfig, AxisOrientation, AxisPosition, AxisStyle, AxisValue, CustomAxisBuilder,
    CustomTickGenerator, LinearAxis, LinearAxisBuilder, LinearTickGenerator, TickStyle,
};

pub use crate::axes::builder::presets;

pub use crate::axes::traits::{Axis, AxisRenderer, Tick, TickGenerator};

// Axis range calculation
pub use crate::axes::range::{
    calculate_nice_range, calculate_nice_ranges_from_bounds, calculate_nice_ranges_separate_config,
    RangeCalculationConfig,
};

// Grid types
pub use crate::grid::{
    CustomGrid, CustomGridBuilder, GridBuilder, GridContainer, GridLineStyle, GridSpacing,
    GridStyle, GridSystem, GridType, GridVisibility, LinearGrid, LinearGridBuilder, MajorGridStyle,
    MinorGridStyle, TickBasedGrid, TickBasedGridBuilder,
};

pub use crate::grid::traits::{
    DefaultGridRenderer, Grid, GridConfiguration, GridOrientation, GridRenderer,
};

pub use crate::grid::traits::TickAlignedGrid;

// Chart types
#[cfg(feature = "line")]
pub use crate::chart::{LineChart, LineChartBuilder, LineChartStyle, MarkerShape, MarkerStyle};

#[cfg(all(feature = "line", feature = "animations"))]
pub use crate::chart::{AnimatedLineChart, AnimatedLineChartBuilder};

#[cfg(feature = "bar")]
pub use crate::chart::{BarChart, BarChartBuilder, BarChartStyle, BarOrientation};

#[cfg(all(feature = "bar", feature = "animations"))]
pub use crate::chart::{AnimatedBarChart, AnimatedBarChartBuilder};

#[cfg(feature = "bar")]
pub use crate::chart::bar::BarWidth;

#[cfg(feature = "pie")]
pub use crate::chart::{PieChart, PieChartBuilder, PieChartStyle};

#[cfg(feature = "scatter")]
pub use crate::chart::{
    CollisionSettings, CollisionStrategy, ColorMapping, ColorMappingStrategy, PointShape,
    PointStyle, ScatterChart, ScatterChartBuilder, ScatterChartStyle, SizeMapping, SizeScaling,
};

#[cfg(feature = "gauge")]
pub use crate::chart::{
    ArcStyle, CenterStyle, GaugeChart, GaugeChartBuilder, GaugeChartStyle, GaugeType, NeedleShape,
    NeedleStyle, ThresholdZone, TickStyle as GaugeTickStyle, ValueDisplayStyle, ValueRange,
};

#[cfg(feature = "stacked-charts")]
pub use crate::chart::stacked::{
    AnimatedStackedBarChart, AnimatedStackedBarChartBuilder, AnimatedStackedLineChart,
    AnimatedStackedLineChartBuilder, StackedBarWidth, StackedData,
};

// Data types
pub use crate::data::{
    calculate_bounds, calculate_multi_series_bounds, DataBounds, DataPoint, DataSeries,
    FloatBounds, IntBounds, IntPoint, MultiSeries, Point2D, StaticDataSeries, TimestampedPoint,
};

#[cfg(feature = "animations")]
pub use crate::data::SlidingWindowSeries;

// Streaming types
#[cfg(feature = "animations")]
pub use crate::data::streaming::{
    ChartInstance, ChartInstanceConfig, ChartType, ErrorRecovery, ManagerConfig, ManagerMetrics,
    MemoryStrategy, MonitoringLevel, PipelineConfig, PipelineMetrics, SourceConfig, SourceState,
    StreamingChartManager, StreamingConfig, StreamingDataPipeline, StreamingDataSource,
    StreamingMetrics, SyncMode, SyncState, UnifiedStreamingBuffer,
};

// Style types
pub use crate::style::{
    BorderStyle, ColorInterpolation, ColorPalette, ColorUtils, FillPattern, FillStyle, LineCap,
    LineJoin, LinePattern, LineStyle, StrokeStyle,
};

// Theme types
pub use crate::style::themes::Theme;

#[cfg(feature = "color-support")]
pub use crate::style::rgb565_palettes;

// Layout types
pub use crate::layout::{ChartLayout, ComponentPositioning, Viewport};

// Rendering types
pub use crate::render::{
    ChartRenderer, ClippingRenderer, EnhancedChartRenderer, PrimitiveRenderer,
};

#[cfg(feature = "animations")]
pub use crate::render::AnimationFrameRenderer;

pub use crate::render::text::TextRenderer;

// Memory management
pub use crate::memory::{
    ChartMemoryManager, FixedCapacityCollections, LabelStorage, ManagedSlidingWindow, MemoryStats,
};

// Error types
pub use crate::error::{
    ChartError, ChartResult, DataError, DataResult, LayoutError, LayoutResult, RenderError,
    RenderResult,
};

#[cfg(feature = "animations")]
pub use crate::error::{AnimationError, AnimationResult};

// Animation types
#[cfg(feature = "animations")]
pub use crate::animation::{
    ChartAnimator, EasingFunction, Interpolatable, MultiStateAnimator, Progress, StreamingAnimator,
    TimeBasedProgress,
};

// Time abstraction types
pub use crate::time::{
    ManualTimeProvider, Microseconds, Milliseconds, MonotonicTimeProvider, TimeProvider,
};

#[cfg(feature = "std")]
pub use crate::time::StdTimeProvider;

// Fluent API for convenient chart creation
pub use crate::fluent::quick as fluent_quick;
pub use crate::fluent::{Chart as FluentChart, ChartPreset};

// Re-export embedded-graphics types commonly used with charts
pub use embedded_graphics::{
    pixelcolor::{BinaryColor, Rgb565},
    prelude::*,
    primitives::{Circle, Line, Rectangle},
};

// Re-export heapless types for data storage
pub use heapless::{String, Vec};

/// Common type aliases for convenience
pub mod types {
    use super::*;

    /// Standard RGB565 line chart
    #[cfg(feature = "line")]
    pub type Rgb565LineChart = LineChart<Rgb565>;

    /// Standard RGB565 line chart builder
    #[cfg(feature = "line")]
    pub type Rgb565LineChartBuilder = LineChartBuilder<Rgb565>;

    // Note: Animated chart types are not yet implemented
    // /// Standard RGB565 animated line chart
    // #[cfg(feature = "animations")]
    // pub type Rgb565AnimatedLineChart = AnimatedLineChart<Rgb565>;

    // /// Standard RGB565 animated line chart builder
    // #[cfg(feature = "animations")]
    // pub type Rgb565AnimatedLineChartBuilder = AnimatedLineChartBuilder<Rgb565>;

    // /// Standard RGB565 animated bar chart
    // #[cfg(all(feature = "bar", feature = "animations"))]
    // pub type Rgb565AnimatedBarChart = AnimatedBarChart<Rgb565>;

    // /// Standard RGB565 animated bar chart builder
    // #[cfg(all(feature = "bar", feature = "animations"))]
    // pub type Rgb565AnimatedBarChartBuilder = AnimatedBarChartBuilder<Rgb565>;

    /// Standard floating point data series with 256 point capacity
    pub type StandardDataSeries = StaticDataSeries<Point2D, 256>;

    /// Standard multi-series container (8 series, 256 points each)
    pub type StandardMultiSeries = MultiSeries<Point2D, 8, 256>;

    /// Standard color palette with 8 colors
    pub type StandardColorPalette = ColorPalette<Rgb565, 8>;

    /// Standard sliding window for real-time data (100 points)
    #[cfg(feature = "animations")]
    pub type StandardSlidingWindow = SlidingWindowSeries<Point2D, 100>;

    /// Standard memory manager (4KB pool)
    pub type StandardMemoryManager = ChartMemoryManager<4096>;

    /// Standard label storage (16 labels, 32 chars each)
    pub type StandardLabelStorage = LabelStorage<16, 32>;
}

/// Commonly used constants
pub mod constants {
    use super::*;

    /// Default chart margins
    pub const DEFAULT_MARGINS: Margins = Margins {
        top: 10,
        right: 10,
        bottom: 10,
        left: 10,
    };

    /// Minimal margins for small displays
    pub const MINIMAL_MARGINS: Margins = Margins {
        top: 5,
        right: 5,
        bottom: 5,
        left: 5,
    };

    /// Default grid spacing
    pub const DEFAULT_GRID_SPACING: Size = Size::new(20, 20);

    /// Fine grid spacing
    pub const FINE_GRID_SPACING: Size = Size::new(10, 10);

    /// Coarse grid spacing
    pub const COARSE_GRID_SPACING: Size = Size::new(50, 50);
}

/// Quick start functions for common chart types
pub mod quick {
    use super::*;

    /// Create a simple line chart with default styling
    #[cfg(feature = "line")]
    pub fn line_chart() -> LineChartBuilder<Rgb565> {
        LineChart::builder()
    }

    /// Create a line chart with professional styling
    #[cfg(feature = "line")]
    pub fn professional_line_chart() -> LineChartBuilder<Rgb565> {
        LineChart::builder()
            .line_color(Rgb565::new(70 >> 3, 130 >> 2, 180 >> 3)) // Steel Blue
            .line_width(2)
    }

    /// Create a simple data series from tuples
    pub fn data_series_from_tuples(data: &[(f32, f32)]) -> ChartResult<types::StandardDataSeries> {
        StaticDataSeries::from_tuples(data).map_err(ChartError::from)
    }

    /// Create a default color palette
    #[cfg(feature = "color-support")]
    pub fn default_colors() -> types::StandardColorPalette {
        rgb565_palettes::default_palette()
    }

    /// Create a professional color palette
    #[cfg(feature = "color-support")]
    pub fn professional_colors() -> types::StandardColorPalette {
        rgb565_palettes::professional_palette()
    }

    /// Create a pastel color palette
    #[cfg(feature = "color-support")]
    pub fn pastel_colors() -> types::StandardColorPalette {
        rgb565_palettes::pastel_palette()
    }

    /// Create a vibrant color palette
    #[cfg(feature = "color-support")]
    pub fn vibrant_colors() -> types::StandardColorPalette {
        rgb565_palettes::vibrant_palette()
    }

    /// Create a nature-inspired color palette
    #[cfg(feature = "color-support")]
    pub fn nature_colors() -> types::StandardColorPalette {
        rgb565_palettes::nature_palette()
    }

    /// Create an ocean-inspired color palette
    #[cfg(feature = "color-support")]
    pub fn ocean_colors() -> types::StandardColorPalette {
        rgb565_palettes::ocean_palette()
    }

    /// Create a sunset-inspired color palette
    #[cfg(feature = "color-support")]
    pub fn sunset_colors() -> types::StandardColorPalette {
        rgb565_palettes::sunset_palette()
    }

    /// Create a cyberpunk-inspired color palette
    #[cfg(feature = "color-support")]
    pub fn cyberpunk_colors() -> types::StandardColorPalette {
        rgb565_palettes::cyberpunk_palette()
    }

    /// Create a minimal color palette
    #[cfg(feature = "color-support")]
    pub fn minimal_colors() -> ColorPalette<Rgb565, 6> {
        rgb565_palettes::minimal_palette()
    }

    /// Create a retro color palette
    #[cfg(feature = "color-support")]
    pub fn retro_colors() -> types::StandardColorPalette {
        rgb565_palettes::retro_palette()
    }

    /// Create a light theme
    pub fn light_theme() -> Theme<Rgb565> {
        Theme::light()
    }

    /// Create a dark theme
    pub fn dark_theme() -> Theme<Rgb565> {
        Theme::dark()
    }

    /// Create a vibrant theme
    pub fn vibrant_theme() -> Theme<Rgb565> {
        Theme::vibrant()
    }

    /// Create a pastel theme
    pub fn pastel_theme() -> Theme<Rgb565> {
        Theme::pastel()
    }

    /// Create a nature theme
    pub fn nature_theme() -> Theme<Rgb565> {
        Theme::nature()
    }

    /// Create an ocean theme
    pub fn ocean_theme() -> Theme<Rgb565> {
        Theme::ocean()
    }

    /// Create a sunset theme
    pub fn sunset_theme() -> Theme<Rgb565> {
        Theme::sunset()
    }

    /// Create a cyberpunk theme
    pub fn cyberpunk_theme() -> Theme<Rgb565> {
        Theme::cyberpunk()
    }

    /// Create a minimal theme
    pub fn minimal_theme() -> Theme<Rgb565> {
        Theme::minimal()
    }

    /// Create a retro theme
    pub fn retro_theme() -> Theme<Rgb565> {
        Theme::retro()
    }
}

/// Utility macros for common operations
#[macro_export]
macro_rules! data_points {
    [$(($x:expr, $y:expr)),* $(,)?] => {
        {
            // Use a reasonable default capacity of 256 points
            let mut series = $crate::data::StaticDataSeries::<$crate::data::Point2D, 256>::new();
            $(
                series.push($crate::data::Point2D::new($x, $y)).unwrap();
            )*
            series
        }
    };
}

/// Macro for creating chart configurations with a fluent syntax.
///
/// # Examples
///
/// ```rust,no_run
/// use embedded_charts::prelude::*;
/// use embedded_graphics::pixelcolor::Rgb565;
///
/// let config = chart_config! {
///     title: "My Chart",
///     background: Rgb565::CSS_WHITE,
///     margins: Margins::symmetric(10, 10),
///     grid: true,
/// };
/// ```
#[macro_export]
macro_rules! chart_config {
    (
        $(title: $title:expr,)?
        $(background: $bg:expr,)?
        $(margins: $margins:expr,)?
        $(grid: $grid:expr,)?
    ) => {
        {
            let mut config = $crate::chart::traits::ChartConfig::default();
            $(
                config.title = Some($crate::heapless::String::try_from($title).unwrap());
            )?
            $(
                config.background_color = Some($bg);
            )?
            $(
                config.margins = $margins;
            )?
            $(
                config.show_grid = $grid;
            )?
            config
        }
    };
}

pub use chart_config;
/// Re-export the macros
pub use data_points;
