//! Beautiful Theme Showcase - Visual Category
//!
//! This example demonstrates all the beautiful new themes with actual working charts
//! in a single window with a grid layout. Each theme is showcased with real data visualization.
//!
//! Run with: cargo run --example theme_showcase --features std

use embedded_charts::prelude::*;

// Import the common abstraction
#[path = "../common/mod.rs"]
mod common;

use common::{data, window, WindowConfig, WindowTheme};

#[cfg(feature = "std")]
fn main() -> ChartResult<()> {
    println!("🎨 Beautiful Theme Showcase - Making Eyes Happier!");
    println!("==================================================");
    println!("This example shows all themes in a single window grid layout.");
    println!("Each theme displays with real chart data for comparison.");
    println!("Press ESC or close the window to exit.\n");

    // Generate sample data once for all themes
    let temperature_data = data::temperature_data(24)?; // 24 hours of temperature
    let cpu_data = data::system_metrics(50, data::SystemMetric::CpuUsage)?;

    let window_config = WindowConfig::new("Theme Showcase - All Themes")
        .theme(WindowTheme::Default)
        .fps(30)
        .background(Rgb565::new(10, 20, 10)); // Dark gray background in RGB565 format

    // Pre-create themes outside the animation loop
    let themes = [
        ("Light", Theme::<Rgb565>::light()),
        ("Dark", Theme::<Rgb565>::dark()),
        ("Vibrant", Theme::<Rgb565>::vibrant()),
        ("Pastel", Theme::<Rgb565>::pastel()),
        ("Nature", Theme::<Rgb565>::nature()),
        ("Ocean", Theme::<Rgb565>::ocean()),
        ("Sunset", Theme::<Rgb565>::sunset()),
        ("Cyberpunk", create_improved_cyberpunk_theme()),
        ("Minimal", Theme::<Rgb565>::minimal()),
        ("Retro", Theme::<Rgb565>::retro()),
    ];

    // Pre-calculate grid layout constants outside the render loop
    let cols = 5u32;
    let rows = 2u32;
    let margin = 10u32;
    let spacing = 8u32;

    // Run the themed visualization
    window::run(window_config, move |display, viewport, _elapsed| {
        draw_theme_grid(
            display,
            viewport,
            &temperature_data,
            &cpu_data,
            &themes,
            &ThemeGridParams {
                cols,
                rows,
                margin,
                spacing,
            },
        )
    })
}

#[cfg(feature = "std")]
struct ThemeGridParams {
    cols: u32,
    rows: u32,
    margin: u32,
    spacing: u32,
}

#[cfg(feature = "std")]
#[allow(clippy::too_many_arguments)]
fn draw_theme_grid(
    display: &mut embedded_graphics_simulator::SimulatorDisplay<
        embedded_graphics::pixelcolor::Rgb565,
    >,
    viewport: embedded_graphics::primitives::Rectangle,
    temperature_data: &StaticDataSeries<Point2D, 256>,
    cpu_data: &StaticDataSeries<Point2D, 256>,
    themes: &[(&str, Theme<Rgb565>); 10],
    params: &ThemeGridParams,
) -> ChartResult<()> {
    use embedded_graphics::{
        mono_font::{ascii::FONT_6X10, MonoTextStyle},
        prelude::*,
        primitives::Rectangle,
        text::{Alignment, Text},
        Drawable,
    };

    // Themes are now passed as a parameter instead of being created here

    // Draw main title
    let title_color = Rgb565::new(31, 41, 20); // Dark blue-ish color
    let title_style = MonoTextStyle::new(&FONT_6X10, title_color);
    Text::with_alignment(
        "🎨 Theme Showcase - Beautiful Eye-Friendly Color Palettes",
        Point::new(
            viewport.top_left.x + viewport.size.width as i32 / 2,
            viewport.top_left.y + 15,
        ),
        title_style,
        Alignment::Center,
    )
    .draw(display)
    .map_err(|_| ChartError::RenderingError)?;

    // Calculate grid layout using pre-calculated constants
    let available_width =
        viewport.size.width - (2 * params.margin) - ((params.cols - 1) * params.spacing);
    let available_height =
        viewport.size.height - 40 - (2 * params.margin) - ((params.rows - 1) * params.spacing);

    let cell_width = available_width / params.cols;
    let cell_height = available_height / params.rows;

    // Draw each theme in the grid
    for (i, (theme_name, theme)) in themes.iter().enumerate() {
        let col = i % (params.cols as usize);
        let row = i / (params.cols as usize);

        let x = viewport.top_left.x
            + params.margin as i32
            + (col as u32 * (cell_width + params.spacing)) as i32;
        let y = viewport.top_left.y
            + 30i32
            + params.margin as i32
            + (row as u32 * (cell_height + params.spacing)) as i32;

        let cell_area = Rectangle::new(Point::new(x, y), Size::new(cell_width, cell_height));

        draw_theme_cell(
            display,
            cell_area,
            theme,
            theme_name,
            temperature_data,
            cpu_data,
        )?;
    }

    Ok(())
}

