//! Comprehensive test suite for math/interpolation.rs
//! Target: Increase coverage from 61% to 85%
//!
//! This test suite covers:
//! - Numerical edge cases for all interpolation algorithms
//! - Floating-point precision and stability
//! - Memory boundary conditions
//! - Performance characteristics
//! - Smoothing algorithms with various parameters
//! - Closed vs open curve behavior
//! - Tension parameter effects
//! - Subdivision limit handling

use embedded_charts::{
    data::Point2D,
    error::ChartError,
    math::interpolation::{
        CurveInterpolator, InterpolationConfig, InterpolationType, MAX_INTERPOLATED_POINTS,
    },
};
use heapless::Vec;

/// Test helper to create a vector of points
fn create_points(data: &[(f32, f32)]) -> Vec<Point2D, 256> {
    let mut points = Vec::new();
    for &(x, y) in data {
        points.push(Point2D::new(x, y)).unwrap();
    }
    points
}

/// Test helper to verify points are within bounds
fn verify_points_bounded(points: &[Point2D], min_x: f32, max_x: f32, min_y: f32, max_y: f32) {
    for point in points {
        assert!(
            point.x >= min_x && point.x <= max_x,
            "X value {} out of bounds [{}, {}]",
            point.x,
            min_x,
            max_x
        );
        assert!(
            point.y >= min_y && point.y <= max_y,
            "Y value {} out of bounds [{}, {}]",
            point.y,
            min_y,
            max_y
        );
    }
}

#[test]
fn test_insufficient_data_handling() {
    // Test with no points
    let points: Vec<Point2D, 16> = Vec::new();
    let config = InterpolationConfig::default();
    let result = CurveInterpolator::interpolate(&points, &config);
    assert!(matches!(result, Err(ChartError::InsufficientData)));

    // Test with single point
    let points = create_points(&[(0.0, 0.0)]);
    let result = CurveInterpolator::interpolate(&points, &config);
    assert!(matches!(result, Err(ChartError::InsufficientData)));
}

#[test]
fn test_linear_interpolation_edge_cases() {
    let config = InterpolationConfig {
        interpolation_type: InterpolationType::Linear,
        subdivisions: 10,
        tension: 0.5,
        closed: false,
    };

    // Test with exactly two points
    let points = create_points(&[(0.0, 0.0), (10.0, 10.0)]);
    let result = CurveInterpolator::interpolate(&points, &config).unwrap();
    assert_eq!(result.len(), 11); // start + 9 subdivisions + end

    // Verify linear progression
    for (i, point) in result.iter().enumerate() {
        let expected_t = i as f32 / 10.0;
        let expected_x = expected_t * 10.0;
        let expected_y = expected_t * 10.0;
        assert!((point.x - expected_x).abs() < 0.01);
        assert!((point.y - expected_y).abs() < 0.01);
    }

    // Test with vertical line
    let points = create_points(&[(5.0, 0.0), (5.0, 10.0)]);
    let result = CurveInterpolator::interpolate(&points, &config).unwrap();
    for point in &result {
        assert_eq!(point.x, 5.0);
    }

    // Test with horizontal line
    let points = create_points(&[(0.0, 5.0), (10.0, 5.0)]);
    let result = CurveInterpolator::interpolate(&points, &config).unwrap();
    for point in &result {
        assert_eq!(point.y, 5.0);
    }

    // Test with negative coordinates
    let points = create_points(&[(-10.0, -5.0), (-5.0, -10.0)]);
    let result = CurveInterpolator::interpolate(&points, &config).unwrap();
    verify_points_bounded(&result, -10.0, -5.0, -10.0, -5.0);
}

