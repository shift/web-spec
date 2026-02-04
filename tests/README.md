# BDD Tests for web-spec crate

This directory contains BDD-style tests for the web-spec crate. Tests follow Given-When-Then structure and can serve as both tests and documentation.

## Running Tests

### BDD Integration Tests
```bash
# Run all BDD tests (requires no external services)
nix develop -c -- cargo test --test bdd_integration

# Run specific BDD test
nix develop -c -- cargo test --test bdd_integration given_html_with_links

# Run Hacker News BDD tests (unit tests only, no browser required)
nix develop -c -- cargo test --test hacker_news_bdd
```

### Feature Files (Gherkin Format)

- `browser-navigation.feature` - Scenarios for browser navigation and HTML extraction
- `basic-browser.feature` - Basic browser type validation scenarios
- `hacker-news-extraction.feature` - Scenarios for extracting post titles from Hacker News

## BDD Test Structure

Each test follows the pattern:
- **GIVEN**: Setup initial state
- **WHEN**: Perform the action being tested
- **THEN**: Verify the expected outcome

Example test names follow this pattern:
- `given_browser_type_when_created_then_matches_expected_type`
- `given_html_page_when_converted_to_markdown_then_output_is_non_empty`

### Current Test Suites

### Generic Steps Tests
- Comprehensive reusable step definitions for BDD testing
- 20+ generic steps covering common operations
- See `GENERIC_STEPS.md` for full documentation

### HTML Conversion Tests
- Tests basic HTML to markdown conversion
- Validates headings, paragraphs, code blocks, and links

### Browser Type Tests
- Validates browser type enumeration
- Ensures WebDriver and Chromiumoxide types are properly defined

### Browser Navigation Tests
- Full browser navigation scenarios (ignored by default, require actual browser servers)
- Tests navigation, HTML extraction, and page load waiting

### Hacker News Extraction Tests
- Tests extraction of post titles from Hacker News
- Validates regex-based HTML parsing
- Tests HTML entity decoding
- Tests formatting of extracted titles
- Includes mocked HTML tests (runnable without browser)

## Hacker News Extraction

The `hacker_news_bdd` test suite includes:

1. **Post Title Extraction**: Extracts titles from Hacker News HTML using regex
2. **Entity Decoding**: Properly decodes HTML entities like `&amp;`, `&#x27;`
3. **Mock Tests**: Unit tests with mock HTML to verify extraction logic
4. **Integration Tests**: End-to-end tests (ignored, require actual browser servers)

**Hacker News HTML Structure**:
- Posts are in `<tr class="athing">` elements
- Title link: `<a class="titlelink" href="...">Title</a>`
- Titles may contain HTML entities that need decoding

## Writing New BDD Tests

1. Create a test function with BDD-style naming:
   ```rust
   #[test]
   fn given_condition_when_action_then_expected_outcome() {
       // GIVEN: Setup
       // WHEN: Action
       // THEN: Assertion
   }
   ```

2. Add corresponding Gherkin scenario in `tests/features/*.feature`
3. Run tests to verify

## Example Gherkin Scenario

```gherkin
Feature: Browser Navigation
  As an agentic coding agent
  I want to navigate to websites
  So that I can perform web searches

  Scenario: Navigate to a webpage
    Given I am using a WebDriver browser
    When I navigate to a URL
    And I wait for the page to load
    Then I should be able to extract HTML
```

### Feature Files (Gherkin Format)

- `browser-navigation.feature` - Browser navigation and HTML extraction scenarios
- `basic-browser.feature` - Basic browser type validation scenarios

## BDD Test Structure

Each test follows the pattern:
- **GIVEN**: Setup the initial state
- **WHEN**: Perform the action being tested
- **THEN**: Verify the expected outcome

Example test names follow this pattern:
- `given_browser_type_when_created_then_matches_expected_type`
- `given_html_page_when_converted_to_markdown_then_output_is_non_empty`

## Current Test Suites

### HTML Conversion Tests
- Tests basic HTML to markdown conversion
- Validates headings, paragraphs, code blocks, and links

### Browser Type Tests
- Validates browser type enumeration
- Ensures WebDriver and Chromiumoxide types are properly defined

### Browser Navigation Tests
- Full browser navigation scenarios (ignored by default, require actual browser servers)
- Tests navigation, HTML extraction, and page load waiting

## Writing New BDD Tests

1. Create a test function with BDD-style naming:
   ```rust
   #[test]
   fn given_condition_when_action_then_expected_outcome() {
       // GIVEN: Setup
       // WHEN: Action
       // THEN: Assertion
   }
   ```

2. Add corresponding Gherkin scenario in `tests/features/*.feature`
3. Run tests to verify
4. Document in this README

## Example Gherkin Scenario

```gherkin
Feature: Browser Navigation
  As an agentic coding agent
  I want to navigate to websites
  So that I can perform web searches

  Scenario: Navigate to a webpage
    Given I am using a WebDriver browser
    When I navigate to a URL
    And I wait for the page to load
    Then I should be able to extract HTML
```


### Integration Tests (requires WebDriver or Chromium)
```bash
# WebDriver (requires ChromeDriver on localhost:4444)
nix develop -c -- cargo test --test browser_integration

# Chromiumoxide
nix develop -c -- cargo test --test browser_integration --features chromiumoxide-backend
```

## Feature Files

- `basic-browser.feature` - Basic browser type validation
- `browser-navigation.feature` - Full browser navigation and extraction scenarios

## Step Definitions

- `basic_steps.rs` - Step definitions for basic tests
- `browser_integration.rs` - Step definitions for integration tests

## Writing New Scenarios

1. Add scenarios to `.feature` files in `tests/features/`
2. Implement step definitions in the appropriate `.rs` file using `given`, `when`, `then` macros
3. Re-run tests

## Example Scenario

```gherkin
Feature: New Feature
  As a user
  I want to do something
  So that I get benefit

  Scenario: Simple test
    Given some condition
    When I do something
    Then something should happen
```
