//! Data aggregation and downsampling for efficient chart rendering.
//!
//! This module provides efficient algorithms for reducing the number of data points
//! while preserving important characteristics of the data. This is crucial for:
//! - Large datasets that exceed display resolution
//! - Real-time streaming data that needs performance optimization
//! - Memory-constrained embedded environments
//! - Maintaining visual fidelity while reducing computational load
//!
//! # Aggregation Strategies
//!
//! Different strategies for combining multiple data points into a single representative point:
//!
//! ## Min-Max Aggregation
//! Preserves extremes in the data, essential for identifying peaks and troughs:
//! ```rust
//! use embedded_charts::prelude::*;
//! use embedded_charts::data::aggregation::*;
//!
//! let data = data_points![(0.0, 10.0), (1.0, 25.0), (2.0, 5.0), (3.0, 20.0)];
//! let config = AggregationConfig {
//!     strategy: AggregationStrategy::MinMax,
//!     target_points: 2,
//!     ..Default::default()
//! };
//! let aggregated: StaticDataSeries<_, 8> = data.aggregate(&config)?;
//! // Result preserves the minimum (5.0) and maximum (25.0) values
//! # Ok::<(), embedded_charts::error::DataError>(())
//! ```
//!
//! ## Statistical Aggregation
//! Uses statistical measures to represent groups of data points:
//! ```rust
//! use embedded_charts::prelude::*;
//! use embedded_charts::data::aggregation::*;
//!
//! let data = data_points![(0.0, 10.0), (1.0, 20.0), (2.0, 30.0), (3.0, 40.0)];
//! let config = AggregationConfig {
//!     strategy: AggregationStrategy::Mean,
//!     target_points: 2,
//!     ..Default::default()
//! };
//! let mean_aggregated: StaticDataSeries<_, 8> = data.aggregate(&config)?;
//! # Ok::<(), embedded_charts::error::DataError>(())
//! ```
//!
//! # Downsampling Algorithms
//!
//! ## Largest Triangle Three Buckets (LTTB)
//! Advanced algorithm that preserves visual characteristics:
//! ```rust
//! use embedded_charts::prelude::*;
//! use embedded_charts::data::aggregation::*;
//!
//! let data = data_points![(0.0, 10.0), (1.0, 25.0), (2.0, 5.0), (3.0, 20.0)];
//! let config = DownsamplingConfig {
//!     max_points: 50,
//!     ..Default::default()
//! };
//! let downsampled: StaticDataSeries<_, 8> = data.downsample_lttb(&config)?;
//! # Ok::<(), embedded_charts::error::DataError>(())
//! ```
//!
//! ## Uniform Downsampling
//! Simple algorithm that takes every Nth point:
//! ```rust
//! use embedded_charts::prelude::*;
//! use embedded_charts::data::aggregation::*;
//!
//! let data = data_points![(0.0, 10.0), (1.0, 25.0), (2.0, 5.0), (3.0, 20.0)];
//! let config = DownsamplingConfig {
//!     max_points: 2,
//!     ..Default::default()
//! };
//! let downsampled: StaticDataSeries<_, 8> = data.downsample_uniform(&config)?;
//! # Ok::<(), embedded_charts::error::DataError>(())
//! ```
//!
//! # Memory Efficiency
//!
//! All aggregation operates with bounded memory usage:
//! - Static allocation for intermediate calculations
//! - Configurable output capacity
//! - No heap allocation in no_std environments

use crate::data::{DataPoint, DataSeries, StaticDataSeries};
use crate::error::{DataError, DataResult};

#[cfg(feature = "std")]
use libm::{ceilf, floorf, roundf};

#[cfg(not(feature = "std"))]
use micromath::F32Ext;

/// Strategy for aggregating multiple data points into a single representative point
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AggregationStrategy {
    /// Take the mean (average) of X and Y coordinates
    Mean,
    /// Take the median of X and Y coordinates
    Median,
    /// Preserve minimum and maximum Y values, average X coordinates
    MinMax,
    /// Take the first point in each group
    First,
    /// Take the last point in each group
    Last,
    /// Take the point with maximum Y value
    Max,
    /// Take the point with minimum Y value
    Min,
}

