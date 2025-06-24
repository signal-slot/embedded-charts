//! Comprehensive tests for colors module

use embedded_charts::{
    error::DataError,
    style::colors::{ColorPalette, ColorUtils},
};
use embedded_graphics::pixelcolor::BinaryColor;

#[cfg(feature = "color-support")]
use embedded_charts::style::colors::{rgb565_palettes, ColorInterpolation};
#[cfg(feature = "color-support")]
use embedded_graphics::{pixelcolor::Rgb565, prelude::*};

#[test]
fn test_color_palette_new() {
    let mut palette: ColorPalette<BinaryColor, 10> = ColorPalette::new();
    assert!(palette.is_empty());
    assert_eq!(palette.len(), 0);
    assert!(palette.next_color().is_none());
}

#[test]
fn test_color_palette_default_trait() {
    let palette: ColorPalette<BinaryColor, 10> = ColorPalette::default();
    assert!(palette.is_empty());
}

#[test]
fn test_color_palette_add_color() {
    let mut palette: ColorPalette<BinaryColor, 3> = ColorPalette::new();

    // Add colors
    assert!(palette.add_color(BinaryColor::On).is_ok());
    assert!(palette.add_color(BinaryColor::Off).is_ok());
    assert!(palette.add_color(BinaryColor::On).is_ok());

    assert_eq!(palette.len(), 3);

    // Try to add beyond capacity
    let result = palette.add_color(BinaryColor::Off);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), DataError::BufferFull { .. }));
}

#[test]
fn test_color_palette_from_colors() {
    // Test successful creation
    let colors = [BinaryColor::On, BinaryColor::Off, BinaryColor::On];
    let palette: ColorPalette<BinaryColor, 5> = ColorPalette::from_colors(&colors).unwrap();
    assert_eq!(palette.len(), 3);

    // Test creation with empty slice
    let empty: &[BinaryColor] = &[];
    let palette: ColorPalette<BinaryColor, 5> = ColorPalette::from_colors(empty).unwrap();
    assert!(palette.is_empty());

    // Test creation exceeding capacity
    let too_many = [BinaryColor::On; 6];
    let result: Result<ColorPalette<BinaryColor, 5>, _> = ColorPalette::from_colors(&too_many);
    assert!(result.is_err());
}

#[test]
fn test_color_palette_next_color_cycling() {
    let mut palette: ColorPalette<BinaryColor, 5> = ColorPalette::new();
    palette.add_color(BinaryColor::On).unwrap();
    palette.add_color(BinaryColor::Off).unwrap();
    palette.add_color(BinaryColor::On).unwrap();

    // First cycle
    assert_eq!(palette.next_color(), Some(BinaryColor::On));
    assert_eq!(palette.next_color(), Some(BinaryColor::Off));
    assert_eq!(palette.next_color(), Some(BinaryColor::On));

    // Should cycle back to beginning
    assert_eq!(palette.next_color(), Some(BinaryColor::On));
    assert_eq!(palette.next_color(), Some(BinaryColor::Off));
}

#[test]
fn test_color_palette_get_color() {
    let mut palette: ColorPalette<BinaryColor, 5> = ColorPalette::new();
    palette.add_color(BinaryColor::On).unwrap();
    palette.add_color(BinaryColor::Off).unwrap();
    palette.add_color(BinaryColor::On).unwrap();

    assert_eq!(palette.get_color(0), Some(BinaryColor::On));
    assert_eq!(palette.get_color(1), Some(BinaryColor::Off));
    assert_eq!(palette.get_color(2), Some(BinaryColor::On));
    assert_eq!(palette.get_color(3), None); // Out of bounds
    assert_eq!(palette.get_color(100), None); // Way out of bounds
}

