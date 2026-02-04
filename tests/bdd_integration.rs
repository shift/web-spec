// BDD-style integration tests for web-spec crate
// These tests follow Given-When-Then structure and can serve as documentation

use web_spec::{Browser, BrowserType, Converter};

// Browser Navigation Tests
#[tokio::test]
#[ignore] // Requires actual WebDriver server
async fn given_webdriver_backend_when_navigate_to_page_then_html_can_be_extracted() {
    // GIVEN: A WebDriver backend is available
    let browser_type = BrowserType::WebDriver;
    
    // WHEN: We navigate to a webpage
    let mut browser = Browser::new(browser_type).await.expect("Browser should initialize");
    browser.navigate_to("https://news.ycombinator.com/news").await.expect("Navigation should succeed");
    browser.wait_for_load().await.expect("Wait should complete");
    
    // THEN: HTML can be extracted from the page
    let html = browser.get_html().await.expect("HTML should be extracted");
    
    assert!(!html.is_empty(), "HTML should not be empty");
    assert!(html.contains("<html"), "HTML should contain HTML tags");
    assert!(html.contains("Hacker News"), "HTML should contain page title");
}

// HTML Conversion Tests
#[test]
fn given_html_page_when_converted_to_markdown_then_output_is_non_empty() {
    // GIVEN: An HTML page
    let html = "<h1>Test Title</h1><p>Test content</p>";
    
    // WHEN: Converting to markdown
    let converter = Converter::new();
    let markdown = converter.convert(html).expect("Conversion should succeed");
    
    // THEN: Output is non-empty and contains expected content
    assert!(!markdown.is_empty(), "Markdown should not be empty");
    assert!(markdown.contains("Test Title"), "Markdown should preserve headings");
    assert!(markdown.contains("Test content"), "Markdown should preserve paragraphs");
}

#[test]
fn given_html_with_code_block_when_converted_then_code_is_preserved() {
    // GIVEN: HTML with code block
    let html = "<pre><code>fn main() { println!(\"Hello\"); }</code></pre>";
    
    // WHEN: Converting to markdown
    let converter = Converter::new();
    let markdown = converter.convert(html).expect("Conversion should succeed");
    
    // THEN: Code is preserved
    assert!(!markdown.is_empty(), "Markdown should contain code");
    assert!(markdown.contains("fn main"), "Markdown should preserve code content");
}

#[test]
fn given_html_with_links_when_converting_then_links_are_preserved() {
    // GIVEN: HTML with links
    let html = "<a href=\"https://example.com\">Click here</a>";
    
    // WHEN: Converting to markdown
    let converter = Converter::new();
    let markdown = converter.convert(html).expect("Conversion should succeed");
    
    // THEN: Links are preserved in markdown format
    assert!(!markdown.is_empty(), "Markdown should contain links");
    assert!(markdown.contains("[Click here]"), "Markdown should preserve link text");
    assert!(markdown.contains("(https://example.com)"), "Markdown should preserve link URL");
}

// Browser Type Tests
#[test]
fn given_browser_type_when_created_then_matches_expected_type() {
    // GIVEN: WebDriver browser type
    let webdriver_type = BrowserType::WebDriver;
    
    // WHEN: Checking the type
    // THEN: It should be WebDriver
    assert!(matches!(webdriver_type, BrowserType::WebDriver));
}

#[cfg(feature = "chromiumoxide-backend")]
#[test]
fn given_chromiumoxide_browser_type_when_created_then_matches_expected_type() {
    // GIVEN: Chromiumoxide browser type
    let chromiumoxide_type = BrowserType::Chromiumoxide;
    
    // WHEN: Checking the type
    // THEN: It should be Chromiumoxide
    assert!(matches!(chromiumoxide_type, BrowserType::Chromiumoxide));
}
