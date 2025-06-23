//! Dashboard layouts showcase - demonstrates various layout configurations

use embedded_charts::dashboard::{GridPosition, SimpleDashboard};
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle, RoundedRectangle},
    text::{Alignment, Text},
};
use embedded_graphics_simulator::SimulatorDisplay;
use std::time::{Duration, Instant};

#[path = "../common/mod.rs"]
mod common;
use common::window::{self, WindowConfig};

struct LayoutDemo {
    name: &'static str,
    dashboard: SimpleDashboard,
    charts: Vec<(GridPosition, &'static str, Rgb565)>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let window_config = WindowConfig::new("Dashboard Layouts Showcase")
        .size(Size::new(800, 600))
        .background(Rgb565::BLACK);

    // Set up layout demos
    let layouts = create_layout_demos();
    let mut current_layout = 0;
    let mut last_switch = Instant::now();
    let switch_interval = Duration::from_secs(3);

    println!("Dashboard Layouts Showcase");
    println!("==========================");
    println!("Showing {} different layouts", layouts.len());
    println!("Switches every 3 seconds. Press SPACE to switch manually.");
    println!("Close the window to exit.");

    Ok(window::run(
        window_config,
        move |display, _viewport, _elapsed| {
            render_dashboard(
                display,
                &layouts,
                &mut current_layout,
                &mut last_switch,
                switch_interval,
            )
        },
    )?)
}

fn create_layout_demos() -> Vec<LayoutDemo> {

    vec![
        // 1. Single chart
        LayoutDemo {
            name: "Single (1x1)",
            dashboard: SimpleDashboard::new(1, 1, 10),
            charts: vec![(GridPosition::new(0, 0), "Full Screen", Rgb565::RED)],
        },
        // 2. Side by side
        LayoutDemo {
            name: "Side by Side (1x2)",
            dashboard: SimpleDashboard::new(1, 2, 10),
            charts: vec![
                (GridPosition::new(0, 0), "Left", Rgb565::RED),
                (GridPosition::new(0, 1), "Right", Rgb565::BLUE),
            ],
        },
        // 3. Stacked vertically
        LayoutDemo {
            name: "Stacked (2x1)",
            dashboard: SimpleDashboard::new(2, 1, 10),
            charts: vec![
                (GridPosition::new(0, 0), "Top", Rgb565::RED),
                (GridPosition::new(1, 0), "Bottom", Rgb565::BLUE),
            ],
        },
        // 4. Quadrants
        LayoutDemo {
            name: "Quadrants (2x2)",
            dashboard: SimpleDashboard::new(2, 2, 10),
            charts: vec![
                (GridPosition::new(0, 0), "TL", Rgb565::RED),
                (GridPosition::new(0, 1), "TR", Rgb565::BLUE),
                (GridPosition::new(1, 0), "BL", Rgb565::GREEN),
                (GridPosition::new(1, 1), "BR", Rgb565::YELLOW),
            ],
        },
        // 5. Three columns
        LayoutDemo {
            name: "Three Columns (1x3)",
            dashboard: SimpleDashboard::new(1, 3, 10),
            charts: vec![
                (GridPosition::new(0, 0), "Col 1", Rgb565::RED),
                (GridPosition::new(0, 1), "Col 2", Rgb565::GREEN),
                (GridPosition::new(0, 2), "Col 3", Rgb565::BLUE),
            ],
        },
        // 6. 3x3 Grid
        LayoutDemo {
            name: "Grid 3x3",
            dashboard: SimpleDashboard::new(3, 3, 8),
            charts: vec![
                (GridPosition::new(0, 0), "1", Rgb565::RED),
                (GridPosition::new(0, 1), "2", Rgb565::GREEN),
                (GridPosition::new(0, 2), "3", Rgb565::BLUE),
                (GridPosition::new(1, 0), "4", Rgb565::YELLOW),
                (GridPosition::new(1, 1), "5", Rgb565::CYAN),
                (GridPosition::new(1, 2), "6", Rgb565::MAGENTA),
                (GridPosition::new(2, 0), "7", Rgb565::new(255, 128, 0)),
                (GridPosition::new(2, 1), "8", Rgb565::new(128, 255, 0)),
                (GridPosition::new(2, 2), "9", Rgb565::new(128, 0, 255)),
            ],
        },
        // 7. Dashboard with spanning (2x3 with one large chart)
        LayoutDemo {
            name: "With Spanning (2x3)",
            dashboard: SimpleDashboard::new(2, 3, 10),
            charts: vec![
                (
                    GridPosition::with_span(0, 0, 2, 2),
                    "Main\n(2x2)",
                    Rgb565::RED,
                ),
                (GridPosition::new(0, 2), "Side 1", Rgb565::BLUE),
                (GridPosition::new(1, 2), "Side 2", Rgb565::GREEN),
            ],
        },
        // 8. Complex layout
        LayoutDemo {
            name: "Complex Layout (3x4)",
            dashboard: SimpleDashboard::new(3, 4, 8),
            charts: vec![
                (
                    GridPosition::with_span(0, 0, 2, 3),
                    "Main Chart",
                    Rgb565::RED,
                ),
                (GridPosition::new(0, 3), "Info 1", Rgb565::BLUE),
                (GridPosition::new(1, 3), "Info 2", Rgb565::GREEN),
                (GridPosition::new(2, 0), "Status 1", Rgb565::YELLOW),
                (GridPosition::new(2, 1), "Status 2", Rgb565::CYAN),
                (GridPosition::new(2, 2), "Status 3", Rgb565::MAGENTA),
                (
                    GridPosition::new(2, 3),
                    "Status 4",
                    Rgb565::new(255, 128, 0),
                ),
            ],
        },
    ]
}

fn render_dashboard(
    display: &mut SimulatorDisplay<Rgb565>,
    layouts: &[LayoutDemo],
    current_layout: &mut usize,
    last_switch: &mut Instant,
    switch_interval: Duration,
) -> Result<(), embedded_charts::error::ChartError> {

    let total_viewport = Rectangle::new(Point::new(20, 60), Size::new(760, 520));
    let text_style = MonoTextStyle::new(&FONT_6X10, Rgb565::WHITE);
    let title_style = MonoTextStyle::new(&FONT_6X10, Rgb565::new(255, 255, 0));

    // Draw main viewport border
    Rectangle::new(total_viewport.top_left, total_viewport.size)
        .into_styled(PrimitiveStyle::with_stroke(Rgb565::new(64, 64, 64), 1))
        .draw(display)
        .map_err(|_| embedded_charts::error::ChartError::RenderingError)?;

    // Get current layout
    let layout = &layouts[*current_layout];

    // Draw title
    Text::with_alignment(
        &format!(
            "Layout {}/{}: {}",
            *current_layout + 1,
            layouts.len(),
            layout.name
        ),
        Point::new(400, 30),
        title_style,
        Alignment::Center,
    )
    .draw(display)
    .map_err(|_| embedded_charts::error::ChartError::RenderingError)?;

    // Draw each chart in the layout
    for (position, label, color) in &layout.charts {
        let viewport = layout.dashboard.get_viewport(*position, total_viewport);

        // Draw chart border
        RoundedRectangle::with_equal_corners(viewport, Size::new(5, 5))
            .into_styled(PrimitiveStyle::with_stroke(*color, 2))
            .draw(display)
            .map_err(|_| embedded_charts::error::ChartError::RenderingError)?;

        // Draw chart label
        Text::with_alignment(label, viewport.center(), text_style, Alignment::Center)
            .draw(display)
            .map_err(|_| embedded_charts::error::ChartError::RenderingError)?;

        // Show dimensions for smaller viewports
        if viewport.size.width < 200 || viewport.size.height < 200 {
            let size_text = format!("{}x{}", viewport.size.width, viewport.size.height);
            Text::with_alignment(
                &size_text,
                viewport.center() + Point::new(0, 15),
                text_style,
                Alignment::Center,
            )
            .draw(display)
            .map_err(|_| embedded_charts::error::ChartError::RenderingError)?;
        }
    }

    // Draw navigation hint
    Text::with_alignment(
        "Press SPACE for next layout | Auto-switch in 3s",
        Point::new(400, 580),
        text_style,
        Alignment::Center,
    )
    .draw(display)
    .map_err(|_| embedded_charts::error::ChartError::RenderingError)?;

    // Auto-switch based on timer
    if last_switch.elapsed() >= switch_interval {
        *current_layout = (*current_layout + 1) % layouts.len();
        *last_switch = Instant::now();
        println!("Switched to layout: {}", layouts[*current_layout].name);
    }

    Ok(())
}
