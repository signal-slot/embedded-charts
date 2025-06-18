//! Comprehensive tests for data aggregation and downsampling functionality

use embedded_charts::{
    data::{
        aggregation::{
            AggregationConfig, AggregationStrategy, DataAggregation, DownsamplingConfig,
        },
        DataPoint, DataSeries, Point2D, StaticDataSeries,
    },
    error::DataError,
};

// Helper function to create test data
fn create_test_data() -> StaticDataSeries<Point2D, 256> {
    let mut series = StaticDataSeries::new();
    let points = [
        (0.0, 10.0),
        (1.0, 20.0),
        (2.0, 5.0),
        (3.0, 30.0),
        (4.0, 15.0),
        (5.0, 25.0),
        (6.0, 8.0),
        (7.0, 35.0),
    ];

    for (x, y) in points.iter() {
        series.push(Point2D::new(*x, *y)).unwrap();
    }

    series
}

fn create_large_test_data() -> StaticDataSeries<Point2D, 512> {
    let mut series = StaticDataSeries::new();

    for i in 0..100 {
        let x = i as f32;
        let y = (x * 0.1).sin() * 50.0 + 50.0 + (i % 10) as f32; // Sin wave with some variation
        series.push(Point2D::new(x, y)).unwrap();
    }

    series
}

mod aggregation_config_tests {
    use super::*;

    #[test]
    fn test_aggregation_config_default() {
        let config = AggregationConfig::default();
        assert_eq!(config.strategy, AggregationStrategy::Mean);
        assert_eq!(config.target_points, 100);
        assert!(config.preserve_endpoints);
        assert_eq!(config.min_group_size, 1);
    }

    #[test]
    fn test_aggregation_config_custom() {
        let config = AggregationConfig {
            strategy: AggregationStrategy::Max,
            target_points: 50,
            preserve_endpoints: false,
            min_group_size: 2,
        };

        assert_eq!(config.strategy, AggregationStrategy::Max);
        assert_eq!(config.target_points, 50);
        assert!(!config.preserve_endpoints);
        assert_eq!(config.min_group_size, 2);
    }
}

mod downsampling_config_tests {
    use super::*;

    #[test]
    fn test_downsampling_config_default() {
        let config = DownsamplingConfig::default();
        assert_eq!(config.max_points, 1000);
        assert!(config.preserve_endpoints);
        assert_eq!(config.min_reduction_ratio, 1.5);
    }

    #[test]
    fn test_downsampling_config_custom() {
        let config = DownsamplingConfig {
            max_points: 200,
            preserve_endpoints: false,
            min_reduction_ratio: 2.0,
        };

        assert_eq!(config.max_points, 200);
        assert!(!config.preserve_endpoints);
        assert_eq!(config.min_reduction_ratio, 2.0);
    }
}

mod group_stats_tests {
    use super::*;

    #[test]
    fn test_group_stats_calculation() {
        let series = create_test_data();
        let stats = series.calculate_group_stats(series.as_slice()).unwrap();

        assert_eq!(stats.count, 8);
        assert_eq!(stats.min_x, 0.0);
        assert_eq!(stats.max_x, 7.0);
        assert_eq!(stats.min_y, 5.0);
        assert_eq!(stats.max_y, 35.0);
        assert_eq!(stats.first.x(), 0.0);
        assert_eq!(stats.first.y(), 10.0);
        assert_eq!(stats.last.x(), 7.0);
        assert_eq!(stats.last.y(), 35.0);

        // Mean should be (0+1+2+3+4+5+6+7)/8 = 3.5 for X
        assert_eq!(stats.mean_x, 3.5);
        // Mean should be (10+20+5+30+15+25+8+35)/8 = 18.5 for Y
        assert_eq!(stats.mean_y, 18.5);
    }

    #[test]
    fn test_group_stats_single_point() {
        let series = create_test_data();
        let stats = series
            .calculate_group_stats(&[Point2D::new(5.0, 15.0)])
            .unwrap();

        assert_eq!(stats.count, 1);
        assert_eq!(stats.min_x, 5.0);
        assert_eq!(stats.max_x, 5.0);
        assert_eq!(stats.min_y, 15.0);
        assert_eq!(stats.max_y, 15.0);
        assert_eq!(stats.mean_x, 5.0);
        assert_eq!(stats.mean_y, 15.0);
        assert_eq!(stats.first.x(), 5.0);
        assert_eq!(stats.last.x(), 5.0);
    }

