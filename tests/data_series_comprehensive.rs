//! Comprehensive data series testing suite
//!
//! This test suite provides extensive coverage for data series functionality,
//! targeting 90%+ code coverage through systematic testing of all data structures.
//!
//! Note: These tests are designed for development and coverage analysis.
//! Use `cargo test --test data_series_comprehensive` to run these tests specifically.

mod common;

use embedded_charts::{
    data::{
        point::{DataPoint, IntPoint, Point2D},
        series::{DataSeries, MultiSeries, StaticDataSeries},
    },
    error::{DataError, DataResult},
};

/// Test StaticDataSeries creation and basic functionality
#[test]
fn test_static_data_series_creation() {
    let series: StaticDataSeries<Point2D, 256> = StaticDataSeries::new();
    assert_eq!(series.len(), 0);
    assert!(series.is_empty());
    assert_eq!(series.capacity(), 256);
    assert_eq!(series.remaining_capacity(), 256);
    assert!(!series.is_full());
    assert!(series.label().is_none());

    // Test default constructor
    let default_series: StaticDataSeries<Point2D, 128> = StaticDataSeries::default();
    assert_eq!(default_series.len(), 0);
    assert_eq!(default_series.capacity(), 128);
}

/// Test StaticDataSeries with labels
#[test]
fn test_static_data_series_labels() {
    let mut series = StaticDataSeries::<Point2D, 100>::new();

    // Initially no label
    assert!(series.label().is_none());

    // Set label
    series.set_label("Temperature Data");
    assert_eq!(series.label(), Some("Temperature Data"));

    // Test with_label constructor
    let labeled_series = StaticDataSeries::<Point2D, 100>::with_label("Pressure Data");
    assert_eq!(labeled_series.label(), Some("Pressure Data"));

    // Test label with very long string (should fail to set)
    let mut long_label_series = StaticDataSeries::<Point2D, 100>::new();
    long_label_series.set_label(
        "This is a very long label that exceeds the 32 character limit for heapless strings",
    );
    // Should fail to set due to heapless::String<32> capacity limit
    assert!(long_label_series.label().is_none());

    // Test label with exactly 32 characters (should work)
    let mut max_label_series = StaticDataSeries::<Point2D, 100>::new();
    max_label_series.set_label("Exactly32CharacterLabelForTest!");
    assert_eq!(
        max_label_series.label(),
        Some("Exactly32CharacterLabelForTest!")
    );
    assert!(max_label_series.label().unwrap().len() <= 32);
}

/// Test StaticDataSeries data manipulation
#[test]
fn test_static_data_series_data_operations() -> DataResult<()> {
    let mut series: StaticDataSeries<Point2D, 10> = StaticDataSeries::new();

    // Test push operations
    let point1 = Point2D::new(1.0, 10.0);
    let point2 = Point2D::new(2.0, 20.0);
    let point3 = Point2D::new(3.0, 30.0);

    series.push(point1)?;
    assert_eq!(series.len(), 1);
    assert_eq!(series.remaining_capacity(), 9);
    assert!(!series.is_empty());
    assert!(!series.is_full());

    series.push(point2)?;
    series.push(point3)?;
    assert_eq!(series.len(), 3);

    // Test get operations
    assert_eq!(series.get(0), Some(point1));
    assert_eq!(series.get(1), Some(point2));
    assert_eq!(series.get(2), Some(point3));
    assert_eq!(series.get(10), None); // Out of bounds

    // Test as_slice
    let slice = series.as_slice();
    assert_eq!(slice.len(), 3);
    assert_eq!(slice[0], point1);
    assert_eq!(slice[1], point2);
    assert_eq!(slice[2], point3);

    // Test data() method (zero-copy access)
    let data = series.data();
    assert_eq!(data.len(), 3);
    assert_eq!(data[0], point1);

    Ok(())
}

/// Test StaticDataSeries extend functionality
#[test]
fn test_static_data_series_extend() -> DataResult<()> {
    let mut series: StaticDataSeries<Point2D, 20> = StaticDataSeries::new();

    // Test extend with iterator
    let points = vec![
        Point2D::new(1.0, 10.0),
        Point2D::new(2.0, 20.0),
        Point2D::new(3.0, 30.0),
    ];

    series.extend(points.into_iter())?;
    assert_eq!(series.len(), 3);

    // Test extend with additional points
    let more_points = [Point2D::new(4.0, 40.0), Point2D::new(5.0, 50.0)];

    series.extend(more_points.into_iter())?;
    assert_eq!(series.len(), 5);

    Ok(())
}

