//! Error types and result handling for the embedded graphics chart library.
//!
//! This module provides comprehensive error handling for all chart operations while maintaining
//! compatibility with both `std` and `no_std` environments. All error types implement the
//! necessary traits for proper error propagation and debugging.
//!
//! # Error Categories
//!
//! The library uses a hierarchical error system with specific error types for different
//! operation categories:
//!
//! - [`ChartError`] - Main error type for high-level chart operations
//! - [`DataError`] - Errors related to data management and processing
//! - [`RenderError`] - Errors during rendering and drawing operations
//! - [`LayoutError`] - Errors in chart layout and positioning
//! - [`AnimationError`] - Errors in animation system (feature-gated)
//!
//! # Error Handling Patterns
//!
//! ## Basic Error Handling
//! ```rust
//! use embedded_charts::prelude::*;
//! use embedded_graphics::pixelcolor::Rgb565;
//!
//! let mut data: StaticDataSeries<Point2D, 256> = StaticDataSeries::new();
//! match data.push(Point2D::new(0.0, 10.0)) {
//!     Ok(()) => println!("Data added successfully"),
//!     Err(DataError::BufferFull { .. }) => println!("Data series is full"),
//!     Err(e) => println!("Other error: {}", e),
//! }
//! ```
//!
//! ## Using Result Types
//! ```rust
//! # #[cfg(feature = "line")]
//! # {
//! use embedded_charts::prelude::*;
//! use embedded_charts::error::ChartResult;
//!
//! fn create_chart() -> ChartResult<LineChart<embedded_graphics::pixelcolor::Rgb565>> {
//!     LineChart::builder()
//!         .line_color(embedded_graphics::pixelcolor::Rgb565::BLUE)
//!         .build()
//! }
//! # }
//! ```
//!
//! ## Error Propagation
//! ```rust
//! use embedded_charts::prelude::*;
//! use embedded_charts::error::{ChartResult, DataError};
//!
//! fn process_data() -> ChartResult<()> {
//!     let mut data: StaticDataSeries<Point2D, 256> = StaticDataSeries::new();
//!     data.push(Point2D::new(0.0, 10.0))?; // Automatically converts DataError to ChartError
//!     Ok(())
//! }
//! ```
//!
//! # no_std Compatibility
//!
//! All error types are designed to work in `no_std` environments:
//! - No heap allocation for error messages
//! - Implement `core::fmt::Display` instead of `std::fmt::Display`
//! - Optional `std::error::Error` implementation when `std` feature is enabled
//!
//! # Memory Efficiency
//!
//! Error types are designed for minimal memory usage:
//! - All error variants are `Copy` types
//! - No dynamic string allocation
//! - Efficient error code representation

#[cfg(feature = "std")]
extern crate std;

/// Error context information for better debugging
///
/// This struct provides additional context for errors while maintaining
/// no_std compatibility by using static string references.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ErrorContext {
    /// The operation that was being performed when the error occurred
    pub operation: &'static str,
    /// A hint for resolving the error
    pub hint: &'static str,
    /// Optional numeric context (e.g., data point count, buffer size)
    pub numeric_context: Option<usize>,
}

impl ErrorContext {
    /// Create a new error context
    pub const fn new(operation: &'static str, hint: &'static str) -> Self {
        Self {
            operation,
            hint,
            numeric_context: None,
        }
    }

    /// Create error context with numeric information
    pub const fn with_numeric(operation: &'static str, hint: &'static str, value: usize) -> Self {
        Self {
            operation,
            hint,
            numeric_context: Some(value),
        }
    }
}

