//! Gradient fills and advanced styling showcase

use embedded_charts::{
    chart::{BarChart, BarWidth, Chart, ChartBuilder, ChartConfig},
    data::{Point2D, StaticDataSeries},
    render::ChartRenderer,
    style::{
        FillStyle, GradientDirection, LinearGradient, PatternFill, PatternType, RadialGradient,
    },
};
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::*,
    primitives::Rectangle,
    text::{Alignment, Text},
};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create display
    let mut display = SimulatorDisplay::<Rgb565>::new(Size::new(800, 600));
    let output_settings = OutputSettingsBuilder::new().build();
    let mut window = Window::new("Gradient Showcase", &output_settings);

    println!("Gradient Showcase - Rendering gradients...");
    let start_time = std::time::Instant::now();

    // Clear display
    display.clear(Rgb565::new(0, 0, 0))?;

    // Text style for labels
    let text_style = MonoTextStyle::new(&FONT_6X10, Rgb565::WHITE);

    // 1. Horizontal Linear Gradient
    {
        let rect = Rectangle::new(Point::new(20, 20), Size::new(200, 100));
        let gradient: LinearGradient<Rgb565, 8> =
            LinearGradient::simple(Rgb565::RED, Rgb565::BLUE, GradientDirection::Horizontal)?;

        // Use the optimized Rgb565 gradient rendering
        ChartRenderer::draw_linear_gradient_rect_rgb565(rect, &gradient, &mut display)?;

        Text::with_alignment(
            "Horizontal Gradient",
            rect.center() + Point::new(0, 60),
            text_style,
            Alignment::Center,
        )
        .draw(&mut display)?;
    }

    // 2. Vertical Linear Gradient
    {
        let rect = Rectangle::new(Point::new(240, 20), Size::new(200, 100));
        let gradient: LinearGradient<Rgb565, 8> =
            LinearGradient::simple(Rgb565::GREEN, Rgb565::YELLOW, GradientDirection::Vertical)?;

        // Use the optimized Rgb565 gradient rendering
        ChartRenderer::draw_linear_gradient_rect_rgb565(rect, &gradient, &mut display)?;

        Text::with_alignment(
            "Vertical Gradient",
            rect.center() + Point::new(0, 60),
            text_style,
            Alignment::Center,
        )
        .draw(&mut display)?;
    }

    // 3. Diagonal Linear Gradient
    {
        let rect = Rectangle::new(Point::new(460, 20), Size::new(200, 100));
        let gradient: LinearGradient<Rgb565, 8> =
            LinearGradient::simple(Rgb565::CYAN, Rgb565::MAGENTA, GradientDirection::Diagonal)?;

        // Use the optimized Rgb565 gradient rendering
        ChartRenderer::draw_linear_gradient_rect_rgb565(rect, &gradient, &mut display)?;

        Text::with_alignment(
            "Diagonal Gradient",
            rect.center() + Point::new(0, 60),
            text_style,
            Alignment::Center,
        )
        .draw(&mut display)?;
    }

    // 4. Multi-stop Linear Gradient
    {
        let rect = Rectangle::new(Point::new(20, 140), Size::new(200, 100));
        let mut gradient: LinearGradient<Rgb565, 8> =
            LinearGradient::new(GradientDirection::Horizontal);
        gradient.add_stop(0.0, Rgb565::RED)?;
        gradient.add_stop(0.33, Rgb565::YELLOW)?;
        gradient.add_stop(0.66, Rgb565::GREEN)?;
        gradient.add_stop(1.0, Rgb565::BLUE)?;

        // Use the optimized Rgb565 gradient rendering
        ChartRenderer::draw_linear_gradient_rect_rgb565(rect, &gradient, &mut display)?;

        Text::with_alignment(
            "Multi-stop Gradient",
            rect.center() + Point::new(0, 60),
            text_style,
            Alignment::Center,
        )
        .draw(&mut display)?;
    }

    // 5. Radial Gradient
    {
        let rect = Rectangle::new(Point::new(240, 140), Size::new(200, 100));
        let gradient: RadialGradient<Rgb565, 8> = RadialGradient::simple(
            Rgb565::WHITE,
            Rgb565::new(0, 0, 128), // Dark blue
            Point::new(50, 50),     // Center at 50%, 50%
        )?;

        // Use the optimized Rgb565 gradient rendering
        ChartRenderer::draw_radial_gradient_rect_rgb565(rect, &gradient, &mut display)?;

        Text::with_alignment(
            "Radial Gradient",
            rect.center() + Point::new(0, 60),
            text_style,
            Alignment::Center,
        )
        .draw(&mut display)?;
    }

    // 6. Pattern Fill - Horizontal Lines
    {
        let rect = Rectangle::new(Point::new(460, 140), Size::new(200, 100));
        let pattern = PatternFill::new(
            Rgb565::new(255, 128, 0), // Orange
            Rgb565::new(64, 64, 64),  // Dark gray
            PatternType::HorizontalLines {
                spacing: 5,
                width: 2,
            },
        );
        let fill = FillStyle::pattern(pattern);

        ChartRenderer::draw_filled_rectangle(rect, &fill, &mut display)?;

        Text::with_alignment(
            "Horizontal Lines",
            rect.center() + Point::new(0, 60),
            text_style,
            Alignment::Center,
        )
        .draw(&mut display)?;
    }

    // 7. Pattern Fill - Dots
    {
        let rect = Rectangle::new(Point::new(20, 260), Size::new(200, 100));
        let pattern = PatternFill::new(
            Rgb565::new(255, 255, 0), // Yellow
            Rgb565::new(0, 64, 0),    // Dark green
            PatternType::Dots {
                spacing: 10,
                radius: 3,
            },
        );
        let fill = FillStyle::pattern(pattern);

        ChartRenderer::draw_filled_rectangle(rect, &fill, &mut display)?;

        Text::with_alignment(
            "Dot Pattern",
            rect.center() + Point::new(0, 60),
            text_style,
            Alignment::Center,
        )
        .draw(&mut display)?;
    }

    // 8. Pattern Fill - Checkerboard
    {
        let rect = Rectangle::new(Point::new(240, 260), Size::new(200, 100));
        let pattern = PatternFill::new(
            Rgb565::BLACK,
            Rgb565::WHITE,
            PatternType::Checkerboard { size: 10 },
        );
        let fill = FillStyle::pattern(pattern);

        ChartRenderer::draw_filled_rectangle(rect, &fill, &mut display)?;

        Text::with_alignment(
            "Checkerboard",
            rect.center() + Point::new(0, 60),
            text_style,
            Alignment::Center,
        )
        .draw(&mut display)?;
    }

    // 9. Pattern Fill - Cross Hatch
    {
        let rect = Rectangle::new(Point::new(460, 260), Size::new(200, 100));
        let pattern = PatternFill::new(
            Rgb565::new(128, 0, 255), // Purple
            Rgb565::new(32, 32, 32),  // Very dark gray
            PatternType::CrossHatch {
                spacing: 8,
                width: 2,
            },
        );
        let fill = FillStyle::pattern(pattern);

        ChartRenderer::draw_filled_rectangle(rect, &fill, &mut display)?;

        Text::with_alignment(
            "Cross Hatch",
            rect.center() + Point::new(0, 60),
            text_style,
            Alignment::Center,
        )
        .draw(&mut display)?;
    }

    // Chart example with gradient fill
    {
        let rect = Rectangle::new(Point::new(20, 380), Size::new(640, 180));

        // Create a bar chart with gradient fills
        let bar_chart = BarChart::builder()
            .bar_width(BarWidth::Fixed(40))
            .spacing(20)
            .build()?;

        // Create sample data
        let mut data: StaticDataSeries<Point2D, 256> = StaticDataSeries::new();
        for i in 0..8 {
            let _ = data.push(Point2D::new(i as f32, 20.0 + (i as f32 * 5.0).sin() * 15.0));
        }

        // Note: In a real implementation, we would modify BarChart to support gradient fills
        // For now, just draw the background with a gradient
        let bg_gradient: LinearGradient<Rgb565, 8> = LinearGradient::simple(
            Rgb565::new(128, 128, 128), // Gray at top
            Rgb565::WHITE,              // White at bottom
            GradientDirection::Vertical,
        )?;
        // Use the optimized RGB565 gradient rendering for smooth gradients
        ChartRenderer::draw_linear_gradient_rect_rgb565(rect, &bg_gradient, &mut display)?;

        // Draw the chart over it with transparent background
        let config = ChartConfig {
            background_color: None, // Make background transparent so gradient shows through
            ..ChartConfig::default()
        };
        bar_chart.draw(&data, &config, rect, &mut display)?;

        Text::with_alignment(
            "Bar Chart with Gradient Background",
            rect.center() + Point::new(0, 100),
            text_style,
            Alignment::Center,
        )
        .draw(&mut display)?;
    }

    // Show title
    Text::with_alignment(
        "Gradient Fills & Advanced Styling Showcase",
        Point::new(400, 10),
        MonoTextStyle::new(&FONT_6X10, Rgb565::new(255, 255, 0)),
        Alignment::Center,
    )
    .draw(&mut display)?;

    // Show timing
    let elapsed = start_time.elapsed();
    println!("\nRendering completed in {:.2}s", elapsed.as_secs_f32());

    // Show the window and keep it open
    println!("\nGradient Showcase");
    println!("=================");
    println!("Demonstrating various gradient and pattern fills:");
    println!("- Linear gradients (horizontal, vertical, diagonal)");
    println!("- Multi-stop gradients");
    println!("- Radial gradients");
    println!("- Pattern fills (lines, dots, checkerboard, cross-hatch)");
    println!();
    println!("All rendering is no_std compatible!");
    println!();
    println!("Close the window to exit...");

    // Initial update to ensure window is shown
    window.update(&display);

    // Process any initial events
    for event in window.events() {
        if event == SimulatorEvent::Quit {
            return Ok(());
        }
    }

    // Main event loop
    'running: loop {
        window.update(&display);

        // Check for quit event
        for event in window.events() {
            if event == SimulatorEvent::Quit {
                break 'running;
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(16)); // ~60 FPS
    }

    Ok(())
}
