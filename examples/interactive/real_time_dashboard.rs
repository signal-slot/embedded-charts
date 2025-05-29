//! Real-Time Dashboard - Interactive Category
//!
//! This example demonstrates a comprehensive real-time dashboard that simulates
//! IoT sensor data with multiple charts and professional styling.
//! Perfect for industrial monitoring, environmental sensing, or any real-time
//! data visualization application.
//!
//! Run with: cargo run --example real_time_dashboard --features std

use embedded_charts::prelude::*;
use std::time::{Duration, Instant};

// Import the common abstraction
#[path = "../common/mod.rs"]
mod common;

use common::{configs, data, utils, window, WindowConfig};

/// Simulated sensor data structure
#[derive(Debug, Clone)]
struct SensorReading {
    timestamp: f32,
    temperature: f32,
    humidity: f32,
    pressure: f32,
    air_quality: f32,
}

/// Pre-calculated viewport rectangles for dashboard layout
#[derive(Clone)]
struct DashboardViewports {
    temp_viewport: Rectangle,
    humidity_viewport: Rectangle,
    pressure_viewport: Rectangle,
    air_quality_viewport: Rectangle,
}

impl DashboardViewports {
    /// Calculate viewports for 2x2 grid layout
    fn new(display_size: Size) -> Self {
        let width = display_size.width as i32;
        let height = display_size.height as i32;

        // Calculate chart areas (2x2 grid)
        let chart_width = width / 2 - 20;
        let chart_height = height / 2 - 40;

        Self {
            // Top-left: Temperature
            temp_viewport: Rectangle::new(
                Point::new(10, 30),
                Size::new(chart_width as u32, chart_height as u32),
            ),
            // Top-right: Humidity
            humidity_viewport: Rectangle::new(
                Point::new(width / 2 + 10, 30),
                Size::new(chart_width as u32, chart_height as u32),
            ),
            // Bottom-left: Pressure
            pressure_viewport: Rectangle::new(
                Point::new(10, height / 2 + 20),
                Size::new(chart_width as u32, chart_height as u32),
            ),
            // Bottom-right: Air Quality
            air_quality_viewport: Rectangle::new(
                Point::new(width / 2 + 10, height / 2 + 20),
                Size::new(chart_width as u32, chart_height as u32),
            ),
        }
    }
}

/// IoT Dashboard with multiple real-time charts
struct IoTDashboard {
    temperature_chart: LineChart<Rgb565>,
    humidity_chart: LineChart<Rgb565>,
    pressure_chart: LineChart<Rgb565>,
    air_quality_chart: LineChart<Rgb565>,

    temperature_data: StaticDataSeries<Point2D, 256>,
    humidity_data: StaticDataSeries<Point2D, 256>,
    pressure_data: StaticDataSeries<Point2D, 256>,
    air_quality_data: StaticDataSeries<Point2D, 256>,

    start_time: Instant,
}

impl IoTDashboard {
    /// Create a new IoT dashboard with professional styling
    fn new() -> ChartResult<Self> {
        // Use professional colors from common configuration
        let colors = configs::professional_colors();

        // Create temperature chart (crimson theme) with legend
        let temperature_chart = LineChart::builder()
            .line_color(colors[1]) // Crimson
            .line_width(2)
            .with_title("Temperature (°C)")
            // Note: Legend support will be added in a future version
            .build()?;

        // Create humidity chart (steel blue theme) with legend
        let humidity_chart = LineChart::builder()
            .line_color(colors[0]) // Steel Blue
            .line_width(2)
            .with_title("Humidity (%)")
            // Note: Legend support will be added in a future version
            .build()?;

        // Create pressure chart (lime green theme) with legend
        let pressure_chart = LineChart::builder()
            .line_color(colors[2]) // Lime Green
            .line_width(2)
            .with_title("Pressure (hPa)")
            // Note: Legend support will be added in a future version
            .build()?;

        // Create air quality chart (dark orange theme) with legend
        let air_quality_chart = LineChart::builder()
            .line_color(colors[3]) // Dark Orange
            .line_width(2)
            .with_title("Air Quality Index")
            // Note: Legend support will be added in a future version
            .build()?;

        Ok(Self {
            temperature_chart,
            humidity_chart,
            pressure_chart,
            air_quality_chart,
            temperature_data: StaticDataSeries::new(),
            humidity_data: StaticDataSeries::new(),
            pressure_data: StaticDataSeries::new(),
            air_quality_data: StaticDataSeries::new(),
            start_time: Instant::now(),
        })
    }

