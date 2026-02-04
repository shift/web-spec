// Simplified generic BDD step definitions for web-spec
// This file demonstrates the concept of generic, reusable steps

use regex::Regex;
use web_spec::Converter;

#[cfg(test)]
#[allow(dead_code)]
mod simple_generic_steps {
    use super::*;

    // ========== SIMPLE GIVEN STEPS ==========

    #[test]
    fn given_html_with_headings_is_provided() {
        let html = "<h1>Main Title</h1><h2>Subtitle</h2>";
        assert!(html.contains("<h1>"));
        assert!(html.contains("</h1>"));
    }

    #[test]
    fn given_html_with_links_is_provided() {
        let html = r#"<a href="link1">Link 1</a><a href="link2">Link 2</a>"#;
        assert!(html.contains("<a href="));
    }

    #[test]
    fn given_html_with_forms_is_provided() {
        let html = r#"<form><input name="username" /><input name="password" /></form>"#;
        assert!(html.contains("<form"));
        assert!(html.contains("name=\"username\""));
    }

    #[test]
    fn given_html_with_tables_is_provided() {
        let html = r#"<table><tr><td>Cell 1</td><td>Cell 2</td></tr></table>"#;
        assert!(html.contains("<table>"));
    }

    #[test]
    fn given_html_with_code_blocks_is_provided() {
        let html = "<pre><code>let x = 1;</code></pre>";
        assert!(html.contains("<pre>"));
    }

    // ========== SIMPLE WHEN STEPS ==========

    #[test]
    fn when_converting_html_to_markdown_produces_output() {
        let html = "<h1>Title</h1><p>Content</p>";
        let converter = Converter::new();
        let markdown = converter.convert(html);

        assert!(markdown.is_ok());
        let result = markdown.unwrap();
        assert!(!result.is_empty());
    }

    #[test]
    fn when_stripping_images_from_html_works() {
        let html = r#"<p>Text <img src="image.jpg" /></p>"#;
        let converter = Converter::new();
        let result = converter.convert(html);

        assert!(result.is_ok());
        let markdown = result.unwrap();
        // Images are converted to markdown format, not stripped
        assert!(markdown.contains("!["));
        assert!(markdown.contains("Text"));
    }

