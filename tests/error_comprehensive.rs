//! Comprehensive tests for error types and error handling
//! Target: Increase coverage from 2.46% to 70%

use embedded_charts::error::{
    AnimationError, ChartError, DataError, DataErrorKind, ErrorContext, LayoutError, RenderError,
};

#[test]
fn test_error_context_creation() {
    // Basic context
    let ctx = ErrorContext::new("test_operation", "test hint");
    assert_eq!(ctx.operation, "test_operation");
    assert_eq!(ctx.hint, "test hint");
    assert_eq!(ctx.numeric_context, None);

    // Context with numeric value
    let ctx_num = ErrorContext::with_numeric("buffer_add", "Increase buffer size", 256);
    assert_eq!(ctx_num.operation, "buffer_add");
    assert_eq!(ctx_num.hint, "Increase buffer size");
    assert_eq!(ctx_num.numeric_context, Some(256));
}

#[test]
fn test_error_context_debug_and_equality() {
    let ctx1 = ErrorContext::new("op1", "hint1");
    let ctx2 = ErrorContext::new("op1", "hint1");
    let ctx3 = ErrorContext::new("op2", "hint2");

    assert_eq!(ctx1, ctx2);
    assert_ne!(ctx1, ctx3);

    let debug_str = format!("{ctx1:?}");
    assert!(debug_str.contains("ErrorContext"));
    assert!(debug_str.contains("op1"));
    assert!(debug_str.contains("hint1"));
}

#[test]
fn test_chart_error_display() {
    let errors = vec![
        (
            ChartError::InsufficientData,
            "Insufficient data to render the chart",
        ),
        (
            ChartError::InvalidRange,
            "Invalid range specified for axis or data",
        ),
        (ChartError::InvalidData, "Invalid data provided"),
        (
            ChartError::MemoryFull,
            "Memory allocation failed or buffer is full",
        ),
        (
            ChartError::RenderingError,
            "Error occurred during rendering",
        ),
        (
            ChartError::InvalidConfiguration,
            "Invalid configuration provided",
        ),
        (
            ChartError::ConfigurationError,
            "Configuration error occurred",
        ),
    ];

    for (error, expected_msg) in errors {
        assert_eq!(format!("{error}"), expected_msg);
    }
}

#[test]
fn test_chart_error_from_sub_errors() {
    // From DataError
    let data_err = DataError::BUFFER_FULL;
    let chart_err: ChartError = data_err.into();
    assert_eq!(chart_err, ChartError::DataError(data_err));

    // From LayoutError
    let layout_err = LayoutError::InsufficientSpace;
    let chart_err: ChartError = layout_err.into();
    assert_eq!(chart_err, ChartError::InvalidConfiguration);

    // From RenderError
    let render_err = RenderError::DrawingFailed;
    let chart_err: ChartError = render_err.into();
    assert_eq!(chart_err, ChartError::RenderingError);
}

#[test]
#[cfg(feature = "animations")]
fn test_chart_error_from_animation_error() {
    let anim_err = AnimationError::InvalidDuration;
    let chart_err: ChartError = anim_err.into();
    assert_eq!(chart_err, ChartError::AnimationError(anim_err));
}

#[test]
fn test_data_error_constants() {
    assert_eq!(
        DataError::SERIES_NOT_FOUND,
        DataError::SeriesNotFound { context: None }
    );
    assert_eq!(
        DataError::INDEX_OUT_OF_BOUNDS,
        DataError::IndexOutOfBounds { context: None }
    );
    assert_eq!(
        DataError::INVALID_DATA_POINT,
        DataError::InvalidDataPoint { context: None }
    );
    assert_eq!(
        DataError::SCALING_ERROR,
        DataError::ScalingError { context: None }
    );
    assert_eq!(
        DataError::BUFFER_FULL,
        DataError::BufferFull { context: None }
    );
    assert_eq!(
        DataError::INSUFFICIENT_DATA,
        DataError::InsufficientData { context: None }
    );
}

