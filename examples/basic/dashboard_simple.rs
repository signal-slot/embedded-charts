//! Simple dashboard demo showing layout calculation

use embedded_charts::{
    chart::{LineChart, BarChart, PieChart, GaugeChart},
    dashboard::{SimpleDashboard, GridPosition},
    data::{Point2D, StaticDataSeries},
    prelude::*,
};
use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{Rectangle, PrimitiveStyle, RoundedRectangle},
    text::{Alignment, Text},
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, Window,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create display
    let mut display = SimulatorDisplay::<Rgb565>::new(Size::new(800, 600));
    let output_settings = OutputSettingsBuilder::new().build();
    let mut window = Window::new("Simple Dashboard Demo", &output_settings);

    // Clear display
    display.clear(Rgb565::new(0, 0, 0))?;

    // Create dashboard layout manager
    let dashboard = SimpleDashboard::new(2, 2, 10);
    let total_viewport = Rectangle::new(Point::new(20, 20), Size::new(760, 560));

    // Define text style
    let text_style = MonoTextStyle::new(&FONT_6X10, Rgb565::WHITE);

    // Chart 1: Line Chart (top-left)
    {
        let viewport = dashboard.get_viewport(GridPosition::new(0, 0), total_viewport);
        
        // Draw border
        RoundedRectangle::with_equal_corners(viewport, Size::new(5, 5))
            .into_styled(PrimitiveStyle::with_stroke(Rgb565::RED, 2))
            .draw(&mut display)?;
        
        // Create and draw line chart
        let chart = LineChart::builder()
            .line_color(Rgb565::RED)
            .line_width(2)
            .build()?;
        
        let mut data: StaticDataSeries<Point2D, 50> = StaticDataSeries::new();
        for i in 0..20 {
            let x = i as f32;
            let y = 50.0 + 30.0 * (x * 0.2).sin();
            let _ = data.push(Point2D::new(x, y));
        }
        
        let config = ChartConfig::default();
        chart.draw(&data, &config, viewport, &mut display)?;
        
        // Add title
        Text::with_alignment(
            "Temperature",
            viewport.center() + Point::new(0, -viewport.size.height as i32 / 2 + 10),
            text_style,
            Alignment::Center,
        )
        .draw(&mut display)?;
    }

    // Chart 2: Bar Chart (top-right)
    {
        let viewport = dashboard.get_viewport(GridPosition::new(0, 1), total_viewport);
        
        // Draw border
        RoundedRectangle::with_equal_corners(viewport, Size::new(5, 5))
            .into_styled(PrimitiveStyle::with_stroke(Rgb565::BLUE, 2))
            .draw(&mut display)?;
        
        // Create and draw bar chart
        let chart = BarChart::builder()
            .bar_color(Rgb565::BLUE)
            .bar_width(20)
            .build()?;
        
        let mut data: StaticDataSeries<Point2D, 10> = StaticDataSeries::new();
        let values = [45.0, 38.0, 52.0, 41.0, 55.0];
        for (i, &value) in values.iter().enumerate() {
            let _ = data.push(Point2D::new(i as f32, value));
        }
        
        let config = ChartConfig::default();
        chart.draw(&data, &config, viewport, &mut display)?;
        
        // Add title
        Text::with_alignment(
            "Weekly Sales",
            viewport.center() + Point::new(0, -viewport.size.height as i32 / 2 + 10),
            text_style,
            Alignment::Center,
        )
        .draw(&mut display)?;
    }

    // Chart 3: Pie Chart (bottom-left)
    {
        let viewport = dashboard.get_viewport(GridPosition::new(1, 0), total_viewport);
        
        // Draw border
        RoundedRectangle::with_equal_corners(viewport, Size::new(5, 5))
            .into_styled(PrimitiveStyle::with_stroke(Rgb565::GREEN, 2))
            .draw(&mut display)?;
        
        // Create and draw pie chart
        let chart = PieChart::builder()
            .inner_radius(0)
            .build()?;
        
        let mut data: StaticDataSeries<Point2D, 5> = StaticDataSeries::new();
        let _ = data.push(Point2D::new(0.0, 35.0));
        let _ = data.push(Point2D::new(1.0, 25.0));
        let _ = data.push(Point2D::new(2.0, 20.0));
        let _ = data.push(Point2D::new(3.0, 15.0));
        let _ = data.push(Point2D::new(4.0, 5.0));
        
        let config = ChartConfig::default();
        chart.draw(&data, &config, viewport, &mut display)?;
        
        // Add title
        Text::with_alignment(
            "Market Share",
            viewport.center() + Point::new(0, -viewport.size.height as i32 / 2 + 10),
            text_style,
            Alignment::Center,
        )
        .draw(&mut display)?;
    }

    // Chart 4: Gauge Chart (bottom-right)
    {
        let viewport = dashboard.get_viewport(GridPosition::new(1, 1), total_viewport);
        
        // Draw border
        RoundedRectangle::with_equal_corners(viewport, Size::new(5, 5))
            .into_styled(PrimitiveStyle::with_stroke(Rgb565::YELLOW, 2))
            .draw(&mut display)?;
        
        // Create and draw gauge chart
        let chart = GaugeChart::builder()
            .min_value(0.0)
            .max_value(100.0)
            .needle_color(Rgb565::YELLOW)
            .build()?;
        
        let mut data: StaticDataSeries<Point2D, 1> = StaticDataSeries::new();
        let _ = data.push(Point2D::new(0.0, 73.0));
        
        let config = ChartConfig::default();
        chart.draw(&data, &config, viewport, &mut display)?;
        
        // Add title
        Text::with_alignment(
            "CPU Usage",
            viewport.center() + Point::new(0, -viewport.size.height as i32 / 2 + 10),
            text_style,
            Alignment::Center,
        )
        .draw(&mut display)?;
    }

    // Show the window
    window.show_static(&display);

    Ok(())
}