/// Main error type for chart operations.
///
/// This is the primary error type returned by most chart operations. It encompasses
/// all possible error conditions that can occur during chart creation, configuration,
/// and rendering.
///
/// # Error Variants
///
/// - **Data-related errors**: Issues with data series, points, or bounds
/// - **Rendering errors**: Problems during drawing operations
/// - **Configuration errors**: Invalid chart or component configuration
/// - **Memory errors**: Buffer overflow or allocation failures
/// - **Animation errors**: Issues with animation system (feature-gated)
///
/// # Examples
///
/// ```rust,no_run
/// use embedded_charts::prelude::*;
/// use embedded_charts::error::ChartError;
///
/// // Example function that might return a ChartError
/// fn render_chart() -> Result<(), ChartError> {
///     // This would be actual chart rendering logic
///     Err(ChartError::InsufficientData)
/// }
///
/// match render_chart() {
///     Ok(()) => println!("Chart rendered successfully"),
///     Err(ChartError::InsufficientData) => println!("Not enough data to render"),
///     Err(ChartError::MemoryFull) => println!("Out of memory"),
///     Err(e) => println!("Other error: {}", e),
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChartError {
    /// Insufficient data to render the chart.
    ///
    /// This error occurs when a chart requires a minimum number of data points
    /// but the provided data series doesn't meet that requirement.
    InsufficientData,
    /// Invalid range specified for axis or data.
    ///
    /// Returned when axis ranges are invalid (e.g., min > max) or when
    /// data values fall outside expected ranges.
    InvalidRange,
    /// Invalid data provided.
    ///
    /// Generic error for data that doesn't meet the chart's requirements,
    /// such as NaN values, infinite values, or malformed data points.
    InvalidData,
    /// Memory allocation failed or buffer is full.
    ///
    /// Occurs when static buffers reach capacity or when memory allocation
    /// fails in `std` environments.
    MemoryFull,
    /// Error occurred during rendering.
    ///
    /// Generic rendering error for issues during the drawing process.
    RenderingError,
    /// Invalid configuration provided.
    ///
    /// Returned when chart configuration contains invalid or conflicting settings.
    InvalidConfiguration,
    /// Configuration error occurred.
    ///
    /// More specific configuration error, typically with additional context.
    ConfigurationError,
    /// Render error occurred.
    ///
    /// Specific rendering error with detailed error information.
    RenderError(RenderError),
    /// Layout error occurred.
    ///
    /// Error during chart layout calculation or component positioning.
    LayoutError(LayoutError),
    /// Data error occurred.
    ///
    /// Specific data-related error with detailed error information.
    DataError(DataError),
    /// Animation error occurred.
    ///
    /// Error in the animation system (only available with "animations" feature).
    #[cfg(feature = "animations")]
    AnimationError(AnimationError),
}

/// Error type for data operations.
///
/// This error type covers all data-related operations including data series
/// management, data point validation, and data processing operations.
///
/// # Common Scenarios
///
/// - Adding data to a full buffer
/// - Accessing data with invalid indices
/// - Processing invalid data points
/// - Data scaling and transformation errors
///
/// # Examples
///
/// ```rust
/// use embedded_charts::prelude::*;
/// use embedded_charts::error::DataError;
///
/// let mut series = StaticDataSeries::<Point2D, 10>::new();
///
/// // Fill the series to capacity
/// for i in 0..10 {
///     series.push(Point2D::new(i as f32, i as f32)).unwrap();
/// }
///
/// // This will return BufferFull error with context
/// match series.push(Point2D::new(10.0, 10.0)) {
///     Err(DataError::BufferFull { context: Some(ctx) }) => {
///         println!("Series is full: {}", ctx.hint);
///     },
///     _ => unreachable!(),
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataError {
    /// Requested data series was not found.
    ///
    /// Occurs when trying to access a data series by name or index
    /// that doesn't exist in the collection.
    SeriesNotFound {
        /// Optional context information
        context: Option<ErrorContext>,
    },
    /// Index is out of bounds for the data collection.
    ///
    /// Returned when accessing data with an invalid index.
    IndexOutOfBounds {
        /// Optional context information
        context: Option<ErrorContext>,
    },
    /// Invalid data point provided.
    ///
    /// Occurs when a data point contains invalid values such as
    /// NaN, infinity, or values outside acceptable ranges.
    InvalidDataPoint {
        /// Optional context information
        context: Option<ErrorContext>,
    },
    /// Error occurred during data scaling.
    ///
    /// Returned when data scaling or normalization operations fail,
    /// typically due to invalid ranges or mathematical errors.
    ScalingError {
        /// Optional context information
        context: Option<ErrorContext>,
    },
    /// Buffer capacity exceeded.
    ///
    /// Occurs when trying to add data to a full static buffer.
    BufferFull {
        /// Optional context information
        context: Option<ErrorContext>,
    },
    /// Insufficient data to perform operation.
    ///
    /// Returned when an operation requires a minimum amount of data
    /// that isn't available.
    InsufficientData {
        /// Optional context information
        context: Option<ErrorContext>,
    },
}

