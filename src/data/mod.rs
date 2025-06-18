//! Data management module for chart data structures and operations.
//!
//! This module provides efficient data structures for storing and managing chart data
//! with static allocation and predictable memory usage. All data structures are designed
//! for embedded systems with limited memory.
//!
//! ## Core Data Types
//!
//! ### Point2D
//! Basic 2D point representation for chart data:
//! ```rust
//! use embedded_charts::prelude::*;
//!
//! let point = Point2D::new(1.0, 25.5);
//! println!("X: {}, Y: {}", point.x(), point.y());
//! ```
//!
//! ### StaticDataSeries
//! Fixed-capacity data series with compile-time bounds:
//! ```rust
//! use embedded_charts::prelude::*;
//!
//! // Create a series with capacity for 256 points
//! let mut series: StaticDataSeries<Point2D, 256> = StaticDataSeries::new();
//!
//! // Add data points
//! series.push(Point2D::new(0.0, 10.0))?;
//! series.push(Point2D::new(1.0, 20.0))?;
//! series.push(Point2D::new(2.0, 15.0))?;
//!
//! // Create from tuples
//! let series: StaticDataSeries<Point2D, 256> = StaticDataSeries::from_tuples(&[
//!     (0.0, 10.0),
//!     (1.0, 20.0),
//!     (2.0, 15.0),
//! ])?;
//!
//! println!("Series has {} points", series.len());
//! # Ok::<(), embedded_charts::error::DataError>(())
//! ```
//!
//! ### MultiSeries
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
//!
//! println!("Multi-series has {} series", multi_series.series_count());
//! # Ok::<(), embedded_charts::error::DataError>(())
//! ```
//!
//! ## Data Bounds
//!
//! Automatic calculation of data bounds for optimal chart scaling:
//! ```rust
//! use embedded_charts::prelude::*;
//!
//! let data = data_points![(0.0, 10.0), (5.0, 30.0), (10.0, 15.0)];
//!
//! // Calculate bounds for single series
//! let bounds = data.bounds()?;
//! println!("X: {} to {}, Y: {} to {}",
//!          bounds.min_x, bounds.max_x, bounds.min_y, bounds.max_y);
//!
//! // For multi-series, bounds are calculated per chart implementation
//! # Ok::<(), embedded_charts::error::DataError>(())
//! ```
//!
//! ## Streaming Data (feature: "animations")
//!
//! Real-time data management with sliding windows:
//! ```rust,no_run
//! # #[cfg(feature = "animations")]
//! # {
//! use embedded_charts::prelude::*;
//!
//! // Sliding window for real-time data (100 points)
//! let mut streaming_data: SlidingWindowSeries<Point2D, 100> =
//!     SlidingWindowSeries::new();
//!
//! // Add data points (automatically removes old ones when full)
//! for i in 0..150 {
//!     let timestamp = i as f32 * 0.1;
//!     let value = (timestamp * 2.0).sin() * 10.0 + 50.0;
//!     let _ = streaming_data.push(Point2D::new(timestamp, value));
//! }
//!
//! println!("Streaming data has {} points", streaming_data.len());
//! # }
//! # Ok::<(), embedded_charts::error::DataError>(())
//! ```
//!
//! ## Memory Efficiency
//!
//! All data structures use static allocation for predictable memory usage:
//! - **No heap allocation**: All data is stored in fixed-size arrays
//! - **Compile-time bounds**: Memory usage is known at compile time
//! - **Zero-cost abstractions**: High-level API with no runtime overhead
//! - **Configurable capacity**: Adjust memory usage per application needs
//!
//! ## Data Point Types
//!
//! ### Basic Points
//! - [`Point2D`] - Standard 2D floating-point coordinates
//! - [`IntPoint`] - Integer coordinates for memory-constrained systems
//! - [`TimestampedPoint`] - Points with timestamp information
//!
//! ### Specialized Points
//! Different point types for specific use cases:
//! ```rust
//! use embedded_charts::prelude::*;
//!
//! // Integer points for memory efficiency
//! let int_point = IntPoint::new(10, 25);
//!
//! // Timestamped points for time-series data
//! let timestamped = TimestampedPoint::new(1234567890.0, 25.5);
//! ```
//!
//! ## Data Series Trait
//!
//! All data containers implement the [`DataSeries`] trait:
//! ```rust,no_run
//! use embedded_charts::data::DataSeries;
//!
//! fn process_data<T: DataSeries>(data: &T) {
//!     println!("Processing {} data points", data.len());
//!     if data.is_empty() {
//!         println!("No data available");
//!     }
//! }
//! ```
//!
//! ## Utility Macros
//!
//! Convenient macros for creating data:
//! ```rust
//! use embedded_charts::prelude::*;
//!
//! // Create data points from tuples
//! let data = data_points![
//!     (0.0, 10.0),
//!     (1.0, 20.0),
//!     (2.0, 15.0),
//!     (3.0, 25.0),
//! ];
//!
//! assert_eq!(data.len(), 4);
//! ```

pub mod bounds;
pub mod point;
pub mod ring_buffer;
pub mod series;

#[cfg(feature = "animations")]
pub mod streaming;

pub use bounds::*;
pub use point::*;
pub use ring_buffer::*;
pub use series::*;

#[cfg(feature = "animations")]
pub use streaming::*;
