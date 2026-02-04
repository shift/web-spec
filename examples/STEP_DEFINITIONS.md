# Comprehensive Step Definitions for Gherkin Runner

This document describes all available step patterns supported by web-spec Gherkin runner.

The runner now supports **400+ step patterns** across 35+ categories.

## Navigation Steps

### URL Navigation
- `I navigate to "https://example.com"` - Navigate to a specific URL
- `I go to "https://example.com"` - Navigate to a specific URL (alternative wording)
- `I navigate to Hacker News` - Navigate to Hacker News homepage

### Page Loading
- `the page loads` - Wait for the page to finish loading
- `I wait for the page to load` - Wait for the page to finish loading (alternative)

## Waiting Steps

### Time-based Waiting
- `I wait 5 seconds` - Wait for 5 seconds
- `I wait 1000 milliseconds` - Wait for 1000 milliseconds
- `I wait 1 second` - Wait for 1 second

### Element Visibility Waiting
- `I wait for element ".button" to be visible` - Wait up to 10 seconds for element to become visible
- `I wait for element "#submit-btn" to appear` - Wait up to 10 seconds for element to appear

## Clicking Steps

### Basic Clicking
- `I click on ".submit-button"` - Click on an element
- `I click the "submit" button` - Click on a button (alternative wording)
- `I click the "close" link` - Click on a link (alternative wording)
- `I click submit button` - Click on any submit button
- `I click search button` - Click on any search button

### Advanced Clicking
- `I double click on ".item"` - Double-click on an element
- `I right click on ".item"` - Right-click (context menu) on an element
- `I hover over ".menu-item"` - Hover mouse over an element

## Input Steps

### Text Input
- `I type "Hello World" into "#search-input"` - Type text into an input field
- `I enter "user@example.com" in "#email-field"` - Enter text into an input field (alternative)
- `I fill in "#username" with "john_doe"` - Fill input with text

### Other Input Operations
- `I clear "#search-input"` - Clear the content of an input field
- `I select "Option 1" from "#dropdown"` - Select an option from a dropdown/select

## Scrolling Steps

### Directional Scrolling
- `I scroll to bottom` - Scroll to the bottom of the page
- `I scroll to top` - Scroll to the top of the page
- `I scroll to ".content-section"` - Scroll to a specific element

### Pixel-based Scrolling
- `I scroll down by 500 pixels` - Scroll down by 500 pixels
- `I scroll up by 300 pixels` - Scroll up by 300 pixels

## Visibility Assertions

### Element Visibility
- `I should see ".header"` - Assert element exists on the page
- `I should not see ".error-message"` - Assert element does not exist on the page
- `".button" should be visible` - Assert element is visible
- `".modal" should not be visible` - Assert element is not visible

### Element Counting
- `I should see at least 5 ".product-card"` - Assert at least N elements exist
- `I should see exactly 10 ".list-item"` - Assert exactly N elements exist

## Text Assertions

### Text Presence
- `I should see text "Welcome"` - Assert text exists anywhere on the page

### Text Content
- `the text of ".status" should be "Success"` - Assert element text equals exact value
- `the text of ".status" should contain "Loading"` - Assert element text contains value

## Attribute Assertions

- `the "href" attribute of ".link" should be "https://example.com"` - Assert element attribute equals value

## Content Extraction

### Page Content
- `I extract page HTML` - Extract full page HTML
- `I extract the page HTML` - Extract full page HTML (alternative wording)
- `I extract all links from the page` - Extract all href attributes from link elements
- `I extract all links` - Extract all href attributes from link elements (shorter)
- `I extract all images from the page` - Extract all src attributes from img elements
- `I extract all images` - Extract all src attributes from img elements (shorter)

### Heading Extraction
- `I extract all h1 headings` - Extract all h1 headings
- `I extract all h2 headings` - Extract all h2 headings
- `I extract all h3 headings` - Extract all h3 headings
- `I extract all h4 headings` - Extract all h4 headings
- `I extract all h5 headings` - Extract all h5 headings
- `I extract all h6 headings` - Extract all h6 headings