impl DataError {
    /// Create a BufferFull error with context
    pub const fn buffer_full(operation: &'static str, capacity: usize) -> Self {
        Self::BufferFull {
            context: Some(ErrorContext::with_numeric(
                operation,
                "Increase buffer capacity or remove old data",
                capacity,
            )),
        }
    }

    /// Create an InsufficientData error with context
    pub const fn insufficient_data(
        operation: &'static str,
        _required: usize,
        found: usize,
    ) -> Self {
        Self::InsufficientData {
            context: Some(ErrorContext::with_numeric(
                operation,
                "Add more data points",
                found,
            )),
        }
    }

    /// Create an IndexOutOfBounds error with context
    pub const fn index_out_of_bounds(
        operation: &'static str,
        _index: usize,
        length: usize,
    ) -> Self {
        Self::IndexOutOfBounds {
            context: Some(ErrorContext::with_numeric(
                operation,
                "Check array bounds before accessing",
                length,
            )),
        }
    }

    /// Create an InvalidDataPoint error with context
    pub const fn invalid_data_point(operation: &'static str) -> Self {
        Self::InvalidDataPoint {
            context: Some(ErrorContext::new(
                operation,
                "Ensure data points contain valid finite values",
            )),
        }
    }

    /// Create a simple error without context (for backwards compatibility)
    pub const fn simple(kind: DataErrorKind) -> Self {
        match kind {
            DataErrorKind::SeriesNotFound => Self::SeriesNotFound { context: None },
            DataErrorKind::IndexOutOfBounds => Self::IndexOutOfBounds { context: None },
            DataErrorKind::InvalidDataPoint => Self::InvalidDataPoint { context: None },
            DataErrorKind::ScalingError => Self::ScalingError { context: None },
            DataErrorKind::BufferFull => Self::BufferFull { context: None },
            DataErrorKind::InsufficientData => Self::InsufficientData { context: None },
        }
    }
}

// Backwards compatibility constants
impl DataError {
    /// Backwards compatibility: SeriesNotFound variant without context
    pub const SERIES_NOT_FOUND: Self = Self::SeriesNotFound { context: None };

    /// Backwards compatibility: IndexOutOfBounds variant without context  
    pub const INDEX_OUT_OF_BOUNDS: Self = Self::IndexOutOfBounds { context: None };

    /// Backwards compatibility: InvalidDataPoint variant without context
    pub const INVALID_DATA_POINT: Self = Self::InvalidDataPoint { context: None };

    /// Backwards compatibility: ScalingError variant without context
    pub const SCALING_ERROR: Self = Self::ScalingError { context: None };

    /// Backwards compatibility: BufferFull variant without context
    pub const BUFFER_FULL: Self = Self::BufferFull { context: None };

    /// Backwards compatibility: InsufficientData variant without context
    pub const INSUFFICIENT_DATA: Self = Self::InsufficientData { context: None };
}