#[test]
fn test_data_error_factory_methods() {
    // buffer_full
    let err = DataError::buffer_full("add_point", 100);
    match err {
        DataError::BufferFull { context: Some(ctx) } => {
            assert_eq!(ctx.operation, "add_point");
            assert_eq!(ctx.hint, "Increase buffer capacity or remove old data");
            assert_eq!(ctx.numeric_context, Some(100));
        }
        _ => panic!("Expected BufferFull with context"),
    }

    // insufficient_data
    let err = DataError::insufficient_data("calculate_bounds", 3, 1);
    match err {
        DataError::InsufficientData { context: Some(ctx) } => {
            assert_eq!(ctx.operation, "calculate_bounds");
            assert_eq!(ctx.hint, "Add more data points");
            assert_eq!(ctx.numeric_context, Some(1));
        }
        _ => panic!("Expected InsufficientData with context"),
    }

    // index_out_of_bounds
    let err = DataError::index_out_of_bounds("get_item", 10, 5);
    match err {
        DataError::IndexOutOfBounds { context: Some(ctx) } => {
            assert_eq!(ctx.operation, "get_item");
            assert_eq!(ctx.hint, "Check array bounds before accessing");
            assert_eq!(ctx.numeric_context, Some(5));
        }
        _ => panic!("Expected IndexOutOfBounds with context"),
    }

    // invalid_data_point
    let err = DataError::invalid_data_point("add_nan_point");
    match err {
        DataError::InvalidDataPoint { context: Some(ctx) } => {
            assert_eq!(ctx.operation, "add_nan_point");
            assert_eq!(ctx.hint, "Ensure data points contain valid finite values");
            assert_eq!(ctx.numeric_context, None);
        }
        _ => panic!("Expected InvalidDataPoint with context"),
    }
}

#[test]
fn test_data_error_simple() {
    let kinds = vec![
        (
            DataErrorKind::SeriesNotFound,
            DataError::SeriesNotFound { context: None },
        ),
        (
            DataErrorKind::IndexOutOfBounds,
            DataError::IndexOutOfBounds { context: None },
        ),
        (
            DataErrorKind::InvalidDataPoint,
            DataError::InvalidDataPoint { context: None },
        ),
        (
            DataErrorKind::ScalingError,
            DataError::ScalingError { context: None },
        ),
        (
            DataErrorKind::BufferFull,
            DataError::BufferFull { context: None },
        ),
        (
            DataErrorKind::InsufficientData,
            DataError::InsufficientData { context: None },
        ),
    ];

    for (kind, expected) in kinds {
        assert_eq!(DataError::simple(kind), expected);
    }
}

#[test]
fn test_data_error_from_str() {
    let err: DataError = "some error message".into();
    assert_eq!(err, DataError::InvalidDataPoint { context: None });
}

#[test]
fn test_data_error_display() {
    // Without context
    let err = DataError::SeriesNotFound { context: None };
    assert_eq!(format!("{err}"), "Requested data series was not found");

    // With context
    let err = DataError::BufferFull {
        context: Some(ErrorContext::with_numeric(
            "push_data",
            "Remove old data",
            512,
        )),
    };
    let display = format!("{err}");
    assert!(display.contains("Buffer capacity exceeded"));
    assert!(display.contains("during push_data"));
    assert!(display.contains("hint: Remove old data"));
    assert!(display.contains("[capacity: 512]"));

    // Index out of bounds with context
    let err = DataError::IndexOutOfBounds {
        context: Some(ErrorContext::with_numeric(
            "array_access",
            "Check bounds",
            10,
        )),
    };
    let display = format!("{err}");
    assert!(display.contains("Index is out of bounds"));
    assert!(display.contains("during array_access"));
    assert!(display.contains("[length: 10]"));

    // Invalid data point with context
    let err = DataError::InvalidDataPoint {
        context: Some(ErrorContext::new("validate", "Use finite values")),
    };
    let display = format!("{err}");
    assert!(display.contains("Invalid data point"));
    assert!(display.contains("during validate"));
    assert!(display.contains("hint: Use finite values"));

    // Scaling error
    let err = DataError::ScalingError {
        context: Some(ErrorContext::new("scale_data", "Check ranges")),
    };
    let display = format!("{err}");
    assert!(display.contains("Error occurred during data scaling"));
    assert!(display.contains("during scale_data"));

    // Insufficient data with found count
    let err = DataError::InsufficientData {
        context: Some(ErrorContext::with_numeric("render", "Need more points", 2)),
    };
    let display = format!("{err}");
    assert!(display.contains("Insufficient data"));
    assert!(display.contains("[found: 2]"));
}