### Specialized Extraction
- `I extract post titles from the page` - Extract Hacker News post titles (specialized)

## Screenshots

### Capture Screenshots
- `I take a screenshot "screenshot.png"` - Take a screenshot and save to file
- `I take a screenshot "test.png"` - Take a screenshot and save to file

## JavaScript Execution

### Run JavaScript
- `I execute JavaScript "alert('Hello')"` - Execute JavaScript code
- `I execute script "document.body.style.display = 'none'"` - Execute JavaScript code (alternative)

## Selector Syntax

### CSS Selectors
All steps use CSS selectors for element targeting:

- **By ID**: `#element-id`
- **By Class**: `.class-name`
- **By Tag**: `button`, `a`, `div`
- **By Attribute**: `[type="submit"]`, `[data-test="value"]`
- **By Pseudo-class**: `:first-child`, `:last-child`, `:hover`
- **By Combination**: `.class-name > button.primary`, `form#login input[type="email"]`

### Examples
```css
/* ID selector */
#submit-button

/* Class selector */
.product-card

/* Tag selector */
button

/* Attribute selector */
input[type="text"]

/* Pseudo-class */
li:first-child

/* Combination */
.form-group > label.required
```

### Best Practices

1. **Use Specific Selectors**: Prefer IDs and unique classes over generic tag names
2. **Wait Before Acting**: Always use wait steps before clicking or typing
3. **Chain Steps Logically**: Navigate → Wait → Interact → Verify
4. **Use Descriptive Step Text**: Makes features self-documenting and readable
5. **Test Incrementally**: Build scenarios step by step, verifying each works

## Step Syntax Patterns

### Flexible Parameters
Many steps accept parameters for flexibility:
- URLs can be absolute or relative
- Time values can be seconds or milliseconds
- Selectors support full CSS syntax
- Text can contain any Unicode characters
- Counts use integers for precise assertions

### Example Features

### Simple Form Submission
```gherkin
Feature: Contact Form Submission

  Scenario: Submit contact form
    Given a browser is available
    When I navigate to "https://example.com/contact"
    And I wait for the page to load
    And I type "John Doe" into "#name"
    And I type "john@example.com" into "#email"
    And I type "Hello World" into "#message"
    And I click on "#submit-button"
    And I wait 2 seconds
    Then I should see text "Thank you for contacting us"
    And "#success-message" should be visible
```

### E-commerce Shopping
```gherkin
Feature: Add Product to Cart

  Scenario: Add product to cart
    Given a browser is available
    When I navigate to "https://shop.example.com/products"
    And I wait for the page to load
    And I should see at least 5 ".product-card"
    And I hover over ".product-card:first-child"
    And I wait 1 second
    And I click on ".product-card:first-child button.add-to-cart"
    And I wait 2 seconds
    Then I should see ".cart-count"
    And the text of ".cart-count" should be "1"
```

### Content Scraping
```gherkin
Feature: Extract Article Links

  Scenario: Get all article links
    Given a browser is available
    When I navigate to "https://example.com/articles"
    And I wait for the page to load
    When I extract all links
    Then I should see at least 10 links
```

### Multi-page Interaction
```gherkin
Feature: Browse Multiple Pages

  Scenario: Navigate and extract
    Given a browser is available
    When I navigate to "https://example.com/page1"
    And I wait for the page to load
    When I extract all h1 headings
    When I scroll to bottom
    And I wait 1 second
    When I navigate to "https://example.com/page2"
    And I wait for the page to load
    When I take a screenshot "page2.png"
    Then I should see text "Welcome to Page 2"
```

## Extending the Runner

To add custom steps:

1. Add pattern to `build_step_registry()` function
2. Add case to `execute_step()` function
3. Register pattern in step registry

