// Execution result types for JSON output
pub mod alerts;
pub mod batch;
pub mod comparison;
pub mod comparison_output;
pub mod debug;
pub mod html_output;
pub mod json_output;
pub mod profiling;
pub mod result;
pub mod tap_output;
pub mod text_output;
pub mod webhook;
pub mod yaml_output;

pub use alerts::{
    AlertConfig, AlertManager, AlertSeverity, AlertThreshold, PerformanceAlert, PerformanceMonitor,
    PerformanceSummary,
};
pub use batch::{
    BatchConfig, BatchError, BatchExecutor, BatchProgress, BatchResult, FeatureResult,
};
pub use comparison::{ComparisonResult, compare_results};
pub use comparison_output::to_text_output as comparison_to_text_output;
pub use debug::{DebugCommand, Debugger, ExecutionSnapshot, ExecutionState};
pub use html_output::to_html_output;
pub use json_output::{to_json_output, to_json_output_pretty};
pub use profiling::{ProfilingMetrics, analyze_execution};
pub use result::{
    ErrorInfo, ExecutionResult, ExecutionSummary, FeatureInfo, ScenarioResult, StepResult,
};
pub use tap_output::{TapSummary, parse_tap_output, to_tap_output};
pub use text_output::to_text_output;
pub use webhook::{WebhookConfig, WebhookError, WebhookEvent, WebhookManager};
pub use yaml_output::to_yaml_output;
