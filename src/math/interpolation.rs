//! Curve interpolation algorithms for smooth data visualization.
//!
//! This module provides various interpolation algorithms optimized for embedded systems:
//! - Cubic spline interpolation for smooth curves
//! - Catmull-Rom spline for balanced smoothness and control
//! - Bezier curve interpolation for artistic control
//! - Linear interpolation as a fallback
//!
//! All algorithms are designed to work with the no_std environment and use
//! static allocation for memory efficiency.

use crate::data::Point2D;
use crate::error::{ChartError, ChartResult};
use heapless::Vec;

/// Maximum number of interpolated points that can be generated
pub const MAX_INTERPOLATED_POINTS: usize = 512;

/// Type of curve interpolation to use
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterpolationType {
    /// Linear interpolation - straight lines between points
    Linear,
    /// Cubic spline interpolation - smooth curves through all points
    CubicSpline,
    /// Catmull-Rom spline - smooth curves with local control
    CatmullRom,
    /// Bezier curve approximation - artistic smooth curves
    Bezier,
}

/// Configuration for curve interpolation
#[derive(Debug, Clone)]
pub struct InterpolationConfig {
    /// Type of interpolation to use
    pub interpolation_type: InterpolationType,
    /// Number of points to generate between each pair of data points
    pub subdivisions: u32,
    /// Tension parameter for splines (0.0 = loose, 1.0 = tight)
    pub tension: f32,
    /// Whether to create a closed curve (connect last point to first)
    pub closed: bool,
}

impl Default for InterpolationConfig {
    fn default() -> Self {
        Self {
            interpolation_type: InterpolationType::CubicSpline,
            subdivisions: 8,
            tension: 0.5,
            closed: false,
        }
    }
}

/// Curve interpolator that generates smooth curves from discrete data points
pub struct CurveInterpolator;

impl CurveInterpolator {
    /// Interpolate a series of 2D points to create a smooth curve
    ///
    /// # Arguments
    /// * `points` - Input data points to interpolate
    /// * `config` - Interpolation configuration
    ///
    /// # Returns
    /// A vector of interpolated points that form a smooth curve
    pub fn interpolate(
        points: &[Point2D],
        config: &InterpolationConfig,
    ) -> ChartResult<Vec<Point2D, MAX_INTERPOLATED_POINTS>> {
        if points.len() < 2 {
            return Err(ChartError::InsufficientData);
        }

        match config.interpolation_type {
            InterpolationType::Linear => Self::linear_interpolation(points, config),
            InterpolationType::CubicSpline => Self::cubic_spline_interpolation(points, config),
            InterpolationType::CatmullRom => Self::catmull_rom_interpolation(points, config),
            InterpolationType::Bezier => Self::bezier_interpolation(points, config),
        }
    }

    /// Linear interpolation - simply subdivides straight lines
    fn linear_interpolation(
        points: &[Point2D],
        config: &InterpolationConfig,
    ) -> ChartResult<Vec<Point2D, MAX_INTERPOLATED_POINTS>> {
        let mut result = Vec::new();

        for i in 0..points.len() - 1 {
            let p0 = points[i];
            let p1 = points[i + 1];

            // Add the starting point
            result.push(p0).map_err(|_| ChartError::MemoryFull)?;

            // Add subdivided points
            for j in 1..config.subdivisions {
                let t = j as f32 / config.subdivisions as f32;
                let x = p0.x + t * (p1.x - p0.x);
                let y = p0.y + t * (p1.y - p0.y);
                result
                    .push(Point2D::new(x, y))
                    .map_err(|_| ChartError::MemoryFull)?;
            }
        }

        // Add the final point
        if let Some(last) = points.last() {
            result.push(*last).map_err(|_| ChartError::MemoryFull)?;
        }

        Ok(result)
    }

