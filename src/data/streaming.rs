//! Unified streaming data functionality for real-time chart updates.
//!
//! This module provides a comprehensive streaming data architecture that combines
//! the best of both streaming implementations with enhanced performance and reliability.

use crate::data::point::{DataPoint, Point2D};
use crate::error::{ChartError, ChartResult, DataError};
use crate::memory::{ManagedSlidingWindow, MemoryStats};
use heapless::Vec;

/// Configuration for streaming data behavior
#[derive(Debug, Clone, Copy)]
pub struct StreamingConfig {
    /// Buffer capacity for data points
    pub buffer_capacity: usize,
    /// Update interval in milliseconds
    pub update_interval: u32,
    /// Enable automatic data pruning when buffer is full
    pub auto_prune: bool,
    /// Maximum age of data points in milliseconds (0 = no age limit)
    pub max_data_age: u32,
    /// Enable auto-scaling of chart bounds
    pub auto_scale: bool,
    /// Memory usage threshold for warnings (percentage)
    pub memory_threshold: f32,
}

impl Default for StreamingConfig {
    fn default() -> Self {
        Self {
            buffer_capacity: 100,
            update_interval: 100, // 10 Hz
            auto_prune: true,
            max_data_age: 0,
            auto_scale: true,
            memory_threshold: 80.0,
        }
    }
}

/// A unified streaming data buffer that combines efficient memory management
/// with real-time data handling capabilities.
pub struct UnifiedStreamingBuffer<const N: usize> {
    /// Primary data buffer using managed sliding window
    buffer: ManagedSlidingWindow<Point2D, N>,
    /// Configuration for streaming behavior
    config: StreamingConfig,
    /// Last update timestamp
    last_update: u32,
    /// Data bounds for auto-scaling
    bounds: Option<crate::data::bounds::DataBounds<f32, f32>>,
    /// Performance metrics
    metrics: StreamingMetrics,
}

/// Performance metrics for streaming operations
#[derive(Debug, Clone, Copy, Default)]
pub struct StreamingMetrics {
    /// Total number of data points processed
    pub total_points: u64,
    /// Number of points dropped due to buffer overflow
    pub dropped_points: u64,
    /// Number of points pruned due to age
    pub pruned_points: u64,
    /// Average update latency in microseconds
    pub avg_latency_us: u32,
    /// Peak memory usage
    pub peak_memory_usage: usize,
    /// Current update rate (Hz)
    pub current_update_rate: f32,
}

impl<const N: usize> UnifiedStreamingBuffer<N> {
    /// Create a new unified streaming buffer with default configuration
    pub fn new() -> Self {
        Self::with_config(StreamingConfig::default())
    }

    /// Create a new unified streaming buffer with custom configuration
    pub fn with_config(config: StreamingConfig) -> Self {
        Self {
            buffer: ManagedSlidingWindow::new(),
            config,
            last_update: 0,
            bounds: None,
            metrics: StreamingMetrics::default(),
        }
    }

    /// Add a new data point to the buffer with timestamp
    pub fn push_with_timestamp(&mut self, point: Point2D, timestamp: u32) -> ChartResult<()> {
        let start_time = self.get_current_time_us();

        // Check if we should prune old data
        if self.config.auto_prune && self.config.max_data_age > 0 {
            self.prune_old_data(timestamp)?;
        }

        // Check memory usage
        if self.buffer.memory_stats().utilization_percent() > self.config.memory_threshold {
            return Err(ChartError::DataError(DataError::BUFFER_FULL));
        }

        // Add the point
        self.buffer.push(point);
        self.metrics.total_points += 1;

        // Update bounds if auto-scaling is enabled
        if self.config.auto_scale {
            self.update_bounds();
        }

        // Update performance metrics
        let end_time = self.get_current_time_us();
        self.update_latency_metrics(end_time - start_time);
        self.update_memory_metrics();

        self.last_update = timestamp;
        Ok(())
    }

    /// Add a new data point to the buffer (uses current time)
    pub fn push(&mut self, point: Point2D) -> ChartResult<()> {
        let timestamp = self.get_current_time_ms();
        self.push_with_timestamp(point, timestamp)
    }