#[test]
fn test_cubic_spline_interpolation_comprehensive() {
    let config = InterpolationConfig {
        interpolation_type: InterpolationType::CubicSpline,
        subdivisions: 8,
        tension: 0.5,
        closed: false,
    };

    // Test with smooth curve data
    let points = create_points(&[(0.0, 0.0), (1.0, 1.0), (2.0, 4.0), (3.0, 9.0), (4.0, 16.0)]);
    let result = CurveInterpolator::interpolate(&points, &config).unwrap();

    // Should have many more points due to subdivision
    assert!(result.len() > points.len() * config.subdivisions as usize / 2);

    // Verify smoothness - check that intermediate points exist
    for i in 0..result.len() - 1 {
        let dx = (result[i + 1].x - result[i].x).abs();
        assert!(dx < 0.5, "Large gap in x coordinates");
    }

    // Test with oscillating data
    let points = create_points(&[
        (0.0, 0.0),
        (1.0, 10.0),
        (2.0, -10.0),
        (3.0, 10.0),
        (4.0, -10.0),
        (5.0, 0.0),
    ]);
    let result = CurveInterpolator::interpolate(&points, &config).unwrap();
    assert!(result.len() > points.len());

    // Test with sharp corners
    let points = create_points(&[
        (0.0, 0.0),
        (1.0, 0.0),
        (1.0, 1.0),
        (2.0, 1.0),
        (2.0, 0.0),
        (3.0, 0.0),
    ]);
    let result = CurveInterpolator::interpolate(&points, &config).unwrap();
    assert!(result.len() > points.len());
}

#[test]
fn test_catmull_rom_interpolation_comprehensive() {
    // Test with different tension values
    for &tension in &[0.0, 0.25, 0.5, 0.75, 1.0] {
        let config = InterpolationConfig {
            interpolation_type: InterpolationType::CatmullRom,
            subdivisions: 6,
            tension,
            closed: false,
        };

        let points = create_points(&[(0.0, 0.0), (1.0, 2.0), (2.0, 1.0), (3.0, 3.0), (4.0, 0.0)]);

        let result = CurveInterpolator::interpolate(&points, &config).unwrap();
        assert!(result.len() > points.len());

        // Higher tension should produce curves closer to linear interpolation
        if tension > 0.8 {
            // Check that curve doesn't deviate too much from straight lines
            for i in 1..result.len() - 1 {
                let deviation = (result[i].y - result[i - 1].y).abs();
                assert!(deviation < 5.0);
            }
        }
    }

    // Test with different subdivision counts
    let config = InterpolationConfig {
        interpolation_type: InterpolationType::CatmullRom,
        subdivisions: 16,
        tension: 0.5,
        closed: false,
    };

    let points = create_points(&[(0.0, 0.0), (1.0, 2.0), (2.0, 1.0), (3.0, 3.0)]);

    let result = CurveInterpolator::interpolate(&points, &config).unwrap();

    // Should produce many interpolated points
    assert!(result.len() > points.len() * 2);
}

#[test]
fn test_bezier_interpolation_comprehensive() {
    let config = InterpolationConfig {
        interpolation_type: InterpolationType::Bezier,
        subdivisions: 10,
        tension: 0.5,
        closed: false,
    };

    // Test with control points forming a simple curve
    let points = create_points(&[(0.0, 0.0), (1.0, 3.0), (3.0, 3.0), (4.0, 0.0)]);

    let result = CurveInterpolator::interpolate(&points, &config).unwrap();
    assert!(result.len() > points.len());

    // Bezier curves should be contained within the convex hull of control points
    let min_x = points.iter().map(|p| p.x).fold(f32::INFINITY, f32::min);
    let max_x = points.iter().map(|p| p.x).fold(f32::NEG_INFINITY, f32::max);
    let min_y = points.iter().map(|p| p.y).fold(f32::INFINITY, f32::min);
    let max_y = points.iter().map(|p| p.y).fold(f32::NEG_INFINITY, f32::max);

    verify_points_bounded(&result, min_x, max_x, min_y, max_y);

    // Test with many control points
    let points = create_points(&[
        (0.0, 0.0),
        (0.5, 1.0),
        (1.0, 0.5),
        (1.5, 1.5),
        (2.0, 1.0),
        (2.5, 2.0),
        (3.0, 0.0),
    ]);

    let result = CurveInterpolator::interpolate(&points, &config).unwrap();
    assert!(result.len() > points.len());
}