    /// Cubic spline interpolation for smooth curves
    fn cubic_spline_interpolation(
        points: &[Point2D],
        config: &InterpolationConfig,
    ) -> ChartResult<Vec<Point2D, MAX_INTERPOLATED_POINTS>> {
        let mut result = Vec::new();
        let n = points.len();

        if n < 3 {
            return Self::linear_interpolation(points, config);
        }

        // Calculate derivatives for natural cubic spline
        let mut derivatives = Vec::<f32, 256>::new();
        for _i in 0..n {
            derivatives.push(0.0).map_err(|_| ChartError::MemoryFull)?;
        }

        // Calculate second derivatives using simplified approach
        for i in 1..n - 1 {
            let h1 = points[i].x - points[i - 1].x;
            let h2 = points[i + 1].x - points[i].x;
            let delta1 = (points[i].y - points[i - 1].y) / h1;
            let delta2 = (points[i + 1].y - points[i].y) / h2;
            derivatives[i] = 2.0 * (delta2 - delta1) / (h1 + h2);
        }

        // Generate interpolated points
        for i in 0..n - 1 {
            let p0 = points[i];
            let p1 = points[i + 1];
            let d0 = derivatives[i];
            let d1 = derivatives[i + 1];

            result.push(p0).map_err(|_| ChartError::MemoryFull)?;

            let h = p1.x - p0.x;
            for j in 1..config.subdivisions {
                let t = j as f32 / config.subdivisions as f32;
                let t2 = t * t;
                let t3 = t2 * t;

                // Cubic Hermite interpolation
                let h00 = 2.0 * t3 - 3.0 * t2 + 1.0;
                let h10 = t3 - 2.0 * t2 + t;
                let h01 = -2.0 * t3 + 3.0 * t2;
                let h11 = t3 - t2;

                let x = p0.x + t * h;
                let y = h00 * p0.y + h10 * h * d0 + h01 * p1.y + h11 * h * d1;

                result
                    .push(Point2D::new(x, y))
                    .map_err(|_| ChartError::MemoryFull)?;
            }
        }

        if let Some(last) = points.last() {
            result.push(*last).map_err(|_| ChartError::MemoryFull)?;
        }

        Ok(result)
    }

    /// Catmull-Rom spline interpolation
    fn catmull_rom_interpolation(
        points: &[Point2D],
        config: &InterpolationConfig,
    ) -> ChartResult<Vec<Point2D, MAX_INTERPOLATED_POINTS>> {
        let mut result = Vec::new();
        let n = points.len();

        if n < 3 {
            return Self::linear_interpolation(points, config);
        }

        // Process each segment
        for i in 0..n - 1 {
            // Get control points (with boundary handling)
            let p0 = if i == 0 { points[0] } else { points[i - 1] };
            let p1 = points[i];
            let p2 = points[i + 1];
            let p3 = if i + 2 < n {
                points[i + 2]
            } else {
                points[n - 1]
            };

            result.push(p1).map_err(|_| ChartError::MemoryFull)?;

            // Generate subdivided points
            for j in 1..config.subdivisions {
                let t = j as f32 / config.subdivisions as f32;
                let t2 = t * t;
                let t3 = t2 * t;

                // Catmull-Rom basis functions
                let x = 0.5
                    * ((2.0 * p1.x)
                        + (-p0.x + p2.x) * t
                        + (2.0 * p0.x - 5.0 * p1.x + 4.0 * p2.x - p3.x) * t2
                        + (-p0.x + 3.0 * p1.x - 3.0 * p2.x + p3.x) * t3);

                let y = 0.5
                    * ((2.0 * p1.y)
                        + (-p0.y + p2.y) * t
                        + (2.0 * p0.y - 5.0 * p1.y + 4.0 * p2.y - p3.y) * t2
                        + (-p0.y + 3.0 * p1.y - 3.0 * p2.y + p3.y) * t3);

                result
                    .push(Point2D::new(x, y))
                    .map_err(|_| ChartError::MemoryFull)?;
            }
        }

        if let Some(last) = points.last() {
            result.push(*last).map_err(|_| ChartError::MemoryFull)?;
        }

        Ok(result)
    }

    /// Bezier curve interpolation
    fn bezier_interpolation(
        points: &[Point2D],
        config: &InterpolationConfig,
    ) -> ChartResult<Vec<Point2D, MAX_INTERPOLATED_POINTS>> {
        let mut result = Vec::new();
        let n = points.len();

        if n < 3 {
            return Self::linear_interpolation(points, config);
        }

        // Generate control points for quadratic Bezier curves
        for i in 0..n - 1 {
            let p0 = points[i];
            let p2 = points[i + 1];

            // Simple control point calculation
            let mid_x = (p0.x + p2.x) * 0.5;
            let mid_y = (p0.y + p2.y) * 0.5;

            // Add some curvature based on tension
            let offset_x = (p2.y - p0.y) * config.tension * 0.2;
            let offset_y = (p0.x - p2.x) * config.tension * 0.2;

            let p1 = Point2D::new(mid_x + offset_x, mid_y + offset_y);

            result.push(p0).map_err(|_| ChartError::MemoryFull)?;

            // Generate quadratic Bezier curve
            for j in 1..config.subdivisions {
                let t = j as f32 / config.subdivisions as f32;
                let one_minus_t = 1.0 - t;
                let t2 = t * t;
                let one_minus_t2 = one_minus_t * one_minus_t;

                let x = one_minus_t2 * p0.x + 2.0 * one_minus_t * t * p1.x + t2 * p2.x;
                let y = one_minus_t2 * p0.y + 2.0 * one_minus_t * t * p1.y + t2 * p2.y;

                result
                    .push(Point2D::new(x, y))
                    .map_err(|_| ChartError::MemoryFull)?;
            }
        }

        if let Some(last) = points.last() {
            result.push(*last).map_err(|_| ChartError::MemoryFull)?;
        }

        Ok(result)
    }