#[cfg(feature = "std")]
fn create_improved_cyberpunk_theme() -> Theme<embedded_graphics::pixelcolor::Rgb565> {
    // Improved cyberpunk theme with better color balance
    Theme {
        background: Rgb565::new(2, 3, 2),  // Very dark gray
        primary: Rgb565::new(0, 63, 15),   // Spring green (changed from cyan)
        secondary: Rgb565::new(31, 0, 31), // Magenta
        text: Rgb565::new(0, 63, 31),      // Cyan (moved from primary)
        grid: Rgb565::new(8, 16, 8),       // Dark gray
        accent: Rgb565::new(31, 63, 0),    // Yellow
        success: Rgb565::new(6, 51, 6),    // Lime green
        warning: Rgb565::new(31, 41, 0),   // Orange
        error: Rgb565::new(31, 17, 0),     // Red orange
    }
}

#[cfg(feature = "std")]
fn draw_theme_cell(
    display: &mut embedded_graphics_simulator::SimulatorDisplay<
        embedded_graphics::pixelcolor::Rgb565,
    >,
    area: Rectangle,
    theme: &Theme<embedded_graphics::pixelcolor::Rgb565>,
    theme_name: &str,
    temperature_data: &StaticDataSeries<Point2D, 256>,
    cpu_data: &StaticDataSeries<Point2D, 256>,
) -> ChartResult<()> {
    use embedded_graphics::{
        mono_font::{ascii::FONT_6X10, MonoTextStyle},
        prelude::*,
        primitives::{PrimitiveStyle, Rectangle},
        text::{Alignment, Text},
        Drawable,
    };

    // Draw cell background
    area.into_styled(PrimitiveStyle::with_fill(theme.background))
        .draw(display)
        .map_err(|_| ChartError::RenderingError)?;

    // Draw cell border
    area.into_styled(PrimitiveStyle::with_stroke(theme.grid, 1))
        .draw(display)
        .map_err(|_| ChartError::RenderingError)?;

    // Draw theme name
    let title_style = MonoTextStyle::new(&FONT_6X10, theme.text);
    Text::with_alignment(
        theme_name,
        Point::new(
            area.top_left.x + area.size.width as i32 / 2,
            area.top_left.y + 12,
        ),
        title_style,
        Alignment::Center,
    )
    .draw(display)
    .map_err(|_| ChartError::RenderingError)?;

    // Chart area (split into two parts)
    let chart_margin = 4;
    let chart_area = Rectangle::new(
        Point::new(area.top_left.x + chart_margin, area.top_left.y + 18),
        Size::new(
            area.size.width - 2 * chart_margin as u32,
            area.size.height - 18 - chart_margin as u32,
        ),
    );

    let chart_height = chart_area.size.height / 2;

    // Temperature chart (top half)
    let temp_area = Rectangle::new(
        chart_area.top_left,
        Size::new(chart_area.size.width, chart_height - 2),
    );

    // CPU chart (bottom half)
    let cpu_area = Rectangle::new(
        Point::new(
            chart_area.top_left.x,
            chart_area.top_left.y + chart_height as i32 + 2,
        ),
        Size::new(chart_area.size.width, chart_height - 2),
    );

    // Draw mini charts
    draw_mini_chart(
        display,
        temp_area,
        temperature_data,
        theme.primary,
        theme.grid,
        "Temp",
    )?;
    draw_mini_chart(
        display,
        cpu_area,
        cpu_data,
        theme.secondary,
        theme.grid,
        "CPU",
    )?;

    // Draw color palette strip at bottom
    draw_mini_color_palette(display, area, theme)?;

    Ok(())
}