/// Test StaticDataSeries from_tuples functionality
#[test]
fn test_static_data_series_from_tuples() -> DataResult<()> {
    let tuples = [
        (0.0, 5.0),
        (1.0, 10.0),
        (2.0, 15.0),
        (3.0, 20.0),
        (4.0, 25.0),
    ];

    let series: StaticDataSeries<Point2D, 256> = StaticDataSeries::from_tuples(&tuples)?;
    assert_eq!(series.len(), 5);

    for (i, &(x, y)) in tuples.iter().enumerate() {
        assert_eq!(series.get(i), Some(Point2D::new(x, y)));
    }

    // Test empty tuples
    let empty_tuples: &[(f32, f32)] = &[];
    let empty_series: StaticDataSeries<Point2D, 100> = StaticDataSeries::from_tuples(empty_tuples)?;
    assert_eq!(empty_series.len(), 0);

    Ok(())
}

/// Test StaticDataSeries capacity limits and error handling
#[test]
fn test_static_data_series_capacity_limits() {
    let mut series: StaticDataSeries<Point2D, 3> = StaticDataSeries::new();

    // Fill to capacity
    series.push(Point2D::new(1.0, 1.0)).unwrap();
    series.push(Point2D::new(2.0, 2.0)).unwrap();
    series.push(Point2D::new(3.0, 3.0)).unwrap();

    assert!(series.is_full());
    assert_eq!(series.remaining_capacity(), 0);

    // Test buffer full error
    let result = series.push(Point2D::new(4.0, 4.0));
    assert!(result.is_err());
    match result {
        Err(DataError::BufferFull { .. }) => {} // Expected
        _ => panic!("Expected BufferFull error"),
    }

    // Test extend with too many points
    let too_many_points = [Point2D::new(5.0, 5.0), Point2D::new(6.0, 6.0)];

    let result = series.extend(too_many_points);
    assert!(result.is_err());
}

/// Test StaticDataSeries clear functionality
#[test]
fn test_static_data_series_clear() -> DataResult<()> {
    let mut series: StaticDataSeries<Point2D, 10> = StaticDataSeries::new();

    // Add some data
    series.push(Point2D::new(1.0, 1.0))?;
    series.push(Point2D::new(2.0, 2.0))?;
    series.push(Point2D::new(3.0, 3.0))?;
    assert_eq!(series.len(), 3);

    // Clear all data
    series.clear();
    assert_eq!(series.len(), 0);
    assert!(series.is_empty());
    assert!(!series.is_full());
    assert_eq!(series.remaining_capacity(), 10);

    Ok(())
}

/// Test StaticDataSeries sorting by X coordinate
#[test]
fn test_static_data_series_sort_by_x() -> DataResult<()> {
    let mut series: StaticDataSeries<IntPoint, 20> = StaticDataSeries::new();

    // Add unsorted data
    let unsorted_points = [(5, 50), (1, 10), (3, 30), (2, 20), (4, 40)];

    for (x, y) in unsorted_points.iter() {
        series.push(IntPoint::new(*x, *y))?;
    }

    // Sort by X coordinate
    series.sort_by_x();

    // Verify sorted order
    let expected_x_values = [1, 2, 3, 4, 5];
    for (i, expected_x) in expected_x_values.iter().enumerate() {
        assert_eq!(series.get(i).unwrap().x(), *expected_x);
    }

    Ok(())
}

/// Test StaticDataSeries sorting by Y coordinate
#[test]
fn test_static_data_series_sort_by_y() -> DataResult<()> {
    let mut series: StaticDataSeries<IntPoint, 20> = StaticDataSeries::new();

    // Add unsorted data
    let unsorted_points = [(1, 50), (2, 10), (3, 30), (4, 20), (5, 40)];

    for (x, y) in unsorted_points.iter() {
        series.push(IntPoint::new(*x, *y))?;
    }

    // Sort by Y coordinate
    series.sort_by_y();

    // Verify sorted order
    let expected_y_values = [10, 20, 30, 40, 50];
    for (i, expected_y) in expected_y_values.iter().enumerate() {
        assert_eq!(series.get(i).unwrap().y(), *expected_y);
    }

    Ok(())
}