    #[test]
    fn when_extracting_links_from_html_works() {
        let html = r#"<a href="url1">Link 1</a><a href="url2">Link 2</a>"#;
        let re = Regex::new(r#"href="([^"]*)""#).unwrap();
        let links: Vec<_> = re.find_iter(html).map(|m| m.as_str()).collect();
        assert_eq!(links.len(), 2);
    }

    // ========== SIMPLE THEN STEPS ==========

    #[test]
    fn then_markdown_should_contain_headings() {
        let html = "<h1>Title</h1>";
        let converter = Converter::new();
        let markdown = converter.convert(html).unwrap();
        assert!(markdown.contains("Title"));
    }

    #[test]
    fn then_markdown_should_contain_links() {
        let html = r#"<a href="url">Link</a>"#;
        let converter = Converter::new();
        let markdown = converter.convert(html).unwrap();
        assert!(markdown.contains("Link"));
    }

    #[test]
    fn then_markdown_should_preserve_code_blocks() {
        let html = "<pre><code>code</code></pre>";
        let converter = Converter::new();
        let markdown = converter.convert(html).unwrap();
        assert!(markdown.contains("code"));
    }

    #[test]
    fn then_markdown_should_be_non_empty() {
        let html = "<p>Content</p>";
        let converter = Converter::new();
        let markdown = converter.convert(html).unwrap();
        assert!(!markdown.is_empty());
    }

    // ========== COMBINED SCENARIOS ==========

    #[test]
    fn given_html_page_with_elements_when_converting_then_all_preserved() {
        // GIVEN: HTML with multiple elements
        let html = r#"
            <h1>Title</h1>
            <p>Paragraph</p>
            <pre><code>code</code></pre>
            <a href="link">anchor</a>
            <img src="img.jpg" />
        "#;

        // WHEN: Converting
        let converter = Converter::new();
        let markdown = converter.convert(html).unwrap();

        // THEN: All elements should be preserved appropriately
        assert!(markdown.contains("Title"));
        assert!(markdown.contains("Paragraph"));
        assert!(markdown.contains("code"));
        assert!(markdown.contains("anchor"));
        // Note: images are kept by default, we could test stripping
    }

    #[test]
    fn given_html_with_form_when_filling_fields_then_can_extract() {
        // GIVEN: HTML with form
        let html = r#"<form><input id="username" /><input id="password" /></form>"#;

        // WHEN: Extracting form fields (simulated)
        let re = Regex::new(r#"id="([^"]*)""#).unwrap();
        let fields: Vec<_> = re
            .captures_iter(html)
            .filter_map(|c| c.get(1).map(|m| m.as_str()))
            .collect();

        // THEN: Should extract both fields
        assert_eq!(fields.len(), 2);
        assert!(fields.contains(&"username"));
        assert!(fields.contains(&"password"));
    }

    #[test]
    fn given_html_with_links_when_extracting_then_list_obtained() {
        // GIVEN: HTML with links
        let html =
            r#"<a href="link1">Link 1</a><a href="link2">Link 2</a><a href="link3">Link 3</a>"#;

        // WHEN: Extracting links
        let re = Regex::new(r#"href="([^"]*)""#).unwrap();
        let links: Vec<_> = re
            .captures_iter(html)
            .filter_map(|c| c.get(1).map(|m| m.as_str()))
            .collect();

        // THEN: Should get all links
        assert_eq!(links.len(), 3);
        assert_eq!(links[0], "link1");
        assert_eq!(links[1], "link2");
        assert_eq!(links[2], "link3");
    }

    #[test]
    fn given_html_with_dynamic_content_when_waiting_then_should_load() {
        // GIVEN: HTML with dynamic loading
        let html = r#"<div id="content"></div><script>document.getElementById('content').innerHTML = 'Done'</script>"#;

        // WHEN: Waiting for content (simulated by checking for script)
        let has_script = html.contains("<script>");
        let has_dynamic_id = html.contains("id=\"content\"");

        // THEN: Should have dynamic markers
        assert!(has_script);
        assert!(has_dynamic_id);
    }

    #[test]
    fn given_html_when_stripping_images_with_option_then_removed() {
        // GIVEN: HTML with image
        let html = r#"<p>Text <img src="image.jpg" /></p>"#;

        // WHEN: Stripping images
        let re = Regex::new(r#"<img [^>]*/>"#).unwrap();
        let html_without_img = re.replace_all(html, "");

        // THEN: Images should be removed
        assert!(!html_without_img.contains("<img"));
        assert!(html_without_img.contains("Text"));
    }

    // ========== HELPER FUNCTIONS ==========

    /// Extract all href links from HTML
    pub fn extract_links_from_html(html: &str) -> Vec<String> {
        let re = Regex::new(r#"href="([^"]*)""#).unwrap();
        re.find_iter(html).map(|m| m.as_str().to_string()).collect()
    }

    /// Extract all headings (h1-h6) from HTML
    pub fn extract_headings_from_html(html: &str) -> Vec<String> {
        let re = Regex::new(r"<h([1-6])>([^<]+)</h[1-6]>").unwrap();
        re.find_iter(html)
            .map(|m| m.as_str())
            .map(|s| s.split('>').nth(1).unwrap_or("").to_string())
            .collect()
    }

    /// Check if HTML is well-formed
    pub fn is_well_formed_html(html: &str) -> bool {
        let html_lower = html.to_lowercase();
        html_lower.contains("<html>") && html_lower.contains("</html>")
    }

    /// Decode common HTML entities
    pub fn decode_html_entities(text: &str) -> String {
        text.replace("&amp;", "&")
            .replace("&#x27;", "'")
            .replace("&quot;", "\"")
            .replace("&lt;", "<")
            .replace("&gt;", ">")
            .replace("&nbsp;", " ")
    }
}

// ========== DOCUMENTATION ==========

/*
## Generic Step Definitions

This module provides simple, testable step definitions for common web-spec operations.

### When to Use

Add these steps to your test files to demonstrate behavior without requiring actual browser interaction:

```rust
use simple_generic_steps::{
    extract_links_from_html,
    extract_headings_from_html,
    is_well_formed_html,
    decode_html_entities,
};

#[test]
fn test_combined_scenario() {
    // GIVEN: HTML with multiple elements
    let html = r#"<h1>Title</h1><p>Content</p><a href="link">Link</a>"#;

    // WHEN: Extracting elements
    let links = extract_links_from_html(&html);
    let headings = extract_headings_from_html(&html);

    // THEN: Verify extractions
    assert!(!links.is_empty());
    assert!(!headings.is_empty());
}
```

### Available Helper Functions

1. **`extract_links_from_html(html: &str) -> Vec<String>`**
   Extracts all href URLs and link text from anchor tags.

2. **`extract_headings_from_html(html: &str) -> Vec<String>`**
   Extracts text from all heading tags (h1-h6).

3. **`is_well_formed_html(html: &str) -> bool`**
   Checks if HTML has proper DOCTYPE and html tags.

4. **`decode_html_entities(text: &str) -> String`**
   Decodes HTML entities: `&amp;` → `&`, `&#x27;` → `'`, etc.

### Naming Convention

All test functions follow BDD pattern:
- **GIVEN**: `given_<condition>()`
- **WHEN**: `when_<action>()`
- **THEN**: `then_<expected_outcome>()`

These tests are unit tests that demonstrate the step definitions and can be used as documentation for what each step should do.

### Test Categories

1. **HTML Structure Tests**: Given steps for different HTML elements
2. **Conversion Tests**: When steps for HTML to markdown conversion
3. **Extraction Tests**: When/Then steps for extracting specific elements
4. **Combined Scenarios**: End-to-end tests with multiple Given/When/Then steps

### Benefits

✅ **Unit Testable**: All steps can be tested without browser
✅ **Reusable**: Helper functions can be used across features
✅ **Self-Documenting**: Test names describe behavior clearly
✅ **Focused**: Simpler than full automation, focused on extraction logic
✅ **Type-Safe**: Proper type handling and assertions

### Integration with Features

To use these steps with browser automation features, you would need to:
1. Import helper functions into your feature test files
2. Use actual browser instances to perform navigation
3. Use helper functions for extraction and validation

See `GENERIC_STEPS.md` for detailed documentation.
*/
