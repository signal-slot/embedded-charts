//! Demonstrates various chart animation techniques
#![cfg(all(feature = "std", feature = "animations", feature = "line"))]

use embedded_charts::{
    animation::*,
    chart::{Chart, ChartBuilder, ChartConfig, LineChart, MarkerShape, MarkerStyle},
    data::{DataPoint, DataSeries, Point2D, StaticDataSeries},
};
use embedded_graphics::{
    mono_font::{
        ascii::{FONT_10X20, FONT_8X13},
        MonoTextStyle,
    },
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
    text::{Alignment, Text},
};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use heapless::Vec;
use std::{thread, time::Duration};

/// Generate sample data for animation
fn generate_sample_data() -> StaticDataSeries<Point2D, 256> {
    let mut series = StaticDataSeries::new();

    for i in 0..20 {
        let x = i as f32 * 0.5;
        let y = 50.0 + 30.0 * (x * 0.5).sin() + 10.0 * (x * 1.5).cos();
        series.push(Point2D::new(x, y)).unwrap();
    }

    series
}

/// Generate alternative data for transitions
fn generate_alternative_data() -> StaticDataSeries<Point2D, 256> {
    let mut series = StaticDataSeries::new();

    for i in 0..20 {
        let x = i as f32 * 0.5;
        let y = 60.0 + 25.0 * (x * 0.3).cos() + 15.0 * (x * 2.0).sin();
        series.push(Point2D::new(x, y)).unwrap();
    }

    series
}

/// Animate a transition between two data sets
fn animate_data_transition(
    display: &mut SimulatorDisplay<Rgb565>,
    window: &mut Window,
    chart: &LineChart<Rgb565>,
    from_data: &StaticDataSeries<Point2D, 256>,
    to_data: &StaticDataSeries<Point2D, 256>,
    easing: EasingFunction,
    title: &str,
) -> bool {
    let config = ChartConfig::default();
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(640, 480));

    // Create animator for the transition
    let animator = ChartAnimator::new(from_data.clone(), to_data.clone(), easing);

    println!("Animating: {title}");

    // Animate over 60 frames
    for frame in 0..=60 {
        display.clear(Rgb565::BLACK).unwrap();

        // Calculate progress (0-100)
        let progress = ((frame as f32 / 60.0) * 100.0) as u8;

        // Get interpolated data
        if let Some(animated_data) = animator.value_at(progress) {
            // Draw the chart with animated data
            chart
                .draw(&animated_data, &config, viewport, display)
                .unwrap();
        }

        // Draw title and progress
        let title_text = format!("{title} - {progress}%");
        Text::with_alignment(
            &title_text,
            Point::new(320, 20),
            MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE),
            Alignment::Center,
        )
        .draw(display)
        .unwrap();

        // Draw progress bar
        let progress_rect = Rectangle::new(Point::new(200, 440), Size::new(240, 20));
        progress_rect
            .into_styled(PrimitiveStyle::with_stroke(Rgb565::WHITE, 1))
            .draw(display)
            .unwrap();

        let filled_width = (240.0 * (progress as f32 / 100.0)) as u32;
        Rectangle::new(Point::new(201, 441), Size::new(filled_width, 18))
            .into_styled(PrimitiveStyle::with_fill(Rgb565::GREEN))
            .draw(display)
            .unwrap();

        // Update window to show the frame
        window.update(display);

        // Check for window close event
        if window.events().any(|e| matches!(e, SimulatorEvent::Quit)) {
            return false;
        }

        // Small delay for animation
        thread::sleep(Duration::from_millis(16)); // ~60 FPS
    }

    thread::sleep(Duration::from_millis(500)); // Pause at end
    true
}