    #[test]
    fn test_group_stats_empty_error() {
        let series = create_test_data();
        let result = series.calculate_group_stats(&[]);
        assert!(matches!(result, Err(DataError::InsufficientData { .. })));
    }
}

mod mean_aggregation_tests {
    use super::*;

    #[test]
    fn test_mean_aggregation_basic() {
        let series = create_test_data();

        let config = AggregationConfig {
            strategy: AggregationStrategy::Mean,
            target_points: 4,
            preserve_endpoints: false,
            min_group_size: 1,
        };

        let aggregated: StaticDataSeries<Point2D, 256> = series.aggregate(&config).unwrap();
        assert_eq!(aggregated.len(), 4);

        // Each group should have 2 points (8 total / 4 target = 2 per group)
        // Group 1: (0,10) and (1,20) -> mean (0.5, 15.0)
        let first = aggregated.get(0).unwrap();
        assert_eq!(first.x(), 0.5);
        assert_eq!(first.y(), 15.0);

        // Group 2: (2,5) and (3,30) -> mean (2.5, 17.5)
        let second = aggregated.get(1).unwrap();
        assert_eq!(second.x(), 2.5);
        assert_eq!(second.y(), 17.5);
    }

    #[test]
    fn test_mean_aggregation_with_endpoints() {
        let series = create_test_data();

        let config = AggregationConfig {
            strategy: AggregationStrategy::Mean,
            target_points: 4,
            preserve_endpoints: true,
            min_group_size: 1,
        };

        let aggregated: StaticDataSeries<Point2D, 256> = series.aggregate(&config).unwrap();

        // Should preserve first and last points exactly
        let first = aggregated.get(0).unwrap();
        assert_eq!(first.x(), 0.0);
        assert_eq!(first.y(), 10.0);

        let last = aggregated.get(aggregated.len() - 1).unwrap();
        assert_eq!(last.x(), 7.0);
        assert_eq!(last.y(), 35.0);
    }

    #[test]
    fn test_mean_aggregation_no_reduction_needed() {
        let series = create_test_data();

        let config = AggregationConfig {
            strategy: AggregationStrategy::Mean,
            target_points: 20, // More than we have
            preserve_endpoints: false,
            min_group_size: 1,
        };

        let aggregated: StaticDataSeries<Point2D, 256> = series.aggregate(&config).unwrap();
        assert_eq!(aggregated.len(), series.len()); // Should be unchanged

        // Points should be identical
        for i in 0..series.len() {
            let original = series.get(i).unwrap();
            let aggregated_point = aggregated.get(i).unwrap();
            assert_eq!(original.x(), aggregated_point.x());
            assert_eq!(original.y(), aggregated_point.y());
        }
    }
}

mod other_aggregation_strategies_tests {
    use super::*;

    #[test]
    fn test_first_aggregation() {
        let series = create_test_data();

        let config = AggregationConfig {
            strategy: AggregationStrategy::First,
            target_points: 4,
            preserve_endpoints: false,
            min_group_size: 1,
        };

        let aggregated: StaticDataSeries<Point2D, 256> = series.aggregate(&config).unwrap();
        assert_eq!(aggregated.len(), 4);

        // Each group should take the first point
        let first = aggregated.get(0).unwrap();
        assert_eq!(first.x(), 0.0);
        assert_eq!(first.y(), 10.0);

        let second = aggregated.get(1).unwrap();
        assert_eq!(second.x(), 2.0);
        assert_eq!(second.y(), 5.0);
    }