/// Test StaticDataSeries sorting edge cases
#[test]
fn test_static_data_series_sorting_edge_cases() -> DataResult<()> {
    // Test sorting empty series
    let mut empty_series: StaticDataSeries<IntPoint, 10> = StaticDataSeries::new();
    empty_series.sort_by_x(); // Should not panic
    empty_series.sort_by_y(); // Should not panic
    assert_eq!(empty_series.len(), 0);

    // Test sorting single element
    let mut single_series: StaticDataSeries<IntPoint, 10> = StaticDataSeries::new();
    single_series.push(IntPoint::new(5, 10))?;
    single_series.sort_by_x();
    single_series.sort_by_y();
    assert_eq!(single_series.len(), 1);
    assert_eq!(single_series.get(0), Some(IntPoint::new(5, 10)));

    // Test sorting already sorted data
    let mut sorted_series: StaticDataSeries<IntPoint, 10> = StaticDataSeries::new();
    for i in 1..=5 {
        sorted_series.push(IntPoint::new(i, i))?;
    }
    sorted_series.sort_by_x(); // Should remain sorted
    for i in 1..=5 {
        assert_eq!(sorted_series.get(i - 1).unwrap().x(), i as i32);
    }

    // Test sorting duplicate values
    let mut duplicate_series: StaticDataSeries<IntPoint, 10> = StaticDataSeries::new();
    duplicate_series.push(IntPoint::new(2, 20))?;
    duplicate_series.push(IntPoint::new(1, 10))?;
    duplicate_series.push(IntPoint::new(2, 25))?;
    duplicate_series.push(IntPoint::new(1, 15))?;

    duplicate_series.sort_by_x();
    // Should be stable sort, maintaining relative order for equal X values
    assert!(duplicate_series.get(0).unwrap().x() <= duplicate_series.get(1).unwrap().x());
    assert!(duplicate_series.get(1).unwrap().x() <= duplicate_series.get(2).unwrap().x());
    assert!(duplicate_series.get(2).unwrap().x() <= duplicate_series.get(3).unwrap().x());

    Ok(())
}

/// Test StaticDataSeries large data sorting (testing merge sort path)
#[test]
fn test_static_data_series_large_data_sorting() -> DataResult<()> {
    let mut large_series: StaticDataSeries<IntPoint, 256> = StaticDataSeries::new();

    // Add 50 unsorted points to trigger merge sort (> 16 elements)
    for i in (0..50).rev() {
        large_series.push(IntPoint::new(i, i * 2))?;
    }

    // Sort by X coordinate (should use merge sort)
    large_series.sort_by_x();

    // Verify sorted order
    for i in 0..50 {
        assert_eq!(large_series.get(i).unwrap().x(), i as i32);
    }

    // Test Y sorting with large data
    for i in (0..40).rev() {
        large_series.push(IntPoint::new(i + 100, i))?;
    }

    large_series.sort_by_y();

    // Verify Y values are in ascending order
    for i in 1..large_series.len() {
        assert!(large_series.get(i - 1).unwrap().y() <= large_series.get(i).unwrap().y());
    }

    Ok(())
}

/// Test StaticDataSeries bounds calculation
#[test]
fn test_static_data_series_bounds() -> DataResult<()> {
    let mut series: StaticDataSeries<Point2D, 20> = StaticDataSeries::new();

    // Add test data with known bounds
    let test_points = [
        (1.0, 5.0),
        (3.0, 15.0),
        (2.0, 3.0),  // Min Y
        (5.0, 12.0), // Max X
        (0.5, 20.0), // Min X, Max Y
    ];

    for (x, y) in test_points.iter() {
        series.push(Point2D::new(*x, *y))?;
    }

    let bounds = series.bounds()?;
    assert_eq!(bounds.min_x, 0.5);
    assert_eq!(bounds.max_x, 5.0);
    assert_eq!(bounds.min_y, 3.0);
    assert_eq!(bounds.max_y, 20.0);

    // Test bounds with single point
    let mut single_series: StaticDataSeries<Point2D, 10> = StaticDataSeries::new();
    single_series.push(Point2D::new(10.0, 25.0))?;

    let single_bounds = single_series.bounds()?;
    assert_eq!(single_bounds.min_x, 10.0);
    assert_eq!(single_bounds.max_x, 10.0);
    assert_eq!(single_bounds.min_y, 25.0);
    assert_eq!(single_bounds.max_y, 25.0);

    Ok(())
}