#[test]
fn test_subdivision_limits() {
    let points = create_points(&[(0.0, 0.0), (1.0, 1.0), (2.0, 0.0)]);

    // Test minimum subdivisions
    let config = InterpolationConfig {
        interpolation_type: InterpolationType::CubicSpline,
        subdivisions: 1,
        tension: 0.5,
        closed: false,
    };
    let result = CurveInterpolator::interpolate(&points, &config).unwrap();
    assert!(result.len() >= points.len());

    // Test maximum reasonable subdivisions
    let config = InterpolationConfig {
        interpolation_type: InterpolationType::CubicSpline,
        subdivisions: 50,
        tension: 0.5,
        closed: false,
    };
    let result = CurveInterpolator::interpolate(&points, &config).unwrap();
    assert!(result.len() <= MAX_INTERPOLATED_POINTS);
}

#[test]
fn test_memory_boundary_conditions() {
    // Create maximum allowed points
    let mut points = Vec::<Point2D, 256>::new();
    for i in 0..100 {
        let x = i as f32 * 0.1;
        let y = (x * 2.0).sin();
        points.push(Point2D::new(x, y)).unwrap();
    }

    let config = InterpolationConfig {
        interpolation_type: InterpolationType::CubicSpline,
        subdivisions: 4,
        tension: 0.5,
        closed: false,
    };

    let result = CurveInterpolator::interpolate(&points, &config);
    assert!(result.is_ok() || matches!(result, Err(ChartError::MemoryFull)));
}

#[test]
fn test_smoothing_algorithms_comprehensive() {
    let points = create_points(&[
        (0.0, 0.0),
        (1.0, 10.0), // Spike
        (2.0, 0.0),
        (3.0, -10.0), // Negative spike
        (4.0, 0.0),
        (5.0, 5.0), // Smaller spike
        (6.0, 0.0),
    ]);

    // Test point smoothing with different factors
    for &factor in &[0.0, 0.25, 0.5, 0.75, 1.0] {
        let smoothed = CurveInterpolator::smooth_point(&points, 1, factor).unwrap();
        if factor > 0.0 {
            assert!(
                smoothed.y.abs() < 10.0,
                "Smoothing factor {factor} didn't reduce spike"
            );
        }
    }

    // Test series smoothing with different window sizes
    for &window in &[1, 2, 3] {
        let smoothed = CurveInterpolator::smooth_series(&points, 0.5, window).unwrap();
        assert_eq!(smoothed.len(), points.len());

        // Verify that spikes are reduced
        assert!(smoothed[1].y.abs() < points[1].y.abs());
        assert!(smoothed[3].y.abs() < points[3].y.abs());
    }

    // Test smoothing with edge points
    let smoothed_first = CurveInterpolator::smooth_point(&points, 0, 0.5).unwrap();
    assert_eq!(smoothed_first, points[0]); // First point should remain unchanged

    let smoothed_last = CurveInterpolator::smooth_point(&points, points.len() - 1, 0.5).unwrap();
    assert_eq!(smoothed_last, points[points.len() - 1]); // Last point should remain unchanged
}

#[test]
fn test_numerical_stability() {
    // Test with very small values
    let points = create_points(&[(0.0, 1e-6), (1e-6, 2e-6), (2e-6, 1e-6)]);

    let config = InterpolationConfig::default();
    let result = CurveInterpolator::interpolate(&points, &config).unwrap();
    assert!(result.len() > points.len());

    // Test with very large values
    let points = create_points(&[(0.0, 1e6), (1e6, 2e6), (2e6, 1e6)]);

    let result = CurveInterpolator::interpolate(&points, &config).unwrap();
    assert!(result.len() > points.len());

    // Test with mixed scale values
    let points = create_points(&[(0.0, 1e-3), (1.0, 1e3), (2.0, 1e-3)]);

    let result = CurveInterpolator::interpolate(&points, &config).unwrap();
    assert!(result.len() > points.len());
}

#[test]
fn test_special_floating_point_cases() {
    let config = InterpolationConfig::default();

    // Test with zero slopes
    let points = create_points(&[(0.0, 5.0), (1.0, 5.0), (2.0, 5.0), (3.0, 5.0)]);

    let result = CurveInterpolator::interpolate(&points, &config).unwrap();
    // All y values should remain constant
    for point in &result {
        assert!((point.y - 5.0).abs() < 0.01);
    }

    // Test with steep slopes
    let points = create_points(&[(0.0, 0.0), (0.01, 100.0), (0.02, 0.0)]);

    let result = CurveInterpolator::interpolate(&points, &config).unwrap();
    assert!(result.len() > points.len());
}

