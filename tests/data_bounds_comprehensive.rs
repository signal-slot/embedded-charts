//! Comprehensive tests for data bounds functionality
//! Target: Increase coverage from 61.82% to 85%

use embedded_charts::data::{
    bounds::{calculate_bounds, calculate_multi_series_bounds, DataBounds, FloatBounds, IntBounds},
    point::{Point2D, IntPoint},
};
use embedded_charts::error::DataError;
use heapless::Vec;

#[test]
fn test_data_bounds_creation() {
    // Valid bounds
    let bounds = DataBounds::new(0.0f32, 10.0, -5.0, 15.0).unwrap();
    assert_eq!(bounds.min_x, 0.0);
    assert_eq!(bounds.max_x, 10.0);
    assert_eq!(bounds.min_y, -5.0);
    assert_eq!(bounds.max_y, 15.0);

    // Invalid bounds - min > max
    let result = DataBounds::new(10.0f32, 0.0, 0.0, 20.0);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), DataError::INVALID_DATA_POINT);

    let result = DataBounds::new(0.0f32, 10.0, 20.0, 0.0);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), DataError::INVALID_DATA_POINT);

    // Equal bounds (valid)
    let bounds = DataBounds::new(5.0f32, 5.0, 10.0, 10.0).unwrap();
    assert_eq!(bounds.min_x, 5.0);
    assert_eq!(bounds.max_x, 5.0);
}

#[test]
fn test_data_bounds_width_height() {
    let bounds = DataBounds::new(2.0f32, 8.0, 3.0, 15.0).unwrap();
    assert_eq!(bounds.width(), 6.0);
    assert_eq!(bounds.height(), 12.0);

    // Zero width/height
    let bounds = DataBounds::new(5.0f32, 5.0, 10.0, 10.0).unwrap();
    assert_eq!(bounds.width(), 0.0);
    assert_eq!(bounds.height(), 0.0);

    // Negative values
    let bounds = DataBounds::new(-10.0f32, -2.0, -20.0, -5.0).unwrap();
    assert_eq!(bounds.width(), 8.0);
    assert_eq!(bounds.height(), 15.0);
}

#[test]
fn test_data_bounds_contains() {
    let bounds = DataBounds::new(-5.0f32, 5.0, -10.0, 10.0).unwrap();

    // Points inside bounds
    assert!(bounds.contains(&Point2D::new(0.0, 0.0)));
    assert!(bounds.contains(&Point2D::new(-5.0, -10.0))); // On min boundary
    assert!(bounds.contains(&Point2D::new(5.0, 10.0))); // On max boundary
    assert!(bounds.contains(&Point2D::new(-4.9, -9.9)));
    assert!(bounds.contains(&Point2D::new(4.9, 9.9)));

    // Points outside bounds
    assert!(!bounds.contains(&Point2D::new(-5.1, 0.0)));
    assert!(!bounds.contains(&Point2D::new(5.1, 0.0)));
    assert!(!bounds.contains(&Point2D::new(0.0, -10.1)));
    assert!(!bounds.contains(&Point2D::new(0.0, 10.1)));
    assert!(!bounds.contains(&Point2D::new(10.0, 20.0)));
    assert!(!bounds.contains(&Point2D::new(-10.0, -20.0)));
}

#[test]
fn test_data_bounds_expand_to_include() {
    let mut bounds = DataBounds::new(0.0f32, 5.0, 0.0, 5.0).unwrap();

    // Expand in all directions
    bounds.expand_to_include(&Point2D::new(-2.0, 3.0));
    assert_eq!(bounds.min_x, -2.0);
    assert_eq!(bounds.max_x, 5.0);

    bounds.expand_to_include(&Point2D::new(10.0, 3.0));
    assert_eq!(bounds.max_x, 10.0);

    bounds.expand_to_include(&Point2D::new(3.0, -5.0));
    assert_eq!(bounds.min_y, -5.0);

    bounds.expand_to_include(&Point2D::new(3.0, 15.0));
    assert_eq!(bounds.max_y, 15.0);

    // Point already inside - bounds shouldn't change
    let before = bounds;
    bounds.expand_to_include(&Point2D::new(0.0, 0.0));
    assert_eq!(bounds, before);
}