/// Test StaticDataSeries iterator functionality
#[test]
fn test_static_data_series_iterators() -> DataResult<()> {
    let mut series: StaticDataSeries<Point2D, 20> = StaticDataSeries::new();

    let test_points = [
        Point2D::new(1.0, 10.0),
        Point2D::new(2.0, 20.0),
        Point2D::new(3.0, 30.0),
    ];

    for point in test_points.iter() {
        series.push(*point)?;
    }

    // Test cloning iterator (iter method)
    let collected: Vec<_> = series.iter().collect();
    assert_eq!(collected.len(), 3);
    assert_eq!(collected[0], test_points[0]);
    assert_eq!(collected[1], test_points[1]);
    assert_eq!(collected[2], test_points[2]);

    // Test reference iterator (iter_ref method)
    let refs: Vec<_> = series.iter_ref().collect();
    assert_eq!(refs.len(), 3);
    assert_eq!(*refs[0], test_points[0]);
    assert_eq!(*refs[1], test_points[1]);
    assert_eq!(*refs[2], test_points[2]);

    // Test iterator size hints
    let mut iter = series.iter();
    let (lower, upper) = iter.size_hint();
    assert_eq!(lower, 3);
    assert_eq!(upper, Some(3));

    // Consume one element and check size hint again
    let _first = iter.next();
    let (lower, upper) = iter.size_hint();
    assert_eq!(lower, 2);
    assert_eq!(upper, Some(2));

    // Test ExactSizeIterator
    let iter = series.iter();
    assert_eq!(iter.len(), 3);

    Ok(())
}

/// Test StaticDataSeries DataSeries trait implementation
#[test]
fn test_static_data_series_data_series_trait() -> DataResult<()> {
    let mut series: StaticDataSeries<Point2D, 10> = StaticDataSeries::new();

    // Test empty series
    assert_eq!(series.len(), 0);
    assert!(series.is_empty());
    assert_eq!(series.get(0), None);

    // Add some data
    series.push(Point2D::new(1.0, 10.0))?;
    series.push(Point2D::new(2.0, 20.0))?;

    // Test trait methods
    assert_eq!(series.len(), 2);
    assert!(!series.is_empty());
    assert_eq!(series.get(0), Some(Point2D::new(1.0, 10.0)));
    assert_eq!(series.get(1), Some(Point2D::new(2.0, 20.0)));
    assert_eq!(series.get(2), None);

    // Test calculate_bounds (placeholder implementation)
    let result = series.calculate_bounds();
    assert!(result.is_ok());

    Ok(())
}

/// Test MultiSeries creation and basic functionality
#[test]
fn test_multi_series_creation() {
    let multi: MultiSeries<Point2D, 5, 100> = MultiSeries::new();
    assert_eq!(multi.series_count(), 0);
    assert!(multi.is_empty());

    // Test default constructor
    let default_multi: MultiSeries<Point2D, 3, 50> = MultiSeries::default();
    assert_eq!(default_multi.series_count(), 0);
}

/// Test MultiSeries add and access functionality
#[test]
fn test_multi_series_operations() -> DataResult<()> {
    let mut multi: MultiSeries<Point2D, 3, 100> = MultiSeries::new();

    // Create test series
    let mut series1 = StaticDataSeries::with_label("Temperature");
    series1.push(Point2D::new(1.0, 25.0))?;
    series1.push(Point2D::new(2.0, 26.0))?;

    let mut series2 = StaticDataSeries::with_label("Humidity");
    series2.push(Point2D::new(1.0, 60.0))?;
    series2.push(Point2D::new(2.0, 65.0))?;

    // Add series to multi-series
    let index1 = multi.add_series(series1)?;
    let index2 = multi.add_series(series2)?;

    assert_eq!(index1, 0);
    assert_eq!(index2, 1);
    assert_eq!(multi.series_count(), 2);
    assert!(!multi.is_empty());

    // Test accessing series
    let retrieved1 = multi.get_series(0).unwrap();
    assert_eq!(retrieved1.label(), Some("Temperature"));
    assert_eq!(retrieved1.len(), 2);

    let retrieved2 = multi.get_series(1).unwrap();
    assert_eq!(retrieved2.label(), Some("Humidity"));
    assert_eq!(retrieved2.len(), 2);

    // Test out of bounds access
    assert!(multi.get_series(5).is_none());

    Ok(())
}