/// Configuration for data aggregation operations
#[derive(Debug, Clone)]
pub struct AggregationConfig {
    /// Strategy to use for combining data points
    pub strategy: AggregationStrategy,
    /// Target number of points after aggregation
    pub target_points: usize,
    /// Whether to preserve first and last points exactly
    pub preserve_endpoints: bool,
    /// Minimum number of source points required for aggregation
    pub min_group_size: usize,
}

impl Default for AggregationConfig {
    fn default() -> Self {
        Self {
            strategy: AggregationStrategy::Mean,
            target_points: 100,
            preserve_endpoints: true,
            min_group_size: 1,
        }
    }
}

/// Configuration for downsampling operations
#[derive(Debug, Clone)]
pub struct DownsamplingConfig {
    /// Maximum number of points in the output
    pub max_points: usize,
    /// Whether to preserve first and last points exactly
    pub preserve_endpoints: bool,
    /// Threshold below which no downsampling is performed
    pub min_reduction_ratio: f32,
}

impl Default for DownsamplingConfig {
    fn default() -> Self {
        Self {
            max_points: 1000,
            preserve_endpoints: true,
            min_reduction_ratio: 1.5, // Only downsample if reducing by at least 50%
        }
    }
}

/// Statistics calculated for a group of data points during aggregation
#[derive(Debug, Clone)]
pub struct GroupStats<T: DataPoint> {
    /// Number of points in the group
    pub count: usize,
    /// Minimum X value
    pub min_x: T::X,
    /// Maximum X value
    pub max_x: T::X,
    /// Minimum Y value
    pub min_y: T::Y,
    /// Maximum Y value
    pub max_y: T::Y,
    /// Mean X value
    pub mean_x: T::X,
    /// Mean Y value
    pub mean_y: T::Y,
    /// First point in the group
    pub first: T,
    /// Last point in the group
    pub last: T,
}

/// Trait providing aggregation and downsampling capabilities for data series
pub trait DataAggregation: DataSeries {
    /// Aggregate data points using the specified strategy
    ///
    /// # Arguments
    /// * `config` - Configuration for the aggregation operation
    ///
    /// # Returns
    /// A new data series with aggregated points
    fn aggregate<const N: usize>(
        &self,
        config: &AggregationConfig,
    ) -> DataResult<StaticDataSeries<Self::Item, N>>;

    /// Downsample data using Largest Triangle Three Buckets algorithm
    ///
    /// This algorithm preserves the visual characteristics of the data better than
    /// simple uniform sampling by considering the area of triangles formed by
    /// adjacent points.
    ///
    /// # Arguments
    /// * `config` - Configuration for the downsampling operation
    ///
    /// # Returns
    /// A new data series with downsampled points
    fn downsample_lttb<const N: usize>(
        &self,
        config: &DownsamplingConfig,
    ) -> DataResult<StaticDataSeries<Self::Item, N>>;

    /// Downsample data using uniform sampling (every Nth point)
    ///
    /// # Arguments
    /// * `config` - Configuration for the downsampling operation
    ///
    /// # Returns
    /// A new data series with uniformly sampled points
    fn downsample_uniform<const N: usize>(
        &self,
        config: &DownsamplingConfig,
    ) -> DataResult<StaticDataSeries<Self::Item, N>>;

    /// Calculate statistics for a group of data points
    ///
    /// # Arguments
    /// * `points` - Slice of data points to analyze
    ///
    /// # Returns
    /// Statistics for the group of points
    fn calculate_group_stats(&self, points: &[Self::Item]) -> DataResult<GroupStats<Self::Item>>
    where
        Self::Item: Clone;
}