    /// Get the current data points in the buffer
    pub fn data(&self) -> impl Iterator<Item = Point2D> + '_ {
        self.buffer.iter()
    }

    /// Get the number of points currently in the buffer
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    /// Check if the buffer is empty
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    /// Clear all data from the buffer
    pub fn clear(&mut self) {
        self.buffer.clear();
        self.bounds = None;
        self.metrics = StreamingMetrics::default();
    }

    /// Get the buffer capacity
    pub fn capacity(&self) -> usize {
        N
    }

    /// Get current memory statistics
    pub fn memory_stats(&self) -> &MemoryStats {
        self.buffer.memory_stats()
    }

    /// Get performance metrics
    pub fn metrics(&self) -> &StreamingMetrics {
        &self.metrics
    }

    /// Get current data bounds
    pub fn bounds(&self) -> Option<crate::data::bounds::DataBounds<f32, f32>> {
        self.bounds
    }

    /// Update streaming configuration
    pub fn update_config(&mut self, config: StreamingConfig) {
        self.config = config;
    }

    /// Get current configuration
    pub fn config(&self) -> &StreamingConfig {
        &self.config
    }

    /// Prune old data points based on age
    fn prune_old_data(&mut self, _current_time: u32) -> ChartResult<()> {
        // For now, we'll implement a simple approach since we can't easily remove
        // specific items from the sliding window. In a real implementation,
        // we might need a different data structure for age-based pruning.

        // This is a placeholder - the sliding window automatically handles
        // overflow by removing the oldest items
        Ok(())
    }

    /// Update data bounds for auto-scaling
    fn update_bounds(&mut self) {
        if let Ok(bounds) = crate::data::bounds::calculate_bounds(self.buffer.iter()) {
            self.bounds = Some(bounds);
        }
    }

    /// Update latency metrics
    fn update_latency_metrics(&mut self, latency_us: u32) {
        // Simple moving average for latency
        if self.metrics.avg_latency_us == 0 {
            self.metrics.avg_latency_us = latency_us;
        } else {
            self.metrics.avg_latency_us = (self.metrics.avg_latency_us * 7 + latency_us) / 8;
        }
    }

    /// Update memory usage metrics
    fn update_memory_metrics(&mut self) {
        let current_usage = self.buffer.memory_stats().used;
        if current_usage > self.metrics.peak_memory_usage {
            self.metrics.peak_memory_usage = current_usage;
        }
    }

    /// Get current time in milliseconds (placeholder implementation)
    fn get_current_time_ms(&self) -> u32 {
        // In a real embedded system, this would use a hardware timer
        // For now, we'll use a simple counter based on updates
        self.metrics.total_points as u32
    }

    /// Get current time in microseconds (placeholder implementation)
    fn get_current_time_us(&self) -> u32 {
        // In a real embedded system, this would use a high-resolution timer
        self.get_current_time_ms() * 1000
    }
}

impl<const N: usize> Default for UnifiedStreamingBuffer<N> {
    fn default() -> Self {
        Self::new()
    }
}

/// Streaming data pipeline that connects data sources to animated charts
pub struct StreamingDataPipeline<T: Copy + Clone + DataPoint, const N: usize> {
    /// Data sources feeding the pipeline
    sources: Vec<StreamingDataSource<N>, 8>,
    /// Animation controller for smooth updates
    animation: crate::animation::StreamingAnimator<T>,
    /// Pipeline configuration
    config: PipelineConfig,
    /// Pipeline metrics
    metrics: PipelineMetrics,
}

/// Configuration for the streaming data pipeline
#[derive(Debug, Clone, Copy)]
pub struct PipelineConfig {
    /// Maximum number of data sources
    pub max_sources: usize,
    /// Synchronization mode for multiple sources
    pub sync_mode: SyncMode,
    /// Error recovery strategy
    pub error_recovery: ErrorRecovery,
    /// Performance monitoring enabled
    pub monitoring_enabled: bool,
}

