//! Simple animation demo that shows one animation at a time
#![cfg(all(feature = "std", feature = "animations", feature = "line"))]

use embedded_charts::{
    animation::*,
    chart::{Chart, ChartBuilder, ChartConfig, LineChart, MarkerShape, MarkerStyle},
    data::{Point2D, StaticDataSeries},
};
use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
    text::{Alignment, Text},
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create display and window
    let mut display = SimulatorDisplay::<Rgb565>::new(Size::new(800, 600));
    let output_settings = OutputSettingsBuilder::new().build();
    let mut window = Window::new("Chart Animation Demo", &output_settings);

    // Create two data sets for animation
    let mut data1 = StaticDataSeries::new();
    let mut data2 = StaticDataSeries::new();

    // Generate sine wave data
    for i in 0..50 {
        let x = i as f32 * 0.2;
        let y1 = 50.0 + 30.0 * (x * 0.5).sin();
        let y2 = 60.0 + 25.0 * (x * 0.3).cos();
        data1.push(Point2D::new(x, y1))?;
        data2.push(Point2D::new(x, y2))?;
    }

    // Create chart
    let chart = LineChart::builder()
        .line_color(Rgb565::BLUE)
        .line_width(3)
        .with_markers(MarkerStyle {
            shape: MarkerShape::Circle,
            size: 6,
            color: Rgb565::RED,
            visible: true,
        })
        .build()?;

    let config = ChartConfig::default();
    let viewport = Rectangle::new(Point::new(50, 100), Size::new(700, 400));

    // Create animator for smooth transitions
    let animator = ChartAnimator::new(data1.clone(), data2.clone(), EasingFunction::EaseInOut);

    println!("Animating chart transition with EaseInOut easing...");
    println!("Close the window to exit.");

    // Animation loop
    let mut progress = 0u8;
    let mut direction = 1i8;
    let mut frame_count = 0u32;

    'running: loop {
        // Clear display
        display.clear(Rgb565::BLACK)?;

        // Draw title
        Text::with_alignment(
            "Chart Animation Demo - Data Morphing",
            Point::new(400, 30),
            MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE),
            Alignment::Center,
        )
        .draw(&mut display)?;

        // Get interpolated data
        if let Some(animated_data) = animator.value_at(progress) {
            // Draw the animated chart
            chart.draw(&animated_data, &config, viewport, &mut display)?;
        }

        // Draw progress info
        let progress_text = format!("Progress: {progress}% | Frame: {frame_count}");
        Text::with_alignment(
            &progress_text,
            Point::new(400, 550),
            MonoTextStyle::new(&FONT_10X20, Rgb565::GREEN),
            Alignment::Center,
        )
        .draw(&mut display)?;

        // Draw progress bar
        let bar_rect = Rectangle::new(Point::new(250, 520), Size::new(300, 10));
        bar_rect
            .into_styled(PrimitiveStyle::with_stroke(Rgb565::WHITE, 1))
            .draw(&mut display)?;

        let filled_width = (300.0 * (progress as f32 / 100.0)) as u32;
        Rectangle::new(Point::new(251, 521), Size::new(filled_width, 8))
            .into_styled(PrimitiveStyle::with_fill(Rgb565::GREEN))
            .draw(&mut display)?;

        // Update the window
        window.update(&display);

        // Check for window events
        if window
            .events()
            .any(|e| matches!(e, embedded_graphics_simulator::SimulatorEvent::Quit))
        {
            break 'running;
        }

        // Update animation progress
        progress = (progress as i16 + direction as i16).clamp(0, 100) as u8;

        // Reverse direction at ends
        if progress == 0 || progress == 100 {
            direction = -direction;
        }

        frame_count += 1;

        // Control frame rate
        std::thread::sleep(std::time::Duration::from_millis(16)); // ~60 FPS
    }

    println!("Demo completed!");
    Ok(())
}
