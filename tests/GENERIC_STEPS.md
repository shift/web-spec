# Generic Step Definitions

This module contains reusable BDD step definitions that can be used across multiple features.

## Available Steps

### GIVEN Steps

- `given_webdriver_browser_is_available()` - WebDriver backend is available
- `given_chromiumoxide_browser_is_available()` - Chromiumoxide backend is available
- `given_html_content_is_provided()` - HTML content is available for testing
- `given_html_contains_links()` - HTML contains anchor links
- `given_html_contains_headings()` - HTML contains heading tags
- `given_html_contains_code_blocks()` - HTML contains pre/code blocks
- `given_html_contains_tables()` - HTML contains table elements
- `given_html_contains_images()` - HTML contains image elements
- `given_html_contains_paragraphs()` - HTML contains paragraph tags

### WHEN Steps

- `when_converting_html_to_markdown()` - Convert HTML to markdown
- `when_converting_html_with_cleanup_options()` - Convert with custom options
- `when_stripping_images_from_markdown()` - Remove images from markdown output
- `when_extracting_links_from_html()` - Extract all href links from HTML
- `when_extracting_headings_from_html()` - Extract all heading elements
- `when_normalizing_whitespace_in_text()` - Normalize line breaks and whitespace

### THEN Steps

- `then_html_should_be_non_empty()` - HTML is not empty
- `then_markdown_should_be_non_empty()` - Markdown output is not empty
- `then_markdown_should_contain_headings()` - Markdown contains heading markers
- `then_markdown_should_contain_links()` - Markdown contains link syntax
- `then_markdown_should_preserve_code_blocks()` - Markdown contains code blocks
- `then_markdown_should_contain_paragraphs()` - Markdown contains paragraph text
- `then_images_should_be_stripped_when_option_enabled()` - Images removed when strip_images=true
- `then_links_should_be_preserved_when_option_enabled()` - Links converted to markdown format
- `then_html_should_be_valid()` - HTML is well-formed
- `then_markdown_should_be_well_formatted()` - Markdown has proper formatting
- `then_extracted_content_should_not_contain_html_tags()` - Markdown doesn't contain HTML tags
- `then_converted_text_should_be_readable()` - Output is human-readable
- `then_conversion_should_handle_special_characters()` - HTML entities properly decoded
- `then_conversion_should_handle_empty_html()` - Empty HTML produces empty markdown
- `then_conversion_should_handle_whitespace_only()` - Whitespace-only produces empty markdown

### Combined Scenarios

- `given_html_page_with_heading_and_paragraph_when_converting_then_both_preserved()` - Headings + paragraphs
- `given_html_with_multiple_elements_when_converting_then_all_converted()` - Multiple element types
- `given_html_with_images_when_stripping_then_removed()` - Image stripping
- `given_html_with_special_entities_when_converting_then_decoded()` - Entity decoding

## Helper Functions

### `extract_links_from_html(html: &str) -> Vec<String>`
Extracts all href URLs and link text from anchor tags.

### `extract_headings_from_html(html: &str) -> Vec<String>`
Extracts all heading text from h1-h6 tags.

### `is_well_formed_html(html: &str) -> bool`
Checks if HTML has proper DOCTYPE and html tags.

### `count_html_tags(html: &str, tag: &str) -> usize`
Counts how many times a specific tag appears (both opening and closing).

## Usage Examples

### Feature File (Gherkin)
```gherkin
Feature: HTML to Markdown Conversion
  Scenario: Convert simple HTML
    Given HTML content is provided
    When converting HTML to markdown
    Then markdown should be non-empty
    Then markdown should contain headings
```

### Using in Tests
```rust
#[test]
fn test_conversion() {
    given_html_content_is_provided();
    when_converting_html_to_markdown();
    then_markdown_should_be_non_empty();
}
```

## Running Tests

```bash
# Run all generic steps tests
nix develop -c -- cargo test --test generic_steps

# Run specific test
nix develop -c -- cargo test --test generic_steps given_html_content_is_provided
```

## Benefits of Generic Steps

✅ **Reusable**: Can be used across multiple features
✅ **Comprehensive**: Covers common HTML/markdown operations
✅ **Self-Documenting**: Step names describe behavior clearly
✅ **Maintainable**: Centralized location for common operations
✅ **Testable**: Each step has corresponding unit test

## Extending Generic Steps

To add new generic steps:

1. Add a test function following naming convention:
   ```rust
   #[test]
   fn given_your_condition() { ... }
   ```

2. Add corresponding WHEN/THEN steps as needed
3. Update this README with new step documentation
4. Run tests to verify

## Step Naming Convention

- **GIVEN**: `given_<condition>()` - Setup state
- **WHEN**: `when_<action>()` - Perform operation
- **THEN**: `then_<expected_outcome>()` - Verify result
- **Helpers**: `<function_name>()` - Utility functions

This convention makes it clear what each step does and when to use it.