Example:
```rust
// In build_step_registry():
registry.register(
    r##I perform custom action on "([^"]+)"##.to_string(), 
    "custom_action"
);

// In execute_step():
"custom_action" => {
    let selector = params.get(0).cloned().unwrap_or_default();
    // Your custom logic here
    Ok("Custom action completed".to_string())
}
```

## Error Messages

The runner provides descriptive error messages for failures:

- **Navigation Failed**: Browser could not load the page
- **Wait Failed**: Timeout waiting for element
- **Click Failed**: Could not click on element
- **Type Failed**: Could not type into input field
- **Element Not Found**: Selector did not match any element
- **Element Should Not Be Visible**: Element exists when it shouldn't
- **Text Not Found**: Expected text not found on page
- **Count Mismatch**: Expected X elements, found Y
- **Attribute Mismatch**: Expected attribute value X, got Y
- **Extraction Failed**: Could not extract content from page

## Running the Runner

```bash
cargo run --example gherkin_runner --features chromiumoxide-backend -- --feature examples/my-feature.feature
```

## Output Format

```
=== Flexible Gherkin Feature Runner ===
Running feature file: "examples/my-feature.feature"

Feature: My Feature Name
2 scenario(s)

  Scenario: My Scenario Name
    Given a browser is available ... ✓
      Browser initialized
    When I navigate to "https://example.com" ... ✓
      Navigated to https://example.com
    ...
    ✓ Scenario PASSED

=== Extracted Data ===

links (25 items):
  1. https://example.com/link1
  ...

=== Feature Run Complete ===
```

## Limitations

1. **Browser Dependency**: Requires real browser (chromiumoxide)
2. **Single Browser Instance**: Reuses same browser across all scenarios
3. **Sequential Execution**: Scenarios run in order, not parallel
4. **No Page Cleanup**: Browser remains open after feature completes
5. **CSS Only**: Does not support XPath selectors (could be added)
6. **JavaScript Execution**: Limited to single statements (no async callbacks)

## Future Enhancements

- [ ] Add XPath selector support
- [ ] Support for multiple browser instances
- [ ] Page cleanup and browser reset between scenarios
- [ ] Parallel scenario execution
- [ ] Data export to JSON/CSV
- [ ] Custom step definitions from external files
- [ ] Background/Before/After/After hooks
- [ ] Tags and scenario filtering
- [ ] Step timeout configuration
- [ ] Retry logic for flaky steps
- [ ] Element state caching
- [ ] Screenshot comparison/visual testing
- [ ] Network request mocking
- [ ] Cookie/session management
- [ ] File download/upload support

## Navigation Steps

### URL Navigation
- `I navigate to "https://example.com"` - Navigate to a specific URL
- `I go to "https://example.com"` - Navigate to a specific URL (alternative wording)
- `I open "https://example.com"` - Navigate to a specific URL (alternative wording)
- `I visit "https://example.com"` - Navigate to a specific URL (alternative wording)
- `I navigate to Hacker News` - Navigate to Hacker News homepage
- `I go to Hacker News` - Navigate to Hacker News homepage

### History Navigation
- `I go back` - Navigate back in browser history
- `I navigate back` - Navigate back (alternative wording)
- `I go forward` - Navigate forward in browser history
- `I navigate forward` - Navigate forward (alternative wording)
- `I refresh the page` - Refresh current page
- `I reload the page` - Reload current page

### Page Loading
- `the page loads` - Wait for the page to finish loading
- `I wait for the page to load` - Wait for the page to finish loading
- `I wait for page to load` - Wait for the page to finish loading

### Waiting Steps

### Time-based Waiting
- `I wait 5 seconds` - Wait for 5 seconds
- `I wait for 5 seconds?` - Wait for 5 seconds (optional seconds)
- `I wait for 5 milliseconds` - Wait for 5 milliseconds
- `I wait for 5 milliseconds?` - Wait for 5 milliseconds (optional milliseconds)

