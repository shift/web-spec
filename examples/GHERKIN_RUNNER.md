# Gherkin Feature Runner

A CLI tool for running Gherkin feature files with web-spec using a real browser.

## Usage

```bash
cargo run --example gherkin_runner --features chromiumoxide-backend -- --feature examples/hacker_news.feature
```

## Feature File Format

Gherkin feature files use the standard Given-When-Then syntax:

```gherkin
Feature: Extract Top 10 Hacker News Posts

  Scenario: Extract top 10 post titles from Hacker News
    Given a browser backend is available
    When I navigate to Hacker News
    And the page loads
    And I extract post titles from the page
    Then I should see at least 10 post titles
    And each title has meaningful text
```

## Output

The runner will:
1. Parse the Gherkin feature file
2. Execute each step using a real browser
3. Display step-by-step results with checkmarks
4. Show extracted data at the end

Example output:
```
=== Gherkin Feature Runner ===
Running feature file: "examples/hacker_news.feature"

Feature: Extract Top 10 Hacker News Posts
1 scenario(s)

  Scenario: Extract top 10 post titles from Hacker News
    Given a browser backend is available ... ✓
    When I navigate to Hacker News ... ✓
    When the page loads ... ✓
    When I extract post titles from the page ... ✓
      Extracted 30 post titles
    Then I should see at least 10 post titles ... ✓
    Then each title has meaningful text ... ✓
    ✓ Scenario PASSED

=== Top 10 Hacker News Posts ===

From scenario 'Extract top 10 post titles from Hacker News':
  1. Agent Skills
  2. What's up with all those equals signs anyway?
  ...

=== Feature Run Complete ===
```

## CLI Options

- `--feature, -f <path>` - Path to the Gherkin feature file to run (required)

## Supported Steps

Currently supported step patterns:
- "a browser backend is available"
- "I navigate to Hacker News"
- "the page loads"
- "I extract post titles from the page"
- "I should see at least X post titles"
- "each title has meaningful text"

You can add more step patterns by extending the `execute_step` function in `gherkin_runner.rs`.
