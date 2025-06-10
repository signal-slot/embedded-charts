//! Visual testing and regression testing utilities
//!
//! Provides tools for validating visual output and detecting rendering regressions

use embedded_charts::{
    chart::traits::{Chart, ChartConfig},
    data::{point::Point2D, series::StaticDataSeries, DataSeries},
    error::ChartResult,
};
use embedded_graphics::{
    mock_display::MockDisplay, pixelcolor::Rgb565, prelude::*, primitives::Rectangle,
};

use super::{create_test_display, TEST_VIEWPORT};

/// Visual testing framework for chart output validation
pub struct VisualTester;

impl VisualTester {
    /// Capture a chart's rendered output for comparison
    pub fn capture_chart_output<T>(
        chart: &T,
        data: &StaticDataSeries<Point2D, 256>,
    ) -> ChartResult<ChartSnapshot>
    where
        T: Chart<Rgb565, Data = StaticDataSeries<Point2D, 256>, Config = ChartConfig<Rgb565>>,
    {
        let config = super::create_test_config();
        let mut display = create_test_display();

        chart.draw(data, &config, TEST_VIEWPORT, &mut display)?;

        Ok(ChartSnapshot {
            affected_area: display.affected_area(),
            pixel_count: Self::count_drawn_pixels(&display),
            bounds: display.bounding_box(),
        })
    }

    /// Compare two chart snapshots for visual regression testing
    pub fn compare_snapshots(baseline: &ChartSnapshot, current: &ChartSnapshot) -> VisualDiff {
        VisualDiff {
            area_match: baseline.affected_area == current.affected_area,
            pixel_count_diff: current.pixel_count as i32 - baseline.pixel_count as i32,
            bounds_match: baseline.bounds == current.bounds,
            similarity_score: Self::calculate_similarity(baseline, current),
        }
    }

    /// Validate that chart output meets visual quality standards
    pub fn validate_visual_quality<T>(
        chart: &T,
        data: &StaticDataSeries<Point2D, 256>,
    ) -> ChartResult<VisualQualityReport>
    where
        T: Chart<Rgb565, Data = StaticDataSeries<Point2D, 256>, Config = ChartConfig<Rgb565>>,
    {
        let snapshot = Self::capture_chart_output(chart, data)?;

        let mut report = VisualQualityReport::new();

        // Check that chart actually draws something for non-empty data
        if !data.is_empty() && snapshot.pixel_count == 0 {
            report
                .issues
                .push(heapless::String::try_from("Chart draws no pixels with valid data").unwrap())
                .ok();
        }

        // Check that chart stays within bounds
        if snapshot.bounds.size.width > TEST_VIEWPORT.size.width
            || snapshot.bounds.size.height > TEST_VIEWPORT.size.height
        {
            report
                .issues
                .push(
                    heapless::String::try_from("Chart rendering exceeds viewport bounds").unwrap(),
                )
                .ok();
        }

        // Check reasonable pixel density
        let viewport_area = TEST_VIEWPORT.size.width * TEST_VIEWPORT.size.height;
        let pixel_ratio = snapshot.pixel_count as f32 / viewport_area as f32;

        if pixel_ratio > 0.8 {
            report
                .issues
                .push(heapless::String::try_from("Chart may be over-rendered (too dense)").unwrap())
                .ok();
        } else if pixel_ratio < 0.01 && !data.is_empty() {
            report
                .issues
                .push(
                    heapless::String::try_from("Chart may be under-rendered (too sparse)").unwrap(),
                )
                .ok();
        }

        Ok(report)
    }

