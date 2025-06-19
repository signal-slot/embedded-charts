//! Demonstrates data aggregation and downsampling for large datasets
#![cfg(feature = "std")]

use embedded_charts::{
    chart::{Chart, ChartBuilder, ChartConfig, LineChart},
    data::{
        aggregation::{
            AggregationConfig, AggregationStrategy, DataAggregation, DownsamplingConfig,
        },
        DataSeries, Point2D, StaticDataSeries,
    },
};
use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{Circle, PrimitiveStyle, Rectangle},
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};

/// Generate a large dataset with noise and trends
fn generate_large_dataset() -> StaticDataSeries<Point2D, 1024> {
    let mut series = StaticDataSeries::new();

    for i in 0..1000 {
        let x = i as f32 * 0.1;

        // Create a signal with multiple components:
        // 1. Slow trend (sine wave with long period)
        let trend = 20.0 * (x * 0.05).sin();

        // 2. Medium frequency oscillation
        let oscillation = 10.0 * (x * 0.5).sin();

        // 3. High frequency noise
        let noise = 3.0 * (x * 5.0).sin() * (x * 7.0).cos();

        // 4. Occasional spikes
        let spike = if i % 200 == 0 { 15.0 } else { 0.0 };

        let y = 50.0 + trend + oscillation + noise + spike;

        series.push(Point2D::new(x, y)).unwrap();
    }

    series
}