#[test]
fn test_color_palette_reset() {
    let mut palette: ColorPalette<BinaryColor, 5> = ColorPalette::new();
    palette.add_color(BinaryColor::On).unwrap();
    palette.add_color(BinaryColor::Off).unwrap();

    // Advance the index
    palette.next_color();
    palette.next_color();
    palette.next_color(); // Should be at index 1 again

    // Reset and verify we're back at the beginning
    palette.reset();
    assert_eq!(palette.next_color(), Some(BinaryColor::On));
}

#[test]
fn test_color_palette_as_slice() {
    let mut palette: ColorPalette<BinaryColor, 5> = ColorPalette::new();
    palette.add_color(BinaryColor::On).unwrap();
    palette.add_color(BinaryColor::Off).unwrap();
    palette.add_color(BinaryColor::On).unwrap();

    let slice = palette.as_slice();
    assert_eq!(slice.len(), 3);
    assert_eq!(slice[0], BinaryColor::On);
    assert_eq!(slice[1], BinaryColor::Off);
    assert_eq!(slice[2], BinaryColor::On);
}

#[test]
fn test_color_palette_empty_operations() {
    let mut palette: ColorPalette<BinaryColor, 5> = ColorPalette::new();

    assert!(palette.next_color().is_none());
    assert!(palette.get_color(0).is_none());
    assert_eq!(palette.as_slice().len(), 0);

    // Reset on empty palette should be safe
    palette.reset();
    assert!(palette.next_color().is_none());
}

#[cfg(feature = "color-support")]
#[test]
fn test_default_palette() {
    let mut palette = rgb565_palettes::default_palette();
    assert_eq!(palette.len(), 8);

    // Verify specific colors
    assert_eq!(
        palette.get_color(0),
        Some(Rgb565::new(59 >> 3, 130 >> 2, 246 >> 3))
    );
    assert_eq!(
        palette.get_color(1),
        Some(Rgb565::new(239 >> 3, 68 >> 2, 68 >> 3))
    );

    // Test cycling through all colors
    for _ in 0..8 {
        assert!(palette.next_color().is_some());
    }

    // Should cycle back to first color
    assert_eq!(
        palette.next_color(),
        Some(Rgb565::new(59 >> 3, 130 >> 2, 246 >> 3))
    );
}

#[cfg(feature = "color-support")]
#[test]
fn test_professional_palette() {
    let palette = rgb565_palettes::professional_palette();
    assert_eq!(palette.len(), 8);

    // Verify it contains expected professional colors
    assert_eq!(
        palette.get_color(0),
        Some(Rgb565::new(30 >> 3, 58 >> 2, 138 >> 3))
    ); // Navy blue
    assert_eq!(
        palette.get_color(1),
        Some(Rgb565::new(185 >> 3, 28 >> 2, 28 >> 3))
    ); // Dark red
}

#[cfg(feature = "color-support")]
#[test]
fn test_pastel_palette() {
    let palette = rgb565_palettes::pastel_palette();
    assert_eq!(palette.len(), 8);

    // Verify pastel colors
    assert_eq!(
        palette.get_color(0),
        Some(Rgb565::new(147 >> 3, 197 >> 2, 253 >> 3))
    ); // Sky blue
    assert_eq!(
        palette.get_color(1),
        Some(Rgb565::new(252 >> 3, 165 >> 2, 165 >> 3))
    ); // Light pink
}

#[cfg(feature = "color-support")]
#[test]
fn test_vibrant_palette() {
    let palette = rgb565_palettes::vibrant_palette();
    assert_eq!(palette.len(), 8);

    // Verify vibrant colors
    assert_eq!(
        palette.get_color(0),
        Some(Rgb565::new(236 >> 3, 72 >> 2, 153 >> 3))
    ); // Hot pink
}

#[cfg(feature = "color-support")]
#[test]
fn test_nature_palette() {
    let palette = rgb565_palettes::nature_palette();
    assert_eq!(palette.len(), 8);

    // Verify nature-inspired colors
    assert_eq!(
        palette.get_color(0),
        Some(Rgb565::new(34 >> 3, 139 >> 2, 34 >> 3))
    ); // Forest green
}