    #[test]
    fn test_last_aggregation() {
        let series = create_test_data();

        let config = AggregationConfig {
            strategy: AggregationStrategy::Last,
            target_points: 4,
            preserve_endpoints: false,
            min_group_size: 1,
        };

        let aggregated: StaticDataSeries<Point2D, 256> = series.aggregate(&config).unwrap();
        assert_eq!(aggregated.len(), 4);

        // Each group should take the last point
        let first = aggregated.get(0).unwrap();
        assert_eq!(first.x(), 1.0);
        assert_eq!(first.y(), 20.0);
    }

    #[test]
    fn test_max_aggregation() {
        let series = create_test_data();

        let config = AggregationConfig {
            strategy: AggregationStrategy::Max,
            target_points: 4,
            preserve_endpoints: false,
            min_group_size: 1,
        };

        let aggregated: StaticDataSeries<Point2D, 256> = series.aggregate(&config).unwrap();
        assert_eq!(aggregated.len(), 4);

        // First group: (0,10) and (1,20) -> max should be (1,20)
        let first = aggregated.get(0).unwrap();
        assert_eq!(first.x(), 1.0);
        assert_eq!(first.y(), 20.0);

        // Second group: (2,5) and (3,30) -> max should be (3,30)
        let second = aggregated.get(1).unwrap();
        assert_eq!(second.x(), 3.0);
        assert_eq!(second.y(), 30.0);
    }

    #[test]
    fn test_min_aggregation() {
        let series = create_test_data();

        let config = AggregationConfig {
            strategy: AggregationStrategy::Min,
            target_points: 4,
            preserve_endpoints: false,
            min_group_size: 1,
        };

        let aggregated: StaticDataSeries<Point2D, 256> = series.aggregate(&config).unwrap();
        assert_eq!(aggregated.len(), 4);

        // First group: (0,10) and (1,20) -> min should be (0,10)
        let first = aggregated.get(0).unwrap();
        assert_eq!(first.x(), 0.0);
        assert_eq!(first.y(), 10.0);

        // Second group: (2,5) and (3,30) -> min should be (2,5)
        let second = aggregated.get(1).unwrap();
        assert_eq!(second.x(), 2.0);
        assert_eq!(second.y(), 5.0);
    }

    #[test]
    fn test_minmax_aggregation() {
        let series = create_test_data();

        let config = AggregationConfig {
            strategy: AggregationStrategy::MinMax,
            target_points: 4,
            preserve_endpoints: false,
            min_group_size: 1,
        };

        let aggregated: StaticDataSeries<Point2D, 256> = series.aggregate(&config).unwrap();
        assert_eq!(aggregated.len(), 4);

        // MinMax should preserve extreme values (currently returns point with max Y)
        let first = aggregated.get(0).unwrap();
        assert_eq!(first.y(), 20.0); // Max in first group
    }

    #[test]
    fn test_median_aggregation() {
        // Create data with clear median values
        let mut series: StaticDataSeries<Point2D, 256> = StaticDataSeries::new();
        series.push(Point2D::new(0.0, 10.0)).unwrap();
        series.push(Point2D::new(1.0, 20.0)).unwrap();
        series.push(Point2D::new(2.0, 30.0)).unwrap();
        series.push(Point2D::new(3.0, 40.0)).unwrap();

        let config = AggregationConfig {
            strategy: AggregationStrategy::Median,
            target_points: 2,
            preserve_endpoints: false,
            min_group_size: 1,
        };

        let aggregated: StaticDataSeries<Point2D, 256> = series.aggregate(&config).unwrap();
        assert_eq!(aggregated.len(), 2);

        // First group: (0,10) and (1,20) -> median should be (0.5, 15.0)
        let first = aggregated.get(0).unwrap();
        assert_eq!(first.x(), 0.5);
        assert_eq!(first.y(), 15.0);

        // Second group: (2,30) and (3,40) -> median should be (2.5, 35.0)
        let second = aggregated.get(1).unwrap();
        assert_eq!(second.x(), 2.5);
        assert_eq!(second.y(), 35.0);
    }
}

mod lttb_downsampling_tests {
    use super::*;