    /// Add new sensor reading to the dashboard
    fn add_reading(&mut self, reading: SensorReading) -> ChartResult<()> {
        // Add data points to each series
        self.temperature_data
            .push(Point2D::new(reading.timestamp, reading.temperature))?;
        self.humidity_data
            .push(Point2D::new(reading.timestamp, reading.humidity))?;
        self.pressure_data
            .push(Point2D::new(reading.timestamp, reading.pressure))?;
        self.air_quality_data
            .push(Point2D::new(reading.timestamp, reading.air_quality))?;

        // Keep only last 50 points for real-time display
        while self.temperature_data.len() > 50 {
            // For this demo, we'll just clear when we get too many points
            // In a real implementation, you'd use a sliding window
            if self.temperature_data.len() > 100 {
                self.temperature_data.clear();
                self.humidity_data.clear();
                self.pressure_data.clear();
                self.air_quality_data.clear();
            }
            break;
        }

        Ok(())
    }

    /// Render the complete dashboard with pre-calculated viewports
    fn render<D>(&self, display: &mut D, viewports: &DashboardViewports) -> ChartResult<()>
    where
        D: DrawTarget<Color = Rgb565>,
    {
        // Render all charts (only if they have data)
        if !self.temperature_data.is_empty() {
            self.temperature_chart.draw(
                &self.temperature_data,
                self.temperature_chart.config(),
                viewports.temp_viewport,
                display,
            )?;
        }
        if !self.humidity_data.is_empty() {
            self.humidity_chart.draw(
                &self.humidity_data,
                self.humidity_chart.config(),
                viewports.humidity_viewport,
                display,
            )?;
        }
        if !self.pressure_data.is_empty() {
            self.pressure_chart.draw(
                &self.pressure_data,
                self.pressure_chart.config(),
                viewports.pressure_viewport,
                display,
            )?;
        }
        if !self.air_quality_data.is_empty() {
            self.air_quality_chart.draw(
                &self.air_quality_data,
                self.air_quality_chart.config(),
                viewports.air_quality_viewport,
                display,
            )?;
        }

        Ok(())
    }
}

/// Simulate sensor readings with realistic patterns
fn simulate_sensor_reading(time: f32) -> SensorReading {
    // Simulate daily temperature cycle
    let temp_base = 22.0;
    let temp_variation = 8.0 * (time * 0.1).sin(); // Daily cycle
    let temp_noise = (time * 2.3).sin() * 0.5; // Small fluctuations
    let temperature = temp_base + temp_variation + temp_noise;

    // Simulate humidity (inverse correlation with temperature)
    let humidity_base = 60.0;
    let humidity_variation = -temp_variation * 0.8; // Inverse correlation
    let humidity_noise = (time * 1.7).cos() * 2.0;
    let humidity = (humidity_base + humidity_variation + humidity_noise)
        .max(20.0)
        .min(90.0);

    // Simulate atmospheric pressure
    let pressure_base = 1013.25;
    let pressure_variation = 15.0 * (time * 0.05).sin(); // Weather patterns
    let pressure_noise = (time * 3.1).sin() * 2.0;
    let pressure = pressure_base + pressure_variation + pressure_noise;

    // Simulate air quality index
    let aqi_base = 50.0;
    let aqi_variation = 30.0 * (time * 0.08).cos(); // Pollution cycles
    let aqi_noise = (time * 1.9).sin() * 5.0;
    let air_quality = (aqi_base + aqi_variation + aqi_noise).max(0.0).min(200.0);

    SensorReading {
        timestamp: time,
        temperature,
        humidity,
        pressure,
        air_quality,
    }
}

