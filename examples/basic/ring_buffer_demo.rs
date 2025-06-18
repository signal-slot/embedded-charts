//! Ring buffer demonstration for real-time data streaming
//!
//! This example shows how to use the high-performance ring buffer
//! for efficient real-time data visualization.

use embedded_charts::{
    chart::{
        line::{LineChart, MarkerShape, MarkerStyle},
        traits::{Chart, ChartBuilder, ChartConfig, Margins},
    },
    data::{
        OverflowMode, Point2D, PointRingBuffer, RingBuffer, RingBufferConfig, RingBufferEvent,
        StaticDataSeries,
    },
};
use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle},
};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
// String type is not needed
use std::{thread, time::Duration};

// Import capture utilities for GIF generation
#[path = "../common/capture.rs"]
mod capture;
use capture::GifCapture;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create display
    let mut display = SimulatorDisplay::<Rgb565>::new(Size::new(800, 480));
    let output_settings = OutputSettingsBuilder::new().build();
    let mut window = Window::new("Ring Buffer Real-Time Demo", &output_settings);

    // Create ring buffer with capacity for 100 points
    let _data_buffer: PointRingBuffer<100> = PointRingBuffer::new();

    // Configure ring buffer for real-time use
    let config = RingBufferConfig {
        overflow_mode: OverflowMode::Overwrite,
        enable_events: true,
        track_bounds: true,
        ..Default::default()
    };
    let mut streaming_buffer: RingBuffer<Point2D, 100> = RingBuffer::with_config(config);

    // Set up event handler
    streaming_buffer.set_event_handler(|event| match event {
        RingBufferEvent::BufferFull => println!("Buffer is now full!"),
        RingBufferEvent::BoundsChanged => println!("Data bounds have changed"),
        _ => {}
    });

    // Chart configuration
    let chart_config = ChartConfig {
        margins: Margins::default(),
        show_grid: true,
        ..Default::default()
    };

    // Create line chart
    let line_chart = LineChart::builder()
        .line_color(Rgb565::CYAN)
        .line_width(2)
        .with_markers(MarkerStyle {
            shape: MarkerShape::Circle,
            size: 4,
            color: Rgb565::YELLOW,
            visible: true,
        })
        .build()?;

    // Simulation parameters
    let mut time = 0.0f32;
    let mut frame_count = 0u32;
    let mut last_fps_time = std::time::Instant::now();
    let mut fps = 0.0f32;

    // GIF capture setup
    let mut gif_capture = GifCapture::new(50); // 50ms per frame = 20fps
    let mut capture_frame_count = 0;
    let max_capture_frames = 60; // Capture 3 seconds at 20fps for a more reasonable GIF size
    let capture_enabled = std::env::var("CAPTURE_GIF").is_ok();

    if capture_enabled {
        println!("GIF capture enabled! Will save to docs/assets/ring_buffer_demo.gif");
    }

    println!("Ring Buffer Demo Started!");
    println!("- Buffer capacity: 100 points");
    println!("- Overflow mode: Overwrite oldest");
    println!("- Real-time bounds tracking enabled");
    println!();

    'running: loop {
        // Handle events
        window.update(&display);
        for event in window.events() {
            if event == SimulatorEvent::Quit {
                break 'running;
            }
        }

        // Generate new data point
        let value = generate_sensor_data(time);
        let point = Point2D::new(time, value);

        // Add to ring buffer
        streaming_buffer.push_point(point)?;

        // Update display every 5 frames for smoother animation
        if frame_count % 5 == 0 {
            // Clear display
            display.clear(Rgb565::BLACK)?;

            // Convert ring buffer to data series for chart
            let mut chart_data = StaticDataSeries::<Point2D, 256>::new();

            // Use chronological iterator to get data in proper time order
            for point in streaming_buffer.iter_chronological() {
                chart_data.push(*point)?;
            }

            // Draw main chart
            let main_viewport = Rectangle::new(Point::new(10, 40), Size::new(580, 340));
            line_chart.draw(&chart_data, &chart_config, main_viewport, &mut display)?;

            // Draw statistics panel
            draw_stats_panel(&mut display, &streaming_buffer, fps)?;

            // Draw moving average
            if let Some(avg) = streaming_buffer.moving_average(20) {
                draw_indicator(&mut display, "Moving Avg (20)", avg.y, Rgb565::GREEN, 650)?;
            }

            // Draw rate of change
            if let Some(rate) = streaming_buffer.rate_of_change() {
                draw_indicator(&mut display, "Rate of Change", rate, Rgb565::MAGENTA, 680)?;
            }

            // Show buffer visualization
            draw_buffer_visualization(&mut display, &streaming_buffer)?;

            // Capture frame for GIF if enabled
            if capture_enabled && capture_frame_count < max_capture_frames {
                gif_capture.add_frame(&display);
                capture_frame_count += 1;
                println!("Captured frame {capture_frame_count}/{max_capture_frames}");

                // Save GIF when we have enough frames
                if capture_frame_count >= max_capture_frames {
                    println!("Saving GIF...");
                    gif_capture.save_gif("docs/assets/ring_buffer_demo.gif")?;
                    println!("GIF saved successfully!");
                    // Exit after saving GIF
                    break 'running;
                }
            }
        }

        // Update time
        time += 0.05;
        frame_count += 1;

        // Calculate FPS
        if frame_count % 30 == 0 {
            let elapsed = last_fps_time.elapsed();
            fps = 30.0 / elapsed.as_secs_f32();
            last_fps_time = std::time::Instant::now();
        }

        // Simulate real-time delay
        thread::sleep(Duration::from_millis(5)); // ~200 FPS
    }

    Ok(())
}