/// Synchronization mode for multiple data sources
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SyncMode {
    /// No synchronization - sources update independently
    Independent,
    /// Synchronize updates across all sources
    Synchronized,
    /// Use the fastest source as the master clock
    FastestMaster,
}

/// Error recovery strategy
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ErrorRecovery {
    /// Stop pipeline on first error
    StopOnError,
    /// Continue with other sources on error
    ContinueOnError,
    /// Retry failed operations
    RetryOnError,
}

/// Pipeline performance metrics
#[derive(Debug, Clone, Copy, Default)]
pub struct PipelineMetrics {
    /// Total data points processed across all sources
    pub total_processed: u64,
    /// Number of synchronization events
    pub sync_events: u64,
    /// Number of errors encountered
    pub error_count: u64,
    /// Average pipeline latency
    pub avg_pipeline_latency_us: u32,
    /// Current throughput (points per second)
    pub throughput_pps: f32,
}

impl Default for PipelineConfig {
    fn default() -> Self {
        Self {
            max_sources: 8,
            sync_mode: SyncMode::Independent,
            error_recovery: ErrorRecovery::ContinueOnError,
            monitoring_enabled: true,
        }
    }
}

impl<T: Copy + Clone + DataPoint, const N: usize> StreamingDataPipeline<T, N> {
    /// Create a new streaming data pipeline
    pub fn new(update_rate_hz: u32) -> Self {
        Self::with_config(update_rate_hz, PipelineConfig::default())
    }

    /// Create a new streaming data pipeline with custom configuration
    pub fn with_config(_update_rate_hz: u32, config: PipelineConfig) -> Self {
        Self {
            sources: Vec::new(),
            animation: crate::animation::StreamingAnimator::new(),
            config,
            metrics: PipelineMetrics::default(),
        }
    }

    /// Add a data source to the pipeline
    pub fn add_source(&mut self, source: StreamingDataSource<N>) -> ChartResult<usize> {
        if self.sources.len() >= self.config.max_sources {
            return Err(ChartError::DataError(DataError::BUFFER_FULL));
        }

        let index = self.sources.len();
        self.sources
            .push(source)
            .map_err(|_| ChartError::DataError(DataError::BUFFER_FULL))?;

        Ok(index)
    }

    /// Update the pipeline with new data
    pub fn update(&mut self, delta_time: crate::time::Milliseconds) -> ChartResult<bool> {
        let start_time = self.get_current_time_us();
        let mut updated = false;

        // Update animation
        if self.animation.update_with_delta(delta_time)? {
            updated = true;
        }

        // Update metrics if monitoring is enabled
        if self.config.monitoring_enabled {
            let end_time = self.get_current_time_us();
            self.update_pipeline_metrics(end_time - start_time);
        }

        Ok(updated)
    }

    /// Get current data from the pipeline
    pub fn current_data(&self) -> impl Iterator<Item = T> + '_ {
        self.animation.current_data()
    }

    /// Get pipeline metrics
    pub fn metrics(&self) -> &PipelineMetrics {
        &self.metrics
    }

    /// Get number of active sources
    pub fn source_count(&self) -> usize {
        self.sources.len()
    }

    /// Update pipeline performance metrics
    fn update_pipeline_metrics(&mut self, latency_us: u32) {
        // Update average latency using exponential moving average
        if self.metrics.avg_pipeline_latency_us == 0 {
            self.metrics.avg_pipeline_latency_us = latency_us;
        } else {
            self.metrics.avg_pipeline_latency_us =
                (self.metrics.avg_pipeline_latency_us * 7 + latency_us) / 8;
        }
    }

    /// Get current time in microseconds (placeholder)
    fn get_current_time_us(&self) -> u32 {
        // In a real system, this would use hardware timers
        self.metrics.total_processed as u32
    }
}

/// Enhanced streaming data source with improved capabilities
pub struct StreamingDataSource<const N: usize> {
    /// Unified streaming buffer
    buffer: UnifiedStreamingBuffer<N>,
    /// Source identifier
    id: u32,
    /// Source configuration
    config: SourceConfig,
    /// Source state
    state: SourceState,
}

