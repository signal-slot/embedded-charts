//! Simple line chart example without SDL2 dependency
//! This example creates a line chart and saves it to a buffer.

use embedded_charts::prelude::*;
use embedded_graphics::{mock_display::MockDisplay, pixelcolor::Rgb565};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a mock display (no SDL2 required)
    let mut display = MockDisplay::<Rgb565>::new();

    // Create sample data
    let mut data = StaticDataSeries::<Point2D, 256>::new();
    for i in 0..20 {
        let x = i as f32;
        let y = (x * 0.5).sin() * 20.0 + 50.0;
        data.push(Point2D::new(x, y))?;
    }

    // Create line chart
    let chart = LineChart::builder()
        .line_color(Rgb565::BLUE)
        .line_width(2)
        .with_title("Simple Line Chart")
        .build()?;

    // Define viewport
    let viewport = Rectangle::new(Point::zero(), Size::new(320, 240));

    // Draw chart
    chart.draw(&data, chart.config(), viewport, &mut display)?;

    println!("Line chart created successfully!");
    println!(
        "Display size: {}x{}",
        display.size().width,
        display.size().height
    );

    Ok(())
}