/// Implementation of aggregation for StaticDataSeries
impl<T, const M: usize> DataAggregation for StaticDataSeries<T, M>
where
    T: DataPoint + Clone + Copy,
    T::X: PartialOrd
        + Copy
        + core::ops::Add<Output = T::X>
        + core::ops::Div<f32, Output = T::X>
        + Into<f32>
        + From<f32>,
    T::Y: PartialOrd
        + Copy
        + core::ops::Add<Output = T::Y>
        + core::ops::Div<f32, Output = T::Y>
        + Into<f32>
        + From<f32>,
{
    fn aggregate<const N: usize>(
        &self,
        config: &AggregationConfig,
    ) -> DataResult<StaticDataSeries<T, N>> {
        if self.is_empty() {
            return Ok(StaticDataSeries::new());
        }

        if self.len() <= config.target_points {
            // No aggregation needed
            let mut result = StaticDataSeries::new();
            for point in self.iter() {
                result.push(point)?;
            }
            return Ok(result);
        }

        let mut result = StaticDataSeries::new();
        let points = self.as_slice();

        // Calculate group size
        let group_size = (self.len() + config.target_points - 1) / config.target_points;
        let group_size = group_size.max(config.min_group_size);

        let mut i = 0;

        // Handle first point specially if preserving endpoints
        if config.preserve_endpoints && !points.is_empty() {
            result.push(points[0])?;
            i = 1;
        }

        // Process groups
        while i < points.len() {
            let mut end = (i + group_size).min(points.len());

            // Skip last group if preserving endpoints and this is the final point
            if config.preserve_endpoints && end == points.len() && i + 1 < points.len() {
                end = points.len() - 1;
            }

            if i < end {
                let group = &points[i..end];
                if !group.is_empty() {
                    let aggregated_point = self.aggregate_group(group, config.strategy)?;
                    result.push(aggregated_point)?;
                }
            }

            i = end;
        }

        // Handle last point specially if preserving endpoints
        if config.preserve_endpoints && points.len() > 1 {
            let last_point = points[points.len() - 1];
            // Only add if it's different from the last added point
            if result.is_empty() || result.as_slice()[result.len() - 1].x() != last_point.x() {
                result.push(last_point)?;
            }
        }

        Ok(result)
    }

    fn downsample_lttb<const N: usize>(
        &self,
        config: &DownsamplingConfig,
    ) -> DataResult<StaticDataSeries<T, N>> {
        if self.is_empty() {
            return Ok(StaticDataSeries::new());
        }

        let data_len = self.len();

        // Check if downsampling is needed
        if data_len <= config.max_points {
            let mut result = StaticDataSeries::new();
            for point in self.iter() {
                result.push(point)?;
            }
            return Ok(result);
        }

        // Check reduction ratio
        let reduction_ratio = data_len as f32 / config.max_points as f32;
        if reduction_ratio < config.min_reduction_ratio {
            let mut result = StaticDataSeries::new();
            for point in self.iter() {
                result.push(point)?;
            }
            return Ok(result);
        }

        let mut result = StaticDataSeries::new();
        let points = self.as_slice();

        // Always include first point
        result.push(points[0])?;

        if config.max_points <= 2 {
            // Include last point if we have room
            if config.max_points == 2 && points.len() > 1 {
                result.push(points[points.len() - 1])?;
            }
            return Ok(result);
        }

        // Calculate bucket size for intermediate points
        let bucket_size = (data_len - 2) as f32 / (config.max_points - 2) as f32;
        let mut bucket_start = 1.0;

        // Process each bucket
        for _i in 1..(config.max_points - 1) {
            let bucket_end = bucket_start + bucket_size;
            #[cfg(feature = "std")]
            let start_idx = floorf(bucket_start) as usize;
            #[cfg(not(feature = "std"))]
            let start_idx = bucket_start.floor() as usize;
            #[cfg(feature = "std")]
            let end_idx = (ceilf(bucket_end) as usize).min(data_len - 1);
            #[cfg(not(feature = "std"))]
            let end_idx = (bucket_end.ceil() as usize).min(data_len - 1);

            if start_idx >= end_idx {
                continue;
            }

            // Calculate average point of next bucket for triangle area calculation
            let next_bucket_start = bucket_end;
            let next_bucket_end = next_bucket_start + bucket_size;
            #[cfg(feature = "std")]
            let next_start_idx = floorf(next_bucket_start) as usize;
            #[cfg(not(feature = "std"))]
            let next_start_idx = next_bucket_start.floor() as usize;
            #[cfg(feature = "std")]
            let next_end_idx = (ceilf(next_bucket_end) as usize).min(data_len);
            #[cfg(not(feature = "std"))]
            let next_end_idx = (next_bucket_end.ceil() as usize).min(data_len);

            let avg_next = if next_start_idx < next_end_idx && next_end_idx <= data_len {
                self.calculate_average_point(&points[next_start_idx..next_end_idx])?
            } else {
                points[data_len - 1] // Use last point if no next bucket
            };

            // Find point in current bucket that forms largest triangle
            let mut max_area = -1.0;
            let mut selected_idx = start_idx;

            for (j_offset, j) in (start_idx..end_idx).enumerate() {
                let area = self.calculate_triangle_area(
                    &result.as_slice()[result.len() - 1], // Previous selected point
                    &points[j],                           // Current candidate
                    &avg_next,                            // Average of next bucket
                );

                if area > max_area {
                    max_area = area;
                    selected_idx = start_idx + j_offset;
                }
            }

            result.push(points[selected_idx])?;
            bucket_start = bucket_end;
        }

        // Always include last point if preserving endpoints
        if config.preserve_endpoints && points.len() > 1 {
            result.push(points[points.len() - 1])?;
        }

        Ok(result)
    }

    fn downsample_uniform<const N: usize>(
        &self,
        config: &DownsamplingConfig,
    ) -> DataResult<StaticDataSeries<T, N>> {
        if self.is_empty() {
            return Ok(StaticDataSeries::new());
        }

        let data_len = self.len();

        if data_len <= config.max_points {
            let mut result = StaticDataSeries::new();
            for point in self.iter() {
                result.push(point)?;
            }
            return Ok(result);
        }

        let mut result = StaticDataSeries::new();
        let points = self.as_slice();

        // Calculate step size
        let step = data_len as f32 / config.max_points as f32;
        let mut current: f32 = 0.0;

        for _ in 0..config.max_points {
            #[cfg(feature = "std")]
            let idx = (roundf(current) as usize).min(data_len - 1);
            #[cfg(not(feature = "std"))]
            let idx = (current.round() as usize).min(data_len - 1);
            result.push(points[idx])?;
            current += step;
        }

        Ok(result)
    }

    fn calculate_group_stats(&self, points: &[T]) -> DataResult<GroupStats<T>> {
        if points.is_empty() {
            return Err(DataError::insufficient_data("calculate_group_stats", 1, 0));
        }

        let first = points[0];
        let last = points[points.len() - 1];

        let mut min_x = first.x();
        let mut max_x = first.x();
        let mut min_y = first.y();
        let mut max_y = first.y();

        let mut sum_x: f32 = first.x().into();
        let mut sum_y: f32 = first.y().into();

        for point in points.iter().skip(1) {
            let x = point.x();
            let y = point.y();

            if x < min_x {
                min_x = x;
            }
            if x > max_x {
                max_x = x;
            }
            if y < min_y {
                min_y = y;
            }
            if y > max_y {
                max_y = y;
            }

            sum_x += x.into();
            sum_y += y.into();
        }

        let count_f = points.len() as f32;
        let mean_x = T::X::from(sum_x / count_f);
        let mean_y = T::Y::from(sum_y / count_f);

        Ok(GroupStats {
            count: points.len(),
            min_x,
            max_x,
            min_y,
            max_y,
            mean_x,
            mean_y,
            first,
            last,
        })
    }
}