/// Test MultiSeries mutable access
#[test]
fn test_multi_series_mutable_access() -> DataResult<()> {
    let mut multi: MultiSeries<Point2D, 3, 100> = MultiSeries::new();

    let mut series = StaticDataSeries::with_label("Pressure");
    series.push(Point2D::new(1.0, 1013.25))?;
    multi.add_series(series)?;

    // Test mutable access
    {
        let series_mut = multi.get_series_mut(0).unwrap();
        series_mut.push(Point2D::new(2.0, 1015.0))?;
        series_mut.set_label("Atmospheric Pressure");
    }

    // Verify changes
    let series = multi.get_series(0).unwrap();
    assert_eq!(series.len(), 2);
    assert_eq!(series.label(), Some("Atmospheric Pressure"));
    assert_eq!(series.get(1), Some(Point2D::new(2.0, 1015.0)));

    // Test out of bounds mutable access
    assert!(multi.get_series_mut(5).is_none());

    Ok(())
}

/// Test MultiSeries iterator functionality
#[test]
fn test_multi_series_iteration() -> DataResult<()> {
    let mut multi: MultiSeries<Point2D, 3, 50> = MultiSeries::new();

    // Add multiple series
    for i in 0..3 {
        let mut series = StaticDataSeries::with_label(&format!("Series {i}"));
        series.push(Point2D::new(i as f32, (i * 10) as f32))?;
        multi.add_series(series)?;
    }

    // Test iteration
    let mut count = 0;
    for (i, series) in multi.iter_series().enumerate() {
        assert_eq!(series.label(), Some(format!("Series {i}").as_str()));
        assert_eq!(series.len(), 1);
        count += 1;
    }
    assert_eq!(count, 3);

    Ok(())
}

/// Test MultiSeries combined bounds calculation
#[test]
fn test_multi_series_combined_bounds() -> DataResult<()> {
    let mut multi: MultiSeries<Point2D, 3, 50> = MultiSeries::new();

    // Create series with different bounds
    let mut series1 = StaticDataSeries::new();
    series1.push(Point2D::new(0.0, 10.0))?;
    series1.push(Point2D::new(5.0, 20.0))?;

    let mut series2 = StaticDataSeries::new();
    series2.push(Point2D::new(-2.0, 5.0))?; // Extends min_x, min_y
    series2.push(Point2D::new(3.0, 25.0))?; // Extends max_y

    let mut series3 = StaticDataSeries::new();
    series3.push(Point2D::new(8.0, 15.0))?; // Extends max_x

    multi.add_series(series1)?;
    multi.add_series(series2)?;
    multi.add_series(series3)?;

    let bounds = multi.combined_bounds()?;
    assert_eq!(bounds.min_x, -2.0);
    assert_eq!(bounds.max_x, 8.0);
    assert_eq!(bounds.min_y, 5.0);
    assert_eq!(bounds.max_y, 25.0);

    Ok(())
}

/// Test MultiSeries bounds calculation edge cases
#[test]
fn test_multi_series_bounds_edge_cases() {
    let multi: MultiSeries<Point2D, 3, 50> = MultiSeries::new();

    // Test empty multi-series bounds
    let result = multi.combined_bounds();
    assert!(result.is_err());
    match result {
        Err(DataError::InsufficientData { .. }) => {} // Expected
        _ => panic!("Expected InsufficientData error"),
    }
}

/// Test MultiSeries capacity limits
#[test]
fn test_multi_series_capacity_limits() -> DataResult<()> {
    let mut multi: MultiSeries<Point2D, 2, 10> = MultiSeries::new();

    // Fill to capacity
    multi.add_series(StaticDataSeries::with_label("Series 1"))?;
    multi.add_series(StaticDataSeries::with_label("Series 2"))?;

    // Test buffer full error
    let result = multi.add_series(StaticDataSeries::with_label("Series 3"));
    assert!(result.is_err());
    match result {
        Err(DataError::BufferFull { .. }) => {} // Expected
        _ => panic!("Expected BufferFull error"),
    }

    Ok(())
}

/// Test MultiSeries clear functionality
#[test]
fn test_multi_series_clear() -> DataResult<()> {
    let mut multi: MultiSeries<Point2D, 3, 50> = MultiSeries::new();

    // Add some series
    multi.add_series(StaticDataSeries::with_label("Series 1"))?;
    multi.add_series(StaticDataSeries::with_label("Series 2"))?;
    assert_eq!(multi.series_count(), 2);

    // Clear all series
    multi.clear();
    assert_eq!(multi.series_count(), 0);
    assert!(multi.is_empty());

    Ok(())
}

/// Test SlidingWindowSeries functionality (animations feature)
#[cfg(feature = "animations")]
mod sliding_window_tests {
    use super::*;
    use embedded_charts::data::series::SlidingWindowSeries;