#[test]
fn test_data_bounds_merge() {
    let bounds1 = DataBounds::new(-5.0f32, 5.0, -10.0, 10.0).unwrap();
    let bounds2 = DataBounds::new(0.0f32, 10.0, -15.0, 5.0).unwrap();

    let merged = bounds1.merge(&bounds2);
    assert_eq!(merged.min_x, -5.0);
    assert_eq!(merged.max_x, 10.0);
    assert_eq!(merged.min_y, -15.0);
    assert_eq!(merged.max_y, 10.0);

    // Merge with self
    let self_merged = bounds1.merge(&bounds1);
    assert_eq!(self_merged, bounds1);

    // Merge disjoint bounds
    let bounds3 = DataBounds::new(20.0f32, 30.0, 20.0, 30.0).unwrap();
    let merged2 = bounds1.merge(&bounds3);
    assert_eq!(merged2.min_x, -5.0);
    assert_eq!(merged2.max_x, 30.0);
    assert_eq!(merged2.min_y, -10.0);
    assert_eq!(merged2.max_y, 30.0);
}

#[test]
fn test_float_bounds_with_padding() {
    let bounds: FloatBounds = DataBounds::new(0.0, 10.0, 0.0, 20.0).unwrap();

    // 10% padding
    let padded = bounds.with_padding(10.0);
    assert_eq!(padded.min_x, -1.0);
    assert_eq!(padded.max_x, 11.0);
    assert_eq!(padded.min_y, -2.0);
    assert_eq!(padded.max_y, 22.0);

    // 0% padding
    let no_padding = bounds.with_padding(0.0);
    assert_eq!(no_padding, bounds);

    // Large padding
    let large_padding = bounds.with_padding(50.0);
    assert_eq!(large_padding.min_x, -5.0);
    assert_eq!(large_padding.max_x, 15.0);
    assert_eq!(large_padding.min_y, -10.0);
    assert_eq!(large_padding.max_y, 30.0);

    // Negative values with padding
    let neg_bounds: FloatBounds = DataBounds::new(-10.0, -5.0, -20.0, -10.0).unwrap();
    let padded_neg = neg_bounds.with_padding(20.0);
    assert_eq!(padded_neg.min_x, -11.0);
    assert_eq!(padded_neg.max_x, -4.0);
    assert_eq!(padded_neg.min_y, -22.0);
    assert_eq!(padded_neg.max_y, -8.0);
}

#[test]
fn test_float_bounds_nice_bounds() {
    // Simple case
    let bounds: FloatBounds = DataBounds::new(1.2, 8.7, 2.3, 17.8).unwrap();
    let nice = bounds.nice_bounds();
    
    // The nice bounds should have rounded ranges
    assert!(nice.width() >= bounds.width());
    assert!(nice.height() >= bounds.height());

    // Zero bounds
    let zero_bounds: FloatBounds = DataBounds::new(0.0, 0.0, 0.0, 0.0).unwrap();
    let nice_zero = zero_bounds.nice_bounds();
    assert_eq!(nice_zero.min_x, 0.0);
    assert_eq!(nice_zero.max_x, 0.0);
    assert_eq!(nice_zero.min_y, 0.0);
    assert_eq!(nice_zero.max_y, 0.0);

    // Very small range
    let small_bounds: FloatBounds = DataBounds::new(0.0001, 0.0003, 0.0, 0.0001).unwrap();
    let nice_small = small_bounds.nice_bounds();
    assert!(nice_small.width() > 0.0);
    assert!(nice_small.height() >= 0.0);

    // Large numbers
    let large_bounds: FloatBounds = DataBounds::new(1000.0, 2500.0, 5000.0, 15000.0).unwrap();
    let nice_large = large_bounds.nice_bounds();
    assert!(nice_large.width() >= large_bounds.width());
    assert!(nice_large.height() >= large_bounds.height());

    // Negative range
    let neg_bounds: FloatBounds = DataBounds::new(-100.0, -10.0, -50.0, -5.0).unwrap();
    let nice_neg = neg_bounds.nice_bounds();
    assert!(nice_neg.width() >= neg_bounds.width());
    assert!(nice_neg.height() >= neg_bounds.height());
}

#[test]
fn test_calculate_bounds_basic() {
    let mut points = Vec::<Point2D, 8>::new();
    points.push(Point2D::new(1.0, 2.0)).unwrap();
    points.push(Point2D::new(5.0, 8.0)).unwrap();
    points.push(Point2D::new(3.0, 4.0)).unwrap();
    points.push(Point2D::new(-1.0, 10.0)).unwrap();

    let bounds = calculate_bounds(points.into_iter()).unwrap();
    assert_eq!(bounds.min_x, -1.0);
    assert_eq!(bounds.max_x, 5.0);
    assert_eq!(bounds.min_y, 2.0);
    assert_eq!(bounds.max_y, 10.0);
}

