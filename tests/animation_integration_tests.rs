//! Comprehensive integration tests for the new simplified animation system.
//!
//! These tests verify the new progress-based animation system including
//! ChartAnimator, MultiStateAnimator, StreamingAnimator, and TimeBasedProgress.

#![cfg(feature = "animations")]

use embedded_charts::prelude::*;

/// Test basic ChartAnimator functionality
#[test]
fn test_chart_animator_basic() -> ChartResult<()> {
    // Create a simple f32 animator
    let animator = ChartAnimator::new(0.0f32, 100.0f32, EasingFunction::Linear);

    // Test progress values
    assert_eq!(animator.value_at(0), Some(0.0));
    assert_eq!(animator.value_at(50), Some(50.0));
    assert_eq!(animator.value_at(100), Some(100.0));

    // Test easing function
    let ease_in_animator = ChartAnimator::new(0.0f32, 100.0f32, EasingFunction::EaseIn);
    let value_at_50 = ease_in_animator.value_at(50).unwrap();
    assert!(value_at_50 < 50.0); // EaseIn should be slower at the start

    Ok(())
}

/// Test ChartAnimator with Point2D data
#[test]
fn test_chart_animator_point2d() -> ChartResult<()> {
    let from_point = Point2D::new(0.0, 0.0);
    let to_point = Point2D::new(100.0, 200.0);

    let animator = ChartAnimator::new(from_point, to_point, EasingFunction::Linear);

    // Test interpolation
    let mid_point = animator.value_at(50).unwrap();
    assert_eq!(mid_point.x(), 50.0);
    assert_eq!(mid_point.y(), 100.0);

    Ok(())
}

/// Test MultiStateAnimator with keyframes
#[test]
fn test_multi_state_animator() -> ChartResult<()> {
    let mut animator: MultiStateAnimator<f32, 4> = MultiStateAnimator::new();

    // Add keyframes
    animator.add_keyframe(0, 0.0, EasingFunction::Linear)?;
    animator.add_keyframe(25, 50.0, EasingFunction::EaseIn)?;
    animator.add_keyframe(75, 25.0, EasingFunction::EaseOut)?;
    animator.add_keyframe(100, 100.0, EasingFunction::Linear)?;

    // Test interpolation at various points
    assert_eq!(animator.value_at(0), Some(0.0));
    assert_eq!(animator.value_at(100), Some(100.0));

    // Test intermediate values
    let value_at_50 = animator.value_at(50).unwrap();
    assert!(value_at_50 > 25.0 && value_at_50 < 50.0);

    Ok(())
}

/// Test StreamingAnimator functionality
#[test]
fn test_streaming_animator() -> ChartResult<()> {
    let mut animator = StreamingAnimator::new();

    // Test initial state
    assert!(animator.is_empty());
    assert_eq!(animator.len(), 0);

    // Add some data points
    animator.push_data(Point2D::new(1.0, 10.0));
    animator.push_data(Point2D::new(2.0, 20.0));
    animator.push_data(Point2D::new(3.0, 30.0));

    assert_eq!(animator.len(), 3);
    assert!(!animator.is_empty());

    // Test data retrieval
    let data: heapless::Vec<Point2D, 100> = animator.current_data().collect();
    assert_eq!(data.len(), 3);
    assert_eq!(data[0], Point2D::new(1.0, 10.0));
    assert_eq!(data[2], Point2D::new(3.0, 30.0));

    // Test interpolation progress
    animator.set_interpolation_progress(75);
    assert_eq!(animator.interpolation_progress(), 75);

    Ok(())
}