### Element Waiting
- `I wait for element ".button" to be visible` - Wait up to 10 seconds for element to become visible
- `I wait for element ".modal" to appear` - Wait up to 10 seconds for element to appear
- `I wait for element ".button" to be visible` - Wait up to 10 seconds for element to become visible (alternative)
- `I wait until ".button" is visible` - Wait up to 10 seconds for element to become visible
- `I wait until ".modal" appears` - Wait up to 10 seconds for element to appear
- `I wait for element ".button" to be clickable` - Wait for element to become clickable
- `I wait for element ".button" to be enabled` - Wait for element to become enabled

### Advanced Waiting
- `I wait for element ".loader" to be hidden` - Wait for element to be hidden
- `I wait for element ".spinner" to disappear` - Wait for element to disappear
- `I wait for text "Welcome" to appear` - Wait for text to appear anywhere on page
- `I wait for element ".container" to contain "Content"` - Wait for element to contain specific text
- `I wait for element ".output" to be ready` - Wait for element to be in ready state

### More Time-based Waiting
- `I wait 5 minutes` - Wait for 5 minutes
- `I wait for 5 hours` - Wait for 5 hours


### Clicking Steps

### Basic Clicking
- `I click on ".submit-button"` - Click on an element
- `I click ".button"` - Click on an element (shorter)
- `I press ".button"` - Click on an element (alternative wording)
- `I tap ".button"` - Click on an element (alternative wording)
- `I click the "submit" button` - Click on a button by text
- `I click the "close" link` - Click on a link by text
- `I click submit button` - Click on any submit button
- `I click search button` - Click on any search button

### Advanced Clicking
- `I click on ".item" first` - Click on first matching element
- `I click on ".item" second` - Click on second matching element
- `I click on ".item" third` - Click on third matching element
- `I click on ".item" last` - Click on last matching element
- `I click on ".item" at index 5` - Click on nth element by index
- `I double click on ".item"` - Double-click on an element
- `I double-click ".item"` - Double-click on an element (alternative)
- `I right click on ".item"` - Right-click (context menu) on an element
- `I right-click ".item"` - Right-click (context menu) on an element (alternative)

### Mouse Interaction
- `I hover over ".menu-item"` - Hover mouse over element
- `I hover ".menu-item"` - Hover mouse over element (alternative)
- `I move mouse to ".element"` - Move mouse to element

### Drag and Drop
- `I drag ".card" to ".dropzone"` - Drag element to drop zone
- `I drag element ".card" and drop it on ".dropzone"` - Drag element and drop on target element


### Input Steps

### Text Input
- `I type "Hello World" into "#search-input"` - Type text into input field
- `I enter "user@example.com" in "#email-field"` - Enter text into input field (alternative)
- `I type "Hello World" into "#search-input"` - Type text into input field (alternative)
- `I type "Hello World" in "#search-input"` - Type text into input field (alternative)
- `I fill "#username" with "johndoe"` - Fill input with text
- `I fill in "#username" with "johndoe"` - Fill input with text (alternative)
- `I input "Hello World" (into|in) "#search-input"` - Input text into field

### Other Input Operations
- `I clear "#search-input"` - Clear the content of an input field
- `I clear the input "#search-input"` - Clear the content of an input field (alternative)
- `I clear the field "#search-input"` - Clear the content of a field (alternative)
- `I select "Option 1" from "#dropdown"` - Select an option from a dropdown
- `I choose "Option 1" from "#dropdown"` - Select an option from a dropdown (alternative)
- `I pick "Option 1" from "#dropdown"` - Select an option from a dropdown (alternative)
- `I select options "Option 1", "Option 2", "Option 3" from "#dropdown"` - Select multiple options from dropdown

### Checkbox and Radio
- `I check "#checkbox"` - Check a checkbox
- `I uncheck "#checkbox"` - Uncheck a checkbox
- `I check ".checkbox"` - Check nth checkbox
- `I uncheck ".checkbox"` - Uncheck nth checkbox

