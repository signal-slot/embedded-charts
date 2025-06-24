//! Example demonstrating platform-specific optimizations

use embedded_charts::{
    chart::{line::LineChart, Chart, ChartBuilder},
    data::{Point2D, StaticDataSeries},
    platform::{self, PlatformOptimized},
};
use embedded_graphics::{
    mock_display::MockDisplay,
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::Rectangle,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a display
    let mut display = MockDisplay::<BinaryColor>::new();
    display.set_allow_overdraw(true);

    // Generate test data using platform-optimized math
    let mut data = StaticDataSeries::<Point2D, 100>::new();
    
    for i in 0..100 {
        let x = i as f32;
        let angle = x * 0.1;
        
        // Use platform-optimized trigonometry
        let y = 30.0 + 20.0 * platform::GenericPlatform::fast_sin(angle);
        
        data.push(Point2D { x, y })?;
    }

    // Create a line chart
    let chart = LineChart::builder()
        .line_color(BinaryColor::On)
        .line_width(1)
        .build()?;

    // Draw the chart
    let viewport = Rectangle::new(Point::new(10, 10), Size::new(100, 50));
    chart.draw(&mut display, &data, viewport)?;

    // Demonstrate platform-optimized line drawing
    println!("Drawing optimized line...");
    let start = Point2D { x: 0.0, y: 0.0 };
    let end = Point2D { x: 50.0, y: 30.0 };
    
    let mut pixel_count = 0;
    platform::GenericPlatform::draw_line_optimized(start, end, |x, y| {
        pixel_count += 1;
        // In a real implementation, this would plot the pixel
        let _ = (x, y);
    });
    
    println!("Line drawn with {} pixels", pixel_count);

    // Demonstrate platform-optimized rectangle filling
    println!("Filling optimized rectangle...");
    let top_left = Point2D { x: 10.0, y: 10.0 };
    
    let mut fill_count = 0;
    platform::GenericPlatform::fill_rect_optimized(top_left, 20, 15, |x, y| {
        fill_count += 1;
        // In a real implementation, this would plot the pixel
        let _ = (x, y);
    });
    
    println!("Rectangle filled with {} pixels", fill_count);

    // Show performance comparison
    println!("\nPerformance comparison:");
    
    // Standard sqrt vs optimized
    let test_val: f32 = 42.0;
    let std_sqrt = test_val.sqrt();
    let fast_sqrt = platform::GenericPlatform::fast_sqrt(test_val);
    println!("sqrt({}) - Standard: {:.4}, Fast: {:.4}, Error: {:.4}%", 
        test_val, std_sqrt, fast_sqrt, 
        ((std_sqrt - fast_sqrt).abs() / std_sqrt) * 100.0);

    // Standard sin vs optimized
    let angle: f32 = 1.0;
    let std_sin = angle.sin();
    let fast_sin = platform::GenericPlatform::fast_sin(angle);
    println!("sin({}) - Standard: {:.4}, Fast: {:.4}, Error: {:.4}%", 
        angle, std_sin, fast_sin,
        ((std_sin - fast_sin).abs() / std_sin.abs().max(0.0001)) * 100.0);

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

    Ok(())
}