#[cfg(feature = "color-support")]
#[test]
fn test_ocean_palette() {
    let palette = rgb565_palettes::ocean_palette();
    assert_eq!(palette.len(), 8);

    // Verify ocean colors
    assert_eq!(
        palette.get_color(0),
        Some(Rgb565::new(30 >> 3, 144 >> 2, 255 >> 3))
    ); // Dodger blue
}

#[cfg(feature = "color-support")]
#[test]
fn test_sunset_palette() {
    let palette = rgb565_palettes::sunset_palette();
    assert_eq!(palette.len(), 8);

    // Verify sunset colors
    assert_eq!(
        palette.get_color(0),
        Some(Rgb565::new(255 >> 3, 99 >> 2, 71 >> 3))
    ); // Tomato
}

#[cfg(feature = "color-support")]
#[test]
fn test_cyberpunk_palette() {
    let palette = rgb565_palettes::cyberpunk_palette();
    assert_eq!(palette.len(), 8);

    // Verify cyberpunk neon colors
    assert_eq!(
        palette.get_color(0),
        Some(Rgb565::new(0 >> 3, 255 >> 2, 255 >> 3))
    ); // Cyan
    assert_eq!(
        palette.get_color(1),
        Some(Rgb565::new(255 >> 3, 0 >> 2, 255 >> 3))
    ); // Magenta
}

#[cfg(feature = "color-support")]
#[test]
fn test_high_contrast_palette() {
    let palette = rgb565_palettes::high_contrast_palette();
    assert_eq!(palette.len(), 6);

    // Verify high contrast colors
    assert_eq!(palette.get_color(0), Some(Rgb565::new(0, 0, 0)));
    assert_eq!(palette.get_color(1), Some(Rgb565::new(31, 63, 31)));
    assert_eq!(
        palette.get_color(2),
        Some(Rgb565::new(255 >> 3, 0 >> 2, 0 >> 3))
    ); // Pure red
}

#[cfg(feature = "color-support")]
#[test]
fn test_monochrome_palette() {
    let palette = rgb565_palettes::monochrome_palette();
    assert_eq!(palette.len(), 8);

    // Verify grayscale progression
    assert_eq!(palette.get_color(0), Some(Rgb565::new(0, 0, 0)));
    assert_eq!(palette.get_color(7), Some(Rgb565::new(31, 63, 31)));

    // Verify intermediate grays exist
    for i in 1..7 {
        let color = palette.get_color(i).unwrap();
        assert_ne!(color, Rgb565::new(0, 0, 0));
        assert_ne!(color, Rgb565::new(31, 63, 31));
    }
}

#[cfg(feature = "color-support")]
#[test]
fn test_minimal_palette() {
    let palette = rgb565_palettes::minimal_palette();
    assert_eq!(palette.len(), 6);

    // Verify minimal sophisticated colors
    assert_eq!(
        palette.get_color(0),
        Some(Rgb565::new(55 >> 3, 65 >> 2, 81 >> 3))
    ); // Slate gray
}

#[cfg(feature = "color-support")]
#[test]
fn test_retro_palette() {
    let palette = rgb565_palettes::retro_palette();
    assert_eq!(palette.len(), 8);

    // Verify retro colors
    assert_eq!(
        palette.get_color(0),
        Some(Rgb565::new(205 >> 3, 92 >> 2, 92 >> 3))
    ); // Indian red
}

#[cfg(feature = "color-support")]
#[test]
fn test_color_interpolation_basic() {
    let black = Rgb565::new(0, 0, 0);
    let white = Rgb565::new(31, 63, 31);

    // Test boundary values
    assert_eq!(Rgb565::interpolate(black, white, 0.0), black);
    assert_eq!(Rgb565::interpolate(black, white, 1.0), white);

    // Test midpoint
    let mid = Rgb565::interpolate(black, white, 0.5);
    assert_ne!(mid, black);
    assert_ne!(mid, white);

    // Test clamping
    assert_eq!(Rgb565::interpolate(black, white, -0.5), black);
    assert_eq!(Rgb565::interpolate(black, white, 1.5), white);
}