/// Test TimeBasedProgress calculations
#[test]
fn test_time_based_progress() -> ChartResult<()> {
    let mut progress_calc = TimeBasedProgress::new(1000); // 1 second duration
    let mut time_provider = ManualTimeProvider::new();

    // First call should initialize and return 0
    assert_eq!(progress_calc.progress_from_time(&time_provider), 0);

    // Advance time by 250ms (25% of duration)
    time_provider.advance_ms(250);
    assert_eq!(progress_calc.progress_from_time(&time_provider), 25);

    // Advance to 50%
    time_provider.advance_ms(250);
    assert_eq!(progress_calc.progress_from_time(&time_provider), 50);

    // Advance to completion
    time_provider.advance_ms(500);
    assert_eq!(progress_calc.progress_from_time(&time_provider), 100);

    // Beyond completion should still return 100
    time_provider.advance_ms(500);
    assert_eq!(progress_calc.progress_from_time(&time_provider), 100);

    Ok(())
}

/// Test TimeBasedProgress with looping
#[test]
fn test_time_based_progress_looping() -> ChartResult<()> {
    let mut progress_calc = TimeBasedProgress::new_looping(1000); // 1 second loop
    let mut time_provider = ManualTimeProvider::new();

    // First call should return 0
    assert_eq!(progress_calc.progress_from_time(&time_provider), 0);

    // Complete one cycle
    time_provider.advance_ms(1000);
    assert_eq!(progress_calc.progress_from_time(&time_provider), 0);

    // Half way through second cycle
    time_provider.advance_ms(500);
    assert_eq!(progress_calc.progress_from_time(&time_provider), 50);

    // Complete second cycle
    time_provider.advance_ms(500);
    assert_eq!(progress_calc.progress_from_time(&time_provider), 0);

    Ok(())
}

/// Test progress calculation from elapsed time
#[test]
fn test_progress_from_elapsed() -> ChartResult<()> {
    let progress_calc = TimeBasedProgress::new(2000); // 2 seconds

    assert_eq!(progress_calc.progress_from_elapsed(0), 0);
    assert_eq!(progress_calc.progress_from_elapsed(500), 25);
    assert_eq!(progress_calc.progress_from_elapsed(1000), 50);
    assert_eq!(progress_calc.progress_from_elapsed(2000), 100);
    assert_eq!(progress_calc.progress_from_elapsed(3000), 100); // Clamped

    Ok(())
}

/// Test easing functions
#[test]
fn test_easing_functions() -> ChartResult<()> {
    // Test linear easing
    assert_eq!(EasingFunction::Linear.apply(0.0), 0.0);
    assert_eq!(EasingFunction::Linear.apply(0.5), 0.5);
    assert_eq!(EasingFunction::Linear.apply(1.0), 1.0);

    // Test ease in (should be slower at start)
    let ease_in_half = EasingFunction::EaseIn.apply(0.5);
    assert!(ease_in_half < 0.5);

    // Test ease out (should be faster at start)
    let ease_out_half = EasingFunction::EaseOut.apply(0.5);
    assert!(ease_out_half > 0.5);

    // Test ease in-out (should be 0.5 at midpoint)
    let ease_in_out_half = EasingFunction::EaseInOut.apply(0.5);
    assert!((ease_in_out_half - 0.5).abs() < 0.01);

    Ok(())
}

/// Test data series interpolation
#[test]
fn test_data_series_interpolation() -> ChartResult<()> {
    // Create two data series
    let mut from_series = StaticDataSeries::new();
    from_series.push(Point2D::new(0.0, 0.0))?;
    from_series.push(Point2D::new(1.0, 10.0))?;
    from_series.push(Point2D::new(2.0, 20.0))?;

    let mut to_series = StaticDataSeries::new();
    to_series.push(Point2D::new(0.0, 100.0))?;
    to_series.push(Point2D::new(1.0, 110.0))?;
    to_series.push(Point2D::new(2.0, 120.0))?;

    // Test interpolation using the Interpolatable trait
    let interpolated = from_series.clone().interpolate(to_series, 0.5).unwrap();

    assert_eq!(interpolated.len(), 3);
    assert_eq!(interpolated.get(0).unwrap(), Point2D::new(0.0, 50.0));
    assert_eq!(interpolated.get(1).unwrap(), Point2D::new(1.0, 60.0));
    assert_eq!(interpolated.get(2).unwrap(), Point2D::new(2.0, 70.0));

    Ok(())
}