#[cfg(feature = "std")]
fn draw_mini_chart(
    display: &mut embedded_graphics_simulator::SimulatorDisplay<
        embedded_graphics::pixelcolor::Rgb565,
    >,
    area: Rectangle,
    data: &StaticDataSeries<Point2D, 256>,
    line_color: embedded_graphics::pixelcolor::Rgb565,
    grid_color: embedded_graphics::pixelcolor::Rgb565,
    _label: &str,
) -> ChartResult<()> {
    use embedded_graphics::{
        prelude::*,
        primitives::{Line, PrimitiveStyle},
        Drawable,
    };

    // Draw simple grid
    let grid_style = PrimitiveStyle::with_stroke(grid_color, 1);

    // One vertical line in the middle
    let mid_x = area.top_left.x + area.size.width as i32 / 2;
    Line::new(
        Point::new(mid_x, area.top_left.y),
        Point::new(mid_x, area.top_left.y + area.size.height as i32),
    )
    .into_styled(grid_style)
    .draw(display)
    .map_err(|_| ChartError::RenderingError)?;

    // One horizontal line in the middle
    let mid_y = area.top_left.y + area.size.height as i32 / 2;
    Line::new(
        Point::new(area.top_left.x, mid_y),
        Point::new(area.top_left.x + area.size.width as i32, mid_y),
    )
    .into_styled(grid_style)
    .draw(display)
    .map_err(|_| ChartError::RenderingError)?;

    // Draw data line
    let line_style = PrimitiveStyle::with_stroke(line_color, 2);

    // Find data bounds
    let mut min_x = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_y = f32::NEG_INFINITY;

    for i in 0..data.len() {
        if let Some(point) = data.get(i) {
            min_x = min_x.min(point.x);
            max_x = max_x.max(point.x);
            min_y = min_y.min(point.y);
            max_y = max_y.max(point.y);
        }
    }

    // Avoid division by zero
    if max_x == min_x {
        max_x = min_x + 1.0;
    }
    if max_y == min_y {
        max_y = min_y + 1.0;
    }

    for i in 0..data.len().saturating_sub(1) {
        if let (Some(p1), Some(p2)) = (data.get(i), data.get(i + 1)) {
            // Convert data coordinates to screen coordinates
            let x1 = area.top_left.x
                + ((p1.x - min_x) / (max_x - min_x) * area.size.width as f32) as i32;
            let y1 = area.top_left.y + area.size.height as i32
                - ((p1.y - min_y) / (max_y - min_y) * area.size.height as f32) as i32;

            let x2 = area.top_left.x
                + ((p2.x - min_x) / (max_x - min_x) * area.size.width as f32) as i32;
            let y2 = area.top_left.y + area.size.height as i32
                - ((p2.y - min_y) / (max_y - min_y) * area.size.height as f32) as i32;

            Line::new(Point::new(x1, y1), Point::new(x2, y2))
                .into_styled(line_style)
                .draw(display)
                .map_err(|_| ChartError::RenderingError)?;
        }
    }

    Ok(())
}

#[cfg(feature = "std")]
fn draw_mini_color_palette(
    display: &mut embedded_graphics_simulator::SimulatorDisplay<
        embedded_graphics::pixelcolor::Rgb565,
    >,
    area: Rectangle,
    theme: &Theme<embedded_graphics::pixelcolor::Rgb565>,
) -> ChartResult<()> {
    use embedded_graphics::{
        prelude::*,
        primitives::{PrimitiveStyle, Rectangle},
        Drawable,
    };

    let colors = [
        theme.primary,
        theme.secondary,
        theme.accent,
        theme.success,
        theme.warning,
        theme.error,
    ];

    let palette_height = 8;
    let palette_y = area.top_left.y + area.size.height as i32 - palette_height - 2;
    let swatch_width = (area.size.width - 8) / colors.len() as u32;

    for (i, color) in colors.iter().enumerate() {
        let x = area.top_left.x + 4 + (i as u32 * swatch_width) as i32;

        // Draw color swatch
        Rectangle::new(
            Point::new(x, palette_y),
            Size::new(swatch_width - 1, palette_height as u32),
        )
        .into_styled(PrimitiveStyle::with_fill(*color))
        .draw(display)
        .map_err(|_| ChartError::RenderingError)?;
    }

    Ok(())
}

#[cfg(not(feature = "std"))]
fn main() {
    println!("⚠️  This visual example requires the 'std' feature to run");
    println!("   Run with: cargo run --example theme_showcase --features std");
}