#[cfg(feature = "color-support")]
#[test]
fn test_color_interpolation_colors() {
    let red = Rgb565::new(31, 0, 0); // Max red in 5-bit
    let blue = Rgb565::new(0, 0, 31); // Max blue in 5-bit

    let purple = Rgb565::interpolate(red, blue, 0.5);

    // Should have some red and some blue
    let purple_storage = purple.into_storage();
    let r = (purple_storage >> 11) & 0x1F;
    let b = purple_storage & 0x1F;

    assert!(r > 0 && r < 31);
    assert!(b > 0 && b < 31);
}

#[cfg(feature = "color-support")]
#[test]
fn test_color_interpolation_same_color() {
    let color = Rgb565::new(15, 30, 20);

    // Interpolating between same color should always return that color
    assert_eq!(Rgb565::interpolate(color, color, 0.0), color);
    assert_eq!(Rgb565::interpolate(color, color, 0.5), color);
    assert_eq!(Rgb565::interpolate(color, color, 1.0), color);
}

#[cfg(feature = "color-support")]
#[test]
fn test_gradient_empty_colors() {
    let colors: &[Rgb565] = &[];
    let gradient = Rgb565::gradient(colors, 10);
    assert!(gradient.is_empty());
}

#[cfg(feature = "color-support")]
#[test]
fn test_gradient_zero_steps() {
    let colors = [Rgb565::new(0, 0, 0), Rgb565::new(31, 63, 31)];
    let gradient = Rgb565::gradient(&colors, 0);
    assert!(gradient.is_empty());
}

#[cfg(feature = "color-support")]
#[test]
fn test_gradient_single_color() {
    let colors = [Rgb565::new(10, 20, 15)];
    let gradient = Rgb565::gradient(&colors, 5);

    assert_eq!(gradient.len(), 5);
    for color in gradient.as_slice() {
        assert_eq!(*color, colors[0]);
    }
}

#[cfg(feature = "color-support")]
#[test]
fn test_gradient_two_colors() {
    let colors = [Rgb565::new(0, 0, 0), Rgb565::new(31, 63, 31)];
    let gradient = Rgb565::gradient(&colors, 5);

    assert_eq!(gradient.len(), 5);
    assert_eq!(gradient[0], Rgb565::new(0, 0, 0));
    assert_eq!(gradient[4], Rgb565::new(31, 63, 31));

    // Middle values should be different
    for i in 1..4 {
        assert_ne!(gradient[i], Rgb565::new(0, 0, 0));
        assert_ne!(gradient[i], Rgb565::new(31, 63, 31));
    }
}

#[cfg(feature = "color-support")]
#[test]
fn test_gradient_multiple_colors() {
    let colors = [
        Rgb565::new(0, 0, 0),
        Rgb565::new(15, 30, 15),
        Rgb565::new(31, 63, 31),
    ];
    let gradient = Rgb565::gradient(&colors, 7);

    assert_eq!(gradient.len(), 7);
    assert_eq!(gradient[0], Rgb565::new(0, 0, 0));

    // Should have smooth transitions
    let mid_idx = gradient.len() / 2;
    assert_ne!(gradient[mid_idx], Rgb565::new(0, 0, 0));
    assert_ne!(gradient[mid_idx], Rgb565::new(31, 63, 31));
}

#[cfg(feature = "color-support")]
#[test]
fn test_gradient_capacity_limit() {
    let colors = [Rgb565::new(0, 0, 0), Rgb565::new(31, 63, 31)];
    let gradient = Rgb565::gradient(&colors, 300); // Request more than 256

    assert_eq!(gradient.len(), 256); // Should be limited to 256
}