#[cfg(feature = "std")]
fn main() -> ChartResult<()> {
    println!("🌡️ Real-time IoT Sensor Dashboard");
    println!("==================================");
    println!("Simulating multi-sensor real-time data visualization");
    println!("Press Ctrl+C to exit");

    // Create dashboard
    let mut dashboard = IoTDashboard::new()?;

    // Simulation parameters
    let mut time = 0.0;
    let time_step = 0.5; // 0.5 second steps

    println!("\n📊 Dashboard Features:");
    println!("  • Temperature monitoring with daily cycles");
    println!("  • Humidity tracking with weather correlation");
    println!("  • Atmospheric pressure measurements");
    println!("  • Air quality index monitoring");
    println!("  • Real-time updates");
    println!("  • Professional color coding");

    // Pre-calculate viewports for 2x2 grid layout (assuming standard display size)
    let display_size = Size::new(800, 600); // Standard size, will be updated in first frame
    let mut viewports = DashboardViewports::new(display_size);

    // Use the common animated example runner
    window::run(
        WindowConfig::new("IoT Sensor Dashboard")
            .theme(common::WindowTheme::OledBlue)
            .fps(10) // 10 FPS for real-time feel
            .background(Rgb565::BLACK),
        move |display, _viewport, elapsed| {
            // Update viewports if display size changed
            let current_size = display.bounding_box().size;
            if current_size != display_size {
                viewports = DashboardViewports::new(current_size);
            }

            // Generate new sensor reading
            let reading = simulate_sensor_reading(time);

            // Add to dashboard
            dashboard.add_reading(reading.clone())?;

            // Render dashboard with pre-calculated viewports
            dashboard.render(display, &viewports)?;

            // Print current readings every 5 seconds
            if (time % 5.0) < time_step {
                println!("\n📈 Current Readings (t={:.1}s):", time);
                println!("  🌡️  Temperature: {:.1}°C", reading.temperature);
                println!("  💧 Humidity: {:.1}%", reading.humidity);
                println!("  🌪️  Pressure: {:.1} hPa", reading.pressure);
                println!("  🌬️  Air Quality: {:.0} AQI", reading.air_quality);
            }

            // Advance time
            time += time_step;

            // Exit condition (for demo purposes, run for 2 minutes)
            if time > 120.0 {
                println!("\n✅ Dashboard simulation completed!");
                println!("🎯 Demonstrated features:");
                println!("  • Multi-chart real-time dashboard");
                println!("  • Professional styling and themes");
                println!("  • Realistic sensor data simulation");
                println!("  • 2x2 grid layout");
                println!("  • Color-coded metrics");
                return Ok(()); // Stop animation
            }

            Ok(()) // Continue animation
        },
    )
}

#[cfg(not(feature = "std"))]
fn main() {
    utils::print_feature_requirement("std", "interactive");
}

#[cfg(test)]
mod tests {
    use super::*;
    use embedded_graphics::mock_display::MockDisplay;

    #[test]
    fn test_dashboard_creation() {
        let dashboard = IoTDashboard::new().unwrap();
        assert_eq!(dashboard.temperature_data.len(), 0);
        assert_eq!(dashboard.humidity_data.len(), 0);
    }

    #[test]
    fn test_sensor_reading_simulation() {
        let reading = simulate_sensor_reading(0.0);
        assert!(reading.temperature > 10.0 && reading.temperature < 40.0);
        assert!(reading.humidity > 0.0 && reading.humidity <= 100.0);
        assert!(reading.pressure > 900.0 && reading.pressure < 1100.0);
        assert!(reading.air_quality >= 0.0 && reading.air_quality <= 200.0);
    }

    #[test]
    fn test_dashboard_rendering() {
        let mut display = MockDisplay::<Rgb565>::new();
        let mut dashboard = IoTDashboard::new().unwrap();

        // Add some test data
        for i in 0..10 {
            let reading = simulate_sensor_reading(i as f32);
            dashboard.add_reading(reading).unwrap();
        }

        // Create test viewports
        let viewports = DashboardViewports::new(Size::new(800, 600));

        // Should render without errors
        dashboard.render(&mut display, &viewports).unwrap();
    }
}