    /// Test chart rendering consistency across multiple renders
    pub fn test_rendering_consistency<T>(
        chart: &T,
        data: &StaticDataSeries<Point2D, 256>,
        iterations: usize,
    ) -> ChartResult<bool>
    where
        T: Chart<Rgb565, Data = StaticDataSeries<Point2D, 256>, Config = ChartConfig<Rgb565>>,
    {
        if iterations == 0 {
            return Ok(true);
        }

        let baseline = Self::capture_chart_output(chart, data)?;

        for _ in 1..iterations {
            let current = Self::capture_chart_output(chart, data)?;
            let diff = Self::compare_snapshots(&baseline, &current);

            if !diff.is_identical() {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Test chart with different color themes
    pub fn test_color_themes<T>(
        chart: &T,
        data: &StaticDataSeries<Point2D, 256>,
    ) -> ChartResult<heapless::Vec<ChartSnapshot, 8>>
    where
        T: Chart<Rgb565, Data = StaticDataSeries<Point2D, 256>, Config = ChartConfig<Rgb565>>,
    {
        let themes = [
            ChartConfig {
                title: None,
                background_color: Some(Rgb565::WHITE),
                margins: super::TEST_MARGINS,
                grid_color: Some(Rgb565::CSS_LIGHT_GRAY),
                show_grid: true,
            },
            ChartConfig {
                title: None,
                background_color: Some(Rgb565::BLACK),
                margins: super::TEST_MARGINS,
                grid_color: Some(Rgb565::CSS_DARK_GRAY),
                show_grid: true,
            },
            ChartConfig {
                title: None,
                background_color: None,
                margins: super::TEST_MARGINS,
                grid_color: Some(Rgb565::BLUE),
                show_grid: false,
            },
        ];

        let mut snapshots = heapless::Vec::new();

        for theme in &themes {
            let mut display = create_test_display();
            chart.draw(data, theme, TEST_VIEWPORT, &mut display)?;

            snapshots
                .push(ChartSnapshot {
                    affected_area: display.affected_area(),
                    pixel_count: Self::count_drawn_pixels(&display),
                    bounds: display.bounding_box(),
                })
                .ok();
        }

        Ok(snapshots)
    }

    /// Generate test pattern for visual debugging
    pub fn generate_test_pattern() -> StaticDataSeries<Point2D, 256> {
        let mut series = StaticDataSeries::new();

        // Create a recognizable test pattern
        let test_points = [
            (0.0, 0.0),   // Origin
            (10.0, 10.0), // Diagonal
            (20.0, 0.0),  // Return to zero
            (30.0, 20.0), // Peak
            (40.0, 5.0),  // Valley
        ];

        for (x, y) in test_points.iter() {
            series.push(Point2D::new(*x, *y)).ok();
        }

        series
    }

    // Helper methods

    fn count_drawn_pixels(display: &MockDisplay<Rgb565>) -> usize {
        // In a real implementation, this would count actual drawn pixels
        // For now, we'll estimate based on affected area
        let area = display.affected_area();
        (area.size.width * area.size.height) as usize
    }

    fn calculate_similarity(baseline: &ChartSnapshot, current: &ChartSnapshot) -> f32 {
        // Simplified similarity calculation
        let area_match = if baseline.affected_area == current.affected_area {
            1.0
        } else {
            0.0
        };
        let bounds_match = if baseline.bounds == current.bounds {
            1.0
        } else {
            0.0
        };

        let pixel_diff = (baseline.pixel_count as i32 - current.pixel_count as i32).abs();
        let max_pixels = baseline.pixel_count.max(current.pixel_count).max(1);
        let pixel_similarity = 1.0 - (pixel_diff as f32 / max_pixels as f32);

        (area_match + bounds_match + pixel_similarity) / 3.0
    }
}

/// Snapshot of chart rendering for comparison
#[derive(Debug, Clone, PartialEq)]
pub struct ChartSnapshot {
    pub affected_area: Rectangle,
    pub pixel_count: usize,
    pub bounds: Rectangle,
}

/// Visual difference analysis between chart renders
#[derive(Debug, Clone)]
pub struct VisualDiff {
    pub area_match: bool,
    pub pixel_count_diff: i32,
    pub bounds_match: bool,
    pub similarity_score: f32,
}

impl VisualDiff {
    /// Check if two renders are visually identical
    pub fn is_identical(&self) -> bool {
        self.area_match && self.bounds_match && self.pixel_count_diff == 0
    }

    /// Check if difference is within acceptable tolerance
    pub fn is_acceptable(&self, tolerance: f32) -> bool {
        self.similarity_score >= tolerance
    }
}

/// Visual quality assessment report
#[derive(Debug, Clone)]
pub struct VisualQualityReport {
    pub issues: heapless::Vec<heapless::String<64>, 10>,
    pub score: f32,
}

impl VisualQualityReport {
    pub fn new() -> Self {
        Self {
            issues: heapless::Vec::new(),
            score: 1.0,
        }
    }

    pub fn has_issues(&self) -> bool {
        !self.issues.is_empty()
    }

    pub fn is_acceptable(&self, min_score: f32) -> bool {
        self.score >= min_score && !self.has_issues()
    }
}

/// Pixel analysis utilities
pub struct PixelAnalyzer;

impl PixelAnalyzer {
    /// Analyze pixel distribution in chart output
    pub fn analyze_pixel_distribution(snapshot: &ChartSnapshot) -> PixelDistribution {
        PixelDistribution {
            total_pixels: snapshot.pixel_count,
            density: snapshot.pixel_count as f32
                / (snapshot.bounds.size.width * snapshot.bounds.size.height) as f32,
            coverage_area: snapshot.affected_area,
        }
    }

    /// Validate that chart uses appropriate pixel density
    pub fn validate_pixel_density(distribution: &PixelDistribution) -> bool {
        // Reasonable density range for charts
        distribution.density >= 0.01 && distribution.density <= 0.8
    }
}

/// Pixel distribution analysis
#[derive(Debug, Clone)]
pub struct PixelDistribution {
    pub total_pixels: usize,
    pub density: f32,
    pub coverage_area: Rectangle,
}

#[cfg(test)]
mod tests {
    use super::super::data_generators;
    use super::*;

    #[test]
    #[cfg(feature = "line")]
    #[ignore = "MockDisplay has limitations with pixel overlap detection"]
    fn test_visual_capture() {
        use embedded_charts::chart::line::LineChart;

        let chart = LineChart::new();
        let data = data_generators::generate_test_data(super::super::TestDataPattern::Linear, 5);

        let result = VisualTester::capture_chart_output(&chart, &data);
        assert!(result.is_ok());
    }

    #[test]
    fn test_snapshot_comparison() {
        let snap1 = ChartSnapshot {
            affected_area: Rectangle::new(Point::new(0, 0), Size::new(100, 100)),
            pixel_count: 1000,
            bounds: Rectangle::new(Point::new(0, 0), Size::new(100, 100)),
        };

        let snap2 = snap1.clone();
        let diff = VisualTester::compare_snapshots(&snap1, &snap2);

        assert!(diff.is_identical());
        assert_eq!(diff.similarity_score, 1.0);
    }

    #[test]
    fn test_visual_quality_report() {
        let mut report = VisualQualityReport::new();
        assert!(!report.has_issues());
        assert!(report.is_acceptable(0.8));

        report
            .issues
            .push(heapless::String::try_from("Test issue").unwrap())
            .ok();
        assert!(report.has_issues());
    }

    #[test]
    fn test_pixel_analyzer() {
        let snapshot = ChartSnapshot {
            affected_area: Rectangle::new(Point::new(0, 0), Size::new(100, 100)),
            pixel_count: 5000,
            bounds: Rectangle::new(Point::new(0, 0), Size::new(100, 100)),
        };

        let distribution = PixelAnalyzer::analyze_pixel_distribution(&snapshot);
        assert_eq!(distribution.total_pixels, 5000);
        assert!(PixelAnalyzer::validate_pixel_density(&distribution));
    }
}
