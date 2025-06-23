//! Memory usage report for embedded-charts
//!
//! Run with: cargo run --example memory_report --release

use embedded_charts::prelude::*;
use std::mem;

fn main() {
    println!("===========================================");
    println!("Embedded Charts Memory Usage Report v0.4.0");
    println!("===========================================\n");

    // Chart type sizes
    println!("📊 Chart Type Memory Footprint:");
    println!("├─ LineChart:    {:>6} bytes", mem::size_of::<LineChart>());
    println!("├─ BarChart:     {:>6} bytes", mem::size_of::<BarChart>());
    println!("├─ PieChart:     {:>6} bytes", mem::size_of::<PieChart>());
    println!("├─ GaugeChart:   {:>6} bytes", mem::size_of::<GaugeChart>());
    println!("├─ ScatterChart: {:>6} bytes", mem::size_of::<ScatterChart>());
    println!("└─ CurveChart:   {:>6} bytes", mem::size_of::<CurveChart>());

    // Core data structures
    println!("\n📦 Core Data Structures:");
    println!("├─ Point2D:      {:>6} bytes", mem::size_of::<Point2D>());
    println!("├─ DataBounds:   {:>6} bytes", mem::size_of::<DataBounds>());
    println!("├─ Margins:      {:>6} bytes", mem::size_of::<Margins>());
    println!("└─ ChartConfig:  {:>6} bytes", mem::size_of::<ChartConfig>());

    // Data series with different capacities
    println!("\n📈 Data Series Sizes:");
    println!("├─ StaticDataSeries<Point2D, 32>:   {:>6} bytes ({} points max)", 
        mem::size_of::<StaticDataSeries<Point2D, 32>>(), 32);
    println!("├─ StaticDataSeries<Point2D, 64>:   {:>6} bytes ({} points max)", 
        mem::size_of::<StaticDataSeries<Point2D, 64>>(), 64);
    println!("├─ StaticDataSeries<Point2D, 128>:  {:>6} bytes ({} points max)", 
        mem::size_of::<StaticDataSeries<Point2D, 128>>(), 128);
    println!("├─ StaticDataSeries<Point2D, 256>:  {:>6} bytes ({} points max)", 
        mem::size_of::<StaticDataSeries<Point2D, 256>>(), 256);
    println!("├─ StaticDataSeries<Point2D, 512>:  {:>6} bytes ({} points max)", 
        mem::size_of::<StaticDataSeries<Point2D, 512>>(), 512);
    println!("└─ StaticDataSeries<Point2D, 1024>: {:>6} bytes ({} points max)", 
        mem::size_of::<StaticDataSeries<Point2D, 1024>>(), 1024);

    // Style components
    println!("\n🎨 Style Components:");
    println!("├─ LineStyle:    {:>6} bytes", mem::size_of::<LineStyle>());
    println!("├─ FillStyle:    {:>6} bytes", mem::size_of::<FillStyle>());
    println!("├─ MarkerStyle:  {:>6} bytes", mem::size_of::<MarkerStyle>());
    println!("└─ BorderStyle:  {:>6} bytes", mem::size_of::<BorderStyle>());

    // Feature-specific sizes
    #[cfg(feature = "animations")]
    {
        println!("\n🎬 Animation Components:");
        println!("├─ ChartAnimator<LineChart>: {:>6} bytes", 
            mem::size_of::<ChartAnimator<LineChart>>());
        println!("└─ AnimationState:           {:>6} bytes", 
            mem::size_of::<AnimationState>());
    }

    // Memory budget scenarios
    println!("\n💾 Memory Budget Scenarios:");
    
    // Minimal setup (1KB target)
    let minimal_data = mem::size_of::<StaticDataSeries<Point2D, 32>>();
    let minimal_chart = mem::size_of::<LineChart>();
    let minimal_total = minimal_data + minimal_chart;
    println!("\n1️⃣  Minimal (1KB budget):");
    println!("   ├─ Data (32 points):  {:>6} bytes", minimal_data);
    println!("   ├─ LineChart:         {:>6} bytes", minimal_chart);
    println!("   └─ Total:             {:>6} bytes ({:.1}% of 1KB)", 
        minimal_total, (minimal_total as f32 / 1024.0) * 100.0);

    // Standard setup (4KB target)
    let standard_data = mem::size_of::<StaticDataSeries<Point2D, 128>>();
    let standard_chart = mem::size_of::<LineChart>();
    let standard_config = mem::size_of::<ChartConfig>();
    let standard_total = standard_data + standard_chart + standard_config;
    println!("\n2️⃣  Standard (4KB budget):");
    println!("   ├─ Data (128 points): {:>6} bytes", standard_data);
    println!("   ├─ LineChart:         {:>6} bytes", standard_chart);
    println!("   ├─ Config:            {:>6} bytes", standard_config);
    println!("   └─ Total:             {:>6} bytes ({:.1}% of 4KB)", 
        standard_total, (standard_total as f32 / 4096.0) * 100.0);

    // Advanced setup (16KB target)
    let advanced_data1 = mem::size_of::<StaticDataSeries<Point2D, 256>>();
    let advanced_data2 = mem::size_of::<StaticDataSeries<Point2D, 256>>();
    let advanced_data3 = mem::size_of::<StaticDataSeries<Point2D, 256>>();
    let advanced_chart = mem::size_of::<CurveChart>();
    let advanced_total = advanced_data1 + advanced_data2 + advanced_data3 + advanced_chart;
    println!("\n3️⃣  Advanced (16KB budget):");
    println!("   ├─ Data series 1:     {:>6} bytes", advanced_data1);
    println!("   ├─ Data series 2:     {:>6} bytes", advanced_data2);
    println!("   ├─ Data series 3:     {:>6} bytes", advanced_data3);
    println!("   ├─ CurveChart:        {:>6} bytes", advanced_chart);
    println!("   └─ Total:             {:>6} bytes ({:.1}% of 16KB)", 
        advanced_total, (advanced_total as f32 / 16384.0) * 100.0);

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