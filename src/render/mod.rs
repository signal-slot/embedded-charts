//! Rendering module for embedded charts
//!
//! This module provides rendering functionality including:
//! - Base rendering operations
//! - Display-specific optimizations
//! - Performance optimizations for embedded systems

mod base;
pub mod optimized;

// Re-export the text module from base
pub use base::text;

// Re-export base rendering functionality
pub use base::{
    AnimationFrameRenderer, ChartRenderer, ClippingRenderer, EnhancedChartRenderer,
    PrimitiveRenderer,
};

// Re-export optimized rendering
pub use optimized::{DisplayType, EPaperRenderer, OLEDRenderer, OptimizedRenderer, TFTRenderer};
