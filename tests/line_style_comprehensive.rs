//! Comprehensive tests for line style module

use embedded_charts::style::line::{
    BorderStyle, FillPattern, FillStyle, LineCap, LineJoin, LinePattern, LineStyle, StrokeStyle,
};

// Note: Gradient support would require a gradient feature flag

use embedded_graphics::pixelcolor::{BinaryColor, Gray8, Rgb565};

#[test]
fn test_line_pattern_enum() {
    // Test equality
    assert_eq!(LinePattern::Solid, LinePattern::Solid);
    assert_ne!(LinePattern::Solid, LinePattern::Dashed);

    // Test copy trait
    let pattern = LinePattern::Dotted;
    let pattern_copy = pattern;
    assert_eq!(pattern, pattern_copy);

    // Test all variants
    let patterns = [
        LinePattern::Solid,
        LinePattern::Dashed,
        LinePattern::Dotted,
        LinePattern::DashDot,
        LinePattern::Custom,
    ];

    for (i, p1) in patterns.iter().enumerate() {
        for (j, p2) in patterns.iter().enumerate() {
            if i == j {
                assert_eq!(p1, p2);
            } else {
                assert_ne!(p1, p2);
            }
        }
    }
}

#[test]
fn test_line_cap_enum() {
    // Test equality
    assert_eq!(LineCap::Butt, LineCap::Butt);
    assert_ne!(LineCap::Butt, LineCap::Round);

    // Test copy trait
    let cap = LineCap::Square;
    let cap_copy = cap;
    assert_eq!(cap, cap_copy);

    // Test all variants
    let caps = [LineCap::Butt, LineCap::Round, LineCap::Square];

    for (i, c1) in caps.iter().enumerate() {
        for (j, c2) in caps.iter().enumerate() {
            if i == j {
                assert_eq!(c1, c2);
            } else {
                assert_ne!(c1, c2);
            }
        }
    }
}

#[test]
fn test_line_join_enum() {
    // Test equality
    assert_eq!(LineJoin::Miter, LineJoin::Miter);
    assert_ne!(LineJoin::Miter, LineJoin::Round);

    // Test copy trait
    let join = LineJoin::Bevel;
    let join_copy = join;
    assert_eq!(join, join_copy);

    // Test all variants
    let joins = [LineJoin::Miter, LineJoin::Round, LineJoin::Bevel];

    for (i, j1) in joins.iter().enumerate() {
        for (j, j2) in joins.iter().enumerate() {
            if i == j {
                assert_eq!(j1, j2);
            } else {
                assert_ne!(j1, j2);
            }
        }
    }
}

#[test]
fn test_line_style_solid() {
    let style = LineStyle::solid(Rgb565::new(10, 20, 15));

    assert_eq!(style.color, Rgb565::new(10, 20, 15));
    assert_eq!(style.width, 1);
    assert_eq!(style.pattern, LinePattern::Solid);
    assert_eq!(style.cap, LineCap::Butt);
    assert_eq!(style.join, LineJoin::Miter);
}

#[test]
fn test_line_style_dashed() {
    let style = LineStyle::dashed(Rgb565::new(31, 0, 0));

    assert_eq!(style.color, Rgb565::new(31, 0, 0));
    assert_eq!(style.width, 1);
    assert_eq!(style.pattern, LinePattern::Dashed);
    assert_eq!(style.cap, LineCap::Butt);
    assert_eq!(style.join, LineJoin::Miter);
}

#[test]
fn test_line_style_dotted() {
    let style = LineStyle::dotted(Rgb565::new(0, 63, 0));

    assert_eq!(style.color, Rgb565::new(0, 63, 0));
    assert_eq!(style.width, 1);
    assert_eq!(style.pattern, LinePattern::Dotted);
    assert_eq!(style.cap, LineCap::Round);
    assert_eq!(style.join, LineJoin::Round);
}

#[test]
fn test_line_style_builder_pattern() {
    let style = LineStyle::solid(Rgb565::new(15, 30, 25))
        .width(5)
        .pattern(LinePattern::DashDot)
        .cap(LineCap::Square)
        .join(LineJoin::Bevel);

    assert_eq!(style.color, Rgb565::new(15, 30, 25));
    assert_eq!(style.width, 5);
    assert_eq!(style.pattern, LinePattern::DashDot);
    assert_eq!(style.cap, LineCap::Square);
    assert_eq!(style.join, LineJoin::Bevel);
}