### File Operations
- `I upload file "document.pdf" to "#file-input"` - Upload a file
- `I attach "document.pdf" to "#file-input"` - Upload a file (alternative)
- `I upload file "report.xlsx" to "#file-upload"` - Upload a file to specific input

### Keyboard Actions
- `I press key "Enter"` - Press Enter key
- `I press Enter key` - Press Enter key (alternative)
- `I press Escape` - Press Escape key
- `I press Tab` - Press Tab key
- `I press Space` - Press Space key
- `I press Backspace` - Press Backspace key
- I type "text" then press Enter - Type text and press Enter

### Form Interactions
- `I submit the form` - Submit the current form
- `I submit form` - Submit form (alternative)
- `I reset the form` - Reset the form
- `I fill in form with test data` - Fill form fields with test data


### Scrolling Steps

### Directional Scrolling
- `I scroll to bottom` - Scroll to the bottom of the page
- `I scroll to top` - Scroll to the top of the page
- `I scroll to ".content-section"` - Scroll to a specific element

### Pixel-based Scrolling
- `I scroll down by 500 pixels` - Scroll down by 500 pixels
- `I scroll up by 300 pixels` - Scroll up by 300 pixels

### Element-based Scrolling
- `I scroll to ".footer"` - Scroll to a specific element


### Visibility Assertions

### Element Visibility
- `I should see ".header"` - Assert element exists on the page
- `I should not see ".error-message"` - Assert element does not exist
- `".button" should be visible` - Assert element is visible
- `".modal" should not be visible` - Assert element is not visible

### Element Counting
- `I should see at least 5 ".product-card"` - Assert at least N elements exist
- `I should see exactly 10 ".list-item"` - Assert exactly N elements exist

### Text Assertions

### Text Presence
- `I should see text "Welcome"` - Assert text exists anywhere on the page

### Text Content
- `the text of ".status" should be "Success"` - Assert element text equals exact value
- `the text of ".status" should contain "Loading"` - Assert element text contains value

### Attribute Assertions
- `the "href" attribute of ".link" should be "https://example.com"` - Assert element attribute equals value

### Screenshot Patterns

### Capture Screenshots
- `I take a screenshot "screenshot.png"` - Take a screenshot and save to file

### JavaScript Execution

### Run JavaScript
- `I execute JavaScript "alert('Hello')"` - Execute JavaScript code


## Category Summary

### Total Patterns: 400+ across 35+ Categories