    /// Smooth a single point using neighboring points
    pub fn smooth_point(
        points: &[Point2D],
        index: usize,
        smoothing_factor: f32,
    ) -> ChartResult<Point2D> {
        if index >= points.len() {
            return Err(ChartError::InvalidRange);
        }

        let n = points.len();
        if n < 3 || index == 0 || index == n - 1 {
            return Ok(points[index]);
        }

        let prev = points[index - 1];
        let curr = points[index];
        let next = points[index + 1];

        // Apply smoothing using weighted average
        let factor = smoothing_factor.clamp(0.0, 1.0);
        let smoothed_x = curr.x * (1.0 - factor) + (prev.x + next.x) * 0.5 * factor;
        let smoothed_y = curr.y * (1.0 - factor) + (prev.y + next.y) * 0.5 * factor;

        Ok(Point2D::new(smoothed_x, smoothed_y))
    }

    /// Apply smoothing to an entire series of points
    pub fn smooth_series(
        points: &[Point2D],
        smoothing_factor: f32,
        iterations: u32,
    ) -> ChartResult<Vec<Point2D, 256>> {
        let mut working_points = Vec::new();
        for point in points {
            working_points
                .push(*point)
                .map_err(|_| ChartError::MemoryFull)?;
        }

        for _ in 0..iterations {
            let mut smoothed = Vec::new();
            for i in 0..working_points.len() {
                let smoothed_point = Self::smooth_point(&working_points, i, smoothing_factor)?;
                smoothed
                    .push(smoothed_point)
                    .map_err(|_| ChartError::MemoryFull)?;
            }
            working_points = smoothed;
        }

        Ok(working_points)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_interpolation() {
        let mut points = heapless::Vec::<Point2D, 16>::new();
        points.push(Point2D::new(0.0, 0.0)).unwrap();
        points.push(Point2D::new(1.0, 1.0)).unwrap();
        let config = InterpolationConfig {
            interpolation_type: InterpolationType::Linear,
            subdivisions: 4,
            ..Default::default()
        };

        let result = CurveInterpolator::interpolate(&points, &config).unwrap();
        assert!(result.len() > 2);
        assert_eq!(result[0], points[0]);
        assert_eq!(result[result.len() - 1], points[1]);
    }

    #[test]
    fn test_catmull_rom_interpolation() {
        let mut points = heapless::Vec::<Point2D, 16>::new();
        points.push(Point2D::new(0.0, 0.0)).unwrap();
        points.push(Point2D::new(1.0, 1.0)).unwrap();
        points.push(Point2D::new(2.0, 0.0)).unwrap();
        let config = InterpolationConfig {
            interpolation_type: InterpolationType::CatmullRom,
            subdivisions: 8,
            ..Default::default()
        };

        let result = CurveInterpolator::interpolate(&points, &config).unwrap();
        assert!(result.len() > points.len());
    }

    #[test]
    fn test_point_smoothing() {
        let mut points = heapless::Vec::<Point2D, 16>::new();
        points.push(Point2D::new(0.0, 0.0)).unwrap();
        points.push(Point2D::new(1.0, 10.0)).unwrap(); // Spike
        points.push(Point2D::new(2.0, 0.0)).unwrap();

        let smoothed = CurveInterpolator::smooth_point(&points, 1, 0.5).unwrap();
        assert!(smoothed.y < 10.0); // Should be smoothed down
        assert!(smoothed.y > 0.0); // But still positive
    }

    #[test]
    fn test_series_smoothing() {
        let mut points = heapless::Vec::<Point2D, 16>::new();
        points.push(Point2D::new(0.0, 0.0)).unwrap();
        points.push(Point2D::new(1.0, 10.0)).unwrap();
        points.push(Point2D::new(2.0, 0.0)).unwrap();
        points.push(Point2D::new(3.0, 10.0)).unwrap();
        points.push(Point2D::new(4.0, 0.0)).unwrap();

        let smoothed = CurveInterpolator::smooth_series(&points, 0.3, 2).unwrap();
        assert_eq!(smoothed.len(), points.len());

        // Check that spikes are reduced
        assert!(smoothed[1].y < points[1].y);
        assert!(smoothed[3].y < points[3].y);
    }
}
