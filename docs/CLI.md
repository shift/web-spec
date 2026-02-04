# CLI Documentation

The web-spec CLI provides commands for executing, validating, and managing BDD feature files for web automation.

## Installation

Build with the chromiumoxide backend to enable full CLI functionality:

```bash
cargo build --features chromiumoxide-backend
```

Run commands via:

```bash
cargo run --features chromiumoxide-backend -- <command> [options]
```

Or after installation:

```bash
web-spec <command> [options]
```

## Commands

### run

Execute a feature file and capture test results.

**Usage:**
```bash
web-spec run --feature <FEATURE_FILE> [OPTIONS]
```

**Options:**
- `--feature <FEATURE_FILE>` (required): Path to the feature file to execute
- `--format <FORMAT>`: Output format (text, json). Default: text
- `--pretty`: Pretty-print JSON output (ignored for text format)
- `-o, --output <OUTPUT>`: Write output to file instead of stdout

**Examples:**

Basic execution with text output:
```bash
web-spec run --feature tests/features/login.feature
```

Output as JSON:
```bash
web-spec run --feature tests/features/login.feature --format json
```

Pretty-printed JSON saved to file:
```bash
web-spec run --feature tests/features/login.feature --format json --pretty -o results.json
```

Save text output to file:
```bash
web-spec run --feature tests/features/login.feature -o results.txt
```

**Output Formats:**

Text (default):
```
=== Execution Report ===

Feature: Login Flow
File: tests/features/login.feature
Status: passed
Duration: 5234ms
Timestamp: 2026-02-04 14:30:00

Scenarios: 2

  1. Valid Credentials [passed]
     Duration: 2456ms
     ✓ 1. Given I navigate to the login page
     ✓ 2. When I enter valid credentials
     ✓ 3. Then I should see the dashboard

  2. Invalid Credentials [failed]
     Duration: 1200ms
     ✓ 1. Given I navigate to the login page
     ✗ 2. When I enter invalid credentials
        Error: Element not found: input[name="password"]

=== Summary ===
Scenarios: 1 passed, 1 failed, 0 skipped (total: 2)
Steps: 5 passed, 1 failed, 0 skipped (total: 6)
```

JSON:
```json
{
  "status": "passed",
  "timestamp": "2026-02-04 14:30:00",
  "duration_ms": 5234,
  "feature": {
    "name": "Login Flow",
    "file": "tests/features/login.feature",
    "description": null
  },
  "scenarios": [
    {
      "name": "Valid Credentials",
      "status": "passed",
      "duration_ms": 2456,
      "steps": [
        {
          "text": "I navigate to the login page",
          "keyword": "Given",
          "status": "passed",
          "duration_ms": 100,
          "output": null,
          "error": null
        }
      ]
    }
  ],
  "summary": {
    "total_scenarios": 2,
    "passed_scenarios": 1,
    "failed_scenarios": 1,
    "skipped_scenarios": 0,
    "total_steps": 6,
    "passed_steps": 5,
    "failed_steps": 1,
    "skipped_steps": 0
  }
}
```

---

### validate

Validate a feature file for syntax errors and step pattern matching.

**Usage:**
```bash
web-spec validate --feature <FEATURE_FILE> [OPTIONS]
```

**Options:**
- `--feature <FEATURE_FILE>` (required): Path to the feature file to validate
- `--format <FORMAT>`: Output format (text, json). Default: text
- `-o, --output <OUTPUT>`: Write validation results to file

**Examples:**

Validate feature file:
```bash
web-spec validate --feature tests/features/login.feature
```

Validate and save results as JSON:
```bash
web-spec validate --feature tests/features/login.feature --format json -o validation_results.json
```

**Output:**

Valid file:
```
✓ Feature syntax is valid
✓ All 6 steps match known patterns

Summary:
- Total steps: 6
- Matched steps: 6
- Unknown steps: 0
```

Invalid file:
```
✗ Validation failed:

Unknown steps in "Login Test":
- "I do something undefined" (line 8) - No matching pattern found

Suggestions:
- Check the step definition syntax
- Ensure you're using exact keywords (Given, When, Then, And, But)
```

