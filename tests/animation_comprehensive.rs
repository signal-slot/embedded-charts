//! Comprehensive tests for chart animation functionality

#[cfg(all(feature = "animations", feature = "line"))]
mod animation_tests {
    use embedded_charts::animation::*;
    use embedded_charts::data::{DataPoint, DataSeries, Point2D, StaticDataSeries};

    // Helper to create test data
    fn create_test_data() -> StaticDataSeries<Point2D, 256> {
        let mut series = StaticDataSeries::new();
        let points = [
            (0.0, 10.0),
            (1.0, 20.0),
            (2.0, 15.0),
            (3.0, 25.0),
            (4.0, 18.0),
        ];

        for (x, y) in points.iter() {
            series.push(Point2D::new(*x, *y)).unwrap();
        }

        series
    }

    fn create_alternative_data() -> StaticDataSeries<Point2D, 256> {
        let mut series = StaticDataSeries::new();
        let points = [
            (0.0, 15.0),
            (1.0, 25.0),
            (2.0, 20.0),
            (3.0, 30.0),
            (4.0, 22.0),
        ];

        for (x, y) in points.iter() {
            series.push(Point2D::new(*x, *y)).unwrap();
        }

        series
    }

    #[test]
    fn test_chart_animator_creation() {
        let from_data = create_test_data();
        let to_data = create_alternative_data();

        let animator =
            ChartAnimator::new(from_data.clone(), to_data.clone(), EasingFunction::Linear);

        assert_eq!(animator.from_state().len(), from_data.len());
        assert_eq!(animator.to_state().len(), to_data.len());
        assert_eq!(animator.easing(), EasingFunction::Linear);
    }

    #[test]
    fn test_chart_animator_interpolation() {
        let from_data = create_test_data();
        let to_data = create_alternative_data();

        let animator = ChartAnimator::new(from_data, to_data, EasingFunction::Linear);

        // Test at 0% progress
        let data_at_0 = animator.value_at(0).unwrap();
        assert_eq!(data_at_0.len(), 5);
        assert_eq!(data_at_0.get(0).unwrap().y(), 10.0);

        // Test at 50% progress
        let data_at_50 = animator.value_at(50).unwrap();
        assert_eq!(data_at_50.len(), 5);
        assert_eq!(data_at_50.get(0).unwrap().y(), 12.5); // Halfway between 10 and 15

        // Test at 100% progress
        let data_at_100 = animator.value_at(100).unwrap();
        assert_eq!(data_at_100.len(), 5);
        assert_eq!(data_at_100.get(0).unwrap().y(), 15.0);
    }

    #[test]
    fn test_easing_functions() {
        // Linear easing
        assert_eq!(EasingFunction::Linear.apply(0.0), 0.0);
        assert_eq!(EasingFunction::Linear.apply(0.5), 0.5);
        assert_eq!(EasingFunction::Linear.apply(1.0), 1.0);

        // Ease In (quadratic)
        assert_eq!(EasingFunction::EaseIn.apply(0.0), 0.0);
        assert_eq!(EasingFunction::EaseIn.apply(0.5), 0.25);
        assert_eq!(EasingFunction::EaseIn.apply(1.0), 1.0);

        // Ease Out
        assert_eq!(EasingFunction::EaseOut.apply(0.0), 0.0);
        assert_eq!(EasingFunction::EaseOut.apply(0.5), 0.75);
        assert_eq!(EasingFunction::EaseOut.apply(1.0), 1.0);
    }

    #[test]
    fn test_streaming_animator() {
        let mut animator = StreamingAnimator::new();

        assert!(animator.is_empty());
        assert_eq!(animator.len(), 0);

        // Add data points
        animator.push_data(Point2D::new(0.0, 10.0));
        animator.push_data(Point2D::new(1.0, 20.0));
        animator.push_data(Point2D::new(2.0, 15.0));

        assert!(!animator.is_empty());
        assert_eq!(animator.len(), 3);

        // Check interpolation progress
        animator.set_interpolation_progress(50);
        assert_eq!(animator.interpolation_progress(), 50);
    }