/// Test AnimatedChart trait integration
#[test]
#[cfg(feature = "line")]
fn test_animated_chart_integration() -> ChartResult<()> {
    // Create an animated line chart
    let chart = AnimatedLineChart::<Rgb565>::new();

    // Create test data
    let mut from_data = StaticDataSeries::new();
    from_data.push(Point2D::new(0.0, 0.0))?;
    from_data.push(Point2D::new(1.0, 10.0))?;

    let mut to_data = StaticDataSeries::new();
    to_data.push(Point2D::new(0.0, 100.0))?;
    to_data.push(Point2D::new(1.0, 110.0))?;

    // Test animator creation
    let animator = chart.create_transition_animator(
        from_data.clone(),
        to_data.clone(),
        EasingFunction::Linear,
    );

    // Test interpolation
    let mid_data = animator.value_at(50).unwrap();
    assert_eq!(mid_data.len(), 2);
    assert_eq!(mid_data.get(0).unwrap(), Point2D::new(0.0, 50.0));
    assert_eq!(mid_data.get(1).unwrap(), Point2D::new(1.0, 60.0));

    Ok(())
}

/// Test memory management with streaming animator
#[test]
fn test_streaming_memory_management() -> ChartResult<()> {
    let mut animator = StreamingAnimator::new();

    // Fill beyond capacity to test sliding window
    for i in 0..150 {
        animator.push_data(Point2D::new(i as f32, (i * 2) as f32));
    }

    // Should be capped at capacity
    assert_eq!(animator.len(), animator.capacity());

    // Latest data should be preserved
    let data: heapless::Vec<Point2D, 100> = animator.current_data().collect();
    let last_point = data.last().unwrap();
    assert_eq!(last_point.x(), 149.0);
    assert_eq!(last_point.y(), 298.0);

    Ok(())
}

/// Test error handling scenarios
#[test]
fn test_animation_error_handling() -> ChartResult<()> {
    // Test MultiStateAnimator capacity limits
    let mut animator: MultiStateAnimator<f32, 2> = MultiStateAnimator::new();

    // Should succeed
    animator.add_keyframe(0, 0.0, EasingFunction::Linear)?;
    animator.add_keyframe(100, 100.0, EasingFunction::Linear)?;

    // Should fail due to capacity limit
    let result = animator.add_keyframe(50, 50.0, EasingFunction::Linear);
    assert!(result.is_err());

    Ok(())
}

/// Test complex animation scenario with multiple animators
#[test]
fn test_complex_animation_scenario() -> ChartResult<()> {
    // Create multiple animators for different chart properties
    let position_animator = ChartAnimator::new(
        Point2D::new(0.0, 0.0),
        Point2D::new(100.0, 100.0),
        EasingFunction::EaseInOut,
    );

    let scale_animator = ChartAnimator::new(1.0f32, 2.0f32, EasingFunction::EaseOut);

    let mut color_animator: MultiStateAnimator<i32, 4> = MultiStateAnimator::new();
    color_animator.add_keyframe(0, 0, EasingFunction::Linear)?;
    color_animator.add_keyframe(33, 128, EasingFunction::EaseIn)?;
    color_animator.add_keyframe(66, 255, EasingFunction::EaseOut)?;
    color_animator.add_keyframe(100, 128, EasingFunction::Linear)?;

    // Test synchronized animation at 75% progress
    let progress = 75;

    let position = position_animator.value_at(progress).unwrap();
    let scale = scale_animator.value_at(progress).unwrap();
    let color = color_animator.value_at(progress).unwrap();

    // Verify values are reasonable
    assert!(position.x() > 0.0 && position.x() < 100.0);
    assert!(position.y() > 0.0 && position.y() < 100.0);
    assert!(scale > 1.0 && scale < 2.0);
    assert!(color > 128 && color <= 255);

    Ok(())
}
