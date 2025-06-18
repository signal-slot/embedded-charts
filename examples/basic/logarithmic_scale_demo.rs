//! Demonstrates logarithmic axis scales for data with large dynamic ranges
#![cfg(feature = "std")]

use embedded_charts::{
    axes::{
        scale::{AxisScale, AxisScaleType, ScaleConfig},
        LinearAxis, AxisOrientation, AxisPosition,
    },
    chart::{ChartConfig, LineChart, ChartBuilder},
    data::{Point2D, StaticDataSeries},
    prelude::*,
};
use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{Circle, PrimitiveStyle, Rectangle},
};
use embedded_graphics_simulator::{SimulatorDisplay, Window, OutputSettingsBuilder};

/// Generate exponential growth data
fn generate_exponential_data() -> StaticDataSeries<Point2D, 128> {
    let mut series = StaticDataSeries::new();
    
    // Generate data from 1 to 10000 (4 orders of magnitude)
    for i in 0..50 {
        let x = i as f32;
        let y = 10.0_f32.powf(i as f32 / 12.5); // Exponential growth from 1 to 10^4
        series.push(Point2D::new(x, y)).unwrap();
    }
    
    series
}

/// Generate frequency response data (dB scale)
fn generate_frequency_response() -> StaticDataSeries<Point2D, 128> {
    let mut series = StaticDataSeries::new();
    
    // Generate frequency response from 10 Hz to 100 kHz
    for i in 0..100 {
        let freq = 10.0 * 10.0_f32.powf(i as f32 / 25.0); // 10 Hz to 100 kHz
        // Simulate a low-pass filter response
        let cutoff = 1000.0;
        let magnitude = 1.0 / (1.0 + (freq / cutoff).powi(2)).sqrt();
        let db = 20.0 * magnitude.log10(); // Convert to dB
        
        series.push(Point2D::new(freq, db)).unwrap();
    }
    
    series
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create display
    let mut display = SimulatorDisplay::<Rgb565>::new(Size::new(800, 600));
    display.clear(Rgb565::WHITE)?;

    // Create two chart areas
    let chart1_area = Rectangle::new(Point::new(50, 50), Size::new(700, 250));
    let chart2_area = Rectangle::new(Point::new(50, 350), Size::new(700, 200));

    // Example 1: Exponential growth with logarithmic Y-axis
    {
        let data = generate_exponential_data();
        
        // Get data bounds
        let bounds = data.bounds().unwrap();
        
        // Create logarithmic scale for Y-axis
        let y_scale_config = ScaleConfig {
            min: bounds.min_y,
            max: bounds.max_y,
            include_zero: false,
            nice_bounds: true,
        };
        let y_scale = AxisScale::new(AxisScaleType::Log10, y_scale_config)?;
        
        // Create chart with logarithmic Y-axis
        let chart = LineChart::<Rgb565>::builder()
            .line_color(Rgb565::BLUE)
            .line_width(2)
            .with_title("Exponential Growth (Log Y-Scale)")
            .build()?;
        
        // Draw chart
        let config = ChartConfig::default();
        chart.draw(&data, &config, chart1_area, &mut display)?;
        
        // Draw custom Y-axis labels showing logarithmic scale
        let ticks = y_scale.get_ticks(5)?;
        for (i, &tick) in ticks.iter().enumerate() {
            let y_pos = chart1_area.bottom_left().y - 
                ((i as i32 * chart1_area.size.height as i32) / (ticks.len() as i32 - 1));
            let label = y_scale.format_value(tick);
            
            // Draw tick mark
            let tick_start = Point::new(chart1_area.top_left.x - 5, y_pos);
            let tick_end = Point::new(chart1_area.top_left.x, y_pos);
            Line::new(tick_start, tick_end)
                .into_styled(PrimitiveStyle::with_stroke(Rgb565::BLACK, 1))
                .draw(&mut display)?;
            
            // Note: In a real implementation, we would draw the label text here
            // For this demo, we'll just show the concept
        }
    }

    // Example 2: Frequency response with logarithmic X-axis
    {
        let data = generate_frequency_response();
        
        // Get data bounds
        let bounds = data.bounds().unwrap();
        
        // Create logarithmic scale for X-axis (frequency)
        let x_scale_config = ScaleConfig {
            min: bounds.min_x,
            max: bounds.max_x,
            include_zero: false,
            nice_bounds: true,
        };
        let x_scale = AxisScale::new(AxisScaleType::Log10, x_scale_config)?;
        
        // Create chart
        let chart = LineChart::<Rgb565>::builder()
            .line_color(Rgb565::RED)
            .line_width(2)
            .with_title("Frequency Response (Log X-Scale)")
            .build()?;
        
        // Draw chart
        let config = ChartConfig::default();
        chart.draw(&data, &config, chart2_area, &mut display)?;
        
        // Draw custom X-axis labels showing logarithmic scale
        let ticks = x_scale.get_ticks(6)?;
        for (i, &tick) in ticks.iter().enumerate() {
            let x_pos = chart2_area.top_left.x + 
                ((i as i32 * chart2_area.size.width as i32) / (ticks.len() as i32 - 1));
            let label = x_scale.format_value(tick);
            
            // Draw tick mark
            let tick_start = Point::new(x_pos, chart2_area.bottom_left().y);
            let tick_end = Point::new(x_pos, chart2_area.bottom_left().y + 5);
            Line::new(tick_start, tick_end)
                .into_styled(PrimitiveStyle::with_stroke(Rgb565::BLACK, 1))
                .draw(&mut display)?;
        }
    }

    // Add demo annotations
    // Title
    let title_pos = Point::new(400, 20);
    Circle::with_center(title_pos, 5)
        .into_styled(PrimitiveStyle::with_fill(Rgb565::BLACK))
        .draw(&mut display)?;
    
    // Show the display
    let output_settings = OutputSettingsBuilder::new().build();
    Window::new("Logarithmic Scale Demo", &output_settings).show_static(&display);

    Ok(())
}