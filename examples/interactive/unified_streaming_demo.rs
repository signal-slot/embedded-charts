//! Unified Streaming Demo - Comprehensive Real-time Data Architecture
//!
//! This example demonstrates the complete unified streaming architecture with:
//! - Multiple data sources feeding different animated charts
//! - Real-time performance metrics and memory usage monitoring
//! - Error handling and recovery scenarios
//! - Optimized memory management for embedded systems
//!
//! Features demonstrated:
//! - UnifiedStreamingBuffer with configurable parameters
//! - StreamingDataPipeline connecting sources to charts
//! - StreamingChartManager coordinating multiple charts
//! - Real-time performance monitoring and metrics
//! - Memory pressure handling and optimization
//! - Error recovery and fault tolerance
//!
//! Run with: cargo run --example unified_streaming_demo --features "std,animations"

use embedded_charts::prelude::*;

// Import the common abstraction
#[path = "../common/mod.rs"]
mod common;

use common::WindowConfig;

#[cfg(feature = "std")]
use std::time::Instant;

/// Sensor data simulator for realistic streaming scenarios
struct SensorSimulator {
    /// Current time in seconds
    time: f32,
    /// Temperature sensor state
    temp_phase: f32,
    /// Pressure sensor state  
    pressure_phase: f32,
    /// Humidity sensor state
    humidity_phase: f32,
    /// CPU usage simulation state
    cpu_load_base: f32,
    /// Memory usage simulation state
    memory_usage: f32,
    /// Network I/O simulation state
    network_burst_timer: f32,
}

impl SensorSimulator {
    fn new() -> Self {
        Self {
            time: 0.0,
            temp_phase: 0.0,
            pressure_phase: 0.0,
            humidity_phase: 0.0,
            cpu_load_base: 30.0,
            memory_usage: 40.0,
            network_burst_timer: 0.0,
        }
    }

    fn update(&mut self, delta_time: f32) {
        self.time += delta_time;
        self.temp_phase += delta_time * 0.5;
        self.pressure_phase += delta_time * 0.3;
        self.humidity_phase += delta_time * 0.7;
        self.network_burst_timer += delta_time;
    }

    fn get_temperature(&self) -> f32 {
        // Simulate realistic temperature variations (20-35°C)
        20.0 + 7.5 * (self.temp_phase).sin()
            + 2.0 * (self.temp_phase * 3.0).cos()
            + 1.0 * (self.time * 0.1).sin() // Daily variation
    }

    fn get_pressure(&self) -> f32 {
        // Simulate atmospheric pressure (980-1040 hPa)
        1013.25
            + 15.0 * (self.pressure_phase).sin()
            + 8.0 * (self.pressure_phase * 2.0).cos()
            + 3.0 * (self.time * 0.05).sin() // Weather patterns
    }

    fn get_humidity(&self) -> f32 {
        // Simulate humidity (30-80%)
        55.0 + 20.0 * (self.humidity_phase).sin()
            + 10.0 * (self.humidity_phase * 1.5).cos()
            + 5.0 * (self.time * 0.2).sin() // Environmental changes
    }

    fn get_cpu_usage(&mut self) -> f32 {
        // Simulate realistic CPU usage with occasional spikes
        let spike = if (self.time * 10.0) as u32 % 47 == 0 {
            40.0
        } else {
            0.0
        };
        let base_load = self.cpu_load_base + 15.0 * (self.time * 0.4).sin();
        (base_load + spike).min(95.0).max(5.0)
    }

    fn get_memory_usage(&mut self) -> f32 {
        // Simulate memory usage with gradual increases and garbage collection
        if (self.time * 10.0) as u32 % 100 == 0 {
            self.memory_usage = 40.0; // Simulate garbage collection
        } else {
            self.memory_usage += 0.1; // Gradual increase
        }
        self.memory_usage.min(85.0).max(35.0)
    }

    fn get_network_io(&mut self) -> f32 {
        // Simulate network I/O with bursts
        let burst = if self.network_burst_timer > 3.0 {
            self.network_burst_timer = 0.0;
            50.0
        } else {
            0.0
        };

        let base_io = 5.0 + 10.0 * (self.time * 1.2).sin().abs();
        (base_io + burst).min(100.0)
    }
}