/// Data error kinds for backwards compatibility
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataErrorKind {
    /// Requested data series was not found
    SeriesNotFound,
    /// Index is out of bounds for the data collection
    IndexOutOfBounds,
    /// Invalid data point provided
    InvalidDataPoint,
    /// Error occurred during data scaling
    ScalingError,
    /// Buffer capacity exceeded
    BufferFull,
    /// Insufficient data to perform operation
    InsufficientData,
}

/// Error type for animation operations.
///
/// This error type is only available when the "animations" feature is enabled.
/// It covers all animation-related operations including animation scheduling,
/// interpolation, and state management.
///
/// # Common Scenarios
///
/// - Invalid animation duration
/// - Animation scheduler at capacity
/// - Interpolation failures
/// - Invalid animation state transitions
///
/// # Examples
///
/// ```rust,no_run
/// # #[cfg(feature = "animations")]
/// # {
/// use embedded_charts::prelude::*;
/// use embedded_charts::error::AnimationError;
///
/// // Example function that might return an AnimationError
/// fn start_animation() -> Result<u32, AnimationError> {
///     // This would be actual animation logic
///     Err(AnimationError::InvalidDuration)
/// }
///
/// match start_animation() {
///     Ok(animation_id) => println!("Animation started: {}", animation_id),
///     Err(AnimationError::InvalidDuration) => println!("Duration must be positive"),
///     Err(AnimationError::SchedulerFull) => println!("Too many active animations"),
///     Err(e) => println!("Animation error: {}", e),
/// }
/// # }
/// ```
#[cfg(feature = "animations")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationError {
    /// Invalid duration specified.
    ///
    /// Occurs when animation duration is zero, negative, or exceeds
    /// maximum allowed duration.
    InvalidDuration,
    /// Animation with specified ID was not found.
    ///
    /// Returned when trying to access or modify an animation that
    /// doesn't exist or has already completed.
    AnimationNotFound,
    /// Animation scheduler is full.
    ///
    /// Occurs when trying to start a new animation but the scheduler
    /// has reached its maximum capacity.
    SchedulerFull,
    /// Error occurred during interpolation.
    ///
    /// Returned when interpolation between animation keyframes fails,
    /// typically due to incompatible data types or invalid values.
    InterpolationError,
    /// Animation state is invalid.
    ///
    /// Occurs when an animation operation is attempted on an animation
    /// in an inappropriate state (e.g., trying to pause a completed animation).
    InvalidState,
}

/// Error type for layout operations.
///
/// This error type covers chart layout calculations, component positioning,
/// and space allocation operations.
///
/// # Common Scenarios
///
/// - Insufficient space for chart components
/// - Invalid layout configuration
/// - Component positioning failures
///
/// # Examples
///
/// ```rust,no_run
/// use embedded_charts::prelude::*;
/// use embedded_charts::error::LayoutError;
/// use embedded_graphics::prelude::*;
///
/// // Example function that might return a LayoutError
/// fn check_layout_error() -> Result<(), LayoutError> {
///     // This would be actual layout calculation logic
///     Err(LayoutError::InsufficientSpace)
/// }
///
/// match check_layout_error() {
///     Ok(()) => println!("Layout calculated successfully"),
///     Err(LayoutError::InsufficientSpace) => println!("Viewport too small"),
///     Err(LayoutError::InvalidConfiguration) => println!("Invalid layout config"),
///     Err(e) => println!("Layout error: {}", e),
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutError {
    /// Insufficient space for layout.
    ///
    /// Occurs when the available viewport is too small to accommodate
    /// the chart and its components with the current configuration.
    InsufficientSpace,
    /// Invalid layout configuration.
    ///
    /// Returned when layout parameters are invalid or conflicting,
    /// such as negative margins or impossible component arrangements.
    InvalidConfiguration,
    /// Component positioning failed.
    ///
    /// Occurs when individual chart components cannot be positioned
    /// within the available space, even with valid overall layout.
    PositioningFailed,
}