/// Demonstrate reveal animation (progressive drawing)
fn animate_reveal(
    display: &mut SimulatorDisplay<Rgb565>,
    window: &mut Window,
    _chart: &LineChart<Rgb565>,
    data: &StaticDataSeries<Point2D, 256>,
) -> bool {
    let config = ChartConfig::default();
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(640, 480));

    println!("Animating: Reveal Effect");

    let total_points = data.len();

    // Animate point by point for dramatic effect
    for points_to_show in 2..=total_points {
        display.clear(Rgb565::BLACK).unwrap();

        let progress = (points_to_show as f32 / total_points as f32 * 100.0) as u8;

        let mut reveal_data = StaticDataSeries::new();
        for i in 0..points_to_show {
            if let Some(point) = data.get(i) {
                reveal_data.push(point).unwrap();
            }
        }

        // Use the original chart with smooth curves
        _chart
            .draw(&reveal_data, &config, viewport, display)
            .unwrap();

        // Always draw title
        Text::with_alignment(
            &format!("Reveal Animation - {progress}% ({points_to_show}/{total_points} points)"),
            Point::new(320, 20),
            MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE),
            Alignment::Center,
        )
        .draw(display)
        .unwrap();

        // Update window to show the frame
        window.update(display);

        // Check for window close event
        if window.events().any(|e| matches!(e, SimulatorEvent::Quit)) {
            return false;
        }

        thread::sleep(Duration::from_millis(50)); // Smooth animation speed
    }

    thread::sleep(Duration::from_millis(500));
    true
}

/// Demonstrate fade-in animation using opacity simulation
fn animate_fade_in(
    display: &mut SimulatorDisplay<Rgb565>,
    window: &mut Window,
    _chart: &LineChart<Rgb565>,
    data: &StaticDataSeries<Point2D, 256>,
) -> bool {
    let config = ChartConfig::default();
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(640, 480));

    println!("Animating: Fade In Effect");

    for progress in (0..=100).step_by(4) {
        display.clear(Rgb565::BLACK).unwrap();

        // Create a chart with fading color
        let opacity = progress as f32 / 100.0;
        let fade_color = Rgb565::new(
            (0.0 * opacity) as u8,
            (255.0 * opacity) as u8,
            (255.0 * opacity) as u8,
        );

        // Create a new chart with faded color
        let faded_chart = LineChart::builder()
            .line_color(fade_color)
            .line_width(3)
            .with_markers(MarkerStyle {
                shape: MarkerShape::Circle,
                size: 6,
                color: Rgb565::new(
                    (255.0 * opacity) as u8,
                    (255.0 * opacity) as u8,
                    (0.0 * opacity) as u8,
                ),
                visible: progress > 20, // Markers appear after 20%
            })
            .smooth(true)
            .smooth_subdivisions(8)
            .build()
            .unwrap();

        // Draw with full data but faded colors
        faded_chart.draw(data, &config, viewport, display).unwrap();

        // Always draw title
        Text::with_alignment(
            &format!("Fade In Animation - {progress}%"),
            Point::new(320, 20),
            MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE),
            Alignment::Center,
        )
        .draw(display)
        .unwrap();

        // Update window to show the frame
        window.update(display);

        // Check for window close event
        if window.events().any(|e| matches!(e, SimulatorEvent::Quit)) {
            return false;
        }

        thread::sleep(Duration::from_millis(30));
    }

    thread::sleep(Duration::from_millis(500));
    true
}