/// Configuration for a streaming data source
#[derive(Debug, Clone, Copy)]
pub struct SourceConfig {
    /// Update interval in milliseconds
    pub update_interval: u32,
    /// Source priority (higher = more important)
    pub priority: u8,
    /// Enable data validation
    pub validate_data: bool,
    /// Maximum consecutive errors before disabling
    pub max_errors: u32,
}

/// State of a streaming data source
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SourceState {
    /// Source is active and receiving data
    Active,
    /// Source is temporarily paused
    Paused,
    /// Source has encountered errors
    Error,
    /// Source is disabled
    Disabled,
}

impl Default for SourceConfig {
    fn default() -> Self {
        Self {
            update_interval: 100,
            priority: 1,
            validate_data: true,
            max_errors: 5,
        }
    }
}

impl<const N: usize> StreamingDataSource<N> {
    /// Create a new streaming data source
    pub fn new(id: u32) -> Self {
        Self::with_config(id, SourceConfig::default())
    }

    /// Create a new streaming data source with custom configuration
    pub fn with_config(id: u32, config: SourceConfig) -> Self {
        let streaming_config = StreamingConfig {
            update_interval: config.update_interval,
            ..Default::default()
        };

        Self {
            buffer: UnifiedStreamingBuffer::with_config(streaming_config),
            id,
            config,
            state: SourceState::Active,
        }
    }

    /// Update the data source with a new point
    pub fn update(&mut self, point: Point2D) -> ChartResult<()> {
        if self.state != SourceState::Active {
            return Err(ChartError::DataError(DataError::INVALID_DATA_POINT));
        }

        // Validate data if enabled
        if self.config.validate_data && !self.is_valid_point(&point) {
            return Err(ChartError::DataError(DataError::INVALID_DATA_POINT));
        }

        self.buffer.push(point)
    }

    /// Get the current data from the source
    pub fn data(&self) -> impl Iterator<Item = Point2D> + '_ {
        self.buffer.data()
    }

    /// Get source ID
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Get source state
    pub fn state(&self) -> SourceState {
        self.state
    }

    /// Set source state
    pub fn set_state(&mut self, state: SourceState) {
        self.state = state;
    }

    /// Get source configuration
    pub fn config(&self) -> &SourceConfig {
        &self.config
    }

    /// Get source metrics
    pub fn metrics(&self) -> &StreamingMetrics {
        self.buffer.metrics()
    }

    /// Validate a data point
    fn is_valid_point(&self, point: &Point2D) -> bool {
        // Basic validation - check for NaN and infinite values
        point.x.is_finite() && point.y.is_finite()
    }
}

/// Manager for coordinating multiple streaming charts
pub struct StreamingChartManager<const MAX_CHARTS: usize> {
    /// Active streaming charts
    charts: Vec<ChartInstance, MAX_CHARTS>,
    /// Global configuration
    config: ManagerConfig,
    /// Manager metrics
    metrics: ManagerMetrics,
    /// Synchronization state
    sync_state: SyncState,
}

/// Configuration for the streaming chart manager
#[derive(Debug, Clone, Copy)]
pub struct ManagerConfig {
    /// Global update rate in Hz
    pub global_update_rate: u32,
    /// Enable cross-chart synchronization
    pub enable_sync: bool,
    /// Memory management strategy
    pub memory_strategy: MemoryStrategy,
    /// Performance monitoring level
    pub monitoring_level: MonitoringLevel,
}

/// Memory management strategy
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MemoryStrategy {
    /// Conservative - prioritize memory efficiency
    Conservative,
    /// Balanced - balance memory and performance
    Balanced,
    /// Performance - prioritize update speed
    Performance,
}

/// Performance monitoring level
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MonitoringLevel {
    /// No monitoring
    None,
    /// Basic metrics only
    Basic,
    /// Detailed performance tracking
    Detailed,
}

