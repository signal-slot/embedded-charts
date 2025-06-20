//! Simple tests for the dashboard layout system

#![cfg(feature = "std")]

use embedded_charts::dashboard::{DashboardLayout, GridPosition, LayoutPreset, SimpleDashboard};
use embedded_graphics::{prelude::*, primitives::Rectangle};

#[test]
fn test_simple_dashboard_2x2() {
    let dashboard = SimpleDashboard::new(2, 2, 10);
    let total_viewport = Rectangle::new(Point::new(0, 0), Size::new(200, 200));

    // Test all four positions
    let positions = [
        (GridPosition::new(0, 0), Point::new(0, 0)),
        (GridPosition::new(0, 1), Point::new(105, 0)),
        (GridPosition::new(1, 0), Point::new(0, 105)),
        (GridPosition::new(1, 1), Point::new(105, 105)),
    ];

    for (pos, expected_top_left) in &positions {
        let viewport = dashboard.get_viewport(*pos, total_viewport);
        assert_eq!(viewport.top_left, *expected_top_left);
        assert_eq!(viewport.size, Size::new(95, 95));
    }
}

#[test]
fn test_simple_dashboard_3x3() {
    let dashboard = SimpleDashboard::new(3, 3, 5);
    let total_viewport = Rectangle::new(Point::new(0, 0), Size::new(315, 315));

    // Each cell should be (315 - 2*5) / 3 = 101.67 ≈ 101 pixels
    let viewport = dashboard.get_viewport(GridPosition::new(1, 1), total_viewport);
    assert_eq!(viewport.top_left, Point::new(106, 106)); // 101 + 5
    assert_eq!(viewport.size.width, 101);
    assert_eq!(viewport.size.height, 101);
}

#[test]
fn test_dashboard_with_span() {
    let dashboard = SimpleDashboard::new(3, 3, 10);
    let total_viewport = Rectangle::new(Point::new(0, 0), Size::new(320, 320));

    // Test a 2x2 span
    let pos = GridPosition::with_span(0, 0, 2, 2);
    let viewport = dashboard.get_viewport(pos, total_viewport);

    assert_eq!(viewport.top_left, Point::new(0, 0));
    // Should be 2 cells + 1 spacing = 2*100 + 10 = 210
    assert_eq!(viewport.size, Size::new(210, 210));
}

#[test]
fn test_get_all_viewports() {
    let dashboard = SimpleDashboard::new(2, 2, 10);
    let total_viewport = Rectangle::new(Point::new(0, 0), Size::new(200, 200));

    let viewports: heapless::Vec<Rectangle, 4> =
        dashboard.get_all_viewports(total_viewport, 3).unwrap();

    assert_eq!(viewports.len(), 3);

    // Check first three positions
    assert_eq!(viewports[0].top_left, Point::new(0, 0));
    assert_eq!(viewports[1].top_left, Point::new(105, 0));
    assert_eq!(viewports[2].top_left, Point::new(0, 105));
}

#[test]
fn test_layout_presets() {
    let presets = vec![
        (LayoutPreset::Single, 1, 1),
        (LayoutPreset::SideBySide, 1, 2),
        (LayoutPreset::Stacked, 2, 1),
        (LayoutPreset::Quadrants, 2, 2),
        (LayoutPreset::ThreeColumns, 1, 3),
        (LayoutPreset::ThreeRows, 3, 1),
        (LayoutPreset::Grid3x3, 3, 3),
        (LayoutPreset::Grid4x4, 4, 4),
    ];

    for (preset, expected_rows, expected_cols) in presets {
        match preset.to_layout() {
            DashboardLayout::Grid(grid) => {
                assert_eq!(grid.rows, expected_rows);
                assert_eq!(grid.cols, expected_cols);
            }
            _ => panic!("Expected grid layout"),
        }
    }
}

#[test]
fn test_asymmetric_layout() {
    let dashboard = SimpleDashboard::new(2, 3, 5);
    let total_viewport = Rectangle::new(Point::new(0, 0), Size::new(315, 210));

    // Width per cell: (315 - 2*5) / 3 = 101.67 ≈ 101
    // Height per cell: (210 - 1*5) / 2 = 102.5 ≈ 102

    let viewport = dashboard.get_viewport(GridPosition::new(1, 2), total_viewport);
    assert_eq!(viewport.top_left.x, 212); // 2 * 101 + 2 * 5
    assert_eq!(viewport.top_left.y, 107); // 1 * 102 + 1 * 5
}