/// Error type for rendering operations.
///
/// This error type covers low-level rendering operations including drawing
/// primitives, text rendering, and color operations.
///
/// # Common Scenarios
///
/// - Drawing operation failures
/// - Text rendering issues
/// - Color conversion problems
/// - Clipping operation failures
///
/// # Examples
///
/// ```rust,no_run
/// use embedded_charts::prelude::*;
/// use embedded_charts::error::RenderError;
///
/// // Example function that might return a RenderError
/// fn draw_something() -> Result<(), RenderError> {
///     // This would be actual rendering logic
///     Err(RenderError::DrawingFailed)
/// }
///
/// match draw_something() {
///     Ok(()) => println!("Drawing completed successfully"),
///     Err(RenderError::DrawingFailed) => println!("Failed to draw"),
///     Err(RenderError::ColorConversionFailed) => println!("Invalid color"),
///     Err(e) => println!("Render error: {}", e),
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderError {
    /// Drawing operation failed.
    ///
    /// Generic error for drawing operations that fail due to
    /// display driver issues or invalid drawing parameters.
    DrawingFailed,
    /// Text rendering failed.
    ///
    /// Occurs when text cannot be rendered, typically due to
    /// font issues, invalid characters, or display limitations.
    TextRenderingFailed,
    /// Clipping operation failed.
    ///
    /// Returned when clipping regions cannot be established
    /// or when clipping operations fail.
    ClippingFailed,
    /// Color conversion failed.
    ///
    /// Occurs when color values cannot be converted between
    /// different color spaces or pixel formats.
    ColorConversionFailed,
}

impl From<&str> for DataError {
    fn from(_msg: &str) -> Self {
        // For no_std compatibility, we can't store the string message
        // so we return a generic error variant
        DataError::simple(DataErrorKind::InvalidDataPoint)
    }
}

impl From<DataError> for ChartError {
    fn from(error: DataError) -> Self {
        ChartError::DataError(error)
    }
}

#[cfg(feature = "animations")]
impl From<AnimationError> for ChartError {
    fn from(error: AnimationError) -> Self {
        ChartError::AnimationError(error)
    }
}

impl From<LayoutError> for ChartError {
    fn from(_error: LayoutError) -> Self {
        ChartError::InvalidConfiguration
    }
}

impl From<RenderError> for ChartError {
    fn from(_error: RenderError) -> Self {
        ChartError::RenderingError
    }
}

/// Result type for chart operations
pub type ChartResult<T> = Result<T, ChartError>;

/// Result type for data operations
pub type DataResult<T> = Result<T, DataError>;

/// Result type for animation operations
#[cfg(feature = "animations")]
pub type AnimationResult<T> = Result<T, AnimationError>;

/// Result type for layout operations
pub type LayoutResult<T> = Result<T, LayoutError>;

/// Result type for rendering operations
pub type RenderResult<T> = Result<T, RenderError>;

// Implement std::error::Error trait for error types when std is available
#[cfg(feature = "std")]
impl std::error::Error for ChartError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ChartError::DataError(err) => Some(err),
            ChartError::RenderError(err) => Some(err),
            ChartError::LayoutError(err) => Some(err),
            #[cfg(feature = "animations")]
            ChartError::AnimationError(err) => Some(err),
            _ => None,
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for DataError {}

#[cfg(feature = "animations")]
#[cfg(feature = "std")]
impl std::error::Error for AnimationError {}

#[cfg(feature = "std")]
impl std::error::Error for LayoutError {}

#[cfg(feature = "std")]
impl std::error::Error for RenderError {}

