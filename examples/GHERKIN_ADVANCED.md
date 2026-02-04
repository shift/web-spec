# Advanced Gherkin Feature Runner

A comprehensive CLI tool for running Gherkin feature files with web-spec, supporting advanced website navigation patterns.

## Usage

```bash
cargo run --example gherkin_runner --features chromiumoxide-backend -- --feature <feature-file.feature>
```

## Supported Step Patterns

### Browser Initialization
- `a browser is available`
- `a browser backend is available`

### Navigation
- `I navigate to "URL"`
- `I go to "URL"`
- `I navigate to Hacker News`
- `the page loads`
- `I wait for page to load`

### Waiting
- `I wait X seconds`
- `I wait X milliseconds`
- `I wait for element "selector" to be visible`
- `I wait for element "selector" to appear`

### Clicking
- `I click on "selector"`
- `I click on "selector" button`
- `I click on "selector" link`
- `I click the submit button`
- `I double click on "selector"`
- `I right click on "selector"`
- `I hover over "selector"`

### Form Input
- `I type "text" into "selector"`
- `I enter "text" in "selector"`
- `I fill in "selector" with "text"`
- `I clear "selector"`
- `I select "value" from "selector"`

### Scrolling
- `I scroll to bottom`
- `I scroll to top`
- `I scroll to "selector"`
- `I scroll by X pixels down`
- `I scroll by X pixels up`

### Element Visibility
- `I should see "selector"`
- `I should not see "selector"`
- `"selector" should be visible`

### Text Validation
- `I should see text "expected text"`
- `the text of "selector" should be "expected text"`

### Content Extraction
- `I extract page HTML`
- `I extract all links from page`
- `I extract all images from page`
- `I extract all h1 headings`
- `I extract all h2 headings`
- `I extract all h3 headings`
- `I extract all h4 headings`
- `I extract all h5 headings`
- `I extract all h6 headings`
- `I extract post titles from page` (Hacker News specific)

### Counting
- `I should see X "selector" elements`

### Screenshots
- `I take a screenshot "filename.png"`

### Assertions
- `I should see at least X` (generic count assertion)
- `each title has meaningful text` (Hacker News specific)

### JavaScript Execution
- `I execute JavaScript "code"`

### Attribute Checking
- `the "attribute" attribute of "selector" should be "value"`

## Example Features

### Basic Navigation (`examples/advanced_navigation.feature`)
```gherkin
Feature: Advanced Website Navigation

  Scenario: Navigate and interact with forms
    Given a browser is available
    When I navigate to "https://example.com"
    And I wait for page to load
    And I wait for element "input[name=search]" to be visible
    And I type "Hello World" into "input[name=search]"
    And I click on "button[type=submit]"
    And I wait 2 seconds
    Then I should see "search results"
```

### E-commerce (`examples/ecommerce.feature`)
```gherkin
Feature: E-commerce Shopping Flow

  Scenario: Browse products and add to cart
    Given a browser is available
    When I navigate to "https://example-shop.com/products"
    And I wait for page to load
    And I should see ".product-grid"
    When I hover over ".product-card:first-child"
    And I wait 1 second
    And I click on ".product-card:first-child button.add-to-cart"
    And I wait 2 seconds
    Then I should see ".cart-count"
    And text of ".cart-count" should be "1"
```

### Social Media (`examples/social_media.feature`)
```gherkin
Feature: Social Media Interaction

  Scenario: Login and post content
    Given a browser is available
    When I navigate to "https://example-social.com/login"
    And I wait for page to load
    When I type "user@example.com" into "#username"
    And I type "password123" into "#password"
    And I click on "button.login"
    And I wait 3 seconds
    Then I should see ".dashboard"
    And I should not see ".error-message"
```

### Data Extraction (`examples/data_extraction.feature`)
```gherkin
Feature: Data Extraction and Scraping

  Scenario: Extract all links from page
    Given a browser is available
    When I navigate to "https://example.com"
    And I wait for page to load
    When I extract all links from page
    Then I should see at least 1 links

  Scenario: Hacker News title extraction
    Given a browser is available
    When I navigate to Hacker News
    And I wait for page to load
    And I extract post titles from page
    Then I should see at least 10
    And each title has meaningful text
```

## Running Examples

```bash
# Advanced navigation
cargo run --example gherkin_runner --features chromiumoxide-backend -- --feature examples/advanced_navigation.feature

# E-commerce flow
cargo run --example gherkin_runner --features chromiumoxide-backend -- --feature examples/ecommerce.feature

# Social media interaction
cargo run --example gherkin_runner --features chromiumoxide-backend -- --feature examples/social_media.feature

# Data extraction
cargo run --example gherkin_runner --features chromiumoxide-backend -- --feature examples/data_extraction.feature

# Hacker News extraction
cargo run --example gherkin_runner --features chromiumoxide-backend -- --feature examples/hacker_news.feature
```

## Output Format

```
=== Advanced Gherkin Feature Runner ===
Running feature file: "examples/example.feature"

Feature: Example Feature
2 scenario(s)

  Scenario: First scenario
    Given a browser is available ... ✓
      Browser initialized
    When I navigate to "https://example.com" ... ✓
      Navigated to https://example.com
    Then I should see ".content" ... ✓
      Element '.content' is visible
    ✓ Scenario PASSED

  Scenario: Second scenario
    ...

=== Extracted Data ===

links (25 items):
  1. https://example.com/link1
  2. https://example.com/link2
  ...

=== Feature Run Complete ===
```

## Selector Syntax

All steps use CSS selectors for element targeting:
- ID: `#element-id`
- Class: `.class-name`
- Attribute: `[type="submit"]`
- Tag: `button`, `a`, `div`, etc.
- Combinations: `div.card > button.primary`
- Pseudo-selectors: `li:first-child`, `p:contains('text')`

## Adding Custom Steps

To add custom steps, extend the `execute_step` function in `gherkin_runner.rs`:

```rust
// Add new pattern matching
if let Some(caps) = Regex::new(r"your pattern here").unwrap().captures(step_text) {
    let value = caps.get(1).unwrap().as_str();
    // Your custom logic here
    return Ok("Result message".to_string());
}
```

## Best Practices

1. **Wait for elements** before interacting: Always use wait steps before clicks/types
2. **Use explicit waits** over implicit sleeps when possible
3. **Chain steps logically**: Navigate → Wait → Interact → Verify
4. **Reuse selectors**: Define selectors in feature files or external config
5. **Test incrementally**: Build scenarios step by step, verifying each works
6. **Use descriptive step text**: Makes features self-documenting

## Error Handling

Failed steps will:
- Display ✗ marker
- Show error message
- Stop current scenario
- Continue with next scenario

Example:
```
When I click on "#non-existent" ... ✗
      Error: Click failed: No element found
    ✗ Scenario FAILED
```

## Browser Backend

Uses Chromiumoxide for real browser automation:
- Full Chrome browser engine
- Supports JavaScript execution
- Handles dynamic content
- Takes screenshots
- Extracts page state