    #[test]
    fn test_sliding_window_creation() {
        let series: SlidingWindowSeries<Point2D, 10> = SlidingWindowSeries::new();
        assert_eq!(series.current_len(), 0);
        assert_eq!(series.capacity(), 10);
        assert!(!series.is_full());
        assert!(series.label().is_none());

        // Test default constructor
        let default_series: SlidingWindowSeries<Point2D, 5> = SlidingWindowSeries::default();
        assert_eq!(default_series.current_len(), 0);

        // Test with_label constructor
        let labeled_series = SlidingWindowSeries::<Point2D, 8>::with_label("Sensor Data");
        assert_eq!(labeled_series.label(), Some("Sensor Data"));
    }

    #[test]
    fn test_sliding_window_labels() {
        let mut series = SlidingWindowSeries::<Point2D, 10>::new();

        // Initially no label
        assert!(series.label().is_none());

        // Set label
        series.set_label("Temperature Stream");
        assert_eq!(series.label(), Some("Temperature Stream"));
    }

    #[test]
    fn test_sliding_window_push_operations() {
        let mut series: SlidingWindowSeries<Point2D, 3> = SlidingWindowSeries::new();

        // Push points until full
        series.push(Point2D::new(1.0, 10.0));
        assert_eq!(series.current_len(), 1);
        assert!(!series.is_full());

        series.push(Point2D::new(2.0, 20.0));
        series.push(Point2D::new(3.0, 30.0));
        assert_eq!(series.current_len(), 3);
        assert!(series.is_full());

        // Push one more (should overwrite oldest)
        series.push(Point2D::new(4.0, 40.0));
        assert_eq!(series.current_len(), 3);
        assert!(series.is_full());

        // Verify chronological order (oldest point should be gone)
        let points: heapless::Vec<Point2D, 3> = series.iter_chronological().collect();
        assert_eq!(points.len(), 3);
        assert_eq!(points[0], Point2D::new(2.0, 20.0));
        assert_eq!(points[1], Point2D::new(3.0, 30.0));
        assert_eq!(points[2], Point2D::new(4.0, 40.0));
    }

    #[test]
    fn test_sliding_window_clear() {
        let mut series: SlidingWindowSeries<Point2D, 5> = SlidingWindowSeries::new();

        // Add some data
        series.push(Point2D::new(1.0, 10.0));
        series.push(Point2D::new(2.0, 20.0));
        assert_eq!(series.current_len(), 2);

        // Clear
        series.clear();
        assert_eq!(series.current_len(), 0);
        assert!(!series.is_full());
    }

    #[test]
    fn test_sliding_window_data_series_trait() {
        let mut series: SlidingWindowSeries<Point2D, 5> = SlidingWindowSeries::new();

        // Test empty series
        assert_eq!(series.len(), 0);
        assert!(series.is_empty());
        assert_eq!(series.get(0), None);

        // Add data
        series.push(Point2D::new(1.0, 10.0));
        series.push(Point2D::new(2.0, 20.0));

        // Test trait methods
        assert_eq!(series.len(), 2);
        assert!(!series.is_empty());
        assert_eq!(series.get(0), Some(Point2D::new(1.0, 10.0)));
        assert_eq!(series.get(1), Some(Point2D::new(2.0, 20.0)));
        assert_eq!(series.get(2), None);

        // Test iterator
        let collected: heapless::Vec<Point2D, 5> = series.iter().collect();
        assert_eq!(collected.len(), 2);
        assert_eq!(collected[0], Point2D::new(1.0, 10.0));
        assert_eq!(collected[1], Point2D::new(2.0, 20.0));
    }

    #[test]
    fn test_sliding_window_wrapping() {
        let mut series: SlidingWindowSeries<Point2D, 4> = SlidingWindowSeries::new();

        // Fill completely and then add more to test wrapping
        for i in 1..=7 {
            series.push(Point2D::new(i as f32, i as f32 * 10.0));
        }

        assert_eq!(series.current_len(), 4);
        assert!(series.is_full());

        // Should contain points 4, 5, 6, 7 in chronological order
        let points: heapless::Vec<Point2D, 4> = series.iter_chronological().collect();
        assert_eq!(points.len(), 4);
        assert_eq!(points[0], Point2D::new(4.0, 40.0));
        assert_eq!(points[1], Point2D::new(5.0, 50.0));
        assert_eq!(points[2], Point2D::new(6.0, 60.0));
        assert_eq!(points[3], Point2D::new(7.0, 70.0));

        // Test get method with wrapping
        assert_eq!(series.get(0), Some(Point2D::new(4.0, 40.0)));
        assert_eq!(series.get(1), Some(Point2D::new(5.0, 50.0)));
        assert_eq!(series.get(2), Some(Point2D::new(6.0, 60.0)));
        assert_eq!(series.get(3), Some(Point2D::new(7.0, 70.0)));
        assert_eq!(series.get(4), None);
    }
}