#[cfg(feature = "color-support")]
#[test]
fn test_gradient_uneven_distribution() {
    let colors = [
        Rgb565::new(0, 0, 0),
        Rgb565::new(15, 30, 15),
        Rgb565::new(31, 63, 31),
    ];
    let gradient = Rgb565::gradient(&colors, 10);

    assert_eq!(gradient.len(), 10);

    // With 3 colors and 10 steps, we have 2 segments
    // Steps should be distributed as evenly as possible
}

#[cfg(feature = "color-support")]
#[test]
fn test_color_utils_from_hex_valid() {
    // Test valid hex colors
    assert_eq!(ColorUtils::from_hex("#000000"), Some(Rgb565::new(0, 0, 0)));

    assert_eq!(
        ColorUtils::from_hex("#FFFFFF"),
        Some(Rgb565::new(31, 63, 31))
    );

    // Test specific color
    let red = ColorUtils::from_hex("#FF0000");
    assert!(red.is_some());
    let red = red.unwrap();
    let storage = red.into_storage();
    let r = (storage >> 11) & 0x1F;
    assert_eq!(r, 31); // Max red value in 5-bit
}

#[cfg(feature = "color-support")]
#[test]
fn test_color_utils_from_hex_invalid() {
    // Missing hash
    assert!(ColorUtils::from_hex("000000").is_none());

    // Wrong length
    assert!(ColorUtils::from_hex("#00").is_none());
    assert!(ColorUtils::from_hex("#0000").is_none());
    assert!(ColorUtils::from_hex("#00000000").is_none());

    // Invalid characters
    assert!(ColorUtils::from_hex("#GGGGGG").is_none());
    assert!(ColorUtils::from_hex("#00XX00").is_none());

    // Empty string
    assert!(ColorUtils::from_hex("").is_none());
}

#[cfg(feature = "color-support")]
#[test]
fn test_color_utils_from_hex_case_sensitivity() {
    // Should handle both upper and lowercase
    let upper = ColorUtils::from_hex("#ABCDEF");
    let lower = ColorUtils::from_hex("#abcdef");

    assert!(upper.is_some());
    assert!(lower.is_some());
    assert_eq!(upper, lower);
}

#[cfg(feature = "color-support")]
#[test]
fn test_contrasting_color_basic() {
    // Black should contrast with white
    assert_eq!(
        ColorUtils::contrasting_color(Rgb565::new(0, 0, 0)),
        Rgb565::new(31, 63, 31)
    );

    // White should contrast with black
    assert_eq!(
        ColorUtils::contrasting_color(Rgb565::new(31, 63, 31)),
        Rgb565::new(0, 0, 0)
    );
}

#[cfg(feature = "color-support")]
#[test]
fn test_contrasting_color_gray() {
    // Test medium gray values
    let gray = Rgb565::new(15, 30, 15); // Roughly middle values
    let contrast = ColorUtils::contrasting_color(gray);

    // Should return either black or white
    assert!(contrast == Rgb565::new(0, 0, 0) || contrast == Rgb565::new(31, 63, 31));
}

#[cfg(feature = "color-support")]
#[test]
fn test_contrasting_color_colors() {
    // Test with various colors
    let red = Rgb565::new(31, 0, 0);
    let green = Rgb565::new(0, 63, 0);
    let blue = Rgb565::new(0, 0, 31);

    // All should have valid contrasting colors
    let red_contrast = ColorUtils::contrasting_color(red);
    let green_contrast = ColorUtils::contrasting_color(green);
    let blue_contrast = ColorUtils::contrasting_color(blue);

    assert!(red_contrast == Rgb565::new(0, 0, 0) || red_contrast == Rgb565::new(31, 63, 31));
    assert!(green_contrast == Rgb565::new(0, 0, 0) || green_contrast == Rgb565::new(31, 63, 31));
    assert!(blue_contrast == Rgb565::new(0, 0, 0) || blue_contrast == Rgb565::new(31, 63, 31));
}