1. **Navigation** (11 patterns): URL, history, page load, relative navigation
2. **Waiting** (13 patterns): Time-based (seconds, milliseconds, minutes, hours), element visibility, text appearance, ready states
3. **Clicking** (14 patterns): Basic, button/link, first/second/third/last/nth-child, double/right-click
4. **Input** (15 patterns): Text, clear, select, checkbox, radio, file upload, keyboard, form submission
5. **Scrolling** (8 patterns): Directional, pixel-based, element-based, infinite, position-based
6. **Visibility** (12 patterns): Element existence, state, count, text/attribute assertions
7. **Text** (15 patterns): Presence, exact match, contains, starts/ends with, regex, boolean, numeric, alphabetic, alphanumeric, case checks, email/URL/phone patterns, comparisons
8. **Attributes** (3 patterns): Exact, contains, exists
9. **CSS** (3 patterns): Property values, color, background
10. **URL/Path** (11 patterns): Exact match, contains, start/end with, regex match, anchors, follow links
11. **Screenshots** (4 patterns): Element, full page, auto-named
12. **JavaScript** (1 pattern): Custom script execution
13. **Storage** (10 patterns): Set, get, remove, verify values, environment variables
14. **Browser Control** (5 patterns): Resize, maximize/minimize/fullscreen, user agent
15. **Alerts** (7 patterns): Accept/dismiss prompts, type into, text validation
16. **Frames/Iframes** (3 patterns): Switch between frames
17. **Windows/Tabs** (7 patterns): Open, switch, close
18. **Extraction** (12 patterns): HTML, links, images, headings (h1-h6), text, attributes, tables
19. **Conditionals** (5 patterns): If-then logic based on visibility/presence
20. **Loops** (4 patterns): For each, repeat N times, until
21. **More CSS Selectors** (15 patterns): Nth-child, class, ID, attribute, text-based selectors
22. **Form Interaction** (3 patterns): Submit, reset, validate
23. **More Assertions** (8 patterns): Present/visible, empty, regex match, boolean, focused
24. **More Interaction** (18 patterns): Focus/blur, keyboard shortcuts (arrows, page up/down, home/end), send keys, type+enter
25. **Utility** (6 patterns): Pause, debug, log, skip, take notes, wait for ready
26. **Data Manipulation** (11 patterns): Append, prepend, transform (uppercase/lowercase/reverse), increment/decrement/multiply/divide, get length, validate numeric
27. **Table/List** (6 patterns): Count rows/columns, get cells, select rows/items, count items
28. **Date/Time** (5 patterns): Should be checks, wait until, timestamps
29. **URL/Path Extended** (8 patterns): Regex match, start/end with, anchors, follow links
30. **Multi-Select** (5 patterns): Select multiple, deselect all/option
31. **Element State** (8 patterns): Selected/checked, enabled/disabled, readonly, required
32. **Text Validation** (9 patterns): Numeric, alphabetic, alphanumeric, case, email/URL/phone patterns, comparisons
33. **Container/Wrapper** (5 patterns): Click/type inside, look inside, contain/not contain element, instance counts
34. **Performance** (6 patterns): Page load timeout, measure load time, wait for images/resources
35. **Accessibility** (8 patterns): Page check, ARIA labels/roles, alt text, title attributes, all images/inputs labels
36. **Localization** (5 patterns): Page language, switch language, set locale, text in language, translation loading
37. **Clipboard** (6 patterns): Copy to clipboard, paste from clipboard, copy element text, paste into, clipboard checks
38. **Mouse Events** (6 patterns): Mouse down/up/move/over/out, drag by offset, drop at, hold/release drag
39. **Touch Events** (5 patterns): Touch, swipe, pinch zoom, rotate, multi-touch gestures
40. **File Operations** (6 patterns): Download, verify, save, upload
41. **Audio/Video** (9 patterns): Play, pause, stop, mute, unmute, seek, volume, playing/paused checks, duration check
42. **Canvas** (7 patterns): Get data, draw, clear, verify pixel, width/height checks
43. **Console** (6 patterns): Should contain/not contain errors, clear console, get log
44. **Performance Metrics** (5 patterns): Check metrics, LCP/CLS/FID/TTI thresholds, wait stable layout
45. **Network Conditions** (8 patterns): Simulate slow/offline/fast, disable/enable network, status checks
46. **Device Emulation** (10 patterns): Device emulation (iPhone/iPad/Pixel/Android), viewport sizes (mobile/tablet/desktop), pixel ratio, rotation (landscape/portrait)
47. **Local Storage** (9 patterns): Clear, set, get, remove, contains, empty, count checks
48. **Session Storage** (5 patterns): Clear, set, get, remove, empty checks
49. **IndexedDB** (4 patterns): Check existence, count entries, clear, count check
50. **Service Worker** (5 patterns): Wait for activation, check active, unregister, clear cache
51. **Web Manifest** (5 patterns): Check manifest, name/short name/theme checks
52. **Security Headers** (5 patterns): CSP, HSTS, security headers, HTTPS certificate
53. **Cookies Advanced** (8 patterns): Secure checks, same-site checks, HttpOnly/Secure flags
54. **Geolocation** (5 patterns): Mock coordinates, clear mock, check permission
55. **Notifications** (6 patterns): Request/grant/deny permission, see notification, visibility checks
56. **WebSocket** (5 patterns): Connect, disconnect, send messages, receive messages, connected status
57. **Media Streams** (9 patterns): Camera/mic start/stop, permission checks, stream visibility
58. **WebGL** (4 patterns): Check support, get renderer, set context, context check
59. **Animations** (6 patterns): Wait complete, see animation, pause/resume/cancel, running check
60. **Print** (5 patterns): Print page, print to PDF, set layout, preview check
61. **Selection Ranges** (5 patterns): Select text range, select all, clear, copy to clipboard
62. **Drag & Drop Enhanced** (4 patterns): Drag to coordinates, hold/release drag
63. **Spell Check** (4 patterns): Check text, no errors check, enable/disable spell check
64. **Autocomplete** (4 patterns): Should see suggestions, select suggestion, close, visible check
65. **Modals** (6 patterns): Wait appear, see/not see, close, close all, dismissible check
66. **Tooltips** (5 patterns): Hover to show, see, contain text, verify position
67. **Progress Bars** (4 patterns): Wait complete, at least/most %, state check
68. **Tabs/Accordion** (6 patterns): Activate/deactivate, reorder, pin/unpin, see active
69. **Sidebar** (5 patterns): Open/close/toggle, visible/collapsed checks
70. **Breadcrumbs** (5 patterns): See, click, clickable check, count
71. **Search** (7 patterns): Focus box, type/clear, submit, see results, result count
72. **Pagination** (6 patterns): Next/prev, go to page number, see indicator, indicator shows page
73. **Filters** (5 patterns): Apply, clear all, select option, see active, active count
74. **Sorting** (4 patterns): Sort by criteria/direction, reverse order, click sort by, should be sorted
75. **Infinite Scroll** (3 patterns): Scroll indefinitely, stop scrolling
76. **Lazy Loading** (4 patterns): Scroll trigger, wait for loaded, item count
77. **Virtual Scroll** (3 patterns): Scroll to percentage, position check
78. **HTML Attributes** (4 patterns): Document language, meta tags (description, keywords, robots, viewport)
79. **Link Relations** (4 patterns): Canonical URL, alternate URLs, next/prev links
80. **OpenSearch** (2 patterns): Check OpenSearch support
81. **RSS/Feed** (4 patterns): Check for RSS, see link, verify valid
82. **PWA** (5 patterns): Check installability, install, uninstall, installed check
83. **Web Worker** (3 patterns): Check for worker, active status

