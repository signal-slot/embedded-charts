//! Example demonstrating platform-specific optimizations
//!
//! This example requires the "line" feature to be enabled.

#[cfg(not(feature = "line"))]
fn main() {
    eprintln!("This example requires the 'line' feature. Run with: cargo run --example platform_optimized_chart --features line");
}

#[cfg(feature = "line")]
use embedded_charts::{
    chart::{line::LineChart, Chart, ChartBuilder},
    data::{DataSeries, Point2D, StaticDataSeries},
    platform::{self, PlatformOptimized},
};
#[cfg(feature = "line")]
use embedded_graphics::{
    mock_display::MockDisplay, pixelcolor::BinaryColor, prelude::*, primitives::Rectangle,
};

#[cfg(feature = "line")]
fn main() {
    // Create a display
    let mut display: MockDisplay<BinaryColor> = MockDisplay::new();
    display.set_allow_overdraw(true);

    // Generate test data using platform-optimized math
    let mut data = StaticDataSeries::<Point2D, 256>::new();

    for i in 0..50 {
        let x = i as f32;
        let angle = x * 0.2;

        // Use platform-optimized trigonometry
        let y = 25.0 + 15.0 * platform::GenericPlatform::fast_sin(angle);

        data.push(Point2D { x, y }).ok();
    }

    // Create a line chart
    let chart = LineChart::builder()
        .line_color(BinaryColor::On)
        .line_width(1)
        .build()
        .expect("Failed to build chart");

    // Draw the chart (MockDisplay default size is 64x64)
    let viewport = Rectangle::new(Point::new(5, 5), Size::new(54, 40));
    let mut config = embedded_charts::chart::ChartConfig::<BinaryColor>::default();
    config.margins.top = 2;
    config.margins.bottom = 2;
    config.margins.left = 2;
    config.margins.right = 2;
    if let Err(e) = chart.draw(&data, &config, viewport, &mut display) {
        eprintln!("Failed to draw chart: {:?}", e);
    }

    // Print chart stats
    println!("\nChart rendered successfully!");
    println!("Data points: {}", data.len());
    println!("Viewport: {}x{} at ({}, {})", 
        viewport.size.width, viewport.size.height,
        viewport.top_left.x, viewport.top_left.y);
    
    // Show some pixel statistics
    let pixel_count = display.affected_area().size.width * display.affected_area().size.height;
    println!("\nChart visualization stats:");
    println!("Affected area: {:?}", display.affected_area());
    println!("Total pixels in affected area: {}", pixel_count);

    // Show performance comparison
    println!("\nPerformance comparison:");

    // Standard sqrt vs optimized
    let test_val: f32 = 42.0;
    let std_sqrt = test_val.sqrt();
    let fast_sqrt = platform::GenericPlatform::fast_sqrt(test_val);
    println!(
        "sqrt({}) - Standard: {:.4}, Fast: {:.4}, Error: {:.4}%",
        test_val,
        std_sqrt,
        fast_sqrt,
        ((std_sqrt - fast_sqrt).abs() / std_sqrt) * 100.0
    );

    // Standard sin vs optimized
    let angle: f32 = 1.0;
    let std_sin = angle.sin();
    let fast_sin = platform::GenericPlatform::fast_sin(angle);
    println!(
        "sin({}) - Standard: {:.4}, Fast: {:.4}, Error: {:.4}%",
        angle,
        std_sin,
        fast_sin,
        ((std_sin - fast_sin).abs() / std_sin.abs().max(0.0001)) * 100.0
    );

    // Platform detection
    println!("\nPlatform detection:");
    #[cfg(target_arch = "arm")]
    println!("ARM architecture detected");

    #[cfg(target_arch = "riscv32")]
    println!("RISC-V architecture detected");

    #[cfg(target_arch = "xtensa")]
    println!("ESP32 (Xtensa) architecture detected");

    #[cfg(not(any(target_arch = "arm", target_arch = "riscv32", target_arch = "xtensa")))]
    println!("Generic platform (x86_64 or other)");
}
