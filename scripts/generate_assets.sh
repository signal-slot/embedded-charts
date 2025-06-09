#!/bin/bash

# Generate Visual Assets for Documentation
# This script runs all examples with capture enabled to generate screenshots

set -e

echo "🎨 Generating visual assets for embedded-graphics-graph documentation..."

# Create assets directory
mkdir -p assets

# Generate theme showcase
echo "📸 Generating theme showcase..."
cargo run --example theme_showcase --features std,capture,line,bar,pie

# Generate basic chart examples
echo "📊 Generating basic chart examples..."
cargo run --example line_chart --features std,capture
cargo run --example bar_chart --features std,capture
cargo run --example pie_chart --features std,capture
cargo run --example donut_chart --features std,capture,pie

# Generate advanced chart examples
echo "🔬 Generating advanced chart examples..."
cargo run --example scatter_chart --features std,capture,scatter
cargo run --example gauge_chart --features std,capture,gauge

# Generate production dashboard
echo "🚀 Generating production dashboard..."
cargo run --example production_ready_demo --features std,capture

# Generate multi-series examples
echo "📈 Generating multi-series examples..."
cargo run --example multi_series_dashboard --features std,capture,line,bar,animations

# Generate real-time examples
echo "⚡ Generating real-time examples..."
cargo run --example real_time_dashboard --features std,capture,line,gauge,animations

echo "✅ All visual assets generated successfully!"
echo "📁 Assets saved to: assets/"
echo ""
echo "Generated files:"
ls -la assets/