#[test]
fn test_line_style_color_setter() {
    let style = LineStyle::solid(Rgb565::new(0, 0, 0)).color(Rgb565::new(31, 63, 31));

    assert_eq!(style.color, Rgb565::new(31, 63, 31));
}

#[test]
fn test_line_style_width_variations() {
    // Test various widths
    let widths = [0, 1, 2, 5, 10, 100, u32::MAX];

    for &width in &widths {
        let style = LineStyle::solid(Rgb565::new(10, 20, 15)).width(width);
        assert_eq!(style.width, width);
    }
}

#[test]
fn test_line_style_const_functions() {
    // Test that const functions work in const context
    const STYLE: LineStyle<Rgb565> = LineStyle::solid(Rgb565::new(10, 20, 15))
        .width(3)
        .pattern(LinePattern::Dashed)
        .cap(LineCap::Round)
        .join(LineJoin::Round);

    assert_eq!(STYLE.color, Rgb565::new(10, 20, 15));
    assert_eq!(STYLE.width, 3);
    assert_eq!(STYLE.pattern, LinePattern::Dashed);
}

#[test]
fn test_line_style_default() {
    let style: LineStyle<Rgb565> = LineStyle::default();

    // Default should be white solid line
    assert_eq!(style.color, Rgb565::new(31, 63, 31)); // White in RGB565
    assert_eq!(style.width, 1);
    assert_eq!(style.pattern, LinePattern::Solid);
    assert_eq!(style.cap, LineCap::Butt);
    assert_eq!(style.join, LineJoin::Miter);
}

#[test]
fn test_line_style_equality() {
    let style1 = LineStyle::solid(Rgb565::new(10, 20, 15)).width(2);
    let style2 = LineStyle::solid(Rgb565::new(10, 20, 15)).width(2);
    let style3 = LineStyle::solid(Rgb565::new(10, 20, 15)).width(3);

    assert_eq!(style1, style2);
    assert_ne!(style1, style3);
}

#[test]
fn test_line_style_with_different_color_types() {
    // Test with BinaryColor
    let binary_style = LineStyle::solid(BinaryColor::On);
    assert_eq!(binary_style.color, BinaryColor::On);

    // Test with Gray8
    let gray_style = LineStyle::dashed(Gray8::new(128));
    assert_eq!(gray_style.color, Gray8::new(128));
    assert_eq!(gray_style.pattern, LinePattern::Dashed);
}

#[test]
fn test_border_style_new() {
    let line = LineStyle::solid(Rgb565::new(20, 40, 30));
    let border = BorderStyle::new(line);

    assert_eq!(border.line.color, Rgb565::new(20, 40, 30));
    assert_eq!(border.radius, 0);
    assert!(border.visible);
}

#[test]
fn test_border_style_rounded() {
    let line = LineStyle::dashed(Rgb565::new(5, 10, 7));
    let border = BorderStyle::rounded(line, 10);

    assert_eq!(border.line.color, Rgb565::new(5, 10, 7));
    assert_eq!(border.line.pattern, LinePattern::Dashed);
    assert_eq!(border.radius, 10);
    assert!(border.visible);
}

#[test]
fn test_border_style_builder() {
    let line = LineStyle::solid(Rgb565::new(0, 0, 0));
    let border = BorderStyle::new(line).radius(15).visible(false);

    assert_eq!(border.radius, 15);
    assert!(!border.visible);
}

#[test]
fn test_border_style_const_functions() {
    const LINE: LineStyle<Rgb565> = LineStyle::solid(Rgb565::new(10, 20, 15));
    const BORDER: BorderStyle<Rgb565> = BorderStyle::rounded(LINE, 5).radius(8).visible(true);

    assert_eq!(BORDER.radius, 8);
    // Visibility is checked by const evaluation succeeding
    let border = BORDER;
    assert!(border.visible);
}

#[test]
fn test_border_style_default() {
    let border: BorderStyle<Rgb565> = BorderStyle::default();

    assert_eq!(border.radius, 0);
    assert!(border.visible);
    // Should use default LineStyle
    assert_eq!(border.line.color, Rgb565::new(31, 63, 31));
}

#[test]
fn test_border_style_equality() {
    let line = LineStyle::solid(Rgb565::new(10, 20, 15));
    let border1 = BorderStyle::new(line).radius(5);
    let border2 = BorderStyle::new(line).radius(5);
    let border3 = BorderStyle::new(line).radius(10);

    assert_eq!(border1, border2);
    assert_ne!(border1, border3);
}