#[cfg(feature = "color-support")]
#[test]
fn test_palette_memory_usage() {
    use core::mem::size_of;

    // Verify palette size is reasonable
    assert!(size_of::<ColorPalette<Rgb565, 8>>() < 256);
    assert!(size_of::<ColorPalette<Rgb565, 16>>() < 512);
}

#[test]
fn test_data_error_display() {
    let error = DataError::buffer_full("test operation", 10);

    // Just verify it can be created and has the expected type
    match error {
        DataError::BufferFull { context } => {
            assert!(context.is_some());
            if let Some(ctx) = context {
                assert_eq!(ctx.operation, "test operation");
                assert_eq!(ctx.numeric_context, Some(10));
            }
        }
        _ => panic!("Wrong error type"),
    }
}

#[cfg(feature = "color-support")]
#[test]
fn test_gradient_single_step_per_segment() {
    let colors = [Rgb565::new(0, 0, 0), Rgb565::new(31, 63, 31)];
    let gradient = Rgb565::gradient(&colors, 1);

    assert_eq!(gradient.len(), 1);
    // With only 1 step, should be the first color
    assert_eq!(gradient[0], Rgb565::new(0, 0, 0));
}

#[cfg(feature = "color-support")]
#[test]
fn test_palette_cycling_persistence() {
    let mut palette = rgb565_palettes::default_palette();
    let palette_len = palette.len();

    // Get first color
    let first_color = palette.next_color();

    // Cycle through all colors to come back to the first
    for _ in 1..palette_len {
        palette.next_color();
    }

    // Should be back at the first color
    let cycled_color = palette.next_color();
    assert_eq!(first_color, cycled_color);
}

#[cfg(feature = "color-support")]
#[test]
fn test_all_palettes_non_empty() {
    // Verify all predefined palettes have colors
    assert!(!rgb565_palettes::default_palette().is_empty());
    assert!(!rgb565_palettes::professional_palette().is_empty());
    assert!(!rgb565_palettes::pastel_palette().is_empty());
    assert!(!rgb565_palettes::vibrant_palette().is_empty());
    assert!(!rgb565_palettes::nature_palette().is_empty());
    assert!(!rgb565_palettes::ocean_palette().is_empty());
    assert!(!rgb565_palettes::sunset_palette().is_empty());
    assert!(!rgb565_palettes::cyberpunk_palette().is_empty());
    assert!(!rgb565_palettes::high_contrast_palette().is_empty());
    assert!(!rgb565_palettes::monochrome_palette().is_empty());
    assert!(!rgb565_palettes::minimal_palette().is_empty());
    assert!(!rgb565_palettes::retro_palette().is_empty());
}

#[cfg(feature = "color-support")]
#[test]
fn test_interpolation_precision() {
    let c1 = Rgb565::new(10, 20, 15);
    let c2 = Rgb565::new(20, 40, 25);

    // Test various interpolation factors
    let factors = [0.0, 0.1, 0.25, 0.33, 0.5, 0.66, 0.75, 0.9, 1.0];

    for &t in &factors {
        let result = Rgb565::interpolate(c1, c2, t);

        // At minimum, boundary conditions should hold
        if t == 0.0 {
            assert_eq!(result, c1);
        } else if t == 1.0 {
            assert_eq!(result, c2);
        }
    }
}

#[cfg(feature = "color-support")]
#[test]
fn test_luminance_calculation_edge_cases() {
    // Test colors with known luminance characteristics
    let bright_yellow = Rgb565::new(31, 63, 0); // High luminance
    let dark_blue = Rgb565::new(0, 0, 31); // Low luminance

    assert_eq!(
        ColorUtils::contrasting_color(bright_yellow),
        Rgb565::new(0, 0, 0)
    );
    assert_eq!(
        ColorUtils::contrasting_color(dark_blue),
        Rgb565::new(31, 63, 31)
    );
}
