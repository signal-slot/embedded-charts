//! Simple pie chart example without SDL2 dependency
//! This example creates a pie chart and saves it to a buffer.

use embedded_charts::prelude::*;
use embedded_graphics::{mock_display::MockDisplay, pixelcolor::Rgb565};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a mock display (no SDL2 required)
    let mut display = MockDisplay::<Rgb565>::new();

    // Create sample data (pie charts use Point2D where Y is the percentage)
    let mut data = StaticDataSeries::<Point2D, 256>::new();
    data.push(Point2D::new(1.0, 30.0))?; // Category A: 30%
    data.push(Point2D::new(2.0, 25.0))?; // Category B: 25%
    data.push(Point2D::new(3.0, 20.0))?; // Category C: 20%
    data.push(Point2D::new(4.0, 15.0))?; // Category D: 15%
    data.push(Point2D::new(5.0, 10.0))?; // Category E: 10%

    // Create pie chart
    let chart = PieChart::builder()
        .radius(80)
        .with_title("Simple Pie Chart")
        .build()?;

    // Define viewport
    let viewport = Rectangle::new(Point::zero(), Size::new(320, 240));

    // Draw chart
    chart.draw(&data, chart.config(), viewport, &mut display)?;

    println!("Pie chart created successfully!");
    println!(
        "Display size: {}x{}",
        display.size().width,
        display.size().height
    );

    Ok(())
}