#[test]
#[cfg(feature = "animations")]
fn test_animation_error_display() {
    let errors = vec![
        (
            AnimationError::InvalidDuration,
            "Invalid duration specified",
        ),
        (
            AnimationError::AnimationNotFound,
            "Animation with specified ID was not found",
        ),
        (AnimationError::SchedulerFull, "Animation scheduler is full"),
        (
            AnimationError::InterpolationError,
            "Error occurred during interpolation",
        ),
        (AnimationError::InvalidState, "Animation state is invalid"),
    ];

    for (error, expected_msg) in errors {
        assert_eq!(format!("{error}"), expected_msg);
    }
}

#[test]
fn test_layout_error_display() {
    let errors = vec![
        (
            LayoutError::InsufficientSpace,
            "Insufficient space for layout",
        ),
        (
            LayoutError::InvalidConfiguration,
            "Invalid layout configuration",
        ),
        (
            LayoutError::PositioningFailed,
            "Component positioning failed",
        ),
    ];

    for (error, expected_msg) in errors {
        assert_eq!(format!("{error}"), expected_msg);
    }
}

#[test]
fn test_render_error_display() {
    let errors = vec![
        (RenderError::DrawingFailed, "Drawing operation failed"),
        (RenderError::TextRenderingFailed, "Text rendering failed"),
        (RenderError::ClippingFailed, "Clipping operation failed"),
        (
            RenderError::ColorConversionFailed,
            "Color conversion failed",
        ),
    ];

    for (error, expected_msg) in errors {
        assert_eq!(format!("{error}"), expected_msg);
    }
}

#[test]
fn test_error_debug_trait() {
    // Test Debug trait implementation
    let chart_err = ChartError::InvalidData;
    let debug_str = format!("{chart_err:?}");
    assert!(debug_str.contains("InvalidData"));

    let data_err = DataError::BufferFull { context: None };
    let debug_str = format!("{data_err:?}");
    assert!(debug_str.contains("BufferFull"));

    let layout_err = LayoutError::InsufficientSpace;
    let debug_str = format!("{layout_err:?}");
    assert!(debug_str.contains("InsufficientSpace"));

    let render_err = RenderError::DrawingFailed;
    let debug_str = format!("{render_err:?}");
    assert!(debug_str.contains("DrawingFailed"));
}

#[test]
fn test_error_equality() {
    // ChartError equality
    assert_eq!(ChartError::InvalidData, ChartError::InvalidData);
    assert_ne!(ChartError::InvalidData, ChartError::MemoryFull);

    // DataError equality
    let err1 = DataError::BufferFull { context: None };
    let err2 = DataError::BufferFull { context: None };
    let err3 = DataError::BufferFull {
        context: Some(ErrorContext::new("op", "hint")),
    };
    assert_eq!(err1, err2);
    assert_ne!(err1, err3);

    // LayoutError equality
    assert_eq!(
        LayoutError::InsufficientSpace,
        LayoutError::InsufficientSpace
    );
    assert_ne!(
        LayoutError::InsufficientSpace,
        LayoutError::InvalidConfiguration
    );

    // RenderError equality
    assert_eq!(RenderError::DrawingFailed, RenderError::DrawingFailed);
    assert_ne!(RenderError::DrawingFailed, RenderError::TextRenderingFailed);
}

#[test]
fn test_error_copy_clone() {
    // Test that all error types implement Copy and Clone
    let chart_err = ChartError::InvalidData;
    let _chart_copy = chart_err; // Copy
    let _chart_clone = chart_err; // Clone (Copy types don't need explicit .clone())

    let data_err = DataError::BUFFER_FULL;
    let _data_copy = data_err; // Copy
    let _data_clone = data_err; // Clone (Copy types don't need explicit .clone())

    let layout_err = LayoutError::InsufficientSpace;
    let _layout_copy = layout_err; // Copy
    let _layout_clone = layout_err; // Clone (Copy types don't need explicit .clone())

    let render_err = RenderError::DrawingFailed;
    let _render_copy = render_err; // Copy
    let _render_clone = render_err; // Clone (Copy types don't need explicit .clone())
}