#[test]
fn test_calculate_bounds_single_point() {
    let mut points = Vec::<Point2D, 1>::new();
    points.push(Point2D::new(5.0, 10.0)).unwrap();

    let bounds = calculate_bounds(points.into_iter()).unwrap();
    assert_eq!(bounds.min_x, 5.0);
    assert_eq!(bounds.max_x, 5.0);
    assert_eq!(bounds.min_y, 10.0);
    assert_eq!(bounds.max_y, 10.0);
}

#[test]
fn test_calculate_bounds_empty() {
    let points = Vec::<Point2D, 8>::new();
    let result = calculate_bounds(points.into_iter());
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), DataError::INSUFFICIENT_DATA);
}

#[test]
fn test_calculate_bounds_extreme_values() {
    let mut points = Vec::<Point2D, 4>::new();
    points.push(Point2D::new(f32::MIN, f32::MIN)).unwrap();
    points.push(Point2D::new(f32::MAX, f32::MAX)).unwrap();
    points.push(Point2D::new(0.0, 0.0)).unwrap();

    let bounds = calculate_bounds(points.into_iter()).unwrap();
    assert_eq!(bounds.min_x, f32::MIN);
    assert_eq!(bounds.max_x, f32::MAX);
    assert_eq!(bounds.min_y, f32::MIN);
    assert_eq!(bounds.max_y, f32::MAX);
}

#[test]
fn test_calculate_multi_series_bounds() {
    // Create multiple series
    let mut series1 = Vec::<Point2D, 4>::new();
    series1.push(Point2D::new(0.0, 0.0)).unwrap();
    series1.push(Point2D::new(5.0, 5.0)).unwrap();

    let mut series2 = Vec::<Point2D, 4>::new();
    series2.push(Point2D::new(-2.0, 3.0)).unwrap();
    series2.push(Point2D::new(7.0, 1.0)).unwrap();

    let mut series3 = Vec::<Point2D, 4>::new();
    series3.push(Point2D::new(1.0, -5.0)).unwrap();
    series3.push(Point2D::new(3.0, 10.0)).unwrap();

    let mut all_series = Vec::<Vec<Point2D, 4>, 3>::new();
    all_series.push(series1).unwrap();
    all_series.push(series2).unwrap();
    all_series.push(series3).unwrap();

    let bounds = calculate_multi_series_bounds(
        all_series.into_iter().map(|s| s.into_iter())
    ).unwrap();

    assert_eq!(bounds.min_x, -2.0);
    assert_eq!(bounds.max_x, 7.0);
    assert_eq!(bounds.min_y, -5.0);
    assert_eq!(bounds.max_y, 10.0);
}

#[test]
fn test_calculate_multi_series_bounds_empty() {
    let series: Vec<Vec<Point2D, 4>, 2> = Vec::new();
    let result = calculate_multi_series_bounds(
        series.into_iter().map(|s| s.into_iter())
    );
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), DataError::INSUFFICIENT_DATA);
}

#[test]
fn test_calculate_multi_series_bounds_empty_series() {
    let mut series1 = Vec::<Point2D, 4>::new();
    series1.push(Point2D::new(0.0, 0.0)).unwrap();

    let series2 = Vec::<Point2D, 4>::new(); // Empty series

    let mut all_series = Vec::<Vec<Point2D, 4>, 2>::new();
    all_series.push(series1).unwrap();
    all_series.push(series2).unwrap();

    // This should fail because series2 is empty
    let result = calculate_multi_series_bounds(
        all_series.into_iter().map(|s| s.into_iter())
    );
    assert!(result.is_err());
}

#[test]
fn test_int_bounds() {
    let bounds: IntBounds = DataBounds::new(0, 100, -50, 200).unwrap();
    assert_eq!(bounds.width(), 100);
    assert_eq!(bounds.height(), 250);

    let point = IntPoint::new(50, 100);
    assert!(bounds.contains(&point));

    let mut mut_bounds = bounds;
    mut_bounds.expand_to_include(&IntPoint::new(150, 300));
    assert_eq!(mut_bounds.max_x, 150);
    assert_eq!(mut_bounds.max_y, 300);
}

