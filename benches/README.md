# Performance Benchmarks

This directory contains comprehensive performance benchmarks for the embedded-charts library, measuring various aspects of performance critical for embedded systems.

## Benchmark Categories

### 1. Chart Rendering (`chart_rendering.rs`)
Measures the rendering performance of different chart types:
- **Data Scaling**: How performance scales with different data sizes (10-256 points)
- **Feature Impact**: Performance cost of enabling features like markers, area fill, smooth curves
- **Chart Types**: Comparative performance of line, bar, pie, and scatter charts
- **Viewport Scaling**: Rendering performance at different display resolutions
- **Configuration Overhead**: Impact of margins, titles, and backgrounds

### 2. Data Operations (`data_operations.rs`)
Benchmarks data management and manipulation:
- **Series Operations**: Push, extend, iteration, and sorting performance
- **Bounds Calculations**: Data bounds computation with various data sizes
- **Multi-Series**: Performance of managing multiple data series
- **Sliding Windows**: Real-time data window operations
- **Label Management**: String label storage and retrieval
- **Data Transformations**: Normalization and filtering operations

### 3. Interpolation (`interpolation.rs`)
Measures smooth curve interpolation performance:
- **Algorithm Comparison**: Linear vs Cubic Spline vs Catmull-Rom vs Bezier
- **Subdivision Scaling**: Performance with different subdivision counts
- **Input Size Scaling**: How algorithms scale with number of input points
- **Tension Effects**: Impact of tension parameter on performance
- **Edge Cases**: Performance with minimal points, straight lines, sharp angles
- **Smoothing Operations**: Noise reduction and data smoothing

### 4. Memory Usage (`memory_usage.rs`)
Analyzes memory allocation patterns and usage:
- **Data Structure Sizes**: Memory footprint of series types
- **Chart Instance Memory**: Memory usage of different chart types
- **Memory Management**: Overhead of memory tracking and pools
- **Scaling Analysis**: Memory usage vs data size
- **Configuration Memory**: Memory cost of chart configuration

## Running Benchmarks

### Run All Benchmarks
```bash
cargo bench
```

### Run Specific Benchmark Suite
```bash
# Chart rendering benchmarks only
cargo bench --bench chart_rendering

# Data operations benchmarks only
cargo bench --bench data_operations

# Interpolation benchmarks only
cargo bench --bench interpolation

# Memory usage benchmarks only
cargo bench --bench memory_usage
```

### Run Specific Test Within a Suite
```bash
# Run only line chart benchmarks
cargo bench --bench chart_rendering -- line_chart

# Run only interpolation type comparisons
cargo bench --bench interpolation -- interpolation_types
```

### Generate HTML Reports
Criterion automatically generates detailed HTML reports in `target/criterion/`:
```bash
# After running benchmarks, open the report
open target/criterion/report/index.html
```

## Benchmark Configuration

### Feature Flags
Some benchmarks require specific features:
```bash
# Run with all features for comprehensive testing
cargo bench --all-features

# Run with minimal features for embedded baseline
cargo bench --no-default-features --features "no_std,basic-charts"
```

### Custom Parameters
You can modify benchmark parameters in the source files:
- Data sizes in scaling benchmarks
- Number of iterations
- Viewport sizes
- Feature combinations

## Interpreting Results

### Performance Metrics
- **Throughput**: Elements processed per second
- **Time**: Average time per iteration
- **Variance**: Consistency of performance

### Baseline Comparisons
After the first run, subsequent runs will show:
- Performance changes vs previous run
- Statistical significance of changes
- Trend visualization over multiple runs

### Memory Profiling
Memory benchmarks provide:
- Static memory footprint
- Dynamic allocation patterns
- Peak memory usage
- Memory efficiency ratios

## Embedded System Considerations

When evaluating benchmarks for embedded deployment:

1. **Target Architecture**: Results on x86_64 may differ from ARM Cortex-M
2. **Memory Constraints**: Focus on memory benchmarks for constrained systems
3. **Real-time Requirements**: Check variance and worst-case performance
4. **Feature Selection**: Test with your specific feature combination

## Adding New Benchmarks

To add a new benchmark:

1. Create a new file in `benches/` or add to existing file
2. Use the Criterion macros:
   ```rust
   use criterion::{black_box, criterion_group, criterion_main, Criterion};
   
   fn bench_my_feature(c: &mut Criterion) {
       c.bench_function("my_feature", |b| {
           b.iter(|| {
               // Code to benchmark
           });
       });
   }
   
   criterion_group!(benches, bench_my_feature);
   criterion_main!(benches);
   ```
3. Add the benchmark to `Cargo.toml` if it's a new file
4. Document the benchmark purpose and parameters

## CI Integration

These benchmarks can be integrated into CI pipelines:
```yaml
# Example GitHub Actions step
- name: Run benchmarks
  run: cargo bench --all-features -- --save-baseline main
  
- name: Compare performance
  run: cargo bench --all-features -- --baseline main
```

## Optimization Workflow

1. **Baseline**: Run benchmarks to establish baseline
2. **Profile**: Identify bottlenecks using benchmark results
3. **Optimize**: Make targeted improvements
4. **Verify**: Re-run benchmarks to confirm improvements
5. **Document**: Record optimization rationale and results