#[test]
fn test_chart_error_with_sub_errors_display() {
    // ChartError with DataError
    let data_err = DataError::buffer_full("test_op", 100);
    let chart_err = ChartError::DataError(data_err);
    let display = format!("{chart_err}");
    assert!(display.contains("Data error:"));
    assert!(display.contains("Buffer capacity exceeded"));

    // ChartError with RenderError
    let render_err = RenderError::TextRenderingFailed;
    let chart_err = ChartError::RenderError(render_err);
    let display = format!("{chart_err}");
    assert!(display.contains("Render error:"));
    assert!(display.contains("Text rendering failed"));

    // ChartError with LayoutError
    let layout_err = LayoutError::PositioningFailed;
    let chart_err = ChartError::LayoutError(layout_err);
    let display = format!("{chart_err}");
    assert!(display.contains("Layout error:"));
    assert!(display.contains("Component positioning failed"));
}

#[test]
#[cfg(feature = "animations")]
fn test_chart_error_with_animation_error_display() {
    let anim_err = AnimationError::SchedulerFull;
    let chart_err = ChartError::AnimationError(anim_err);
    let display = format!("{chart_err}");
    assert!(display.contains("Animation error:"));
    assert!(display.contains("Animation scheduler is full"));
}

#[test]
#[cfg(feature = "std")]
fn test_std_error_trait() {
    use std::error::Error;

    // Test that errors implement std::error::Error
    let chart_err = ChartError::InvalidData;
    let _error_ref: &dyn Error = &chart_err;

    let data_err = DataError::BUFFER_FULL;
    let _error_ref: &dyn Error = &data_err;

    let layout_err = LayoutError::InsufficientSpace;
    let _error_ref: &dyn Error = &layout_err;

    let render_err = RenderError::DrawingFailed;
    let _error_ref: &dyn Error = &render_err;

    // Test source() method
    let data_err = DataError::BUFFER_FULL;
    let chart_err = ChartError::DataError(data_err);
    assert!(chart_err.source().is_some());

    let simple_err = ChartError::InvalidData;
    assert!(simple_err.source().is_none());
}

#[test]
fn test_data_error_kind() {
    // Test DataErrorKind Debug and PartialEq
    let kind = DataErrorKind::BufferFull;
    let debug_str = format!("{kind:?}");
    assert!(debug_str.contains("BufferFull"));

    assert_eq!(DataErrorKind::SeriesNotFound, DataErrorKind::SeriesNotFound);
    assert_ne!(DataErrorKind::SeriesNotFound, DataErrorKind::BufferFull);
}

#[test]
fn test_result_type_aliases() {
    use embedded_charts::error::{ChartResult, DataResult, LayoutResult, RenderResult};

    // Test that result types work correctly
    let _ok_result: ChartResult<i32> = Ok(42);
    // Test that result types work correctly

    let err_result: ChartResult<i32> = Err(ChartError::InvalidData);
    assert!(err_result.is_err());

    let data_result: DataResult<String> = Err(DataError::BUFFER_FULL);
    assert!(data_result.is_err());

    let _layout_result: LayoutResult<f32> = Ok(3.0);
    // Test that layout result works

    let render_result: RenderResult<()> = Err(RenderError::DrawingFailed);
    assert!(render_result.is_err());
}

#[test]
#[cfg(feature = "animations")]
fn test_animation_result_type_alias() {
    use embedded_charts::error::AnimationResult;

    let _ok_result: AnimationResult<u32> = Ok(123);
    // Test that animation result works

    let err_result: AnimationResult<u32> = Err(AnimationError::InvalidDuration);
    assert!(err_result.is_err());
}

#[test]
fn test_error_conversion_chain() {
    // Test conversion chain: DataError -> ChartError
    let data_err = DataError::insufficient_data("test", 5, 2);
    let chart_err: ChartError = data_err.into();

    match chart_err {
        ChartError::DataError(inner) => match inner {
            DataError::InsufficientData { context: Some(ctx) } => {
                assert_eq!(ctx.operation, "test");
                assert_eq!(ctx.numeric_context, Some(2));
            }
            _ => panic!("Expected InsufficientData with context"),
        },
        _ => panic!("Expected ChartError::DataError"),
    }
}