/// Generate simulated sensor data with noise
fn generate_sensor_data(time: f32) -> f32 {
    let base = (time * 0.5).sin() * 30.0 + 50.0;
    let noise = (time * 10.0).sin() * 5.0;
    let spike = if (time as i32) % 20 == 0 { 15.0 } else { 0.0 };
    base + noise + spike
}

/// Draw statistics panel
fn draw_stats_panel(
    display: &mut SimulatorDisplay<Rgb565>,
    buffer: &RingBuffer<Point2D, 100>,
    fps: f32,
) -> Result<(), Box<dyn std::error::Error>> {
    use embedded_graphics::{
        mono_font::{ascii::FONT_6X10, MonoTextStyle},
        text::Text,
    };

    let text_style = MonoTextStyle::new(&FONT_6X10, Rgb565::WHITE);
    let stats = buffer.stats();

    // Draw stats box
    let stats_box = Rectangle::new(Point::new(600, 40), Size::new(190, 140));
    let box_style = PrimitiveStyleBuilder::new()
        .stroke_color(Rgb565::CYAN)
        .stroke_width(1)
        .fill_color(Rgb565::new(0, 0, 10))
        .build();
    stats_box.into_styled(box_style).draw(display)?;

    // Draw title
    Text::new("Ring Buffer Stats", Point::new(610, 55), text_style).draw(display)?;

    // Draw statistics
    let mut y = 75;
    Text::new(
        &format!("Size: {}/{}", buffer.len(), buffer.capacity()),
        Point::new(610, y),
        text_style,
    )
    .draw(display)?;

    y += 15;
    Text::new(
        &format!("Writes: {}", stats.total_writes),
        Point::new(610, y),
        text_style,
    )
    .draw(display)?;

    y += 15;
    Text::new(
        &format!("Overflows: {}", stats.overflow_count),
        Point::new(610, y),
        text_style,
    )
    .draw(display)?;

    y += 15;
    Text::new(
        &format!("Peak: {} pts", stats.peak_usage),
        Point::new(610, y),
        text_style,
    )
    .draw(display)?;

    y += 15;
    Text::new(&format!("FPS: {fps:.1}"), Point::new(610, y), text_style).draw(display)?;

    // Draw bounds if available
    if let Some(bounds) = buffer.bounds() {
        let bounds_box = Rectangle::new(Point::new(600, 190), Size::new(190, 80));
        bounds_box.into_styled(box_style).draw(display)?;

        Text::new("Data Bounds", Point::new(610, 205), text_style).draw(display)?;
        Text::new(
            &format!("X: {:.1} - {:.1}", bounds.min_x, bounds.max_x),
            Point::new(610, 225),
            text_style,
        )
        .draw(display)?;
        Text::new(
            &format!("Y: {:.1} - {:.1}", bounds.min_y, bounds.max_y),
            Point::new(610, 245),
            text_style,
        )
        .draw(display)?;
    }

    Ok(())
}

/// Draw a value indicator
fn draw_indicator(
    display: &mut SimulatorDisplay<Rgb565>,
    label: &str,
    value: f32,
    color: Rgb565,
    x: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    use embedded_graphics::{
        mono_font::{ascii::FONT_6X10, MonoTextStyle},
        text::Text,
    };

    let text_style = MonoTextStyle::new(&FONT_6X10, color);
    Text::new(
        &format!("{label}: {value:.2}"),
        Point::new(x, 300),
        text_style,
    )
    .draw(display)?;

    Ok(())
}

/// Draw buffer visualization at the bottom
fn draw_buffer_visualization(
    display: &mut SimulatorDisplay<Rgb565>,
    buffer: &RingBuffer<Point2D, 100>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Visualize buffer slots

    let viz_area = Rectangle::new(Point::new(10, 400), Size::new(780, 60));
    let box_style = PrimitiveStyleBuilder::new()
        .stroke_color(Rgb565::new(20, 20, 20))
        .stroke_width(1)
        .fill_color(Rgb565::BLACK)
        .build();
    viz_area.into_styled(box_style).draw(display)?;

    // Draw buffer slots
    let slot_width = 780.0 / buffer.capacity() as f32;
    let filled_ratio = buffer.len() as f32 / buffer.capacity() as f32;

    for i in 0..buffer.capacity() {
        let x = 10 + (i as f32 * slot_width) as i32;
        let filled = i < buffer.len();

        let color = if filled {
            // Color based on position in buffer
            let hue = (i as f32 / buffer.len() as f32) * 120.0; // Green to red
            let r = ((120.0 - hue) * 2.0).min(255.0) as u8;
            let g = (hue * 2.0).min(255.0) as u8;
            Rgb565::new(r >> 3, g >> 2, 0)
        } else {
            Rgb565::new(5, 5, 5)
        };

        let slot = Rectangle::new(Point::new(x, 410), Size::new(slot_width as u32 - 1, 40));
        let slot_style = PrimitiveStyleBuilder::new().fill_color(color).build();
        slot.into_styled(slot_style).draw(display)?;
    }

    // Draw position indicators
    use embedded_graphics::{
        mono_font::{ascii::FONT_6X10, MonoTextStyle},
        text::Text,
    };

    let text_style = MonoTextStyle::new(&FONT_6X10, Rgb565::WHITE);
    Text::new(
        &format!("Buffer Fill: {:.0}%", filled_ratio * 100.0),
        Point::new(350, 395),
        text_style,
    )
    .draw(display)?;

    Ok(())
}