    #[test]
    fn test_lttb_basic() {
        let series = create_large_test_data();

        let config = DownsamplingConfig {
            max_points: 20,
            preserve_endpoints: true,
            min_reduction_ratio: 1.0,
        };

        let downsampled: StaticDataSeries<Point2D, 256> = series.downsample_lttb(&config).unwrap();
        assert_eq!(downsampled.len(), 20);

        // Should preserve endpoints
        let first = downsampled.get(0).unwrap();
        let original_first = series.get(0).unwrap();
        assert_eq!(first.x(), original_first.x());
        assert_eq!(first.y(), original_first.y());

        let last = downsampled.get(downsampled.len() - 1).unwrap();
        let original_last = series.get(series.len() - 1).unwrap();
        assert_eq!(last.x(), original_last.x());
        assert_eq!(last.y(), original_last.y());
    }

    #[test]
    fn test_lttb_no_reduction_needed() {
        let series = create_test_data(); // 8 points

        let config = DownsamplingConfig {
            max_points: 20, // More than we have
            preserve_endpoints: true,
            min_reduction_ratio: 1.0,
        };

        let downsampled: StaticDataSeries<Point2D, 256> = series.downsample_lttb(&config).unwrap();
        assert_eq!(downsampled.len(), series.len()); // Should be unchanged
    }

    #[test]
    fn test_lttb_insufficient_reduction() {
        let series = create_test_data(); // 8 points

        let config = DownsamplingConfig {
            max_points: 6,
            preserve_endpoints: true,
            min_reduction_ratio: 2.0, // Require 2x reduction, but 8->6 is only 1.33x
        };

        let downsampled: StaticDataSeries<Point2D, 256> = series.downsample_lttb(&config).unwrap();
        assert_eq!(downsampled.len(), series.len()); // Should be unchanged due to ratio check
    }

    #[test]
    fn test_lttb_minimal_points() {
        let series = create_test_data();

        let config = DownsamplingConfig {
            max_points: 2,
            preserve_endpoints: true,
            min_reduction_ratio: 1.0,
        };

        let downsampled: StaticDataSeries<Point2D, 256> = series.downsample_lttb(&config).unwrap();
        assert_eq!(downsampled.len(), 2);

        // Should be first and last points
        let first = downsampled.get(0).unwrap();
        assert_eq!(first.x(), 0.0);

        let last = downsampled.get(1).unwrap();
        assert_eq!(last.x(), 7.0);
    }

    #[test]
    fn test_lttb_single_point() {
        let series = create_test_data();

        let config = DownsamplingConfig {
            max_points: 1,
            preserve_endpoints: true,
            min_reduction_ratio: 1.0,
        };

        let downsampled: StaticDataSeries<Point2D, 256> = series.downsample_lttb(&config).unwrap();
        assert_eq!(downsampled.len(), 1);

        // Should be the first point
        let first = downsampled.get(0).unwrap();
        assert_eq!(first.x(), 0.0);
        assert_eq!(first.y(), 10.0);
    }
}

mod uniform_downsampling_tests {
    use super::*;

    #[test]
    fn test_uniform_basic() {
        let series = create_test_data(); // 8 points

        let config = DownsamplingConfig {
            max_points: 4,
            preserve_endpoints: true,
            min_reduction_ratio: 1.0,
        };

        let downsampled: StaticDataSeries<Point2D, 256> =
            series.downsample_uniform(&config).unwrap();
        assert_eq!(downsampled.len(), 4);

        // Should sample uniformly across the range
        // With 8 points -> 4 points, step = 2.0
        // Indices should be approximately: 0, 2, 4, 6
        let points: Vec<_> = (0..downsampled.len())
            .map(|i| downsampled.get(i).unwrap())
            .collect();

        // Check that we get a reasonable sampling
        assert!(points[0].x() < points[1].x());
        assert!(points[1].x() < points[2].x());
        assert!(points[2].x() < points[3].x());
    }

    #[test]
    fn test_uniform_no_reduction_needed() {
        let series = create_test_data(); // 8 points

        let config = DownsamplingConfig {
            max_points: 20, // More than we have
            preserve_endpoints: true,
            min_reduction_ratio: 1.0,
        };

        let downsampled: StaticDataSeries<Point2D, 256> =
            series.downsample_uniform(&config).unwrap();
        assert_eq!(downsampled.len(), series.len()); // Should be unchanged
    }