---

### list-steps

List all available step patterns, optionally filtered by category or search term.

**Usage:**
```bash
web-spec list-steps [OPTIONS]
```

**Options:**
- `-c, --category <CATEGORY>`: Filter by step category (Navigation, Click, Form, etc.)
- `-s, --search <SEARCH>`: Search for steps by pattern
- `--format <FORMAT>`: Output format (text, json). Default: text
- `--pretty`: Pretty-print JSON output
- `-o, --output <OUTPUT>`: Write to file

**Examples:**

List all available steps:
```bash
web-spec list-steps
```

Filter by category:
```bash
web-spec list-steps --category Navigation
```

Search for steps:
```bash
web-spec list-steps --search click
```

Export as JSON:
```bash
web-spec list-steps --format json --pretty -o steps.json
```

List Navigation steps and save to file:
```bash
web-spec list-steps --category Navigation -o navigation_steps.txt
```

**Output (text format):**

```
Available Step Patterns (174 steps)

Navigation (15 steps):
  I navigate to "[URL]"
  I go to "[URL]"
  I visit "[URL]"
  I open "[URL]"
  I click on the "[TEXT]" link
  I follow "[TEXT]"
  I go back
  I go forward
  I refresh the page
  I reload the page
  I return to the home page
  I go to the home page
  I navigate to the home page
  I navigate back
  I navigate forward

Click (20 steps):
  I click on "[ELEMENT]"
  I click "[ELEMENT]"
  I click the "[TEXT]" button
  ...more steps...

Form (18 steps):
  I fill "[FIELD]" with "[VALUE]"
  I enter "[VALUE]" in "[FIELD]"
  ...more steps...
```

**Output (JSON format):**

```json
{
  "total_steps": 174,
  "categories": [
    {
      "name": "Navigation",
      "count": 15,
      "steps": [
        {
          "id": "nav_1",
          "pattern": "I navigate to \"[URL]\"",
          "description": "Navigate to a specific URL",
          "category": "Navigation",
          "examples": ["I navigate to \"https://example.com\""]
        }
      ]
    }
  ]
}
```

---

### search-steps

Search for specific step patterns by keyword.

**Usage:**
```bash
web-spec search-steps <QUERY> [OPTIONS]
```

**Arguments:**
- `<QUERY>`: Search term or pattern

**Options:**
- `-c, --category <CATEGORY>`: Limit search to specific category
- `--format <FORMAT>`: Output format (text, json). Default: text
- `-o, --output <OUTPUT>`: Write results to file

**Examples:**

Search for "click" steps:
```bash
web-spec search-steps click
```

Search for "submit" in Form category:
```bash
web-spec search-steps submit --category Form
```

Export matching steps as JSON:
```bash
web-spec search-steps wait --format json -o wait_steps.json
```

**Output:**

```
Search Results: "click" (5 matches)

1. I click on "[ELEMENT]"
   Category: Click
   ID: click_1

2. I click "[ELEMENT]"
   Category: Click
   ID: click_2

3. I click the "[TEXT]" button
   Category: Click
   ID: click_button

4. I double click on "[ELEMENT]"
   Category: Click
   ID: double_click

5. I right click on "[ELEMENT]"
   Category: Click
   ID: right_click
```

---

### export-schema

Export the complete step catalog as a JSON schema.

**Usage:**
```bash
web-spec export-schema [OPTIONS]
```

**Options:**
- `--format <FORMAT>`: Output format (json). Default: json
- `--pretty`: Pretty-print the output
- `-o, --output <OUTPUT>`: Write to file (default: stdout)

**Examples:**

Export schema to stdout:
```bash
web-spec export-schema
```

Pretty-print and save to file:
```bash
web-spec export-schema --pretty -o step_schema.json
```

**Output:**