/// Test iterator implementations in detail
#[test]
fn test_iterator_implementations() -> DataResult<()> {
    let mut series: StaticDataSeries<Point2D, 10> = StaticDataSeries::new();

    // Add test data
    for i in 1..=5 {
        series.push(Point2D::new(i as f32, i as f32 * 2.0))?;
    }

    // Test StaticDataSeriesIter
    let mut iter = series.iter();

    // Test next() method
    assert_eq!(iter.next(), Some(Point2D::new(1.0, 2.0)));
    assert_eq!(iter.next(), Some(Point2D::new(2.0, 4.0)));

    // Test size_hint after consuming some elements
    let (lower, upper) = iter.size_hint();
    assert_eq!(lower, 3);
    assert_eq!(upper, Some(3));

    // Test exact size iterator
    assert_eq!(iter.len(), 3);

    // Consume remaining elements
    assert_eq!(iter.next(), Some(Point2D::new(3.0, 6.0)));
    assert_eq!(iter.next(), Some(Point2D::new(4.0, 8.0)));
    assert_eq!(iter.next(), Some(Point2D::new(5.0, 10.0)));
    assert_eq!(iter.next(), None);

    // Test StaticDataSeriesRefIter
    let mut ref_iter = series.iter_ref();

    // Test reference iteration
    assert_eq!(ref_iter.next(), Some(&Point2D::new(1.0, 2.0)));
    assert_eq!(ref_iter.next(), Some(&Point2D::new(2.0, 4.0)));

    // Test size_hint for reference iterator
    let (lower, upper) = ref_iter.size_hint();
    assert_eq!(lower, 3);
    assert_eq!(upper, Some(3));

    // Test exact size for reference iterator
    assert_eq!(ref_iter.len(), 3);

    Ok(())
}

/// Test error handling and edge cases
#[test]
fn test_error_handling_edge_cases() {
    // Test from_tuples with capacity exceeded
    let too_many_tuples = vec![(1.0, 1.0); 10];
    let result: Result<StaticDataSeries<Point2D, 5>, _> =
        StaticDataSeries::from_tuples(&too_many_tuples);
    assert!(result.is_err());

    // Test bounds calculation with empty series
    let empty_series: StaticDataSeries<Point2D, 10> = StaticDataSeries::new();
    let bounds_result = empty_series.bounds();
    assert!(bounds_result.is_err());

    // Test iterator on empty series
    let empty_iter = empty_series.iter();
    assert_eq!(empty_iter.size_hint(), (0, Some(0)));
    assert_eq!(empty_iter.len(), 0);

    let empty_ref_iter = empty_series.iter_ref();
    assert_eq!(empty_ref_iter.size_hint(), (0, Some(0)));
    assert_eq!(empty_ref_iter.len(), 0);
}

/// Test sorting performance and correctness with different data sizes
#[test]
fn test_sorting_performance_and_correctness() -> DataResult<()> {
    // Test small data (should use insertion sort)
    let mut small_series: StaticDataSeries<IntPoint, 32> = StaticDataSeries::new();
    for i in (0..10).rev() {
        small_series.push(IntPoint::new(i, i))?;
    }

    small_series.sort_by_x();
    for i in 0..10 {
        assert_eq!(small_series.get(i).unwrap().x(), i as i32);
    }

    // Test medium data (should use merge sort)
    let mut medium_series: StaticDataSeries<IntPoint, 100> = StaticDataSeries::new();
    for i in (0..30).rev() {
        medium_series.push(IntPoint::new(i, i))?;
    }

    medium_series.sort_by_x();
    for i in 0..30 {
        assert_eq!(medium_series.get(i).unwrap().x(), i as i32);
    }

    // Test with random-like data
    let mut random_series: StaticDataSeries<IntPoint, 50> = StaticDataSeries::new();
    let random_values = [13, 7, 25, 3, 18, 9, 22, 1, 16, 11, 28, 5, 20];
    for &val in random_values.iter() {
        random_series.push(IntPoint::new(val, val))?;
    }

    random_series.sort_by_x();

    // Verify sorted order
    for i in 1..random_series.len() {
        assert!(random_series.get(i - 1).unwrap().x() <= random_series.get(i).unwrap().x());
    }

    Ok(())
}