/// Generate high-frequency sensor data
fn generate_sensor_data() -> StaticDataSeries<Point2D, 256> {
    let mut series = StaticDataSeries::new();

    for i in 0..250 {
        let time = i as f32 * 0.01; // 100Hz sampling

        // Simulate temperature sensor with:
        // - Daily cycle
        let daily = 5.0 * (time * 0.1).cos();
        // - Random noise
        let noise = 0.5 * (time * 50.0).sin() * (time * 73.0).cos();
        // - Sensor drift
        let drift = time * 0.02;

        let temperature = 22.0 + daily + noise + drift;

        series.push(Point2D::new(time, temperature)).unwrap();
    }

    series
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create display
    let mut display = SimulatorDisplay::<Rgb565>::new(Size::new(1200, 800));
    display.clear(Rgb565::WHITE)?;

    // Create chart areas for comparison
    let chart_areas = [
        Rectangle::new(Point::new(50, 50), Size::new(500, 160)), // Original large dataset
        Rectangle::new(Point::new(650, 50), Size::new(500, 160)), // Mean aggregation
        Rectangle::new(Point::new(50, 270), Size::new(500, 160)), // MinMax aggregation
        Rectangle::new(Point::new(650, 270), Size::new(500, 160)), // LTTB downsampling
        Rectangle::new(Point::new(50, 490), Size::new(500, 160)), // Original sensor data
        Rectangle::new(Point::new(650, 490), Size::new(500, 160)), // Uniform downsampling
    ];

    // Demo 1: Large noisy dataset with different aggregation strategies
    {
        let original_data = generate_large_dataset();
        println!("Original dataset: {} points", original_data.len());

        // Chart 1: Original data (first 250 points to fit in display)
        let mut truncated: StaticDataSeries<Point2D, 256> = StaticDataSeries::new();
        for (i, point) in original_data.iter().enumerate() {
            if i < 250 {
                truncated.push(point)?;
            } else {
                break;
            }
        }

        let chart = LineChart::<Rgb565>::builder()
            .line_color(Rgb565::BLUE)
            .line_width(1)
            .with_title("Original (250/1000 pts)")
            .build()?;

        let config = ChartConfig::default();
        chart.draw(&truncated, &config, chart_areas[0], &mut display)?;

        // Chart 2: Mean aggregation
        let mean_config = AggregationConfig {
            strategy: AggregationStrategy::Mean,
            target_points: 50,
            preserve_endpoints: true,
            ..Default::default()
        };

        let mean_aggregated: StaticDataSeries<Point2D, 256> =
            original_data.aggregate(&mean_config)?;
        println!("Mean aggregated: {} points", mean_aggregated.len());

        let chart = LineChart::<Rgb565>::builder()
            .line_color(Rgb565::GREEN)
            .line_width(2)
            .with_title("Mean Aggregation (50 pts)")
            .build()?;

        chart.draw(&mean_aggregated, &config, chart_areas[1], &mut display)?;

        // Chart 3: MinMax aggregation (preserves extremes)
        let minmax_config = AggregationConfig {
            strategy: AggregationStrategy::MinMax,
            target_points: 50,
            preserve_endpoints: true,
            ..Default::default()
        };

        let minmax_aggregated: StaticDataSeries<Point2D, 256> =
            original_data.aggregate(&minmax_config)?;
        println!("MinMax aggregated: {} points", minmax_aggregated.len());

        let chart = LineChart::<Rgb565>::builder()
            .line_color(Rgb565::RED)
            .line_width(2)
            .with_title("MinMax Aggregation (50 pts)")
            .build()?;

        chart.draw(&minmax_aggregated, &config, chart_areas[2], &mut display)?;

        // Chart 4: LTTB downsampling (preserves visual characteristics)
        let lttb_config = DownsamplingConfig {
            max_points: 50,
            preserve_endpoints: true,
            min_reduction_ratio: 1.0,
        };

        let lttb_downsampled: StaticDataSeries<Point2D, 256> =
            original_data.downsample_lttb(&lttb_config)?;
        println!("LTTB downsampled: {} points", lttb_downsampled.len());

        let chart = LineChart::<Rgb565>::builder()
            .line_color(Rgb565::MAGENTA)
            .line_width(2)
            .with_title("LTTB Downsampling (50 pts)")
            .build()?;

        chart.draw(&lttb_downsampled, &config, chart_areas[3], &mut display)?;
    }

    // Demo 2: High-frequency sensor data
    {
        let sensor_data = generate_sensor_data();
        println!("Sensor data: {} points", sensor_data.len());

        // Chart 5: Original sensor data
        let chart = LineChart::<Rgb565>::builder()
            .line_color(Rgb565::BLUE)
            .line_width(1)
            .with_title("High-Freq Sensor (250 pts)")
            .build()?;

        let config = ChartConfig::default();
        chart.draw(&sensor_data, &config, chart_areas[4], &mut display)?;

        // Chart 6: Uniform downsampling
        let uniform_config = DownsamplingConfig {
            max_points: 40,
            preserve_endpoints: true,
            min_reduction_ratio: 1.0,
        };

        let uniform_downsampled: StaticDataSeries<Point2D, 256> =
            sensor_data.downsample_uniform(&uniform_config)?;
        println!("Uniform downsampled: {} points", uniform_downsampled.len());

        let chart = LineChart::<Rgb565>::builder()
            .line_color(Rgb565::CYAN)
            .line_width(2)
            .with_title("Uniform Downsampling (40 pts)")
            .build()?;

        chart.draw(&uniform_downsampled, &config, chart_areas[5], &mut display)?;
    }

    // Add labels explaining the benefits
    let label_positions = [
        Point::new(50, 230),
        Point::new(650, 230),
        Point::new(50, 450),
        Point::new(650, 450),
        Point::new(50, 670),
        Point::new(650, 670),
    ];

    let labels = [
        "Shows first 250 of 1000 points",
        "Smooth, preserves trends",
        "Preserves peaks and valleys",
        "Best visual preservation",
        "Raw high-freq sensor data",
        "Reduced while preserving trend",
    ];

    for (pos, label) in label_positions.iter().zip(labels.iter()) {
        // Draw a small indicator dot
        Circle::with_center(*pos, 3)
            .into_styled(PrimitiveStyle::with_fill(Rgb565::BLACK))
            .draw(&mut display)?;

        // Note: In a real implementation, we would render the text labels here
        println!("Label at {pos:?}: {label}");
    }

    // Performance comparison
    println!("\n=== Performance Comparison ===");
    println!("Original data: 1000 points");
    println!("Mean aggregation: 20x reduction, preserves overall trend");
    println!("MinMax aggregation: 20x reduction, preserves extremes");
    println!("LTTB downsampling: 20x reduction, best visual fidelity");
    println!("Uniform downsampling: 6x reduction, fastest processing");

    // Show the display
    let output_settings = OutputSettingsBuilder::new().build();
    Window::new("Data Aggregation & Downsampling Demo", &output_settings).show_static(&display);

    Ok(())
}