/// Performance monitor for tracking streaming performance
struct PerformanceMonitor {
    /// Frame count
    frame_count: u64,
    /// Start time
    start_time: Instant,
    /// Last FPS calculation time
    last_fps_time: Instant,
    /// Current FPS
    current_fps: f32,
    /// Frame times for averaging
    frame_times: heapless::Vec<f32, 60>,
    /// Memory usage samples
    memory_samples: heapless::Vec<f32, 100>,
}

impl PerformanceMonitor {
    fn new() -> Self {
        let now = Instant::now();
        Self {
            frame_count: 0,
            start_time: now,
            last_fps_time: now,
            current_fps: 0.0,
            frame_times: heapless::Vec::new(),
            memory_samples: heapless::Vec::new(),
        }
    }

    fn update(&mut self, frame_time: f32) {
        self.frame_count += 1;

        // Update frame times
        if self.frame_times.len() >= 60 {
            self.frame_times.remove(0);
        }
        let _ = self.frame_times.push(frame_time);

        // Calculate FPS every second
        let now = Instant::now();
        if now.duration_since(self.last_fps_time).as_secs_f32() >= 1.0 {
            self.current_fps = self.frame_times.len() as f32 / self.frame_times.iter().sum::<f32>();
            self.last_fps_time = now;
        }
    }

    fn add_memory_sample(&mut self, usage_percent: f32) {
        if self.memory_samples.len() >= 100 {
            self.memory_samples.remove(0);
        }
        let _ = self.memory_samples.push(usage_percent);
    }

    fn get_fps(&self) -> f32 {
        self.current_fps
    }

    fn get_avg_frame_time(&self) -> f32 {
        if self.frame_times.is_empty() {
            0.0
        } else {
            self.frame_times.iter().sum::<f32>() / self.frame_times.len() as f32
        }
    }

    fn get_avg_memory_usage(&self) -> f32 {
        if self.memory_samples.is_empty() {
            0.0
        } else {
            self.memory_samples.iter().sum::<f32>() / self.memory_samples.len() as f32
        }
    }

    fn get_uptime(&self) -> f32 {
        self.start_time.elapsed().as_secs_f32()
    }
}

