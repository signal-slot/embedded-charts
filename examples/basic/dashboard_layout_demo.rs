//! Dashboard layout demo showing viewport calculation

use embedded_charts::dashboard::{SimpleDashboard, GridPosition};
use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle, RoundedRectangle},
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
    let mut window = Window::new("Dashboard Layout Demo", &output_settings);

    // Clear display
    display.clear(Rgb565::new(0, 0, 0))?;

    // Create dashboard layout manager
    let dashboard = SimpleDashboard::new(2, 2, 10);
    let total_viewport = Rectangle::new(Point::new(20, 20), Size::new(760, 560));

    // Define text style
    let text_style = MonoTextStyle::new(&FONT_6X10, Rgb565::WHITE);

    // Draw the main viewport border
    Rectangle::new(total_viewport.top_left, total_viewport.size)
        .into_styled(PrimitiveStyle::with_stroke(Rgb565::new(128, 128, 128), 1))
        .draw(&mut display)?;

    // Chart positions and labels
    let charts = [
        (GridPosition::new(0, 0), "Chart 1\n(0, 0)", Rgb565::RED),
        (GridPosition::new(0, 1), "Chart 2\n(0, 1)", Rgb565::BLUE),
        (GridPosition::new(1, 0), "Chart 3\n(1, 0)", Rgb565::GREEN),
        (GridPosition::new(1, 1), "Chart 4\n(1, 1)", Rgb565::YELLOW),
    ];

    // Draw each chart viewport
    for (position, label, color) in &charts {
        let viewport = dashboard.get_viewport(*position, total_viewport);
        
        // Draw border
        RoundedRectangle::with_equal_corners(viewport, Size::new(5, 5))
            .into_styled(PrimitiveStyle::with_stroke(*color, 2))
            .draw(&mut display)?;
        
        // Draw label in center
        Text::with_alignment(
            label,
            viewport.center(),
            text_style,
            Alignment::Center,
        )
        .draw(&mut display)?;
        
        // Show viewport dimensions
        let size_text = format!("{}x{}", viewport.size.width, viewport.size.height);
        Text::with_alignment(
            &size_text,
            viewport.center() + Point::new(0, 20),
            text_style,
            Alignment::Center,
        )
        .draw(&mut display)?;
    }

    // Example of a chart spanning multiple cells
    let span_position = GridPosition::with_span(0, 0, 1, 2); // spans 2 columns
    let _span_viewport = dashboard.get_viewport(span_position, total_viewport);
    
    // Draw a semi-transparent overlay to show the span
    let overlay_text = "2-column span example";
    Text::with_alignment(
        overlay_text,
        Point::new(total_viewport.center().x, 550),
        text_style,
        Alignment::Center,
    )
    .draw(&mut display)?;

    // Print info to console
    println!("Dashboard Layout Demo");
    println!("====================");
    println!("Total viewport: {}x{}", total_viewport.size.width, total_viewport.size.height);
    println!("Grid: 2x2 with 10px spacing");
    println!();
    println!("Individual viewports:");
    for (position, label, _) in &charts {
        let viewport = dashboard.get_viewport(*position, total_viewport);
        println!("{}: top_left={:?}, size={}x{}", 
            label.lines().next().unwrap(),
            viewport.top_left, 
            viewport.size.width, 
            viewport.size.height
        );
    }
    println!();
    println!("Close the window to exit.");

    // Show the window and handle events
    loop {
        window.update(&display);
        
        // Check for window close event
        if window.events().any(|e| matches!(e, embedded_graphics_simulator::SimulatorEvent::Quit)) {
            break;
        }
        
        // Small delay to prevent busy waiting
        std::thread::sleep(std::time::Duration::from_millis(16)); // ~60 FPS
    }

    Ok(())
}