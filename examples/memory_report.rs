//! Memory usage report for embedded-charts
//!
//! Run with: cargo run --example memory_report --release

#![allow(clippy::uninlined_format_args)]

#[cfg(feature = "bar")]
use embedded_charts::chart::BarChart;
#[cfg(feature = "gauge")]
use embedded_charts::chart::GaugeChart;
#[cfg(feature = "pie")]
use embedded_charts::chart::PieChart;
#[cfg(feature = "scatter")]
use embedded_charts::chart::ScatterChart;
use embedded_charts::prelude::*;
use embedded_graphics::pixelcolor::{BinaryColor, Rgb565};
use std::mem;

fn main() {
    println!("===========================================");
    println!("Embedded Charts Memory Usage Report v0.4.0");
    println!("===========================================\n");

    // Chart type sizes
    println!("📊 Chart Type Memory Footprint:");
    #[cfg(feature = "line")]
    println!(
        "├─ LineChart:    {:>6} bytes",
        mem::size_of::<LineChart<Rgb565>>()
    );
    #[cfg(feature = "bar")]
    println!(
        "├─ BarChart:     {:>6} bytes",
        mem::size_of::<BarChart<Rgb565>>()
    );
    #[cfg(feature = "pie")]
    println!(
        "├─ PieChart:     {:>6} bytes",
        mem::size_of::<PieChart<Rgb565>>()
    );
    #[cfg(feature = "gauge")]
    println!(
        "├─ GaugeChart:   {:>6} bytes",
        mem::size_of::<GaugeChart<Rgb565>>()
    );
    #[cfg(feature = "scatter")]
    println!(
        "├─ ScatterChart: {:>6} bytes",
        mem::size_of::<ScatterChart<Rgb565>>()
    );
    #[cfg(feature = "line")]
    println!(
        "└─ CurveChart:   {:>6} bytes",
        mem::size_of::<CurveChart<Rgb565>>()
    );

    // Core data structures
    println!("\n📦 Core Data Structures:");
    println!("├─ Point2D:      {:>6} bytes", mem::size_of::<Point2D>());
    println!(
        "├─ DataBounds:   {:>6} bytes",
        mem::size_of::<DataBounds<f32, f32>>()
    );
    println!("├─ Margins:      {:>6} bytes", mem::size_of::<Margins>());
    println!(
        "└─ ChartConfig:  {:>6} bytes",
        mem::size_of::<ChartConfig<Rgb565>>()
    );

    // Data series with different capacities
    println!("\n📈 Data Series Sizes:");
    println!(
        "├─ StaticDataSeries<Point2D, 32>:   {:>6} bytes ({} points max)",
        mem::size_of::<StaticDataSeries<Point2D, 32>>(),
        32
    );
    println!(
        "├─ StaticDataSeries<Point2D, 64>:   {:>6} bytes ({} points max)",
        mem::size_of::<StaticDataSeries<Point2D, 64>>(),
        64
    );
    println!(
        "├─ StaticDataSeries<Point2D, 128>:  {:>6} bytes ({} points max)",
        mem::size_of::<StaticDataSeries<Point2D, 128>>(),
        128
    );
    println!(
        "├─ StaticDataSeries<Point2D, 256>:  {:>6} bytes ({} points max)",
        mem::size_of::<StaticDataSeries<Point2D, 256>>(),
        256
    );
    println!(
        "├─ StaticDataSeries<Point2D, 512>:  {:>6} bytes ({} points max)",
        mem::size_of::<StaticDataSeries<Point2D, 512>>(),
        512
    );
    println!(
        "└─ StaticDataSeries<Point2D, 1024>: {:>6} bytes ({} points max)",
        mem::size_of::<StaticDataSeries<Point2D, 1024>>(),
        1024
    );

    // Style components
    println!("\n🎨 Style Components:");
    println!(
        "├─ LineStyle:    {:>6} bytes",
        mem::size_of::<LineStyle<Rgb565>>()
    );
    println!(
        "├─ FillStyle:    {:>6} bytes",
        mem::size_of::<FillStyle<Rgb565>>()
    );
    println!(
        "├─ MarkerStyle:  {:>6} bytes",
        mem::size_of::<MarkerStyle<Rgb565>>()
    );
    println!(
        "└─ BorderStyle:  {:>6} bytes",
        mem::size_of::<BorderStyle<Rgb565>>()
    );

    // Feature-specific sizes
    #[cfg(feature = "animations")]
    {
        println!("\n🎬 Animation Components:");
        // Note: ChartAnimator requires chart types to implement Interpolatable
        // println!(
        //     "├─ ChartAnimator<LineChart>: {:>6} bytes",
        //     mem::size_of::<ChartAnimator<LineChart<Rgb565>>>()
        // );
        println!(
            "└─ TimeBasedProgress:        {:>6} bytes",
            mem::size_of::<TimeBasedProgress>()
        );
    }

    // Memory budget scenarios
    println!("\n💾 Memory Budget Scenarios:");

    // Minimal setup (1KB target)
    let minimal_data = mem::size_of::<StaticDataSeries<Point2D, 32>>();
    #[cfg(feature = "line")]
    let minimal_chart = mem::size_of::<LineChart<BinaryColor>>();
    #[cfg(not(feature = "line"))]
    let minimal_chart = 0;
    let minimal_total = minimal_data + minimal_chart;
    println!("\n1️⃣  Minimal (1KB budget):");
    println!("   ├─ Data (32 points):  {:>6} bytes", minimal_data);
    println!("   ├─ LineChart:         {:>6} bytes", minimal_chart);
    println!(
        "   └─ Total:             {:>6} bytes ({:.1}% of 1KB)",
        minimal_total,
        (minimal_total as f32 / 1024.0) * 100.0
    );

    // Standard setup (4KB target)
    let standard_data = mem::size_of::<StaticDataSeries<Point2D, 128>>();
    #[cfg(feature = "line")]
    let standard_chart = mem::size_of::<LineChart<Rgb565>>();
    #[cfg(not(feature = "line"))]
    let standard_chart = 0;
    let standard_config = mem::size_of::<ChartConfig<Rgb565>>();
    let standard_total = standard_data + standard_chart + standard_config;
    println!("\n2️⃣  Standard (4KB budget):");
    println!("   ├─ Data (128 points): {:>6} bytes", standard_data);
    println!("   ├─ LineChart:         {:>6} bytes", standard_chart);
    println!("   ├─ Config:            {:>6} bytes", standard_config);
    println!(
        "   └─ Total:             {:>6} bytes ({:.1}% of 4KB)",
        standard_total,
        (standard_total as f32 / 4096.0) * 100.0
    );

    // Advanced setup (16KB target)
    let advanced_data1 = mem::size_of::<StaticDataSeries<Point2D, 256>>();
    let advanced_data2 = mem::size_of::<StaticDataSeries<Point2D, 256>>();
    let advanced_data3 = mem::size_of::<StaticDataSeries<Point2D, 256>>();
    #[cfg(feature = "line")]
    let advanced_chart = mem::size_of::<CurveChart<Rgb565>>();
    #[cfg(not(feature = "line"))]
    let advanced_chart = 0;
    let advanced_total = advanced_data1 + advanced_data2 + advanced_data3 + advanced_chart;
    println!("\n3️⃣  Advanced (16KB budget):");
    println!("   ├─ Data series 1:     {:>6} bytes", advanced_data1);
    println!("   ├─ Data series 2:     {:>6} bytes", advanced_data2);
    println!("   ├─ Data series 3:     {:>6} bytes", advanced_data3);
    println!("   ├─ CurveChart:        {:>6} bytes", advanced_chart);
    println!(
        "   └─ Total:             {:>6} bytes ({:.1}% of 16KB)",
        advanced_total,
        (advanced_total as f32 / 16384.0) * 100.0
    );

    // Memory efficiency tips
    println!("\n💡 Memory Optimization Tips:");
    println!("├─ Use smallest data series capacity that fits your needs");
    println!("├─ Share data series between charts when possible");
    println!("├─ Disable unused features to reduce code size");
    println!("├─ Use integer math for memory-constrained systems");
    println!("└─ Consider streaming data instead of storing all points");

    // Platform-specific notes
    println!("\n🎯 Platform Considerations:");
    println!("├─ Cortex-M0: Use integer-math feature (no FPU)");
    println!("├─ Cortex-M4: Enable floating-point for FPU usage");
    println!("├─ ESP32: Consider dual-core rendering strategies");
    println!("└─ RISC-V: Platform-specific optimizations available");

    println!("\n===========================================");
}
