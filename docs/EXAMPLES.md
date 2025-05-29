# Embedded Charts Examples

Comprehensive examples demonstrating the capabilities of the embedded-charts crate. This directory contains a curated set of high-quality examples demonstrating the capabilities of the embedded-charts library. All examples use the common window system abstraction for consistent behavior and easy maintenance.

## Overview

The examples have been carefully organized into **22 focused demonstrations** that showcase the library's features without overwhelming users. Each example follows project standards and demonstrates complete functionality.

**Recent Cleanup (2025-06-07):** All examples have been thoroughly cleaned up and optimized:
- ✅ All compilation errors resolved
- ✅ Unused code and imports removed
- ✅ Common utilities streamlined (633 → ~330 lines)
- ✅ Consistent error handling patterns
- ✅ Verified functionality across all examples

## Table of Contents

- [Categories](#categories)
- [Running Examples](#running-examples)
- [Visual Assets](#visual-assets)
- [Basic Examples](#basic-examples)
- [Chart Types](#chart-types)
- [Styling Examples](#styling-examples)
- [Real-time Examples](#real-time-examples)
- [Embedded Examples](#embedded-examples)
- [Advanced Examples](#advanced-examples)
- [Tips and Best Practices](#tips-and-best-practices)

## Categories

### 📊 Basic Examples (`basic/`)

Core chart types and fundamental features. These examples are perfect for learning and demonstrating chart capabilities.

**Requirements:** `std` feature

#### Chart Types
- **`line_chart.rs`** - Basic line chart with temperature data and markers
- **`bar_chart.rs`** - Colorful bar chart with sales data and professional styling
- **`pie_chart.rs`** - Market share pie chart with segments and legends
- **`scatter_chart.rs`** - Point plotting with clustering and collision detection
- **`bubble_chart.rs`** - Multi-dimensional data visualization with size encoding
- **`gauge_chart.rs`** - Circular gauge for metrics and KPI display
- **`stacked_bar_chart.rs`** - Multi-segment revenue visualization (with animation support)
- **`stacked_line_chart.rs`** - Stacked area charts for composition analysis (with animation support)

#### System Components
- **`axis_demo.rs`** - Comprehensive axis system with grids and tick marks
- **`legend_demo.rs`** - Legend functionality and positioning options
- **`theme_showcase.rs`** - Professional themes and color palettes
- **`temperature_monitor.rs`** - Real-world monitoring dashboard

#### Animation & Data Management
- **`data_transition_demo.rs`** - Smooth data transitions with configurable easing
- **`streaming_animation_demo.rs`** - Real-time streaming with smooth updates
- **`time_provider_demo.rs`** - Time management for animations and real-time data
- **`production_ready_demo.rs`** - Auto-cycling showcase of all features

### 🎮 Interactive Examples (`interactive/`)

Advanced examples with real-time updates, animations, and complex interactions.

**Requirements:** `std` feature

- **`interactive_scatter_demo.rs`** - Interactive scatter plot with real-time updates
- **`multi_series_chart.rs`** - Multiple data series on single charts
- **`multi_series_dashboard.rs`** - Multi-chart dashboard layouts
- **`real_time_dashboard.rs`** - Real-time IoT sensor simulation with 2x2 grid
- **`unified_streaming_demo.rs`** - Unified streaming architecture with multiple animated charts

### 🔧 Common Utilities (`common/`)

Shared abstractions and utilities used by all examples (recently streamlined):

- **`display.rs`** - Unified display abstraction (MockDisplay/SimulatorDisplay)
- **`window.rs`** - Window management and event handling
- **`mod.rs`** - Essential data generation, configurations, and utility functions (cleaned up from 633 to ~330 lines)
- **`capture.rs`** - Screenshot and GIF capture utilities for documentation

**Key utilities available:**
- Data generation: `sine_wave`, `cosine_wave`, `linear_data`, `exponential_data`, `temperature_data`, `system_metrics`
- Chart configurations: `professional_line_chart`, `standard_colors`, `professional_colors`
- Layout helpers: `draw_chart_with_auto_legend`
- Utilities: `print_series_info`, `format_point`, `print_feature_requirement`

## 🎨 Visual Assets

All examples can generate screenshots when run with the `capture` feature:

```bash
# Generate screenshot while running example
cargo run --example line_chart --features std,capture

# Generate all documentation assets
./scripts/generate_assets.sh
```

Generated assets are saved to `assets/` directory:
- `theme_showcase.png` - Complete theme collection
- `line_chart_example.png` - Line chart demonstration
- `bar_chart_example.png` - Bar chart demonstration
- `pie_chart_example.png` - Pie chart demonstration
- `scatter_chart_demo_*.png` - Scatter plot with clustering
- `basic_gauge_chart.png` - Gauge chart demonstration
- `production_ready_demo_*.png` - Production dashboard

## Running Examples

### Basic Chart Examples
```bash
# Core chart types
cargo run --example line_chart --features std
cargo run --example bar_chart --features std
cargo run --example pie_chart --features std
cargo run --example scatter_chart --features "std scatter"
cargo run --example bubble_chart --features std
cargo run --example gauge_chart --features std

# Stacked charts (with optional animation)
cargo run --example stacked_bar_chart --features std
cargo run --example stacked_bar_chart --features std,animations  # Animated version
cargo run --example stacked_line_chart --features std
cargo run --example stacked_line_chart --features std,animations  # Animated version
```

### System Components
```bash
# Axes and legends
cargo run --example axis_demo --features std,fonts
cargo run --example legend_demo --features std,fonts
cargo run --example theme_showcase --features std

# Monitoring and dashboards
cargo run --example temperature_monitor --features std
cargo run --example production_ready_demo --features std
```

### Animation Examples
```bash
# Animation system
cargo run --example data_transition_demo --features std,animations
cargo run --example streaming_animation_demo --features std,animations
cargo run --example time_provider_demo --features std,animations
```

### Interactive Examples
```bash
# Interactive and real-time
cargo run --example interactive_scatter_demo --features std
cargo run --example multi_series_chart --features std
cargo run --example multi_series_dashboard --features std
cargo run --example real_time_dashboard --features std
cargo run --example unified_streaming_demo --features std,animations
```

## Features Demonstrated

### Chart Types
- **Line Charts** - Basic lines, multi-series, markers, area filling
- **Bar Charts** - Single and stacked bars with automatic spacing
- **Pie Charts** - Segmented circles with professional colors
- **Scatter Plots** - Point plotting with clustering and collision detection
- **Bubble Charts** - Multi-dimensional data with size encoding
- **Gauge Charts** - Circular metrics and KPI displays
- **Stacked Charts** - Multi-layer data visualization

### Advanced Features
- **Animation System** - Smooth transitions, easing functions, real-time updates
- **Streaming Data** - Real-time data processing and visualization
- **Interactive Elements** - User interaction and dynamic updates
- **Professional Styling** - Themes, color palettes, typography
- **Legend System** - Multiple positioning and styling options
- **Grid & Axes** - Professional grids with automatic spacing and labeling

### Technical Capabilities
- **Memory Management** - Efficient static allocation patterns
- **Error Handling** - Comprehensive error management
- **Performance** - Optimized rendering and data processing

## Example Structure

Each example follows this consistent pattern:

```rust
use embedded_charts::prelude::*;

#[path = "../common/mod.rs"]
mod common;

use common::{DisplayConfig, WindowConfig, utils};

#[cfg(feature = "std")]
fn main() -> ChartResult<()> {
    utils::run(
        DisplayConfig::new(DisplaySize::Large),
        WindowConfig::new("Window Title"),
        |display, viewport| {
            // Chart rendering code here
            Ok(())
        },
    )
}

#[cfg(not(feature = "std"))]
fn main() {
    utils::print_feature_requirement("std", "basic");
}
```

## Quality Standards

All examples in this curated set meet these standards:

✅ **Complete Functionality** - No placeholder code or TODO comments
✅ **Visual Output** - All examples produce actual visual results
✅ **Project Standards** - Follow consistent patterns and best practices
✅ **Build Successfully** - Verified with `cargo build --examples` (all compilation errors resolved)
✅ **Educational Value** - Clear demonstration of specific features
✅ **Professional Quality** - Production-ready code and styling
✅ **Clean Code** - Unused imports and dead code removed (2025-06-07 cleanup)
✅ **Consistent Patterns** - Standardized error handling and import structure

## Getting Started

1. **Start with basic chart types** - Learn fundamental concepts
2. **Explore system components** - Understand axes, legends, and themes
3. **Try animation examples** - See dynamic capabilities
4. **Examine interactive examples** - Learn real-time patterns
5. **Build your own** - Use the common system for consistency

## Animation Support

Several examples support both static and animated modes:

- **`stacked_bar_chart.rs`** - Add `--features animations` for smooth transitions
- **`stacked_line_chart.rs`** - Add `--features animations` for area morphing
- Other animation examples require the `animations` feature

## Basic Examples

### Simple Line Chart

```rust
use embedded_charts::prelude::*;
use embedded_graphics::{pixelcolor::Rgb565, prelude::*};

fn simple_line_chart() -> ChartResult<()> {
    // Create sample data
    let data = data_points![
        (0.0, 10.0),
        (1.0, 20.0),
        (2.0, 15.0),
        (3.0, 25.0),
        (4.0, 18.0),
    ];

    // Build chart
    let chart = LineChart::builder()
        .line_color(Rgb565::BLUE)
        .line_width(2)
        .build()?;

    // Configure
    let config = chart_config! {
        title: "Simple Line Chart",
        background: Rgb565::WHITE,
        margins: constants::STANDARD_MARGINS,
    };

    // Render (assuming you have a display)
    let viewport = Rectangle::new(Point::zero(), Size::new(320, 240));
    chart.draw(&data, &config, viewport, &mut display)?;

    Ok(())
}
```

### Basic Bar Chart

```rust
use embedded_charts::prelude::*;
use embedded_graphics::{pixelcolor::Rgb565, prelude::*};

fn basic_bar_chart() -> ChartResult<()> {
    // Create data for monthly sales
    let sales_data = data_points![
        (1.0, 1200.0),  // January
        (2.0, 1500.0),  // February
        (3.0, 1100.0),  // March
        (4.0, 1800.0),  // April
        (5.0, 2100.0),  // May
        (6.0, 1900.0),  // June
    ];

    // Build bar chart
    let chart = BarChart::builder()
        .orientation(BarOrientation::Vertical)
        .bar_width(BarWidth::Fixed(30))
        .bar_color(Rgb565::GREEN)
        .spacing(10)
        .build()?;

    let config = chart_config! {
        title: "Monthly Sales",
        background: Rgb565::WHITE,
        margins: constants::STANDARD_MARGINS,
        grid: true,
    };

    let viewport = Rectangle::new(Point::zero(), Size::new(400, 300));
    chart.draw(&sales_data, &config, viewport, &mut display)?;

    Ok(())
}
```

### Simple Pie Chart

```rust
use embedded_charts::prelude::*;
use embedded_graphics::{pixelcolor::Rgb565, prelude::*};

fn simple_pie_chart() -> ChartResult<()> {
    // Market share data
    let market_data = data_points![
        (1.0, 35.0),  // Product A
        (2.0, 25.0),  // Product B
        (3.0, 20.0),  // Product C
        (4.0, 15.0),  // Product D
        (5.0, 5.0),   // Others
    ];

    let chart = PieChart::builder()
        .center_style(CenterStyle::Filled(Rgb565::WHITE))
        .slice_spacing(3)
        .start_angle(0.0)
        .build()?;

    let config = chart_config! {
        title: "Market Share",
        background: Rgb565::WHITE,
        margins: constants::STANDARD_MARGINS,
    };

    let viewport = Rectangle::new(Point::zero(), Size::new(300, 300));
    chart.draw(&market_data, &config, viewport, &mut display)?;

    Ok(())
}
```

## Chart Types

### Multi-Series Line Chart

```rust
use embedded_charts::prelude::*;
use embedded_graphics::{pixelcolor::Rgb565, prelude::*};

fn multi_series_chart() -> ChartResult<()> {
    // Temperature and humidity data
    let temp_data = data_points![
        (0.0, 22.5), (1.0, 23.1), (2.0, 24.2), (3.0, 23.8),
        (4.0, 25.1), (5.0, 24.7), (6.0, 23.9), (7.0, 22.8),
    ];

    let humidity_data = data_points![
        (0.0, 65.0), (1.0, 68.0), (2.0, 72.0), (3.0, 70.0),
        (4.0, 75.0), (5.0, 73.0), (6.0, 69.0), (7.0, 66.0),
    ];

    // Create multi-series container
    let mut multi_series = MultiSeries::new();
    multi_series.add_series("Temperature (°C)", temp_data)?;
    multi_series.add_series("Humidity (%)", humidity_data)?;

    // Build chart with legend
    let chart = LineChart::builder()
        .line_color(Rgb565::BLUE) // Default color
        .line_width(2)
        .with_legend(Legend::builder()
            .position(LegendPosition::TopRight)
            .orientation(LegendOrientation::Vertical)
            .build()?)
        .with_axes(LinearAxis::builder()
            .x_label("Time (hours)")
            .y_label("Value")
            .build()?)
        .build()?;

    let config = chart_config! {
        title: "Environmental Monitoring",
        background: Rgb565::WHITE,
        margins: constants::STANDARD_MARGINS,
        grid: true,
    };

    let viewport = Rectangle::new(Point::zero(), Size::new(480, 320));
    chart.draw(&multi_series, &config, viewport, &mut display)?;

    Ok(())
}
```

### Gauge Chart with Threshold Zones

```rust
use embedded_charts::prelude::*;
use embedded_graphics::{pixelcolor::Rgb565, prelude::*};

fn gauge_with_thresholds() -> ChartResult<()> {
    // Current temperature reading
    let current_temp = 75.0;

    let chart = GaugeChart::builder()
        .gauge_type(GaugeType::Semicircle)
        .value_range(ValueRange::new(0.0, 100.0))
        // Green zone (safe)
        .add_threshold_zone(ThresholdZone {
            min: 0.0,
            max: 60.0,
            color: Rgb565::GREEN,
        })
        // Yellow zone (warning)
        .add_threshold_zone(ThresholdZone {
            min: 60.0,
            max: 80.0,
            color: Rgb565::YELLOW,
        })
        // Red zone (critical)
        .add_threshold_zone(ThresholdZone {
            min: 80.0,
            max: 100.0,
            color: Rgb565::RED,
        })
        .needle_style(NeedleStyle {
            shape: NeedleShape::Arrow,
            color: Rgb565::BLACK,
            width: 3,
        })
        .value_display_style(ValueDisplayStyle {
            show_value: true,
            show_unit: true,
            unit: "°C",
            precision: 1,
        })
        .build()?;

    // Create single-point data for current value
    let gauge_data = data_points![(0.0, current_temp)];

    let config = chart_config! {
        title: "Temperature Monitor",
        background: Rgb565::WHITE,
        margins: constants::STANDARD_MARGINS,
    };

    let viewport = Rectangle::new(Point::zero(), Size::new(300, 200));
    chart.draw(&gauge_data, &config, viewport, &mut display)?;

    Ok(())
}
```

### Scatter Plot with Clustering

```rust
use embedded_charts::prelude::*;
use embedded_graphics::{pixelcolor::Rgb565, prelude::*};

fn scatter_plot_clustering() -> ChartResult<()> {
    // Generate sample data with clusters
    let mut data = StaticDataSeries::<Point2D, 100>::new();
    
    // Cluster 1 (bottom-left)
    for i in 0..20 {
        let x = 10.0 + (i as f32) * 0.5 + random_offset();
        let y = 15.0 + (i as f32) * 0.3 + random_offset();
        data.push(Point2D::new(x, y))?;
    }
    
    // Cluster 2 (top-right)
    for i in 0..20 {
        let x = 40.0 + (i as f32) * 0.4 + random_offset();
        let y = 35.0 + (i as f32) * 0.2 + random_offset();
        data.push(Point2D::new(x, y))?;
    }

    let chart = ScatterChart::builder()
        .point_style(PointStyle {
            shape: PointShape::Circle,
            size: SizeMapping::Fixed(6),
            color: ColorMapping::Gradient {
                start_color: Rgb565::BLUE,
                end_color: Rgb565::RED,
                strategy: ColorMappingStrategy::ByValue,
            },
        })
        .collision_detection(true)
        .clustering_enabled(true)
        .build()?;

    let config = chart_config! {
        title: "Data Clustering Analysis",
        background: Rgb565::WHITE,
        margins: constants::STANDARD_MARGINS,
        grid: true,
    };

    let viewport = Rectangle::new(Point::zero(), Size::new(400, 300));
    chart.draw(&data, &config, viewport, &mut display)?;

    Ok(())
}

fn random_offset() -> f32 {
    // Simple pseudo-random for demo
    (core::ptr::addr_of!(random_offset) as usize % 100) as f32 / 100.0 - 0.5
}
```

## Styling Examples

### Professional Theme

```rust
use embedded_charts::prelude::*;
use embedded_graphics::{pixelcolor::Rgb565, prelude::*};

fn professional_styled_chart() -> ChartResult<()> {
    let data = data_points![
        (0.0, 100.0), (1.0, 120.0), (2.0, 115.0), (3.0, 135.0),
        (4.0, 140.0), (5.0, 125.0), (6.0, 150.0), (7.0, 145.0),
    ];

    // Use professional styling
    let chart = quick::professional_line_chart()
        .with_markers(MarkerStyle {
            shape: MarkerShape::Circle,
            size: 8,
            color: Rgb565::WHITE,
            visible: true,
        })
        .fill_area(Rgb565::new(70 >> 3, 130 >> 2, 180 >> 3)) // Semi-transparent steel blue
        .with_grid(GridSystem::builder()
            .horizontal_linear(GridSpacing::Fixed(20.0))
            .vertical_linear(GridSpacing::Fixed(10.0))
            .style(GridStyle {
                major_line_color: Rgb565::new(200 >> 3, 200 >> 2, 200 >> 3),
                minor_line_color: Rgb565::new(240 >> 3, 240 >> 2, 240 >> 3),
                major_line_width: 1,
                minor_line_width: 1,
            })
            .build()?)
        .build()?;

    let config = chart_config! {
        title: "Quarterly Revenue Growth",
        background: Rgb565::WHITE,
        margins: constants::STANDARD_MARGINS,
        grid: true,
    };

    let viewport = Rectangle::new(Point::zero(), Size::new(500, 350));
    chart.draw(&data, &config, viewport, &mut display)?;

    Ok(())
}
```

### Dark Theme for OLED

```rust
use embedded_charts::prelude::*;
use embedded_graphics::{pixelcolor::Rgb565, prelude::*};

fn dark_theme_chart() -> ChartResult<()> {
    let data = data_points![
        (0.0, 45.0), (1.0, 52.0), (2.0, 48.0), (3.0, 55.0),
        (4.0, 60.0), (5.0, 58.0), (6.0, 62.0), (7.0, 65.0),
    ];

    let chart = LineChart::builder()
        .line_color(Rgb565::CYAN)
        .line_width(3)
        .with_markers(MarkerStyle {
            shape: MarkerShape::Circle,
            size: 6,
            color: Rgb565::WHITE,
            visible: true,
        })
        .fill_area(Rgb565::new(0, 15, 15)) // Dark cyan fill
        .build()?;

    // Apply dark theme
    let theme = quick::dark_theme();
    chart.apply_theme(&theme);

    let config = chart_config! {
        title: "OLED Optimized Chart",
        background: Rgb565::BLACK,
        margins: constants::STANDARD_MARGINS,
        grid: true,
    };

    let viewport = Rectangle::new(Point::zero(), Size::new(320, 240));
    chart.draw(&data, &config, viewport, &mut display)?;

    Ok(())
}
```

### Custom Color Palette

```rust
use embedded_charts::prelude::*;
use embedded_graphics::{pixelcolor::Rgb565, prelude::*};

fn custom_palette_chart() -> ChartResult<()> {
    // Create custom color palette
    let mut custom_palette = ColorPalette::new();
    custom_palette.add_color(Rgb565::new(31, 20, 0))?;  // Orange
    custom_palette.add_color(Rgb565::new(0, 31, 15))?;  // Green
    custom_palette.add_color(Rgb565::new(15, 0, 31))?;  // Purple
    custom_palette.add_color(Rgb565::new(31, 0, 15))?;  // Pink

    // Multi-series data
    let series1 = data_points![(0.0, 10.0), (1.0, 15.0), (2.0, 12.0), (3.0, 18.0)];
    let series2 = data_points![(0.0, 8.0), (1.0, 12.0), (2.0, 14.0), (3.0, 16.0)];
    let series3 = data_points![(0.0, 5.0), (1.0, 8.0), (2.0, 10.0), (3.0, 12.0)];

    let mut multi_series = MultiSeries::new();
    multi_series.add_series("Series A", series1)?;
    multi_series.add_series("Series B", series2)?;
    multi_series.add_series("Series C", series3)?;

    let chart = LineChart::builder()
        .line_width(2)
        .color_palette(custom_palette)
        .with_legend(Legend::builder()
            .position(LegendPosition::BottomCenter)
            .orientation(LegendOrientation::Horizontal)
            .build()?)
        .build()?;

    let config = chart_config! {
        title: "Custom Color Scheme",
        background: Rgb565::WHITE,
        margins: constants::STANDARD_MARGINS,
    };

    let viewport = Rectangle::new(Point::zero(), Size::new(400, 300));
    chart.draw(&multi_series, &config, viewport, &mut display)?;

    Ok(())
}
```

## Real-time Examples

### Streaming Data Chart

```rust
#[cfg(feature = "animations")]
use embedded_charts::prelude::*;
use embedded_graphics::{pixelcolor::Rgb565, prelude::*};

#[cfg(feature = "animations")]
fn streaming_chart_example() -> ChartResult<()> {
    // Create sliding window for real-time data
    let mut streaming_data: SlidingWindowSeries<Point2D, 50> = 
        SlidingWindowSeries::new();

    // Simulate sensor readings
    for i in 0..100 {
        let timestamp = i as f32 * 0.1;
        let temperature = 20.0 + (timestamp * 0.5).sin() * 5.0 + random_noise();
        
        streaming_data.push(Point2D::new(timestamp, temperature))?;
        
        // Render chart (in real application, this would be in update loop)
        let chart = LineChart::builder()
            .line_color(Rgb565::GREEN)
            .line_width(2)
            .fill_area(Rgb565::new(0, 15, 0))
            .build()?;

        let config = chart_config! {
            title: "Real-time Temperature",
            background: Rgb565::BLACK,
            margins: constants::MINIMAL_MARGINS,
        };

        let viewport = Rectangle::new(Point::zero(), Size::new(320, 240));
        chart.draw(&streaming_data, &config, viewport, &mut display)?;
        
        // In real application: delay or wait for next sensor reading
    }

    Ok(())
}

fn random_noise() -> f32 {
    // Simple noise simulation
    (core::ptr::addr_of!(random_noise) as usize % 200) as f32 / 100.0 - 1.0
}
```

### Animated Chart Transitions

```rust
#[cfg(feature = "animations")]
use embedded_charts::prelude::*;
use embedded_graphics::{pixelcolor::Rgb565, prelude::*};

#[cfg(feature = "animations")]
fn animated_transition_example() -> ChartResult<()> {
    // Initial data
    let initial_data = data_points![
        (0.0, 10.0), (1.0, 15.0), (2.0, 12.0), (3.0, 18.0), (4.0, 16.0)
    ];

    // Target data
    let target_data = data_points![
        (0.0, 20.0), (1.0, 25.0), (2.0, 22.0), (3.0, 28.0), (4.0, 26.0)
    ];

    let chart = LineChart::builder()
        .line_color(Rgb565::BLUE)
        .line_width(2)
        .build()?;

    // Create transition animator
    let animator = chart.create_transition_animator(
        initial_data,
        target_data,
        EasingFunction::EaseInOut,
    );

    let config = chart_config! {
        title: "Animated Transition",
        background: Rgb565::WHITE,
        margins: constants::STANDARD_MARGINS,
    };

    // Animate transition (in real application, this would be in update loop)
    for frame in 0..=100 {
        let progress = Progress::from_percentage(frame);
        let viewport = Rectangle::new(Point::zero(), Size::new(400, 300));
        
        chart.draw_animated(&initial_data, &config, viewport, &mut display, progress)?;
        
        // In real application: delay for frame rate control
    }

    Ok(())
}
```

## Embedded Examples

### no_std Temperature Monitor

```rust
#![no_std]

use embedded_charts::prelude::*;
use embedded_graphics::{pixelcolor::Rgb565, prelude::*};

fn embedded_temperature_monitor() -> ChartResult<()> {
    // Create data series with static allocation
    let mut sensor_data: StaticDataSeries<Point2D, 64> = StaticDataSeries::new();
    
    // Simulate sensor readings (in real application, read from actual sensor)
    for i in 0..20 {
        let time = i as f32;
        let temp = 22.0 + (i as f32 * 0.1).sin() * 2.0;
        let _ = sensor_data.push(Point2D::new(time, temp));
    }

    // Create minimal chart for small display
    let chart = LineChart::builder()
        .line_color(Rgb565::BLUE)
        .line_width(2)
        .margins(constants::MINIMAL_MARGINS)
        .build()?;

    let config = chart_config! {
        title: "Temp",
        background: Rgb565::WHITE,
        margins: constants::MINIMAL_MARGINS,
    };

    // Render to small embedded display (128x64 OLED)
    let viewport = Rectangle::new(Point::zero(), Size::new(128, 64));
    chart.draw(&sensor_data, &config, viewport, &mut display)?;

    Ok(())
}
```

### IoT Sensor Dashboard

```rust
#![no_std]

use embedded_charts::prelude::*;
use embedded_graphics::{pixelcolor::Rgb565, prelude::*};

fn iot_dashboard() -> ChartResult<()> {
    // Multiple sensor readings
    let mut temp_data: StaticDataSeries<Point2D, 32> = StaticDataSeries::new();
    let mut humidity_data: StaticDataSeries<Point2D, 32> = StaticDataSeries::new();
    let mut pressure_data: StaticDataSeries<Point2D, 32> = StaticDataSeries::new();

    // Simulate sensor readings
    for i in 0..20 {
        let time = i as f32;
        let _ = temp_data.push(Point2D::new(time, 22.0 + (time * 0.1).sin() * 3.0));
        let _ = humidity_data.push(Point2D::new(time, 65.0 + (time * 0.15).cos() * 10.0));
        let _ = pressure_data.push(Point2D::new(time, 1013.0 + (time * 0.05).sin() * 5.0));
    }

    // Create dashboard layout with multiple charts
    
    // Temperature chart (top-left)
    let temp_chart = LineChart::builder()
        .line_color(Rgb565::RED)
        .line_width(1)
        .margins(constants::MINIMAL_MARGINS)
        .build()?;

    let temp_viewport = Rectangle::new(Point::new(0, 0), Size::new(160, 80));
    temp_chart.draw(&temp_data, &chart_config! { title: "Temp °C" }, temp_viewport, &mut display)?;

    // Humidity chart (top-right)
    let humidity_chart = LineChart::builder()
        .line_color(Rgb565::BLUE)
        .line_width(1)
        .margins(constants::MINIMAL_MARGINS)
        .build()?;

    let humidity_viewport = Rectangle::new(Point::new(160, 0), Size::new(160, 80));
    humidity_chart.draw(&humidity_data, &chart_config! { title: "Humidity %" }, humidity_viewport, &mut display)?;

    // Pressure gauge (bottom)
    let pressure_gauge = GaugeChart::builder()
        .gauge_type(GaugeType::Semicircle)
        .value_range(ValueRange::new(1000.0, 1030.0))
        .needle_style(NeedleStyle {
            shape: NeedleShape::Arrow,
            color: Rgb565::BLACK,
            width: 1,
        })
        .build()?;

    let current_pressure = pressure_data.last().map(|p| p.y()).unwrap_or(1013.0);
    let pressure_point = data_points![(0.0, current_pressure)];
    let pressure_viewport = Rectangle::new(Point::new(80, 80), Size::new(160, 80));
    pressure_gauge.draw(&pressure_point, &chart_config! { title: "Pressure hPa" }, pressure_viewport, &mut display)?;

    Ok(())
}
```

## Advanced Examples

### Performance Monitoring Dashboard

```rust
use embedded_charts::prelude::*;
use embedded_graphics::{pixelcolor::Rgb565, prelude::*};

fn performance_dashboard() -> ChartResult<()> {
    // System metrics data
    let cpu_data = data_points![
        (0.0, 25.0), (1.0, 30.0), (2.0, 45.0), (3.0, 60.0),
        (4.0, 55.0), (5.0, 40.0), (6.0, 35.0), (7.0, 30.0),
    ];

    let memory_data = data_points![
        (0.0, 40.0), (1.0, 42.0), (2.0, 45.0), (3.0, 48.0),
        (4.0, 50.0), (5.0, 52.0), (6.0, 55.0), (7.0, 58.0),
    ];

    let network_data = data_points![
        (0.0, 10.0), (1.0, 25.0), (2.0, 15.0), (3.0, 35.0),
        (4.0, 20.0), (5.0, 30.0), (6.0, 40.0), (7.0, 25.0),
    ];

    // Create multi-series chart
    let mut multi_series = MultiSeries::new();
    multi_series.add_series("CPU Usage (%)", cpu_data)?;
    multi_series.add_series("Memory Usage (%)", memory_data)?;
    multi_series.add_series("Network I/O (MB/s)", network_data)?;

    // Professional dashboard styling
    let chart = quick::professional_line_chart()
        .with_legend(Legend::builder()
            .position(LegendPosition::TopLeft)
            .orientation(LegendOrientation::Vertical)
            .build()?)
        .with_axes(LinearAxis::builder()
            .x_label("Time (minutes)")
            .y_label("Usage (%)")
            .build()?)
        .with_grid(GridSystem::builder()
            .horizontal_linear(GridSpacing::Fixed(10.0))
            .vertical_linear(GridSpacing::Fixed(1.0))
            .build()?)
        .build()?;

    let config = chart_config! {
        title: "System Performance Monitor",
        background: Rgb565::WHITE,
        margins: constants::STANDARD_MARGINS,
        grid: true,
    };

    let viewport = Rectangle::new(Point::zero(), Size::new(600, 400));
    chart.draw(&multi_series, &config, viewport, &mut display)?;

    Ok(())
}
```

### Memory-Optimized Chart for Memory-Constrained Systems

```rust
#![no_std]

use embedded_charts::prelude::*;
use embedded_graphics::{pixelcolor::BinaryColor, prelude::*};

fn memory_optimized_chart() -> ChartResult<()> {
    // Use minimal data series for memory-constrained systems
    let mut sensor_data: StaticDataSeries<IntPoint, 16> = StaticDataSeries::new();
    
    // Use integer coordinates to save memory
    for i in 0..12 {
        let time = i;
        let value = 50 + (i * 3) % 20; // Simple pattern
        let _ = sensor_data.push(IntPoint::new(time, value));
    }

    // Minimal chart configuration for monochrome display
    let chart = LineChart::builder()
        .line_color(BinaryColor::On)
        .line_width(1)
        .margins(Margins::all(2)) // Minimal margins
        .build()?;

    let config = chart_config! {
        background: BinaryColor::Off,
        margins: Margins::all(2),
    };

    // Small display viewport (64x32 OLED)
    let viewport = Rectangle::new(Point::zero(), Size::new(64, 32));
    chart.draw(&sensor_data, &config, viewport, &mut display)?;

    Ok(())
}
```

## Tips and Best Practices

### Memory Management

```rust
// Use appropriate data series sizes for your system
type SmallSeries = StaticDataSeries<Point2D, 32>;   // For memory-constrained systems
type MediumSeries = StaticDataSeries<Point2D, 128>; // For standard systems
type LargeSeries = StaticDataSeries<Point2D, 512>;  // For full-featured systems

// Monitor memory usage
let memory_manager: ChartMemoryManager<2048> = ChartMemoryManager::new();
let stats = memory_manager.stats();
println!("Memory usage: {}/{} bytes", stats.used_bytes, stats.total_bytes);
```

### Performance Optimization

```rust
// Use minimal margins for small displays
let config = chart_config! {
    margins: constants::MINIMAL_MARGINS,
    grid: false, // Disable grid for better performance
};

// Use integer coordinates for memory efficiency
let int_data: StaticDataSeries<IntPoint, 64> = StaticDataSeries::new();

// Optimize viewport size for your display
let small_viewport = Rectangle::new(Point::zero(), Size::new(128, 64));
let medium_viewport = Rectangle::new(Point::zero(), Size::new(320, 240));
```

### Error Handling Patterns

```rust
use embedded_charts::error::*;

fn robust_chart_rendering() -> ChartResult<()> {
    let data = data_points![(0.0, 10.0), (1.0, 20.0)];
    
    let chart = LineChart::builder()
        .line_color(Rgb565::BLUE)
        .build()
        .map_err(|e| ChartError::BuildError(format!("Failed to build chart: {:?}", e)))?;

    match chart.draw(&data, &config, viewport, &mut display) {
        Ok(()) => println!("Chart rendered successfully"),
        Err(ChartError::InvalidData(msg)) => {
            println!("Data validation failed: {}", msg);
            // Handle data error - maybe use default data
        },
        Err(ChartError::RenderError(err)) => {
            println!("Rendering failed: {:?}", err);
            // Handle render error - maybe retry or use simpler chart
        },
        Err(ChartError::MemoryError(err)) => {
            println!("Memory error: {:?}", err);
            // Handle memory error - maybe reduce data size
        },
    }

    Ok(())
}
```

### Feature-Conditional Code

```rust
// Conditional compilation for different features
#[cfg(feature = "animations")]
fn with_animations() {
    // Animation-specific code
    let streaming_data: SlidingWindowSeries<Point2D, 100> = SlidingWindowSeries::new();
}

#[cfg(not(feature = "animations"))]
fn without_animations() {
    // Static chart code
    let static_data: StaticDataSeries<Point2D,
100> = StaticDataSeries::new();
}

#[cfg(feature = "color-support")]
fn with_colors() {
    let palette = quick::professional_colors();
}

#[cfg(not(feature = "color-support"))]
fn monochrome_only() {
    use embedded_graphics::pixelcolor::BinaryColor;
    let chart_color = BinaryColor::On;
}
```

This comprehensive examples documentation demonstrates the full capabilities of the embedded-charts crate, from basic usage to advanced real-time applications and embedded system optimization.