/// Test memory efficiency and zero-copy operations
#[test]
fn test_memory_efficiency() -> DataResult<()> {
    let mut series: StaticDataSeries<Point2D, 100> = StaticDataSeries::new();

    // Add data
    for i in 0..50 {
        series.push(Point2D::new(i as f32, i as f32 * 2.0))?;
    }

    // Test zero-copy slice access
    let slice = series.as_slice();
    assert_eq!(slice.len(), 50);

    // Test zero-copy data access
    let data = series.data();
    assert_eq!(data.len(), 50);

    // Verify these are references to the same data
    assert_eq!(slice.as_ptr(), data.as_ptr());

    // Test reference iterator (zero-copy)
    let ref_count = series.iter_ref().count();
    assert_eq!(ref_count, 50);

    Ok(())
}

/// Test comprehensive multi-series scenarios
#[test]
fn test_comprehensive_multi_series_scenarios() -> DataResult<()> {
    let mut multi: MultiSeries<Point2D, 5, 100> = MultiSeries::new();

    // Create different types of data series
    let mut temperature = StaticDataSeries::with_label("Temperature °C");
    let mut humidity = StaticDataSeries::with_label("Humidity %");
    let mut pressure = StaticDataSeries::with_label("Pressure hPa");

    // Add temperature data (0-50°C range)
    for hour in 0..24 {
        let temp = 20.0 + 10.0 * (hour as f32 * 0.26).sin();
        temperature.push(Point2D::new(hour as f32, temp))?;
    }

    // Add humidity data (30-90% range)
    for hour in 0..24 {
        let humidity_val = 60.0 + 30.0 * (hour as f32 * 0.15).cos();
        humidity.push(Point2D::new(hour as f32, humidity_val))?;
    }

    // Add pressure data (990-1030 hPa range)
    for hour in 0..24 {
        let pressure_val = 1013.25 + 15.0 * (hour as f32 * 0.1).sin();
        pressure.push(Point2D::new(hour as f32, pressure_val))?;
    }

    // Add series to multi-series
    multi.add_series(temperature)?;
    multi.add_series(humidity)?;
    multi.add_series(pressure)?;

    assert_eq!(multi.series_count(), 3);

    // Test combined bounds calculation
    let bounds = multi.combined_bounds()?;
    assert_eq!(bounds.min_x, 0.0);
    assert_eq!(bounds.max_x, 23.0);
    // Y bounds should encompass all three different scales
    assert!(bounds.min_y < 50.0); // Temperature + humidity minimum
    assert!(bounds.max_y > 1000.0); // Pressure maximum

    // Test individual series access
    for i in 0..3 {
        let series = multi.get_series(i).unwrap();
        assert_eq!(series.len(), 24);
        assert!(series.label().is_some());
    }

    Ok(())
}

/// Benchmark and stress test for data series operations
#[test]
fn test_stress_scenarios() -> DataResult<()> {
    // Test maximum capacity series
    let mut max_series: StaticDataSeries<Point2D, 256> = StaticDataSeries::new();

    // Fill to maximum capacity
    for i in 0..256 {
        max_series.push(Point2D::new(i as f32, (i % 100) as f32))?;
    }

    assert!(max_series.is_full());
    assert_eq!(max_series.remaining_capacity(), 0);

    // Test sorting maximum capacity data (convert to IntPoint for sorting)
    let mut max_int_series: StaticDataSeries<IntPoint, 256> = StaticDataSeries::new();
    for i in 0..256 {
        max_int_series.push(IntPoint::new(255 - i, i % 100))?;
    }

    max_int_series.sort_by_x();
    for i in 0..256 {
        assert_eq!(max_int_series.get(i).unwrap().x(), i as i32);
    }

    // Test bounds calculation on large dataset
    let bounds = max_series.bounds()?;
    assert_eq!(bounds.min_x, 0.0);
    assert_eq!(bounds.max_x, 255.0);
    assert_eq!(bounds.min_y, 0.0);
    assert_eq!(bounds.max_y, 99.0);

    // Test iterators on large dataset
    let iter_count = max_series.iter().count();
    assert_eq!(iter_count, 256);

    let ref_iter_count = max_series.iter_ref().count();
    assert_eq!(ref_iter_count, 256);

    Ok(())
}