    #[test]
    fn test_uniform_single_point() {
        let series = create_test_data();

        let config = DownsamplingConfig {
            max_points: 1,
            preserve_endpoints: true,
            min_reduction_ratio: 1.0,
        };

        let downsampled: StaticDataSeries<Point2D, 256> =
            series.downsample_uniform(&config).unwrap();
        assert_eq!(downsampled.len(), 1);

        let point = downsampled.get(0).unwrap();
        assert_eq!(point.x(), 0.0);
        assert_eq!(point.y(), 10.0);
    }
}

mod edge_cases_tests {
    use super::*;

    #[test]
    fn test_empty_series_aggregation() {
        let series: StaticDataSeries<Point2D, 256> = StaticDataSeries::new();

        let config = AggregationConfig::default();
        let aggregated: StaticDataSeries<Point2D, 256> = series.aggregate(&config).unwrap();
        assert_eq!(aggregated.len(), 0);
    }

    #[test]
    fn test_empty_series_lttb() {
        let series: StaticDataSeries<Point2D, 256> = StaticDataSeries::new();

        let config = DownsamplingConfig::default();
        let downsampled: StaticDataSeries<Point2D, 256> = series.downsample_lttb(&config).unwrap();
        assert_eq!(downsampled.len(), 0);
    }

    #[test]
    fn test_empty_series_uniform() {
        let series: StaticDataSeries<Point2D, 256> = StaticDataSeries::new();

        let config = DownsamplingConfig::default();
        let downsampled: StaticDataSeries<Point2D, 256> =
            series.downsample_uniform(&config).unwrap();
        assert_eq!(downsampled.len(), 0);
    }

    #[test]
    fn test_single_point_aggregation() {
        let mut series: StaticDataSeries<Point2D, 256> = StaticDataSeries::new();
        series.push(Point2D::new(1.0, 2.0)).unwrap();

        let config = AggregationConfig::default();
        let aggregated: StaticDataSeries<Point2D, 256> = series.aggregate(&config).unwrap();
        assert_eq!(aggregated.len(), 1);

        let point = aggregated.get(0).unwrap();
        assert_eq!(point.x(), 1.0);
        assert_eq!(point.y(), 2.0);
    }

    #[test]
    fn test_large_group_size() {
        let series = create_test_data();

        let config = AggregationConfig {
            strategy: AggregationStrategy::Mean,
            target_points: 1,
            preserve_endpoints: false,
            min_group_size: 1,
        };

        let aggregated: StaticDataSeries<Point2D, 256> = series.aggregate(&config).unwrap();
        assert_eq!(aggregated.len(), 1);

        // Should be the mean of all points
        let point = aggregated.get(0).unwrap();
        assert_eq!(point.x(), 3.5); // Mean of 0,1,2,3,4,5,6,7
        assert_eq!(point.y(), 18.5); // Mean of Y values
    }
}

mod performance_tests {
    use super::*;

    #[test]
    fn test_large_dataset_performance() {
        // Create a larger dataset to test performance
        let mut series: StaticDataSeries<Point2D, 1024> = StaticDataSeries::new();

        for i in 0..1000 {
            let x = i as f32;
            let y = (x * 0.01).sin() * 100.0 + 100.0;
            series.push(Point2D::new(x, y)).unwrap();
        }

        // Test aggregation performance
        let config = AggregationConfig {
            strategy: AggregationStrategy::Mean,
            target_points: 100,
            preserve_endpoints: true,
            min_group_size: 1,
        };

        let aggregated: StaticDataSeries<Point2D, 256> = series.aggregate(&config).unwrap();
        // With endpoint preservation, we may get slightly more than target_points
        assert!(aggregated.len() >= 100 && aggregated.len() <= 102);

        // Test LTTB performance
        let lttb_config = DownsamplingConfig {
            max_points: 100,
            preserve_endpoints: true,
            min_reduction_ratio: 1.0,
        };

        let downsampled: StaticDataSeries<Point2D, 256> =
            series.downsample_lttb(&lttb_config).unwrap();
        assert_eq!(downsampled.len(), 100);
    }
}