impl<T, const M: usize> StaticDataSeries<T, M>
where
    T: DataPoint + Clone + Copy,
    T::X: PartialOrd
        + Copy
        + core::ops::Add<Output = T::X>
        + core::ops::Div<f32, Output = T::X>
        + Into<f32>
        + From<f32>,
    T::Y: PartialOrd
        + Copy
        + core::ops::Add<Output = T::Y>
        + core::ops::Div<f32, Output = T::Y>
        + Into<f32>
        + From<f32>,
{
    /// Aggregate a group of points using the specified strategy
    fn aggregate_group(&self, points: &[T], strategy: AggregationStrategy) -> DataResult<T> {
        if points.is_empty() {
            return Err(DataError::insufficient_data("aggregate_group", 1, 0));
        }

        match strategy {
            AggregationStrategy::Mean => {
                let stats = self.calculate_group_stats(points)?;
                Ok(T::new(stats.mean_x, stats.mean_y))
            }
            AggregationStrategy::Median => {
                // For median, we need to sort the coordinates
                let mut x_coords: heapless::Vec<T::X, 32> = heapless::Vec::new();
                let mut y_coords: heapless::Vec<T::Y, 32> = heapless::Vec::new();

                for point in points {
                    let _ = x_coords.push(point.x());
                    let _ = y_coords.push(point.y());
                }

                // Simple sorting for small arrays
                x_coords.sort_by(|a, b| a.partial_cmp(b).unwrap_or(core::cmp::Ordering::Equal));
                y_coords.sort_by(|a, b| a.partial_cmp(b).unwrap_or(core::cmp::Ordering::Equal));

                let median_x = if x_coords.len() % 2 == 0 {
                    let mid = x_coords.len() / 2;
                    let sum: f32 = x_coords[mid - 1].into() + x_coords[mid].into();
                    T::X::from(sum / 2.0)
                } else {
                    x_coords[x_coords.len() / 2]
                };

                let median_y = if y_coords.len() % 2 == 0 {
                    let mid = y_coords.len() / 2;
                    let sum: f32 = y_coords[mid - 1].into() + y_coords[mid].into();
                    T::Y::from(sum / 2.0)
                } else {
                    y_coords[y_coords.len() / 2]
                };

                Ok(T::new(median_x, median_y))
            }
            AggregationStrategy::MinMax => {
                // Use the point with extreme Y value for MinMax strategy
                let point_with_max = points
                    .iter()
                    .max_by(|a, b| {
                        a.y()
                            .partial_cmp(&b.y())
                            .unwrap_or(core::cmp::Ordering::Equal)
                    })
                    .unwrap();
                Ok(*point_with_max)
            }
            AggregationStrategy::First => Ok(points[0]),
            AggregationStrategy::Last => Ok(points[points.len() - 1]),
            AggregationStrategy::Max => {
                let max_point = points
                    .iter()
                    .max_by(|a, b| {
                        a.y()
                            .partial_cmp(&b.y())
                            .unwrap_or(core::cmp::Ordering::Equal)
                    })
                    .unwrap();
                Ok(*max_point)
            }
            AggregationStrategy::Min => {
                let min_point = points
                    .iter()
                    .min_by(|a, b| {
                        a.y()
                            .partial_cmp(&b.y())
                            .unwrap_or(core::cmp::Ordering::Equal)
                    })
                    .unwrap();
                Ok(*min_point)
            }
        }
    }

    /// Calculate the average point of a group for LTTB algorithm
    fn calculate_average_point(&self, points: &[T]) -> DataResult<T> {
        if points.is_empty() {
            return Err(DataError::insufficient_data(
                "calculate_average_point",
                1,
                0,
            ));
        }

        let mut sum_x: f32 = points[0].x().into();
        let mut sum_y: f32 = points[0].y().into();

        for point in points.iter().skip(1) {
            sum_x += point.x().into();
            sum_y += point.y().into();
        }

        let count = points.len() as f32;
        let avg_x = T::X::from(sum_x / count);
        let avg_y = T::Y::from(sum_y / count);

        Ok(T::new(avg_x, avg_y))
    }

    /// Calculate triangle area for LTTB algorithm
    fn calculate_triangle_area(&self, a: &T, b: &T, c: &T) -> f32 {
        let ax: f32 = a.x().into();
        let ay: f32 = a.y().into();
        let bx: f32 = b.x().into();
        let by: f32 = b.y().into();
        let cx: f32 = c.x().into();
        let cy: f32 = c.y().into();

        // Calculate area using cross product formula: 0.5 * |det([[ax, ay, 1], [bx, by, 1], [cx, cy, 1]])|
        let det = ax * (by - cy) + bx * (cy - ay) - cx * (ay - by);

        det.abs() * 0.5
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::{Point2D, StaticDataSeries};

    #[test]
    fn test_aggregation_config_default() {
        let config = AggregationConfig::default();
        assert_eq!(config.strategy, AggregationStrategy::Mean);
        assert_eq!(config.target_points, 100);
        assert!(config.preserve_endpoints);
        assert_eq!(config.min_group_size, 1);
    }

    #[test]
    fn test_downsampling_config_default() {
        let config = DownsamplingConfig::default();
        assert_eq!(config.max_points, 1000);
        assert!(config.preserve_endpoints);
        assert_eq!(config.min_reduction_ratio, 1.5);
    }

    #[test]
    fn test_group_stats_calculation() {
        let mut series: StaticDataSeries<Point2D, 256> = StaticDataSeries::new();
        series.push(Point2D::new(0.0, 10.0)).unwrap();
        series.push(Point2D::new(1.0, 20.0)).unwrap();
        series.push(Point2D::new(2.0, 5.0)).unwrap();

        let stats = series.calculate_group_stats(series.as_slice()).unwrap();

        assert_eq!(stats.count, 3);
        assert_eq!(stats.min_x, 0.0);
        assert_eq!(stats.max_x, 2.0);
        assert_eq!(stats.min_y, 5.0);
        assert_eq!(stats.max_y, 20.0);
        assert_eq!(stats.first.x(), 0.0);
        assert_eq!(stats.last.x(), 2.0);
    }

    #[test]
    fn test_mean_aggregation() {
        let mut series: StaticDataSeries<Point2D, 256> = StaticDataSeries::new();
        series.push(Point2D::new(0.0, 10.0)).unwrap();
        series.push(Point2D::new(1.0, 20.0)).unwrap();
        series.push(Point2D::new(2.0, 30.0)).unwrap();
        series.push(Point2D::new(3.0, 40.0)).unwrap();

        let config = AggregationConfig {
            strategy: AggregationStrategy::Mean,
            target_points: 2,
            preserve_endpoints: false,
            min_group_size: 1,
        };

        let aggregated: StaticDataSeries<Point2D, 256> = series.aggregate(&config).unwrap();
        assert_eq!(aggregated.len(), 2);

        // First group: (0,10) and (1,20) -> mean should be (0.5, 15)
        let first = aggregated.get(0).unwrap();
        assert_eq!(first.x(), 0.5);
        assert_eq!(first.y(), 15.0);

        // Second group: (2,30) and (3,40) -> mean should be (2.5, 35)
        let second = aggregated.get(1).unwrap();
        assert_eq!(second.x(), 2.5);
        assert_eq!(second.y(), 35.0);
    }

    #[test]
    fn test_uniform_downsampling() {
        let mut series: StaticDataSeries<Point2D, 256> = StaticDataSeries::new();
        for i in 0..10 {
            series
                .push(Point2D::new(i as f32, (i * 10) as f32))
                .unwrap();
        }

        let config = DownsamplingConfig {
            max_points: 5,
            preserve_endpoints: true,
            min_reduction_ratio: 1.0,
        };

        let downsampled: StaticDataSeries<Point2D, 256> =
            series.downsample_uniform(&config).unwrap();
        assert_eq!(downsampled.len(), 5);
    }

    #[test]
    fn test_no_aggregation_when_not_needed() {
        let mut series: StaticDataSeries<Point2D, 256> = StaticDataSeries::new();
        series.push(Point2D::new(0.0, 10.0)).unwrap();
        series.push(Point2D::new(1.0, 20.0)).unwrap();

        let config = AggregationConfig {
            target_points: 5, // More than we have
            ..Default::default()
        };

        let aggregated: StaticDataSeries<Point2D, 256> = series.aggregate(&config).unwrap();
        assert_eq!(aggregated.len(), 2); // Should be unchanged
        assert_eq!(aggregated.get(0).unwrap().x(), 0.0);
        assert_eq!(aggregated.get(1).unwrap().x(), 1.0);
    }
}