    #[test]
    fn test_multi_state_animator() {
        let mut animator: MultiStateAnimator<f32, 8> = MultiStateAnimator::new();

        // Add keyframes
        animator
            .add_keyframe(0, 0.0, EasingFunction::Linear)
            .unwrap();
        animator
            .add_keyframe(50, 50.0, EasingFunction::EaseIn)
            .unwrap();
        animator
            .add_keyframe(100, 100.0, EasingFunction::EaseOut)
            .unwrap();

        assert_eq!(animator.keyframe_count(), 3);

        // Test interpolation at various points
        assert_eq!(animator.value_at(0), Some(0.0));
        assert_eq!(animator.value_at(25), Some(12.5)); // 25% of the way from 0 to 50
        assert_eq!(animator.value_at(50), Some(50.0));
        assert_eq!(animator.value_at(100), Some(100.0));
    }

    #[test]
    fn test_time_based_progress() {
        use embedded_charts::time::ManualTimeProvider;

        let mut time_provider = ManualTimeProvider::new();
        let mut progress_calc = TimeBasedProgress::new(1000); // 1 second duration

        // Initial progress should be 0
        assert_eq!(progress_calc.progress_from_time(&time_provider), 0);

        // After 250ms, should be 25%
        time_provider.advance_ms(250);
        assert_eq!(progress_calc.progress_from_time(&time_provider), 25);

        // After 500ms total, should be 50%
        time_provider.advance_ms(250);
        assert_eq!(progress_calc.progress_from_time(&time_provider), 50);

        // After 1000ms total, should be 100%
        time_provider.advance_ms(500);
        assert_eq!(progress_calc.progress_from_time(&time_provider), 100);
    }

    #[test]
    fn test_animation_progress_values() {
        // Progress is just a type alias for u8, so direct values are used
        let progress: Progress = 150;
        assert_eq!(progress, 150); // u8 can hold values up to 255

        let progress: Progress = 50;
        assert_eq!(progress, 50);
    }

    #[test]
    fn test_interpolatable_trait() {
        // Test f32 interpolation
        let a = 10.0f32;
        let b = 20.0f32;
        let result = a.interpolate(b, 0.5).unwrap();
        assert_eq!(result, 15.0);

        // Test i32 interpolation
        let a = 10i32;
        let b = 20i32;
        let result = a.interpolate(b, 0.5).unwrap();
        assert_eq!(result, 15);

        // Test Point2D interpolation
        let p1 = Point2D::new(0.0, 10.0);
        let p2 = Point2D::new(10.0, 20.0);
        let result = p1.interpolate(p2, 0.5).unwrap();
        assert_eq!(result.x(), 5.0);
        assert_eq!(result.y(), 15.0);
    }

    #[test]
    fn test_streaming_buffer_capacity() {
        let mut animator = StreamingAnimator::new();

        // Fill buffer beyond capacity
        for i in 0..150 {
            animator.push_data(Point2D::new(i as f32, i as f32 * 2.0));
        }

        // Should only keep last 100 points (sliding window)
        assert_eq!(animator.len(), 100);
    }

    #[test]
    fn test_animation_with_different_easings() {
        let from_data = create_test_data();
        let to_data = create_alternative_data();

        let easings = [
            EasingFunction::Linear,
            EasingFunction::EaseIn,
            EasingFunction::EaseOut,
            EasingFunction::EaseInOut,
        ];

        for easing in easings.iter() {
            let animator = ChartAnimator::new(from_data.clone(), to_data.clone(), *easing);

            // Test that all easings produce valid results
            let data_at_25 = animator.value_at(25);
            let data_at_50 = animator.value_at(50);
            let data_at_75 = animator.value_at(75);

            assert!(data_at_25.is_some());
            assert!(data_at_50.is_some());
            assert!(data_at_75.is_some());
        }
    }
}
