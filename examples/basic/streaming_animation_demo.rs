//! Streaming Animation Demo - Real-time Data Updates
//!
//! This example demonstrates the new StreamingAnimator capabilities for real-time data visualization.
//! It shows multiple data series with different update rates and smooth transitions using the new API.
//!
//! Features demonstrated:
//! - Real-time data streaming with StreamingAnimator
//! - Multiple data series with different characteristics
//! - Smooth visual updates with proper timing
//! - External progress control for interpolation
//!
//! Run with: cargo run --example streaming_animation_demo --features "std,animations"

use embedded_charts::prelude::*;

// Import the common abstraction
#[path = "../common/mod.rs"]
mod common;

use common::WindowConfig;

#[cfg(feature = "std")]
fn main() -> ChartResult<()> {
    // Create streaming animators for different data types
    let mut temperature_stream = StreamingAnimator::<Point2D>::new();
    let mut cpu_usage_stream = StreamingAnimator::<Point2D>::new();
    let mut network_stream = StreamingAnimator::<Point2D>::new();

    // Create line charts for visualization with legends
    let temp_chart = LineChart::builder()
        .line_color(Rgb565::RED)
        .line_width(2)
        .build()?;

    let cpu_chart = LineChart::builder()
        .line_color(Rgb565::BLUE)
        .line_width(2)
        .build()?;

    let network_chart = LineChart::builder()
        .line_color(Rgb565::GREEN)
        .line_width(2)
        .build()?;

    // Simulation state
    let mut last_temp_update = 0.0f32;
    let mut last_cpu_update = 0.0f32;
    let mut last_network_update = 0.0f32;

    println!("🚀 Starting Streaming Animation Demo");
    println!("📊 Red: Temperature (10 Hz), Blue: CPU Usage (5 Hz), Green: Network I/O (20 Hz)");
    println!("⏱️  Each series updates at different rates to demonstrate streaming capabilities");

    // Pre-calculate layout dimensions and create reusable objects outside the loop
    let _chart_config = ChartConfig {
        background_color: Some(Rgb565::WHITE),
        margins: Margins {
            top: 5,
            right: 80, // Space for right-side legend
            bottom: 5,
            left: 5,
        },
        ..Default::default()
    };

    // Pre-create text style for labels
    use embedded_graphics::{
        mono_font::{ascii::FONT_6X10, MonoTextStyle},
        prelude::*,
        text::{Baseline, Text},
    };
    let text_style = MonoTextStyle::new(&FONT_6X10, Rgb565::BLACK);

    // Pre-create data series instances for reuse
    let mut temp_series = StaticDataSeries::<Point2D, 256>::new();
    let mut cpu_series = StaticDataSeries::<Point2D, 256>::new();
    let mut network_series = StaticDataSeries::<Point2D, 256>::new();

    // Pre-create update rate constants (in seconds)
    let _temp_update_rate = 0.1; // 10 Hz - every 100ms
    let _cpu_update_rate = 0.2; // 5 Hz - every 200ms
    let _network_update_rate = 0.05; // 20 Hz - every 50ms
    let _frame_delta_ms = 16; // ~60 FPS frame time

    // Pre-create data generation constants
    let _temp_base = 20.0;
    let _temp_amplitude_1 = 10.0;
    let _temp_amplitude_2 = 2.0;
    let _temp_freq_1 = 0.5;
    let _temp_freq_2 = 2.0;

    let _cpu_base = 30.0;
    let _cpu_amplitude = 20.0;
    let _cpu_freq = 0.3;
    let _cpu_spike_amplitude = 15.0;
    let _cpu_spike_interval = 3;

    let _network_base = 10.0;
    let _network_amplitude_1 = 25.0;
    let _network_amplitude_2 = 20.0;
    let _network_freq = 1.5;
    let _network_burst_multiplier = 10.0;
    let _network_burst_interval = 7;

    // Pre-calculate layout constants outside the render loop
    let chart_height_divisor = 3u32;
    let margin = 5u32;

    // Use the common visual example runner with animation loop
    common::window::run(
        WindowConfig::new("Streaming Animation Demo")
            .fps(60)
            .background(Rgb565::WHITE),
        move |display, viewport, elapsed| {
            let time = elapsed;

            // Generate and add new data points at different rates

            // Temperature data (10 Hz - every 100ms)
            if time - last_temp_update >= 0.1 {
                let temp_value = 20.0 + 10.0 * (time * 0.5).sin() + 2.0 * (time * 2.0).cos();
                temperature_stream.push_data(Point2D::new(time, temp_value));
                last_temp_update = time;
            }

            // CPU usage data (5 Hz - every 200ms)
            if time - last_cpu_update >= 0.2 {
                let cpu_value = 30.0
                    + 20.0 * (time * 0.3).sin()
                    + 15.0 * if (time as u32) % 3 == 0 { 1.0 } else { 0.0 };
                cpu_usage_stream.push_data(Point2D::new(time, cpu_value.min(100.0)));
                last_cpu_update = time;
            }

            // Network I/O data (20 Hz - every 50ms)
            if time - last_network_update >= 0.05 {
                let network_value = 10.0
                    + 25.0 * (time * 1.5).sin().abs()
                    + 20.0
                        * if (time * 10.0) as u32 % 7 == 0 {
                            1.0
                        } else {
                            0.0
                        };
                network_stream.push_data(Point2D::new(time, network_value.min(100.0)));
                last_network_update = time;
            }

            // Update streaming animations using delta time (backward compatibility)
            let _ = temperature_stream.update_with_delta(16); // ~60 FPS frame time
            let _ = cpu_usage_stream.update_with_delta(16);
            let _ = network_stream.update_with_delta(16);

            // Calculate layout for three charts using pre-calculated constants
            let chart_height = viewport.size.height / chart_height_divisor;

            // Temperature chart (top)
            let temp_viewport = Rectangle::new(
                viewport.top_left,
                Size::new(viewport.size.width, chart_height - margin),
            );

            // CPU chart (middle)
            let cpu_viewport = Rectangle::new(
                Point::new(
                    viewport.top_left.x,
                    viewport.top_left.y + chart_height as i32,
                ),
                Size::new(viewport.size.width, chart_height - margin),
            );

            // Network chart (bottom)
            let network_viewport = Rectangle::new(
                Point::new(
                    viewport.top_left.x,
                    viewport.top_left.y + (chart_height * 2) as i32,
                ),
                Size::new(viewport.size.width, chart_height - margin),
            );

            // Clear and reuse existing series instances
            temp_series.clear();
            cpu_series.clear();
            network_series.clear();

            // Collect data from streaming animators
            for point in temperature_stream.current_data() {
                let _ = temp_series.push(point);
            }
            for point in cpu_usage_stream.current_data() {
                let _ = cpu_series.push(point);
            }
            for point in network_stream.current_data() {
                let _ = network_series.push(point);
            }

            // Render charts
            if !temp_series.is_empty() {
                temp_chart.draw(&temp_series, temp_chart.config(), temp_viewport, display)?;
            }

            if !cpu_series.is_empty() {
                cpu_chart.draw(&cpu_series, cpu_chart.config(), cpu_viewport, display)?;
            }

            if !network_series.is_empty() {
                network_chart.draw(
                    &network_series,
                    network_chart.config(),
                    network_viewport,
                    display,
                )?;
            }

            // Draw labels using pre-created text style
            Text::with_baseline(
                "Temperature (°C)",
                Point::new(temp_viewport.top_left.x + 5, temp_viewport.top_left.y + 15),
                text_style,
                Baseline::Top,
            )
            .draw(display)
            .map_err(|_| ChartError::RenderingError)?;

            Text::with_baseline(
                "CPU Usage (%)",
                Point::new(cpu_viewport.top_left.x + 5, cpu_viewport.top_left.y + 15),
                text_style,
                Baseline::Top,
            )
            .draw(display)
            .map_err(|_| ChartError::RenderingError)?;

            Text::with_baseline(
                "Network I/O (MB/s)",
                Point::new(
                    network_viewport.top_left.x + 5,
                    network_viewport.top_left.y + 15,
                ),
                text_style,
                Baseline::Top,
            )
            .draw(display)
            .map_err(|_| ChartError::RenderingError)?;

            Ok(())
        },
    )
}

#[cfg(not(feature = "std"))]
fn main() {
    common::utils::print_feature_requirement("std", "streaming animation");
}