```json
{
  "version": "1.0",
  "total_steps": 174,
  "categories": [
    {
      "name": "Navigation",
      "description": "Steps for navigating between pages and URLs",
      "steps": [
        {
          "id": "nav_1",
          "pattern": "I navigate to \"[URL]\"",
          "keywords": ["Given", "When"],
          "parameters": ["URL"],
          "category": "Navigation",
          "description": "Navigate to a specific URL"
        }
      ]
    }
  ],
  "metadata": {
    "generated": "2026-02-04T14:30:00Z",
    "backend": "chromiumoxide"
  }
}
```

---

## Common Workflows

### CI/CD Integration

Run tests and save results as JSON for processing:

```bash
# Run tests and save results
web-spec run --feature tests/features/critical_path.feature \
  --format json --pretty -o test_results.json

# Check exit status
if [ $? -eq 0 ]; then
  echo "Tests passed"
else
  echo "Tests failed"
fi

# Parse results with jq
cat test_results.json | jq '.summary'
```

### Validate All Features

```bash
#!/bin/bash
for feature in tests/features/*.feature; do
  echo "Validating $feature..."
  web-spec validate --feature "$feature"
  if [ $? -ne 0 ]; then
    echo "Validation failed for $feature"
    exit 1
  fi
done
echo "All features validated successfully"
```

### Generate Test Report

```bash
# Run tests and generate HTML report
web-spec run --feature tests/features/login.feature \
  --format json --pretty -o report.json

# Convert JSON to HTML (use external tool like jq or python)
jq -r '.summary | "Passed: \(.passed_scenarios)/\(.total_scenarios)"' report.json
```

### Step Documentation

```bash
# List all steps by category
web-spec list-steps --category Navigation -o docs/navigation_steps.txt
web-spec list-steps --category Click -o docs/click_steps.txt
web-spec list-steps --category Form -o docs/form_steps.txt

# Export complete schema
web-spec export-schema --pretty -o docs/step_schema.json
```

---

## Step Categories

The CLI supports the following step categories:

- **Navigation**: URL navigation, page switching, redirects
- **Click**: Element clicking, button presses, link following
- **Wait**: Timeout-based waiting, element visibility
- **Form**: Input filling, selection, submission
- **Assertion**: Validation and verification steps
- **Mouse**: Advanced mouse interactions (double-click, right-click)
- **Text**: Text extraction and verification
- **Screenshot**: Image capture and comparison
- **Custom**: User-defined steps

---

## Exit Codes

- `0`: Command succeeded
- `1`: Command failed or validation error
- `2`: Invalid arguments or syntax error
- `3`: Feature file not found or read error

---

## Environment Variables

- `RUST_LOG`: Set log level (error, warn, info, debug, trace)

Example:
```bash
RUST_LOG=debug web-spec run --feature test.feature
```

---

## Troubleshooting

### Browser Connection Issues

If you see "Creating browser config..." but no progress:

1. Ensure Chromium is installed: `chromium --version`
2. Check available system resources (RAM, disk space)
3. Try running with verbose logging: `RUST_LOG=debug web-spec run --feature test.feature`

### Step Not Recognized

If a step is not recognized:

1. Check the exact step text in your feature file
2. Use `web-spec search-steps` to find matching patterns
3. Verify keywords (Given, When, Then, And, But)
4. Check for quotes and special characters

Example:
```bash
# If this fails:
# Given I click on "Login"

# Search for available click steps:
web-spec search-steps click
```

### File Not Found

Ensure the feature file path is correct and relative to the working directory:

```bash
# Wrong (relative path):
web-spec run --feature features/login.feature

# Correct (full path or correct relative path):
web-spec run --feature ./tests/features/login.feature
# or
web-spec run --feature /home/user/project/tests/features/login.feature
```

---

## Performance Tips

1. **Parallel Execution**: Run multiple feature files in separate processes
2. **Selective Testing**: Use search/filter to run only relevant steps
3. **Output Caching**: Save JSON results for analysis without re-running tests
4. **Timeout Settings**: Adjust wait durations based on network conditions

---

## Examples Repository

See `examples/` directory for working examples of CLI usage and feature files.