#[allow(dead_code)]
/// Demonstrate streaming data animation
fn animate_streaming(
    display: &mut SimulatorDisplay<Rgb565>,
    window: &mut Window,
    chart: &LineChart<Rgb565>,
) -> bool {
    let config = ChartConfig::default();
    let viewport = Rectangle::new(Point::new(0, 0), Size::new(640, 480));

    println!("Animating: Streaming Data");

    let mut streaming_animator = StreamingAnimator::new();
    let mut time = 0.0f32;

    for frame in 0..150 {
        display.clear(Rgb565::BLACK).unwrap();

        // Generate new data point
        let value =
            50.0 + 30.0 * (time * 0.5).sin() + 10.0 * ((time + frame as f32 * 0.1) * 1.5).cos();
        streaming_animator.push_data(Point2D::new(time, value));
        time += 0.2;

        // Convert streaming data to StaticDataSeries for rendering
        let mut render_data = StaticDataSeries::new();
        let points: Vec<_, 100> = streaming_animator.current_data().collect();
        // Only render the last 50 points to fit in our StaticDataSeries
        let start = if points.len() > 50 {
            points.len() - 50
        } else {
            0
        };
        for point in points.iter().skip(start) {
            render_data.push(*point).ok();
        }

        // Draw the chart
        if render_data.len() > 0 {
            chart
                .draw(&render_data, &config, viewport, display)
                .unwrap();
        }

        // Draw title and info
        Text::with_alignment(
            &format!("Streaming Animation - {} points", streaming_animator.len()),
            Point::new(320, 20),
            MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE),
            Alignment::Center,
        )
        .draw(display)
        .unwrap();

        // Draw latest value indicator
        if let Some(latest) = streaming_animator.current_data().last() {
            let indicator_text = format!("Latest: ({:.1}, {:.1})", latest.x(), latest.y());
            Text::with_alignment(
                &indicator_text,
                Point::new(320, 460),
                MonoTextStyle::new(&FONT_8X13, Rgb565::YELLOW),
                Alignment::Center,
            )
            .draw(display)
            .unwrap();
        }

        // Update window and check events
        window.update(display);
        if window.events().any(|e| matches!(e, SimulatorEvent::Quit)) {
            return false;
        }

        thread::sleep(Duration::from_millis(50));
    }

    thread::sleep(Duration::from_millis(500));
    true
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create display and window
    let mut display = SimulatorDisplay::<Rgb565>::new(Size::new(640, 480));
    let output_settings = OutputSettingsBuilder::new().build();
    let mut window = Window::new("Chart Animation Demo", &output_settings);

    // Create chart with styling
    let chart = LineChart::builder()
        .line_color(Rgb565::CYAN)
        .line_width(3)
        .with_markers(MarkerStyle {
            shape: MarkerShape::Circle,
            size: 6,
            color: Rgb565::YELLOW,
            visible: true,
        })
        .smooth(true)
        .smooth_subdivisions(8)
        .build()?;

    // Generate test data
    let data1 = generate_sample_data();
    let data2 = generate_alternative_data();

    // Demo different animation types
    println!("\n=== Chart Animation Demo ===\n");

    // 1. Reveal animation
    if !animate_reveal(&mut display, &mut window, &chart, &data1) {
        println!("\nDemo interrupted by user");
        return Ok(());
    }

    // 2. Fade in animation (replaced grow)
    if !animate_fade_in(&mut display, &mut window, &chart, &data1) {
        println!("\nDemo interrupted by user");
        return Ok(());
    }

    // 3. Data transitions with different easings
    if !animate_data_transition(
        &mut display,
        &mut window,
        &chart,
        &data1,
        &data2,
        EasingFunction::Linear,
        "Linear Transition",
    ) {
        println!("\nDemo interrupted by user");
        return Ok(());
    }

    if !animate_data_transition(
        &mut display,
        &mut window,
        &chart,
        &data2,
        &data1,
        EasingFunction::EaseIn,
        "Ease In Transition",
    ) {
        println!("\nDemo interrupted by user");
        return Ok(());
    }

    if !animate_data_transition(
        &mut display,
        &mut window,
        &chart,
        &data1,
        &data2,
        EasingFunction::EaseOut,
        "Ease Out Transition",
    ) {
        println!("\nDemo interrupted by user");
        return Ok(());
    }

    if !animate_data_transition(
        &mut display,
        &mut window,
        &chart,
        &data2,
        &data1,
        EasingFunction::EaseInOut,
        "Ease In-Out Transition",
    ) {
        println!("\nDemo interrupted by user");
        return Ok(());
    }

    // 4. Streaming data animation (skip for now due to memory constraints)
    // if !animate_streaming(&mut display, &mut window, &chart) {
    //     println!("\nDemo interrupted by user");
    //     return Ok(());
    // }

    // Final screen
    display.clear(Rgb565::BLACK).unwrap();
    Text::with_alignment(
        "Animation Demo Complete!",
        Point::new(320, 240),
        MonoTextStyle::new(&FONT_10X20, Rgb565::GREEN),
        Alignment::Center,
    )
    .draw(&mut display)?;

    // Update window with final screen
    window.update(&display);

    // Keep window open until closed or timeout
    let start = std::time::Instant::now();
    loop {
        window.update(&display);
        if window.events().any(|e| matches!(e, SimulatorEvent::Quit)) {
            break;
        }
        if start.elapsed() > Duration::from_secs(5) {
            break;
        }
        thread::sleep(Duration::from_millis(16));
    }

    println!("\n=== Demo Complete ===");
    println!("The animation demo showcased:");
    println!("1. Reveal animation - Progressive line drawing");
    println!("2. Fade in animation - Opacity transition effect");
    println!("3. Data transitions - Smooth morphing between datasets");
    println!("4. Streaming animation - Real-time data updates");

    Ok(())
}
