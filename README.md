# web-spec

A production-ready Gherkin Feature Runner with enterprise CI/CD integration, performance monitoring, and multi-channel notifications.

## Features

### Core Functionality
- **Gherkin Feature Execution**: Full support for `.feature` files with BDD syntax
- **Multiple Output Formats**: Text, JSON, YAML, HTML, TAP (Test Anything Protocol)
- **Step Validation**: Validates steps against registered patterns
- **Browser Automation**: WebDriver and Chromiumoxide backend support

### Advanced Features

#### Result Comparison
Compare test executions to detect:
- **Regressions**: Scenarios that now fail but previously passed
- **Improvements**: Performance gains or fixed scenarios
- **Metrics Changes**: Pass/fail rate differences

```bash
web-spec compare --baseline run1.json --current run2.json
```

#### Interactive Debugging
Step-through debugging with:
- **Breakpoints**: Pause at specific scenarios
- **REPL Commands**: continue, step, skip, repeat, info
- **Auto-step Mode**: Automatic step-through execution

```bash
web-spec debug --feature test.feature --breakpoint "Failing scenario"
```

#### Webhook Notifications
Multi-channel notifications:
- **Slack**: Rich message attachments with status
- **Discord**: Embedded messages with color coding
- **Microsoft Teams**: MessageCard format
- **Custom Webhooks**: Any HTTP endpoint

```bash
web-spec webhook --config webhooks.yml
```

#### Performance Alerts
Configurable thresholds with:
- **Duration Monitoring**: Scenario and step timing
- **Failure Rate Alerts**: Configurable percentage thresholds
- **Throughput Metrics**: Scenarios/steps per second
- **Severity Levels**: Warning, Critical, Info

```bash
web-spec alerts --config alerts.yml --enabled
```

#### Batch Execution
Parallel test execution with:
- **Rayon Parallelism**: Configurable worker threads
- **Progress Tracking**: Real-time execution status
- **Result Aggregation**: Combined metrics across features
- **Error Tolerance**: Continue on failure option

```bash
web-spec batch --path ./features --workers 4 --continue-on-failure
```

## Installation

### From Source

```bash
git clone https://github.com/anomalyco/web-spec.git
cd web-spec
cargo build --release
```

### With Nix

```bash
nix develop -c -- cargo build --release
```

## Quick Start

### Run a Feature File

```bash
# Text output (default)
web-spec run --feature login.feature

# JSON output
web-spec run --feature test.feature --format json --pretty

# HTML report
web-spec run --feature test.feature --format html --output report.html
```

### Validate Features

```bash
web-spec validate --feature feature.feature
```

### List Available Steps

```bash
# All steps
web-spec list-steps

# Search for steps
web-spec search-steps "click"
```

## Architecture

```
web-spec/
├── src/
│   ├── cli/           # Command-line interface
│   ├── execution/      # Result handling and output
│   ├── discovery/     # Feature and step discovery
│   ├── validation/    # Feature/step validation
│   ├── browser/       # Browser automation
│   ├── automation/    # High-level actions
│   └── converter/      # HTML to Markdown
├── features/           # BDD test scenarios
├── tests/            # Integration tests
└── docs/             # Documentation
```

## Command Reference

| Command | Description |
|---------|-------------|
| `run` | Execute a feature file |
| `validate` | Validate without execution |
| `compare` | Compare two executions |
| `debug` | Interactive debugging |
| `batch` | Run multiple features |
| `webhook` | Send notifications |
| `alerts` | Configure performance alerts |
| `list-steps` | List available steps |
| `search-steps` | Search step patterns |
| `export-schema` | Export step catalog |

See [CLI Commands Documentation](docs/CLI_COMMANDS.md) for complete reference.

## Configuration

### Webhooks (`webhooks.yml`)

```yaml
- url: https://hooks.slack.com/services/xxx
  name: slack-notifications
  events: [completion, failure]
  retry_count: 3
```

### Performance Alerts (`alerts.yml`)

```yaml
- name: performance-monitoring
  enabled: true
  thresholds:
    - name: slow_scenario
      metric: ScenarioDurationMs
      operator: GreaterThan
      value: 30000
      severity: Warning
```

### Batch Execution (`batch.yml`)

```yaml
parallel: true
max_workers: 4
timeout_seconds: 300
continue_on_failure: true
```

## Testing

### Unit Tests

```bash
cargo test --lib
```

### Integration Tests

```bash
cargo test
```

### BDD Tests

```bash
cargo test --test bdd_integration
cargo test --test bdd_comprehensive
```

## Development

### Building

```bash
cargo build
cargo build --release
cargo build --features chromiumoxide-backend
```

### Running Examples

```bash
cargo run --example <example_name>
```

## Requirements

- Rust 1.93+
- ChromeDriver (for WebDriver backend) or Chromium (for embedded backend)
- Optional: Nix for reproducible builds

## License

MIT License - see [LICENSE](LICENSE) file.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Add BDD tests for new functionality
4. Ensure all tests pass
5. Submit a pull request

## Support

- **Issues**: GitHub Issues
- **Documentation**: [docs/CLI_COMMANDS.md](docs/CLI_COMMANDS.md)
- **Examples**: [examples/](examples/)
