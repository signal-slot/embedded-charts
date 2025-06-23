//! Simple bar chart example without SDL2 dependency
//! This example creates a bar chart and saves it to a buffer.

use embedded_charts::prelude::*;
use embedded_graphics::{
    mock_display::MockDisplay,
    pixelcolor::Rgb565,
    prelude::*,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a mock display (no SDL2 required)
    let mut display = MockDisplay::<Rgb565>::new();
    
    // Create sample data
    let mut data = StaticDataSeries::<Point2D, 256>::new();
    data.push(Point2D::new(0.0, 30.0))?;
    data.push(Point2D::new(1.0, 50.0))?;
    data.push(Point2D::new(2.0, 20.0))?;
    data.push(Point2D::new(3.0, 60.0))?;
    data.push(Point2D::new(4.0, 40.0))?;
    
    // Create bar chart
    let chart = BarChart::builder()
        .bar_width(BarWidth::Fixed(40))
        .spacing(10)
        .with_title("Simple Bar Chart")
        .build()?;
    
    // Define viewport
    let viewport = Rectangle::new(Point::zero(), Size::new(320, 240));
    
    // Draw chart
    chart.draw(&data, chart.config(), viewport, &mut display)?;
    
    println!("Bar chart created successfully!");
    println!("Display size: {}x{}", display.size().width, display.size().height);
    
    Ok(())
}