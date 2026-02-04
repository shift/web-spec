# Command Reference

Comprehensive documentation for all web-spec CLI commands.

## Table of Contents

- [Quick Start](#quick-start)
- [Core Commands](#core-commands)
- [Comparison Commands](#comparison-commands)
- [Debugging Commands](#debugging-commands)
- [Notification Commands](#notification-commands)
- [Batch Commands](#batch-commands)
- [Monitoring Commands](#monitoring-commands)
- [Configuration Files](#configuration-files)
- [Examples](#examples)

---

## Quick Start

```bash
# Run a single feature file
web-spec run --feature test.feature

# Compare two test results
web-spec compare --baseline baseline.json --current current.json

# Debug a feature interactively
web-spec debug --feature test.feature

# Run multiple features in parallel
web-spec batch --path ./features

# Send webhook notifications
web-spec webhook --config webhooks.yml

# Configure performance alerts
web-spec alerts --config alerts.yml
```

---

## Core Commands

### run

Execute a Gherkin feature file.

```bash
web-spec run --feature <file> [options]
```

**Options:**

| Flag | Description | Default |
|------|-------------|---------|
| `--feature, -f` | Path to feature file | Required |
| `--format, -o` | Output format (text, json, yaml, tap, html) | text |
| `--output, -O` | Output file path | stdout |
| `--pretty` | Pretty-print JSON/YAML output | false |
| `--dry-run` | Validate without executing | false |

**Examples:**

```bash
# Run with text output
web-spec run --feature login.feature

# Run with JSON output to file
web-spec run --feature test.feature --format json --output results.json --pretty

# Dry-run validation
web-spec run --feature test.feature --dry-run
```

**Output Formats:**

- `text`: Human-readable format with status icons
- `json`: Structured JSON for CI/CD pipelines
- `yaml`: YAML format for configuration management
- `tap`: Test Anything Protocol v13
- `html`: Rich HTML report with charts

---

### validate

Validate a Gherkin feature file without execution.

```bash
web-spec validate --feature <file> [options]
```

**Options:**

| Flag | Description | Default |
|------|-------------|---------|
| `--feature, -f` | Path to feature file | Required |
| `--format, -o` | Output format (text, json, yaml) | text |
| `--output, -O` | Output file path | stdout |

**Example:**

```bash
web-spec validate --feature login.feature --format json
```

---

### list-steps

List available step definitions.

```bash
web-spec list-steps [options]
```

**Options:**

| Flag | Description | Default |
|------|-------------|---------|
| `--category, -c` | Filter by category | All |
| `--search, -s` | Search pattern | None |
| `--format, -o` | Output format | text |
| `--output, -O` | Output file path | stdout |
| `--pretty` | Pretty-print JSON | false |

**Examples:**

```bash
# List all steps
web-spec list-steps

# Search for click steps
web-spec list-steps --search click

# Filter by browser category
web-spec list-steps --category browser
```

---

### export-schema

Export step catalog as JSON schema.

```bash
web-spec export-schema [options]
```

**Options:**

| Flag | Description | Default |
|------|-------------|---------|
| `--format, -o` | Output format (json, yaml) | json |
| `--output, -O` | Output file path | Required |
| `--pretty` | Pretty-print output | false |

**Example:**

```bash
web-spec export-schema --output schema.json --pretty
```

---

### search-steps

Search for steps matching a pattern.

```bash
web-spec search-steps <query> [options]
```

**Options:**

| Flag | Description | Default |
|------|-------------|---------|
| `query` | Search query | Required |
| `--category, -c` | Filter by category | All |
| `--format, -o` | Output format | text |
| `--output, -O` | Output file path | stdout |

**Example:**

```bash
web-spec search-steps "I click" --category browser
```

---

## Comparison Commands

### compare

Compare two test execution results to detect regressions and improvements.

```bash
web-spec compare --baseline <file> --current <file> [options]
```

**Options:**

| Flag | Description | Default |
|------|-------------|---------|
| `--baseline, -b` | Baseline result JSON | Required |
| `--current, -c` | Current result JSON | Required |
| `--format, -o` | Output format (text, json, yaml, html) | text |
| `--output, -O` | Output file path | stdout |
| `--pretty` | Pretty-print JSON/YAML | false |

**Comparison Metrics:**

- **Status**: REGRESSION, IMPROVEMENT, or IDENTICAL
- **Scenarios Changed**: Number of scenarios with different status
- **Duration Change**: Percentage change in execution time
- **Regressions**: Failed scenarios that previously passed
- **Improvements**: Scenarios that got faster or now pass

**Example:**

```bash
# Compare with text output
web-spec compare --baseline run1.json --current run2.json

# Compare with JSON output
web-spec compare --baseline baseline.json --current current.json \
  --format json --output comparison.json --pretty
```

**Sample Output:**

```
=== Test Result Comparison Report ===

Status: REGRESSION

--- Summary ---
Baseline: 2024-02-04T10:00:00Z
Current:  2024-02-04T11:00:00Z
Scenarios Changed: 1
Regressions Detected: 1
Improvements Detected: 0

--- Metrics Change ---
Passed Scenarios:  -1 (5 → 4)
Failed Scenarios:  +1 (0 → 1)
Duration:          +5.2%

--- Regressions (CRITICAL) ---
  1. Scenario 'User Login' changed from passed to failed
     Impact: Critical regression detected
```

---

## Debugging Commands

### debug

Interactive debugging mode for step-through test execution.

```bash
web-spec debug --feature <file> [options]
```

**Options:**

| Flag | Description | Default |
|------|-------------|---------|
| `--feature, -f` | Path to feature file | Required |
| `--scenario, -s` | Debug specific scenario | All |
| `--breakpoint, -b` | Set breakpoint for scenario | None |
| `--auto-step` | Auto-step through all steps | false |

**Debug Commands:**

| Command | Shortcut | Description |
|---------|----------|-------------|
| `continue` | `c` | Continue until next breakpoint |
| `step` | `n` | Execute current step and pause |
| `repeat` | `r` | Repeat current step |
| `skip` | `s` | Skip current step |
| `info` | `i` | Show current step information |
| `breakpoints` | `b` | List all breakpoints |
| `break <name>` | | Set breakpoint |
| `clear <name>` | | Clear breakpoint |
| `help` | `h` | Show help message |
| `quit` | `q` | Quit debugger |

**Examples:**

```bash
# Start interactive debugging
web-spec debug --feature login.feature

# Debug specific scenario
web-spec debug --feature test.feature --scenario "User login"

# Set breakpoint and continue
web-spec debug --feature test.feature --breakpoint "Failed login"

# Auto-step through all steps
web-spec debug --feature test.feature --auto-step
```

**Debug Session Example:**

```
=== BDD Debugger ===
Feature: User Authentication
Debugging mode enabled

Scenario: User Login

📍 Debugger - Scenario: 'User Login' - Step 1

Current Step: [Given] a browser is available
Status: pending

Debugger Commands:
  c, continue    - Continue execution until next breakpoint
  n, next, step  - Execute current step and pause
  r, repeat      - Repeat current step
  s, skip        - Skip current step
  i, info        - Show current step information
  b, breakpoints - List all breakpoints
  break <name>   - Set breakpoint for scenario
  h, help        - Show this help message
  q, quit        - Quit debugger and stop execution

(debugger) > n
  ✓ Given a browser is available
  [Given] the login page is displayed
```

---

## Notification Commands

### webhook

Send webhook notifications or test webhook configuration.

```bash
web-spec webhook --config <file> [options]
```

**Options:**

| Flag | Description | Default |
|------|-------------|---------|
| `--config, -c` | Webhook configuration file | Required |
| `--url, -u` | Test specific URL | Config |
| `--event` | Event type to test | completion |
| `--format, -o` | Output format (text, json) | text |

**Event Types:**

- `start` - Test execution started
- `completion` - Test execution completed
- `failure` - Tests failed
- `success` - All tests passed

**Webhook Configuration:**

```yaml
# webhooks.yml
- url: https://hooks.slack.com/services/xxx
  name: slack-notifications
  events: [completion, failure]
  headers:
    Authorization: Bearer xxx
  retry_count: 3

- url: https://discord.com/api/webhooks/xxx
  name: discord-alerts
  events: [failure]
  retry_count: 3
```

**Examples:**

```bash
# Test webhook configuration
web-spec webhook --config webhooks.yml

# Test specific event type
web-spec webhook --config webhooks.yml --event failure

# Test specific URL override
web-spec webhook --config webhooks.yml --url https://example.com/webhook
```

---

### alerts

Configure performance alerts and monitor execution metrics.

```bash
web-spec alerts --config <file> [options]
```

**Options:**

| Flag | Description | Default |
|------|-------------|---------|
| `--config, -c` | Alerts configuration file | Optional |
| `--enabled, -e` | Enable alerts mode | false |
| `--format, -o` | Output format (text, json, yaml) | text |
| `--output, -O` | Output file path | stdout |
| `--pretty` | Pretty-print JSON/YAML | false |

**Alert Configuration:**

```yaml
# alerts.yml
- name: performance-monitoring
  enabled: true
  thresholds:
    - name: slow_scenario
      metric: ScenarioDurationMs
      operator: GreaterThan
      value: 30000
      severity: Warning
      message: "Scenario exceeded {:.1}s duration"
    
    - name: very_slow_scenario
      metric: ScenarioDurationMs
      operator: GreaterThan
      value: 60000
      severity: Critical
      message: "Scenario exceeded {:.1}s duration"
    
    - name: high_failure_rate
      metric: FailureRatePercent
      operator: GreaterThan
      value: 10.0
      severity: Warning
      message: "Failure rate exceeded {:.1}%"
```

**Metrics:**

- `ScenarioDurationMs` - Average scenario duration
- `StepDurationMs` - Average step duration
- `FailureRatePercent` - Percentage of failed scenarios
- `TotalDurationMs` - Total execution time
- `ScenariosPerSecond` - Throughput metric

**Example:**

```bash
# Enable alerts with custom configuration
web-spec alerts --config alerts.yml --enabled

# View current alert configuration
web-spec alerts --format json
```

---

## Batch Commands

### batch

Execute multiple feature files in batch.

```bash
web-spec batch --path <dir> [options]
```

**Options:**

| Flag | Description | Default |
|------|-------------|---------|
| `--path, -p` | Directory or file | Required |
| `--format, -o` | Output format (text, json, yaml) | text |
| `--output, -O` | Output file path | stdout |
| `--sequential` | Run sequentially | parallel |
| `--workers` | Max parallel workers | CPU count |
| `--continue-on-failure` | Continue on error | false |
| `--pretty` | Pretty-print JSON/YAML | false |

**Examples:**

```bash
# Run all features in parallel (default)
web-spec batch --path ./features

# Run sequentially
web-spec batch --path ./features --sequential

# Limit to 2 workers
web-spec batch --path ./features --workers 2

# Continue on failure
web-spec batch --path ./features --continue-on-failure

# JSON output for CI/CD
web-spec batch --path ./features --format json --output results.json --pretty
```

**Sample Output:**

```
=== Batch Execution Summary ===

Features:  10 total, 9 passed, 1 failed
Scenarios:  50 total, 48 passed, 2 failed
Duration:   12500ms

=== Feature Results ===
✓ Feature 1 - passed (1200ms)
✓ Feature 2 - passed (1500ms)
✗ Feature 3 - failed (800ms)
    Failed: 2/5 scenarios

=== Errors ===
✗ feature3.feature - Step execution failed
```

---

## Monitoring Commands

### health

Check system health and dependencies.

```bash
web-spec health
```

Checks:
- Browser drivers availability
- Configuration validity
- Step definitions loaded
- Memory usage

---

### version

Display version information.

```bash
web-spec version
```

Output includes:
- Version number
- Git commit hash
- Build date
- Rust version

---

## Configuration Files

### Webhook Configuration

**File:** `webhooks.yml`

```yaml
# Webhook endpoints for notifications
- url: <webhook-url>
  name: <identifier>
  events:
    - start
    - completion
    - failure
    - success
  headers:
    Authorization: Bearer <token>
  retry_count: 3
  timeout_seconds: 30
```

### Alert Configuration

**File:** `alerts.yml`

```yaml
# Performance alert thresholds
- name: <config-name>
  enabled: true
  thresholds:
    - name: <threshold-name>
      metric: <MetricType>
      operator: <Operator>
      value: <number>
      severity: <Severity>
      message: <format-string>
```

### Batch Configuration

**File:** `batch.yml`

```yaml
# Batch execution settings
parallel: true
max_workers: 4
timeout_seconds: 300
continue_on_failure: true
output_format: text
```

---

## Examples

### CI/CD Pipeline Integration

```bash
#!/bin/bash
# CI/CD pipeline script

# Run tests
echo "Running feature tests..."
web-spec batch --path ./features \
  --format json \
  --output test_results.json \
  --continue-on-failure

# Check results
if [ $? -eq 0 ]; then
    echo "Tests passed"
    
    # Send success notification
    web-spec webhook --config webhooks.yml --event success
else
    echo "Tests failed"
    
    # Send failure notification
    web-spec webhook --config webhooks.yml --event failure
    
    # Compare with baseline
    web-spec compare --baseline previous_results.json \
      --current test_results.json \
      --format json \
      --output comparison.json
    
    exit 1
fi
```

### Performance Monitoring

```bash
#!/bin/bash
# Performance monitoring script

# Run with alerts
web-spec alerts --config performance_alerts.yml --enabled

# Run batch with performance tracking
web-spec batch --path ./features \
  --format json \
  --output performance_results.json

# Analyze performance
echo "Performance analysis complete"
cat performance_results.json | jq '.summary'
```

### Debugging Workflow

```bash
# Start debugging
web-spec debug --feature login.feature

# Set breakpoint
(debugger) > break "User login"

# Continue to breakpoint
(debugger) > c

# Step through failing test
(debugger) > n
(debugger) > info
(debugger) > n

# Repeat failing step
(debugger) > r

# Quit and investigate
(debugger) > q
```

---

## Exit Codes

| Code | Description |
|------|-------------|
| 0 | Success |
| 1 | General error |
| 2 | Invalid arguments |
| 3 | Feature file not found |
| 4 | Validation failed |
| 5 | Execution failed |
| 6 | Webhook delivery failed |

---

## Environment Variables

| Variable | Description |
|----------|-------------|
| `WEB2MARKDOWN_CONFIG` | Default config file |
| `WEB2MARKDOWN_TIMEOUT` | Default timeout (seconds) |
| `WEB2MARKDOWN_WORKERS` | Default worker count |
| `WEB2MARKDOWN_LOG_LEVEL` | Log level (debug, info, warn, error) |

---

## Support

- **Issues:** https://github.com/anomalyco/web-spec/issues
- **Documentation:** https://web-spec.ai/docs
- **Discord:** https://discord.gg/web-spec
