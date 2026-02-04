//! Command-line argument definitions using clap
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "web-spec",
    author,
    version,
    about = "Flexible and Extensible Gherkin Feature Runner for browser automation",
    long_about = None
)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Legacy: Gherkin feature file to run (kept for backwards compatibility)
    #[arg(short, long)]
    pub feature: Option<PathBuf>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Run a Gherkin feature file
    Run {
        /// Path to the feature file
        #[arg(short, long)]
        feature: PathBuf,

        /// Output format (text, json, yaml, yml, tap, html)
        #[arg(long, default_value = "text")]
        format: String,

        /// Output file path (if not specified, prints to stdout)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Pretty-print JSON output
        #[arg(long)]
        pretty: bool,

        /// Dry-run mode: validate without executing
        #[arg(long)]
        dry_run: bool,
    },

    /// Validate a Gherkin feature file
    Validate {
        /// Path to the feature file
        #[arg(short, long)]
        feature: PathBuf,

        /// Output format (text, json, yaml, yml, tap, html)
        #[arg(long, default_value = "text")]
        format: String,

        /// Output file path (if not specified, prints to stdout)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// List available steps
    ListSteps {
        /// Filter by category
        #[arg(short, long)]
        category: Option<String>,

        /// Search for steps matching a query
        #[arg(short, long)]
        search: Option<String>,

        /// Output format (text, json, yaml, yml, tap, html)
        #[arg(long, default_value = "text")]
        format: String,

        /// Output file path (if not specified, prints to stdout)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Pretty-print JSON output
        #[arg(long)]
        pretty: bool,
    },

    /// Export step catalog as JSON schema
    ExportSchema {
        /// Output format (json, yaml, yml, tap)
        #[arg(short, long, default_value = "json")]
        format: String,

        /// Output file path (required for export)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Pretty-print JSON output
        #[arg(long)]
        pretty: bool,
    },

    /// Search for steps matching a pattern
    SearchSteps {
        /// Search query
        #[arg(value_name = "QUERY")]
        query: String,

        /// Filter by category
        #[arg(short, long)]
        category: Option<String>,

        /// Output format (text, json, yaml, yml, tap, html)
        #[arg(long, default_value = "text")]
        format: String,

        /// Output file path (if not specified, prints to stdout)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Compare two test execution results
    Compare {
        /// Path to baseline execution result (JSON file)
        #[arg(short, long)]
        baseline: PathBuf,

        /// Path to current execution result (JSON file)
        #[arg(short, long)]
        current: PathBuf,

        /// Output format (text, json, yaml, yml, html)
        #[arg(long, default_value = "text")]
        format: String,

        /// Output file path (if not specified, prints to stdout)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Pretty-print JSON output
        #[arg(long)]
        pretty: bool,
    },

    /// Debug a Gherkin feature file with interactive step-through
    Debug {
        /// Path to the feature file
        #[arg(short, long)]
        feature: PathBuf,

        /// Scenario name to debug (optional - debugs all if not specified)
        #[arg(short, long)]
        scenario: Option<String>,

        /// Set breakpoint at scenario name
        #[arg(long)]
        breakpoint: Option<String>,

        /// Enable auto-step mode (step through each step)
        #[arg(long)]
        auto_step: bool,
    },

    /// Send webhook notifications or test webhook configuration
    Webhook {
        /// Path to webhook configuration file (YAML)
        #[arg(short, long)]
        config: PathBuf,

        /// Webhook URL to test (overrides config URL)
        #[arg(short, long)]
        url: Option<String>,

        /// Event type to test (start, completion, failure, success)
        #[arg(long, default_value = "completion")]
        event: String,

        /// Output format (text, json)
        #[arg(long, default_value = "text")]
        format: String,
    },

    /// Execute multiple feature files in batch
    Batch {
        /// Path to directory or feature file
        #[arg(short, long)]
        path: PathBuf,

        /// Output format (text, json, yaml)
        #[arg(long, default_value = "text")]
        format: String,

        /// Output file path (if not specified, prints to stdout)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Run features sequentially (default: parallel)
        #[arg(long)]
        sequential: bool,

        /// Maximum number of parallel workers
        #[arg(long)]
        workers: Option<usize>,

        /// Continue execution even if some features fail
        #[arg(long)]
        continue_on_failure: bool,

        /// Pretty-print JSON/YAML output
        #[arg(long)]
        pretty: bool,
    },

    /// Configure performance alerts and monitor execution metrics
    Alerts {
        /// Path to alerts configuration file (YAML)
        #[arg(short, long)]
        config: Option<PathBuf>,

        /// Enable alerts mode (default thresholds if no config)
        #[arg(long)]
        enabled: bool,

        /// Output format (text, json, yaml)
        #[arg(long, default_value = "text")]
        format: String,

        /// Output file path (if not specified, prints to stdout)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Pretty-print JSON/YAML output
        #[arg(long)]
        pretty: bool,
    },
}

impl Args {
    /// Get the effective command to run
    pub fn get_command(&self) -> Option<&Commands> {
        self.command.as_ref()
    }

    /// Check if this is legacy mode (feature flag without subcommand)
    pub fn is_legacy_mode(&self) -> bool {
        self.command.is_none() && self.feature.is_some()
    }
}