#[cfg(feature = "std")]
fn main() -> ChartResult<()> {
    println!("🚀 Starting Unified Streaming Demo");
    println!("📊 Demonstrating comprehensive streaming architecture");
    println!("⚡ Real-time data processing with performance monitoring");
    println!("🔧 Memory management and error recovery");
    println!();

    // Create sensor simulator
    let mut sensor_sim = SensorSimulator::new();

    // Create performance monitor
    let mut perf_monitor = PerformanceMonitor::new();

    // Create unified streaming buffers with different configurations
    let temp_config = StreamingConfig {
        buffer_capacity: 100,
        update_interval: 100, // 10 Hz
        auto_prune: true,
        max_data_age: 10000, // 10 seconds
        auto_scale: true,
        memory_threshold: 80.0,
    };

    let system_config = StreamingConfig {
        buffer_capacity: 150,
        update_interval: 50, // 20 Hz
        auto_prune: true,
        max_data_age: 5000, // 5 seconds
        auto_scale: true,
        memory_threshold: 85.0,
    };

    let mut temp_buffer: UnifiedStreamingBuffer<100> =
        UnifiedStreamingBuffer::with_config(temp_config);
    let mut pressure_buffer: UnifiedStreamingBuffer<100> =
        UnifiedStreamingBuffer::with_config(temp_config);
    let mut humidity_buffer: UnifiedStreamingBuffer<100> =
        UnifiedStreamingBuffer::with_config(temp_config);

    let mut cpu_buffer: UnifiedStreamingBuffer<150> =
        UnifiedStreamingBuffer::with_config(system_config);
    let mut memory_buffer: UnifiedStreamingBuffer<150> =
        UnifiedStreamingBuffer::with_config(system_config);
    let mut network_buffer: UnifiedStreamingBuffer<150> =
        UnifiedStreamingBuffer::with_config(system_config);

    // Create streaming chart manager
    let mut chart_manager: StreamingChartManager<6> = StreamingChartManager::new();

    // Add charts to manager
    let _temp_chart_id = chart_manager.add_chart(
        ChartType::Line,
        1,
        ChartInstanceConfig {
            priority: 2,
            animations_enabled: true,
            memory_limit_bytes: 2048,
        },
    )?;

    let _pressure_chart_id = chart_manager.add_chart(
        ChartType::Line,
        2,
        ChartInstanceConfig {
            priority: 2,
            animations_enabled: true,
            memory_limit_bytes: 2048,
        },
    )?;

    let _humidity_chart_id = chart_manager.add_chart(
        ChartType::Line,
        3,
        ChartInstanceConfig {
            priority: 2,
            animations_enabled: true,
            memory_limit_bytes: 2048,
        },
    )?;

    let _cpu_chart_id = chart_manager.add_chart(
        ChartType::Bar,
        4,
        ChartInstanceConfig {
            priority: 1,
            animations_enabled: true,
            memory_limit_bytes: 1024,
        },
    )?;

    let _memory_chart_id = chart_manager.add_chart(
        ChartType::Bar,
        5,
        ChartInstanceConfig {
            priority: 1,
            animations_enabled: true,
            memory_limit_bytes: 1024,
        },
    )?;

    let _network_chart_id = chart_manager.add_chart(
        ChartType::Line,
        6,
        ChartInstanceConfig {
            priority: 3,
            animations_enabled: true,
            memory_limit_bytes: 1536,
        },
    )?;

    // Create charts for visualization
    let temp_chart = LineChart::builder()
        .line_color(Rgb565::RED)
        .line_width(2)
        .build()?;

    let pressure_chart = LineChart::builder()
        .line_color(Rgb565::BLUE)
        .line_width(2)
        .build()?;

    let humidity_chart = LineChart::builder()
        .line_color(Rgb565::GREEN)
        .line_width(2)
        .build()?;

    let cpu_chart = LineChart::builder()
        .line_color(Rgb565::YELLOW)
        .line_width(1)
        .build()?;

    let memory_chart = LineChart::builder()
        .line_color(Rgb565::MAGENTA)
        .line_width(1)
        .build()?;

    let network_chart = LineChart::builder()
        .line_color(Rgb565::CYAN)
        .line_width(1)
        .build()?;

    // Simulation state
    let mut last_temp_update = 0.0f32;
    let mut last_system_update = 0.0f32;
    let mut error_injection_timer = 0.0f32;
    let mut total_errors = 0u32;

    println!("📈 Charts configured:");
    println!("  🌡️  Temperature (Red, 10 Hz)");
    println!("  🌊 Pressure (Blue, 10 Hz)");
    println!("  💧 Humidity (Green, 10 Hz)");
    println!("  🖥️  CPU Usage (Yellow, 20 Hz)");
    println!("  💾 Memory Usage (Magenta, 20 Hz)");
    println!("  🌐 Network I/O (Cyan, 20 Hz)");
    println!();

    // Pre-calculate layout constants outside the render loop
    let chart_width_divisor = 3u32;
    let chart_height_divisor = 2u32;
    let margin = 2u32;

    // Use the common visual example runner with animation loop
    common::window::run(
        WindowConfig::new("Unified Streaming Demo - Real-time Performance Monitor")
            .fps(60)
            .background(Rgb565::WHITE),
        |display, viewport, elapsed| {
            let frame_start = Instant::now();

            // Update sensor simulator
            let delta_time = 1.0 / 60.0; // Assume 60 FPS
            sensor_sim.update(delta_time);

            // Update streaming data at different rates

            // Environmental sensors (10 Hz - every 100ms)
            if elapsed - last_temp_update >= 0.1 {
                let temp = sensor_sim.get_temperature();
                let pressure = sensor_sim.get_pressure();
                let humidity = sensor_sim.get_humidity();

                // Add data with error injection simulation
                if error_injection_timer > 5.0 {
                    // Inject an error every 5 seconds to test recovery
                    error_injection_timer = 0.0;
                    total_errors += 1;

                    // Try to add invalid data (should be handled gracefully)
                    let _ = temp_buffer.push(Point2D::new(elapsed, f32::NAN));
                } else {
                    temp_buffer.push(Point2D::new(elapsed, temp))?;
                    pressure_buffer.push(Point2D::new(elapsed, pressure))?;
                    humidity_buffer.push(Point2D::new(elapsed, humidity))?;
                }

                last_temp_update = elapsed;
            }

            // System metrics (20 Hz - every 50ms)
            if elapsed - last_system_update >= 0.05 {
                let cpu = sensor_sim.get_cpu_usage();
                let memory = sensor_sim.get_memory_usage();
                let network = sensor_sim.get_network_io();

                cpu_buffer.push(Point2D::new(elapsed, cpu))?;
                memory_buffer.push(Point2D::new(elapsed, memory))?;
                network_buffer.push(Point2D::new(elapsed, network))?;

                // Update performance monitor
                perf_monitor.add_memory_sample(memory);

                last_system_update = elapsed;
            }

            // Update chart manager
            chart_manager.update(16)?; // ~60 FPS frame time

            // Update error injection timer
            error_injection_timer += delta_time;

            // Calculate layout for six charts (2x3 grid) using pre-calculated constants
            let chart_width = viewport.size.width / chart_width_divisor;
            let chart_height = viewport.size.height / chart_height_divisor;

            // Pre-calculated viewports (optimized layout calculation)
            let temp_viewport = Rectangle::new(
                viewport.top_left,
                Size::new(chart_width - margin, chart_height - margin),
            );

            let pressure_viewport = Rectangle::new(
                Point::new(
                    viewport.top_left.x + chart_width as i32,
                    viewport.top_left.y,
                ),
                Size::new(chart_width - margin, chart_height - margin),
            );

            let humidity_viewport = Rectangle::new(
                Point::new(
                    viewport.top_left.x + (chart_width * 2) as i32,
                    viewport.top_left.y,
                ),
                Size::new(chart_width - margin, chart_height - margin),
            );

            let cpu_viewport = Rectangle::new(
                Point::new(
                    viewport.top_left.x,
                    viewport.top_left.y + chart_height as i32,
                ),
                Size::new(chart_width - margin, chart_height - margin),
            );

            let memory_viewport = Rectangle::new(
                Point::new(
                    viewport.top_left.x + chart_width as i32,
                    viewport.top_left.y + chart_height as i32,
                ),
                Size::new(chart_width - margin, chart_height - margin),
            );

            let network_viewport = Rectangle::new(
                Point::new(
                    viewport.top_left.x + (chart_width * 2) as i32,
                    viewport.top_left.y + chart_height as i32,
                ),
                Size::new(chart_width - margin, chart_height - margin),
            );

            // Render charts with streaming data (using original function for now)
            render_streaming_chart(
                &temp_chart,
                &temp_buffer,
                temp_viewport,
                display,
                "Temperature (°C)",
            )?;
            render_streaming_chart(
                &pressure_chart,
                &pressure_buffer,
                pressure_viewport,
                display,
                "Pressure (hPa)",
            )?;
            render_streaming_chart(
                &humidity_chart,
                &humidity_buffer,
                humidity_viewport,
                display,
                "Humidity (%)",
            )?;
            render_streaming_chart(
                &cpu_chart,
                &cpu_buffer,
                cpu_viewport,
                display,
                "CPU Usage (%)",
            )?;
            render_streaming_chart(
                &memory_chart,
                &memory_buffer,
                memory_viewport,
                display,
                "Memory Usage (%)",
            )?;
            render_streaming_chart(
                &network_chart,
                &network_buffer,
                network_viewport,
                display,
                "Network I/O (MB/s)",
            )?;

            // Update performance metrics
            let frame_time = frame_start.elapsed().as_secs_f32();
            perf_monitor.update(frame_time);

            // Print performance statistics every 5 seconds
            if (elapsed * 10.0) as u32 % 50 == 0 && elapsed > 1.0 {
                print_performance_stats(&perf_monitor, &temp_buffer, &chart_manager, total_errors);
            }

            Ok(())
        },
    )
}