## Key Features

### Selector Flexibility
- Full CSS selector syntax support (ID, class, tag, attribute, pseudo-class, combinations)
- Text-based selectors (click/type on elements containing specific text)
- Index-based selectors (first, second, third, last, nth-child)
- Nth-child/last-of-type patterns for multiple elements
- Container/wrapper queries (click/type inside, look inside)

### Time Support
- Flexible time units: seconds, milliseconds, minutes, hours
- Relative time specifications: "1s", "5s", "10s", "1m", "5m"

### State Assertions
- Element state: visible, hidden, present, enabled, disabled, focused, selected, checked, readonly, required
- Element content: empty, contains, count, attributes
- Text validation: regex patterns, data types, case checks, format validation

### Browser Control
- Window manipulation: resize, maximize, minimize, fullscreen
- Viewport configuration: size, pixel ratio, rotation
- Navigation history: back, forward, refresh
- User agent customization

### Data Persistence
- Environment variables for scenario state
- Value storage with operations: append, prepend, transform, increment, decrement, multiply, divide

### Advanced Capabilities
- LocalStorage/SessionStorage/IndexedDB operations
- WebSocket and MediaStream APIs
- Service Worker and PWA support
- Performance monitoring (LCP, CLS, FID, TTI)
- Accessibility testing (ARIA, alt text, semantic HTML)
- Internationalization (i18n) support

### Conditional Logic
- Skip/continue based on element presence/visibility
- If-then flows based on text matching

### Loop Support
- For each loops
- Repeat N times patterns
- Until visible/present conditions

