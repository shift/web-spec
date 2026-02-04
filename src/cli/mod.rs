//! CLI module for web-spec
//! Provides command-line interface for gherkin feature running and step discovery

pub mod args;
pub mod commands;
pub mod output;

pub use args::{Args, Commands};
pub use commands::{
    handle_export_schema, handle_list_steps, handle_search_steps, handle_validate_feature,
};
pub use output::{format_output, write_output};