/// Chart instance in the manager
#[derive(Debug)]
pub struct ChartInstance {
    /// Chart identifier
    pub id: u32,
    /// Chart type identifier
    pub chart_type: ChartType,
    /// Associated data pipeline
    pub pipeline_id: u32,
    /// Chart-specific configuration
    pub config: ChartInstanceConfig,
}

/// Type of chart
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChartType {
    /// Line chart type
    Line,
    /// Bar chart type
    Bar,
    /// Scatter plot chart type
    Scatter,
    /// Gauge chart type
    Gauge,
    /// Custom chart type
    Custom,
}

/// Configuration for a chart instance
#[derive(Debug, Clone, Copy)]
pub struct ChartInstanceConfig {
    /// Update priority
    pub priority: u8,
    /// Enable animations
    pub animations_enabled: bool,
    /// Memory limit for this chart
    pub memory_limit_bytes: usize,
}

/// Manager performance metrics
#[derive(Debug, Clone, Copy, Default)]
pub struct ManagerMetrics {
    /// Total charts managed
    pub total_charts: u32,
    /// Active charts
    pub active_charts: u32,
    /// Total updates processed
    pub total_updates: u64,
    /// Average update latency across all charts
    pub avg_update_latency_us: u32,
    /// Memory usage across all charts
    pub total_memory_usage: usize,
}

/// Synchronization state for the manager
#[derive(Debug, Clone, Copy, Default)]
pub struct SyncState {
    /// Last global sync timestamp
    pub last_sync_time: u32,
    /// Number of charts waiting for sync
    pub pending_sync_count: u32,
    /// Sync drift in microseconds
    pub sync_drift_us: i32,
}

impl Default for ManagerConfig {
    fn default() -> Self {
        Self {
            global_update_rate: 30,
            enable_sync: true,
            memory_strategy: MemoryStrategy::Balanced,
            monitoring_level: MonitoringLevel::Basic,
        }
    }
}

impl Default for ChartInstanceConfig {
    fn default() -> Self {
        Self {
            priority: 1,
            animations_enabled: true,
            memory_limit_bytes: 4096,
        }
    }
}

impl<const MAX_CHARTS: usize> StreamingChartManager<MAX_CHARTS> {
    /// Create a new streaming chart manager
    pub fn new() -> Self {
        Self::with_config(ManagerConfig::default())
    }

    /// Create a new streaming chart manager with custom configuration
    pub fn with_config(config: ManagerConfig) -> Self {
        Self {
            charts: Vec::new(),
            config,
            metrics: ManagerMetrics::default(),
            sync_state: SyncState::default(),
        }
    }

    /// Add a chart to the manager
    pub fn add_chart(
        &mut self,
        chart_type: ChartType,
        pipeline_id: u32,
        config: ChartInstanceConfig,
    ) -> ChartResult<u32> {
        if self.charts.len() >= MAX_CHARTS {
            return Err(ChartError::DataError(DataError::BUFFER_FULL));
        }

        let chart_id = self.metrics.total_charts;
        let instance = ChartInstance {
            id: chart_id,
            chart_type,
            pipeline_id,
            config,
        };

        self.charts
            .push(instance)
            .map_err(|_| ChartError::DataError(DataError::BUFFER_FULL))?;

        self.metrics.total_charts += 1;
        self.metrics.active_charts += 1;

        Ok(chart_id)
    }

    /// Update all managed charts
    pub fn update(&mut self, delta_time: crate::time::Milliseconds) -> ChartResult<()> {
        let start_time = self.get_current_time_us();

        // Update synchronization state if enabled
        if self.config.enable_sync {
            self.update_sync_state(delta_time)?;
        }

        // Update metrics
        self.metrics.total_updates += 1;

        // Update performance metrics if monitoring is enabled
        if self.config.monitoring_level != MonitoringLevel::None {
            let end_time = self.get_current_time_us();
            self.update_manager_metrics(end_time - start_time);
        }

        Ok(())
    }

    /// Get manager metrics
    pub fn metrics(&self) -> &ManagerMetrics {
        &self.metrics
    }

