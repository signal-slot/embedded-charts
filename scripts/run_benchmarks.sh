#!/bin/bash
# Run comprehensive benchmarks for embedded-charts v0.4.0

set -e

echo "====================================="
echo "Embedded Charts v0.4.0 Benchmark Suite"
echo "====================================="
echo ""

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Create benchmark results directory
RESULTS_DIR="benchmark_results/$(date +%Y%m%d_%H%M%S)"
mkdir -p "$RESULTS_DIR"

echo "Results will be saved to: $RESULTS_DIR"
echo ""

# Function to run a benchmark
run_benchmark() {
    local bench_name=$1
    local description=$2
    
    echo -e "${YELLOW}Running $description...${NC}"
    
    if cargo bench --bench "$bench_name" -- --save-baseline "$bench_name"_baseline 2>&1 | tee "$RESULTS_DIR/$bench_name.log"; then
        echo -e "${GREEN}✓ $description completed${NC}"
    else
        echo -e "${RED}✗ $description failed${NC}"
        exit 1
    fi
    echo ""
}

# Function to generate memory report
generate_memory_report() {
    echo -e "${YELLOW}Generating memory usage report...${NC}"
    
    cat > "$RESULTS_DIR/memory_report.rs" << 'EOF'
use embedded_charts::prelude::*;
use std::mem;

fn main() {
    println!("=== Memory Usage Report ===\n");
    
    // Chart sizes
    println!("Chart Type Memory Footprint:");
    println!("  LineChart: {} bytes", mem::size_of::<LineChart>());
    println!("  BarChart: {} bytes", mem::size_of::<BarChart>());
    println!("  PieChart: {} bytes", mem::size_of::<PieChart>());
    println!("  GaugeChart: {} bytes", mem::size_of::<GaugeChart>());
    println!("  ScatterChart: {} bytes", mem::size_of::<ScatterChart>());
    
    // Data structure sizes
    println!("\nData Structure Sizes:");
    println!("  Point2D: {} bytes", mem::size_of::<Point2D>());
    println!("  DataBounds: {} bytes", mem::size_of::<DataBounds>());
    println!("  StaticDataSeries<Point2D, 32>: {} bytes", mem::size_of::<StaticDataSeries<Point2D, 32>>());
    println!("  StaticDataSeries<Point2D, 256>: {} bytes", mem::size_of::<StaticDataSeries<Point2D, 256>>());
    println!("  StaticDataSeries<Point2D, 1024>: {} bytes", mem::size_of::<StaticDataSeries<Point2D, 1024>>());
}
EOF
    
    rustc "$RESULTS_DIR/memory_report.rs" --edition 2021 -L target/release/deps --extern embedded_charts=target/release/libembedded_charts.rlib -o "$RESULTS_DIR/memory_report" 2>/dev/null || true
    
    if [ -f "$RESULTS_DIR/memory_report" ]; then
        "$RESULTS_DIR/memory_report" > "$RESULTS_DIR/memory_usage.txt"
        echo -e "${GREEN}✓ Memory report generated${NC}"
        cat "$RESULTS_DIR/memory_usage.txt"
    fi
    echo ""
}

# Check if criterion is installed
if ! cargo bench --help | grep -q criterion; then
    echo -e "${RED}Error: criterion not found. Please ensure it's in dev-dependencies${NC}"
    exit 1
fi

# Build release version first for accurate benchmarks
echo -e "${YELLOW}Building release version...${NC}"
cargo build --release --all-features
echo -e "${GREEN}✓ Build completed${NC}"
echo ""

# Run benchmarks
run_benchmark "chart_benchmarks" "Chart Rendering Benchmarks"
run_benchmark "platform_benchmarks" "Platform-Specific Benchmarks"
run_benchmark "memory_benchmarks" "Memory Usage Benchmarks"

# Generate memory report
generate_memory_report

# Generate summary report
echo -e "${YELLOW}Generating summary report...${NC}"
cat > "$RESULTS_DIR/summary.md" << EOF
# Benchmark Results Summary

Date: $(date)
Commit: $(git rev-parse --short HEAD)
Branch: $(git branch --show-current)

## Performance Benchmarks

### Chart Rendering Performance
See chart_benchmarks.log for detailed results.

### Platform-Specific Performance
See platform_benchmarks.log for detailed results.

### Memory Usage Analysis
See memory_benchmarks.log and memory_usage.txt for detailed results.

## Key Metrics

- Line Chart (1000 points): Check chart_benchmarks.log
- Memory footprint: See memory_usage.txt
- Platform optimizations: See platform_benchmarks.log

## Comparison with Baseline

To compare with previous results:
\`\`\`bash
cargo bench --bench chart_benchmarks -- --baseline chart_benchmarks_baseline
\`\`\`

EOF

echo -e "${GREEN}✓ Summary report generated${NC}"
echo ""

# Display summary
echo "====================================="
echo "Benchmark Suite Completed!"
echo "====================================="
echo ""
echo "Results saved to: $RESULTS_DIR"
echo ""
echo "Files generated:"
ls -la "$RESULTS_DIR"
echo ""
echo "To view HTML reports, open:"
echo "  target/criterion/*/report/index.html"
echo ""

# Optional: Open the criterion HTML report
if command -v xdg-open &> /dev/null; then
    read -p "Open benchmark reports in browser? (y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        find target/criterion -name "index.html" -path "*/report/*" | head -5 | xargs -I {} xdg-open {}
    fi
fi