#[test]
fn test_stroke_style_new() {
    let stroke = StrokeStyle::new(Rgb565::new(15, 30, 20), 3);

    assert_eq!(stroke.color, Rgb565::new(15, 30, 20));
    assert_eq!(stroke.width, 3);
}

#[test]
fn test_stroke_style_const_function() {
    const STROKE: StrokeStyle<Rgb565> = StrokeStyle::new(Rgb565::new(10, 20, 15), 5);

    assert_eq!(STROKE.color, Rgb565::new(10, 20, 15));
    assert_eq!(STROKE.width, 5);
}

#[test]
fn test_stroke_style_from_line_style() {
    let line_style = LineStyle::solid(Rgb565::new(25, 50, 35))
        .width(4)
        .pattern(LinePattern::Dotted);

    let stroke_style: StrokeStyle<Rgb565> = line_style.into();

    assert_eq!(stroke_style.color, Rgb565::new(25, 50, 35));
    assert_eq!(stroke_style.width, 4);
    // Pattern is not preserved in conversion
}

#[test]
fn test_stroke_style_from_different_line_styles() {
    // Test conversion preserves color and width
    let test_cases = vec![
        LineStyle::solid(Rgb565::new(31, 0, 0)).width(1),
        LineStyle::dashed(Rgb565::new(0, 63, 0)).width(10),
        LineStyle::dotted(Rgb565::new(0, 0, 31)).width(0),
    ];

    for line_style in test_cases {
        let stroke: StrokeStyle<Rgb565> = line_style.into();
        assert_eq!(stroke.color, line_style.color);
        assert_eq!(stroke.width, line_style.width);
    }
}

#[test]
fn test_stroke_style_equality() {
    let stroke1 = StrokeStyle::new(Rgb565::new(10, 20, 15), 2);
    let stroke2 = StrokeStyle::new(Rgb565::new(10, 20, 15), 2);
    let stroke3 = StrokeStyle::new(Rgb565::new(10, 20, 15), 3);

    assert_eq!(stroke1, stroke2);
    assert_ne!(stroke1, stroke3);
}

#[test]
fn test_fill_style_solid() {
    let fill = FillStyle::solid(Rgb565::new(20, 40, 30));

    assert_eq!(fill.solid_color(), Some(Rgb565::new(20, 40, 30)));

    match &fill.pattern {
        FillPattern::Solid(color) => assert_eq!(*color, Rgb565::new(20, 40, 30)),
        _ => panic!("Expected solid pattern"),
    }
}

#[test]
fn test_fill_style_solid_color_method() {
    // Test solid fill returns color
    let solid_fill = FillStyle::solid(Rgb565::new(10, 20, 15));
    assert_eq!(solid_fill.solid_color(), Some(Rgb565::new(10, 20, 15)));

    // Test non-solid fills would return None (gradient support not enabled)
}

#[test]
fn test_fill_style_const_functions() {
    const FILL: FillStyle<Rgb565> = FillStyle::solid(Rgb565::new(10, 20, 15));

    match &FILL.pattern {
        FillPattern::Solid(color) => assert_eq!(*color, Rgb565::new(10, 20, 15)),
        _ => panic!("Expected solid pattern"),
    }
}

#[test]
fn test_fill_style_default() {
    let fill: FillStyle<Rgb565> = FillStyle::default();

    // Default should be white solid fill
    assert_eq!(fill.solid_color(), Some(Rgb565::new(31, 63, 31)));
}

// Note: Gradient-related tests removed as gradient feature is not available

#[test]
fn test_fill_style_debug_trait() {
    let fill = FillStyle::solid(Rgb565::new(10, 20, 15));
    let debug_str = format!("{fill:?}");
    assert!(debug_str.contains("FillStyle"));
    assert!(debug_str.contains("Solid"));
}

#[test]
fn test_line_style_memory_size() {
    use core::mem::size_of;

    // Verify reasonable memory usage
    assert!(size_of::<LineStyle<Rgb565>>() <= 16);
    assert!(size_of::<BorderStyle<Rgb565>>() <= 24);
    assert!(size_of::<StrokeStyle<Rgb565>>() <= 8);
}