#[test]
fn test_bounds_with_nan_values() {
    // While the DataBounds itself doesn't check for NaN, 
    // this tests the behavior when NaN values are present
    let bounds: FloatBounds = DataBounds::new(0.0, 10.0, 0.0, 10.0).unwrap();
    
    // NaN comparison always returns false, so contains should return false
    assert!(!bounds.contains(&Point2D::new(f32::NAN, 5.0)));
    assert!(!bounds.contains(&Point2D::new(5.0, f32::NAN)));
    assert!(!bounds.contains(&Point2D::new(f32::NAN, f32::NAN)));
}

#[test]
fn test_bounds_partial_ord_edge_cases() {
    // Test with types that implement PartialOrd
    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
    struct CustomValue(f32);
    
    impl core::ops::Sub for CustomValue {
        type Output = Self;
        fn sub(self, rhs: Self) -> Self::Output {
            CustomValue(self.0 - rhs.0)
        }
    }

    let bounds: DataBounds<CustomValue, CustomValue> = 
        DataBounds::new(
            CustomValue(0.0), 
            CustomValue(10.0), 
            CustomValue(-5.0), 
            CustomValue(15.0)
        ).unwrap();
    
    assert_eq!(bounds.width().0, 10.0);
    assert_eq!(bounds.height().0, 20.0);
}

#[test]
fn test_nice_bounds_edge_cases() {
    // Test with range crossing zero
    let bounds: FloatBounds = DataBounds::new(-5.0, 5.0, -10.0, 10.0).unwrap();
    let nice = bounds.nice_bounds();
    assert!(nice.min_x <= bounds.min_x);
    assert!(nice.max_x >= bounds.max_x);
    assert!(nice.min_y <= bounds.min_y);
    assert!(nice.max_y >= bounds.max_y);

    // Test with very large range
    let large_bounds: FloatBounds = DataBounds::new(0.0, 1e6, 0.0, 1e9).unwrap();
    let nice_large = large_bounds.nice_bounds();
    assert!(nice_large.width() >= large_bounds.width());
    assert!(nice_large.height() >= large_bounds.height());

    // Test with very small non-zero values
    let tiny_bounds: FloatBounds = DataBounds::new(1e-10, 2e-10, 1e-10, 2e-10).unwrap();
    let nice_tiny = tiny_bounds.nice_bounds();
    // Nice bounds should expand tiny ranges to something reasonable
    assert!(nice_tiny.width() > 0.0);
    assert!(nice_tiny.height() > 0.0);
}

#[test]
fn test_bounds_debug_format() {
    let bounds: FloatBounds = DataBounds::new(0.0, 10.0, -5.0, 15.0).unwrap();
    let debug_str = format!("{:?}", bounds);
    assert!(debug_str.contains("DataBounds"));
    assert!(debug_str.contains("min_x: 0.0"));
    assert!(debug_str.contains("max_x: 10.0"));
    assert!(debug_str.contains("min_y: -5.0"));
    assert!(debug_str.contains("max_y: 15.0"));
}

#[test] 
fn test_bounds_equality() {
    let bounds1: FloatBounds = DataBounds::new(0.0, 10.0, 0.0, 10.0).unwrap();
    let bounds2: FloatBounds = DataBounds::new(0.0, 10.0, 0.0, 10.0).unwrap();
    let bounds3: FloatBounds = DataBounds::new(0.0, 10.0, 0.0, 11.0).unwrap();
    
    assert_eq!(bounds1, bounds2);
    assert_ne!(bounds1, bounds3);
}

#[test]
fn test_bounds_clone() {
    let bounds: FloatBounds = DataBounds::new(1.0, 2.0, 3.0, 4.0).unwrap();
    let cloned = bounds.clone();
    assert_eq!(bounds, cloned);
}

#[test]
fn test_multiple_expansions() {
    let mut bounds = DataBounds::new(5.0, 5.0, 5.0, 5.0).unwrap();
    
    // Expand gradually in all directions
    let points = [
        Point2D::new(4.0, 5.0),
        Point2D::new(6.0, 5.0),
        Point2D::new(5.0, 4.0),
        Point2D::new(5.0, 6.0),
        Point2D::new(3.0, 3.0),
        Point2D::new(7.0, 7.0),
    ];
    
    for point in &points {
        bounds.expand_to_include(point);
    }
    
    assert_eq!(bounds.min_x, 3.0);
    assert_eq!(bounds.max_x, 7.0);
    assert_eq!(bounds.min_y, 3.0);
    assert_eq!(bounds.max_y, 7.0);
}