// Implement Display trait for error types
impl core::fmt::Display for ChartError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ChartError::InsufficientData => write!(f, "Insufficient data to render the chart"),
            ChartError::InvalidRange => write!(f, "Invalid range specified for axis or data"),
            ChartError::InvalidData => write!(f, "Invalid data provided"),
            ChartError::MemoryFull => write!(f, "Memory allocation failed or buffer is full"),
            ChartError::RenderingError => write!(f, "Error occurred during rendering"),
            ChartError::InvalidConfiguration => write!(f, "Invalid configuration provided"),
            ChartError::ConfigurationError => write!(f, "Configuration error occurred"),
            ChartError::RenderError(err) => write!(f, "Render error: {err}"),
            ChartError::LayoutError(err) => write!(f, "Layout error: {err}"),
            ChartError::DataError(err) => write!(f, "Data error: {err}"),
            #[cfg(feature = "animations")]
            ChartError::AnimationError(err) => write!(f, "Animation error: {err}"),
        }
    }
}

impl core::fmt::Display for DataError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            DataError::SeriesNotFound { context } => {
                write!(f, "Requested data series was not found")?;
                if let Some(ctx) = context {
                    write!(f, " during {} (hint: {})", ctx.operation, ctx.hint)?;
                }
                Ok(())
            }
            DataError::IndexOutOfBounds { context } => {
                write!(f, "Index is out of bounds for the data collection")?;
                if let Some(ctx) = context {
                    write!(f, " during {} (hint: {})", ctx.operation, ctx.hint)?;
                    if let Some(value) = ctx.numeric_context {
                        write!(f, " [length: {value}]")?;
                    }
                }
                Ok(())
            }
            DataError::InvalidDataPoint { context } => {
                write!(f, "Invalid data point provided")?;
                if let Some(ctx) = context {
                    write!(f, " during {} (hint: {})", ctx.operation, ctx.hint)?;
                }
                Ok(())
            }
            DataError::ScalingError { context } => {
                write!(f, "Error occurred during data scaling")?;
                if let Some(ctx) = context {
                    write!(f, " during {} (hint: {})", ctx.operation, ctx.hint)?;
                }
                Ok(())
            }
            DataError::BufferFull { context } => {
                write!(f, "Buffer capacity exceeded")?;
                if let Some(ctx) = context {
                    write!(f, " during {} (hint: {})", ctx.operation, ctx.hint)?;
                    if let Some(capacity) = ctx.numeric_context {
                        write!(f, " [capacity: {capacity}]")?;
                    }
                }
                Ok(())
            }
            DataError::InsufficientData { context } => {
                write!(f, "Insufficient data to perform operation")?;
                if let Some(ctx) = context {
                    write!(f, " during {} (hint: {})", ctx.operation, ctx.hint)?;
                    if let Some(found) = ctx.numeric_context {
                        write!(f, " [found: {found}]")?;
                    }
                }
                Ok(())
            }
        }
    }
}

#[cfg(feature = "animations")]
impl core::fmt::Display for AnimationError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            AnimationError::InvalidDuration => write!(f, "Invalid duration specified"),
            AnimationError::AnimationNotFound => {
                write!(f, "Animation with specified ID was not found")
            }
            AnimationError::SchedulerFull => write!(f, "Animation scheduler is full"),
            AnimationError::InterpolationError => write!(f, "Error occurred during interpolation"),
            AnimationError::InvalidState => write!(f, "Animation state is invalid"),
        }
    }
}

impl core::fmt::Display for LayoutError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            LayoutError::InsufficientSpace => write!(f, "Insufficient space for layout"),
            LayoutError::InvalidConfiguration => write!(f, "Invalid layout configuration"),
            LayoutError::PositioningFailed => write!(f, "Component positioning failed"),
        }
    }
}

impl core::fmt::Display for RenderError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            RenderError::DrawingFailed => write!(f, "Drawing operation failed"),
            RenderError::TextRenderingFailed => write!(f, "Text rendering failed"),
            RenderError::ClippingFailed => write!(f, "Clipping operation failed"),
            RenderError::ColorConversionFailed => write!(f, "Color conversion failed"),
        }
    }
}