#[test]
fn test_pattern_combinations() {
    // Test all pattern/cap/join combinations work
    let patterns = [
        LinePattern::Solid,
        LinePattern::Dashed,
        LinePattern::Dotted,
        LinePattern::DashDot,
        LinePattern::Custom,
    ];

    let caps = [LineCap::Butt, LineCap::Round, LineCap::Square];
    let joins = [LineJoin::Miter, LineJoin::Round, LineJoin::Bevel];

    for pattern in &patterns {
        for cap in &caps {
            for join in &joins {
                let style = LineStyle::solid(Rgb565::new(10, 20, 15))
                    .pattern(*pattern)
                    .cap(*cap)
                    .join(*join);

                assert_eq!(style.pattern, *pattern);
                assert_eq!(style.cap, *cap);
                assert_eq!(style.join, *join);
            }
        }
    }
}

#[test]
fn test_builder_method_order_independence() {
    // Test that builder methods can be called in any order
    let style1 = LineStyle::solid(Rgb565::new(10, 20, 15))
        .width(3)
        .pattern(LinePattern::Dashed)
        .cap(LineCap::Round)
        .join(LineJoin::Bevel);

    let style2 = LineStyle::solid(Rgb565::new(10, 20, 15))
        .join(LineJoin::Bevel)
        .cap(LineCap::Round)
        .pattern(LinePattern::Dashed)
        .width(3);

    assert_eq!(style1, style2);
}

#[test]
fn test_border_visibility_edge_cases() {
    let line = LineStyle::solid(Rgb565::new(10, 20, 15));

    // Test invisible border
    let invisible_border = BorderStyle::new(line).visible(false);
    assert!(!invisible_border.visible);

    // Test toggling visibility
    let mut border = BorderStyle::new(line);
    assert!(border.visible);
    border = border.visible(false);
    assert!(!border.visible);
    border = border.visible(true);
    assert!(border.visible);
}

#[test]
fn test_border_radius_edge_cases() {
    let line = LineStyle::solid(Rgb565::new(10, 20, 15));

    // Test zero radius
    let no_radius = BorderStyle::new(line).radius(0);
    assert_eq!(no_radius.radius, 0);

    // Test large radius
    let large_radius = BorderStyle::new(line).radius(u32::MAX);
    assert_eq!(large_radius.radius, u32::MAX);
}

#[test]
fn test_fill_pattern_clone() {
    // Test that FillStyle and FillPattern implement Clone
    let fill1 = FillStyle::solid(Rgb565::new(10, 20, 15));
    let fill2 = fill1.clone();

    assert_eq!(fill1.solid_color(), fill2.solid_color());
}

#[test]
fn test_const_evaluation_in_static_context() {
    // Test that const functions can be used in static context
    static STATIC_LINE: LineStyle<Rgb565> = LineStyle::solid(Rgb565::new(10, 20, 15))
        .width(2)
        .pattern(LinePattern::Solid);

    static STATIC_BORDER: BorderStyle<Rgb565> =
        BorderStyle::new(STATIC_LINE).radius(5).visible(true);

    static STATIC_STROKE: StrokeStyle<Rgb565> = StrokeStyle::new(Rgb565::new(10, 20, 15), 3);

    assert_eq!(STATIC_LINE.width, 2);
    assert_eq!(STATIC_BORDER.radius, 5);
    assert_eq!(STATIC_STROKE.width, 3);
}

#[test]
fn test_line_style_copy_trait() {
    let style1 = LineStyle::solid(Rgb565::new(10, 20, 15)).width(3);
    let style2 = style1; // Copy
    let style3 = style1; // Can still use style1 because of Copy

    assert_eq!(style1, style2);
    assert_eq!(style1, style3);
}

#[test]
fn test_border_style_copy_trait() {
    let line = LineStyle::solid(Rgb565::new(10, 20, 15));
    let border1 = BorderStyle::new(line).radius(5);
    let border2 = border1; // Copy
    let border3 = border1; // Can still use border1 because of Copy

    assert_eq!(border1, border2);
    assert_eq!(border1, border3);
}

#[test]
fn test_stroke_style_copy_trait() {
    let stroke1 = StrokeStyle::new(Rgb565::new(10, 20, 15), 2);
    let stroke2 = stroke1; // Copy
    let stroke3 = stroke1; // Can still use stroke1 because of Copy

    assert_eq!(stroke1, stroke2);
    assert_eq!(stroke1, stroke3);
}