/// Render a streaming chart with data from a unified buffer (original function)
fn render_streaming_chart<const N: usize>(
    chart: &LineChart<Rgb565>,
    buffer: &UnifiedStreamingBuffer<N>,
    viewport: Rectangle,
    display: &mut impl embedded_graphics::draw_target::DrawTarget<Color = Rgb565>,
    title: &str,
) -> ChartResult<()> {
    if buffer.is_empty() {
        return Ok(());
    }

    // Convert streaming data to series
    let mut series: StaticDataSeries<Point2D, 256> = StaticDataSeries::new();
    for point in buffer.data().take(200) {
        // Limit for performance
        series.push(point)?;
    }

    // Create legend for the streaming data
    let legend = StandardLegendBuilder::new()
        .position(LegendPos::Right)
        .orientation(LegendOrientation::Vertical)
        .add_line_entry(title, Rgb565::BLUE)?
        .professional_style()
        .build()?;

    // Calculate layout with legend
    let calculator = PositionCalculator::new(viewport, viewport);
    let legend_size = legend.calculate_size();
    let legend_rect = calculator.calculate_legend_rect(legend.position(), legend_size)?;

    // Adjust chart area to leave space for legend
    let chart_area = Rectangle::new(
        viewport.top_left,
        Size::new(
            viewport.size.width.saturating_sub(legend_size.width + 10),
            viewport.size.height,
        ),
    );

    // Create chart config with title
    let config = ChartConfig {
        title: Some(heapless::String::try_from(title).unwrap_or_default()),
        background_color: Some(Rgb565::WHITE),
        margins: Margins {
            top: 15,
            right: 5,
            bottom: 5,
            left: 25,
        },
        ..Default::default()
    };

    // Render the chart in adjusted area
    chart.draw(&series, &config, chart_area, display)?;

    // Render the legend
    let renderer = StandardLegendRenderer::new();
    renderer.render(&legend, legend_rect, display)?;

    Ok(())
}