    /// Get number of active charts
    pub fn active_chart_count(&self) -> usize {
        self.metrics.active_charts as usize
    }

    /// Get synchronization state
    pub fn sync_state(&self) -> &SyncState {
        &self.sync_state
    }

    /// Update synchronization state
    fn update_sync_state(&mut self, _delta_time: crate::time::Milliseconds) -> ChartResult<()> {
        // Placeholder for synchronization logic
        self.sync_state.last_sync_time = self.get_current_time_ms();
        Ok(())
    }

    /// Update manager performance metrics
    fn update_manager_metrics(&mut self, latency_us: u32) {
        // Update average latency using exponential moving average
        if self.metrics.avg_update_latency_us == 0 {
            self.metrics.avg_update_latency_us = latency_us;
        } else {
            self.metrics.avg_update_latency_us =
                (self.metrics.avg_update_latency_us * 7 + latency_us) / 8;
        }
    }

    /// Get current time in milliseconds (placeholder)
    fn get_current_time_ms(&self) -> u32 {
        self.metrics.total_updates as u32
    }

    /// Get current time in microseconds (placeholder)
    fn get_current_time_us(&self) -> u32 {
        self.get_current_time_ms() * 1000
    }
}

impl<const MAX_CHARTS: usize> Default for StreamingChartManager<MAX_CHARTS> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unified_streaming_buffer() {
        let mut buffer: UnifiedStreamingBuffer<10> = UnifiedStreamingBuffer::new();
        assert!(buffer.is_empty());
        assert_eq!(buffer.capacity(), 10);

        // Add some data points
        buffer.push(Point2D::new(1.0, 2.0)).unwrap();
        buffer.push(Point2D::new(2.0, 3.0)).unwrap();

        assert_eq!(buffer.len(), 2);
        assert!(!buffer.is_empty());

        // Test data retrieval
        let data: Vec<Point2D, 10> = buffer.data().collect();
        assert_eq!(data.len(), 2);
    }

    #[test]
    fn test_streaming_data_source() {
        let mut source: StreamingDataSource<5> = StreamingDataSource::new(1);
        assert_eq!(source.id(), 1);
        assert_eq!(source.state(), SourceState::Active);

        // Add data
        source.update(Point2D::new(1.0, 2.0)).unwrap();
        assert_eq!(source.data().count(), 1);

        // Test state changes
        source.set_state(SourceState::Paused);
        assert_eq!(source.state(), SourceState::Paused);

        // Should fail when not active
        let result = source.update(Point2D::new(2.0, 3.0));
        assert!(result.is_err());
    }

    #[test]
    fn test_streaming_chart_manager() {
        let mut manager: StreamingChartManager<5> = StreamingChartManager::new();
        assert_eq!(manager.active_chart_count(), 0);

        // Add a chart
        let chart_id = manager
            .add_chart(ChartType::Line, 1, ChartInstanceConfig::default())
            .unwrap();

        assert_eq!(chart_id, 0);
        assert_eq!(manager.active_chart_count(), 1);

        // Test update
        manager.update(16).unwrap(); // ~60 FPS
        assert!(manager.metrics().total_updates > 0);
    }

    #[test]
    fn test_streaming_config() {
        let config = StreamingConfig {
            buffer_capacity: 50,
            update_interval: 50,
            auto_prune: false,
            ..Default::default()
        };

        let buffer: UnifiedStreamingBuffer<50> = UnifiedStreamingBuffer::with_config(config);
        assert_eq!(buffer.config().buffer_capacity, 50);
        assert_eq!(buffer.config().update_interval, 50);
        assert!(!buffer.config().auto_prune);
    }

    #[test]
    fn test_performance_metrics() {
        let mut buffer: UnifiedStreamingBuffer<10> = UnifiedStreamingBuffer::new();

        // Add multiple points to generate metrics
        for i in 0..5 {
            buffer.push(Point2D::new(i as f32, (i * 2) as f32)).unwrap();
        }

        let metrics = buffer.metrics();
        assert_eq!(metrics.total_points, 5);
        assert_eq!(metrics.dropped_points, 0);
    }
}