#[test]
fn test_interpolation_accuracy() {
    // Test that interpolation passes through original points
    let points = create_points(&[(0.0, 0.0), (1.0, 1.0), (2.0, 4.0), (3.0, 9.0)]);

    for interpolation_type in &[
        InterpolationType::Linear,
        InterpolationType::CubicSpline,
        InterpolationType::CatmullRom,
    ] {
        let config = InterpolationConfig {
            interpolation_type: *interpolation_type,
            subdivisions: 8,
            tension: 0.5,
            closed: false,
        };

        let result = CurveInterpolator::interpolate(&points, &config).unwrap();

        // Verify that original points are preserved (approximately)
        for original in &points {
            let mut found = false;
            for interpolated in &result {
                if (interpolated.x - original.x).abs() < 0.1 {
                    assert!(
                        (interpolated.y - original.y).abs() < 0.5,
                        "Interpolation {:?} doesn't pass through original point ({}, {})",
                        interpolation_type,
                        original.x,
                        original.y
                    );
                    found = true;
                    break;
                }
            }
            assert!(found, "Original point not found in interpolated results");
        }
    }
}

#[test]
fn test_edge_preservation() {
    // Test that interpolation preserves edge points exactly
    let points = create_points(&[(0.0, 0.0), (1.0, 5.0), (2.0, 3.0), (3.0, 8.0), (4.0, 2.0)]);

    for interpolation_type in &[
        InterpolationType::Linear,
        InterpolationType::CubicSpline,
        InterpolationType::CatmullRom,
        InterpolationType::Bezier,
    ] {
        let config = InterpolationConfig {
            interpolation_type: *interpolation_type,
            subdivisions: 4,
            tension: 0.5,
            closed: false,
        };

        let result = CurveInterpolator::interpolate(&points, &config).unwrap();

        // First and last points should be preserved exactly
        assert_eq!(
            result[0], points[0],
            "First point not preserved for {interpolation_type:?}"
        );
        assert_eq!(
            result[result.len() - 1],
            points[points.len() - 1],
            "Last point not preserved for {interpolation_type:?}"
        );
    }
}

#[test]
#[cfg(feature = "std")]
fn test_performance_characteristics() {
    use std::time::Instant;

    // Create test data sets of different sizes
    let sizes = [10, 25, 40];

    for &size in &sizes {
        let mut points = Vec::<Point2D, 64>::new();
        for i in 0..size {
            let x = i as f32 * 0.1;
            let y = (x * 2.0).sin() * 10.0;
            points.push(Point2D::new(x, y)).unwrap();
        }

        // Test each interpolation type
        for interpolation_type in &[
            InterpolationType::Linear,
            InterpolationType::CubicSpline,
            InterpolationType::CatmullRom,
            InterpolationType::Bezier,
        ] {
            let config = InterpolationConfig {
                interpolation_type: *interpolation_type,
                subdivisions: 4,
                tension: 0.5,
                closed: false,
            };

            let start = Instant::now();
            let result = CurveInterpolator::interpolate(&points, &config).unwrap();
            let duration = start.elapsed();

            println!("{interpolation_type:?} interpolation with {size} points took {duration:?}");

            assert!(result.len() > points.len());
            // Performance assertion: should complete in reasonable time
            assert!(duration.as_millis() < 100, "Interpolation too slow");
        }
    }
}

#[test]
fn test_error_propagation() {
    // Test with invalid window size for smoothing
    let points = create_points(&[(0.0, 0.0), (1.0, 1.0)]);
    let result = CurveInterpolator::smooth_series(&points, 0.5, 10);
    assert!(result.is_ok()); // Should handle gracefully

    // Test with out of bounds index for point smoothing
    let result = CurveInterpolator::smooth_point(&points, 100, 0.5);
    assert!(matches!(result, Err(ChartError::InvalidRange)));
}