/// Print comprehensive performance statistics
fn print_performance_stats(
    monitor: &PerformanceMonitor,
    sample_buffer: &UnifiedStreamingBuffer<100>,
    chart_manager: &StreamingChartManager<6>,
    total_errors: u32,
) {
    println!(
        "\n📊 Performance Statistics (Uptime: {:.1}s)",
        monitor.get_uptime()
    );
    println!("┌─────────────────────────────────────────────────────────────┐");
    println!("│ Rendering Performance                                       │");
    println!("├─────────────────────────────────────────────────────────────┤");
    println!(
        "│ FPS: {:.1} | Avg Frame Time: {:.2}ms                      │",
        monitor.get_fps(),
        monitor.get_avg_frame_time() * 1000.0
    );
    println!(
        "│ Total Frames: {}                                        │",
        monitor.frame_count
    );
    println!("├─────────────────────────────────────────────────────────────┤");
    println!("│ Memory Management                                           │");
    println!("├─────────────────────────────────────────────────────────────┤");

    let memory_stats = sample_buffer.memory_stats();
    println!(
        "│ Buffer Usage: {:.1}% ({}/{} bytes)                     │",
        memory_stats.utilization_percent(),
        memory_stats.used,
        memory_stats.total_allocated
    );
    println!(
        "│ Peak Usage: {} bytes                                    │",
        memory_stats.peak_usage
    );
    println!(
        "│ Avg Memory: {:.1}%                                      │",
        monitor.get_avg_memory_usage()
    );

    println!("├─────────────────────────────────────────────────────────────┤");
    println!("│ Streaming Performance                                       │");
    println!("├─────────────────────────────────────────────────────────────┤");

    let buffer_metrics = sample_buffer.metrics();
    println!(
        "│ Total Points: {} | Dropped: {}                         │",
        buffer_metrics.total_points, buffer_metrics.dropped_points
    );
    println!(
        "│ Avg Latency: {}μs                                       │",
        buffer_metrics.avg_latency_us
    );

    println!("├─────────────────────────────────────────────────────────────┤");
    println!("│ Chart Management                                            │");
    println!("├─────────────────────────────────────────────────────────────┤");

    let chart_metrics = chart_manager.metrics();
    println!(
        "│ Active Charts: {} | Total Updates: {}                  │",
        chart_metrics.active_charts, chart_metrics.total_updates
    );
    println!(
        "│ Chart Latency: {}μs                                     │",
        chart_metrics.avg_update_latency_us
    );

    println!("├─────────────────────────────────────────────────────────────┤");
    println!("│ Error Handling                                              │");
    println!("├─────────────────────────────────────────────────────────────┤");
    println!(
        "│ Total Errors: {} (Recovered)                            │",
        total_errors
    );
    println!("└─────────────────────────────────────────────────────────────┘");
}

#[cfg(not(feature = "std"))]
fn main() {
    common::utils::print_feature_requirement("std", "unified streaming demo");
}
