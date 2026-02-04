# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-02-05

### Added

- Initial release as web-spec (formerly web2markdown)
- Gherkin Feature Runner for browser automation
- Chromiumoxide and WebDriver backend support
- CLI commands: run, validate, list-steps, search-steps, export-schema, compare, debug, webhook, batch, alerts
- Multiple output formats: JSON, YAML, TAP, HTML, text
- Interactive debugging with breakpoints
- Webhook notifications (Slack, Discord, Teams, custom)
- Performance monitoring and alerts
- Batch execution with parallel processing
- Step catalog with 425+ patterns across 35+ categories

### Changed

- Renamed from web2markdown to web-spec
- Improved CLI output formatting
- Enhanced validation messages

### Features

- Full Gherkin .feature file support
- Browser automation (navigation, interaction, extraction)
- Result comparison for regression detection
- Dry-run mode for validation
- REPL debugging interface
