Feature: Browser Navigation
  As an agentic coding agent
  I want to navigate to websites and extract content
  So that I can perform web searches and operations

  Scenario: Navigate to a webpage using WebDriver
    Given I am using a WebDriver browser backend
    When I navigate to "https://news.ycombinator.com/news"
    And I wait for page to load
    Then the page should be loaded successfully
    And I should be able to extract HTML from the page
    And the HTML should contain the page title

  Scenario: Navigate to a webpage using Chromiumoxide
    Given I am using a Chromiumoxide browser backend
    When I navigate to "https://news.ycombinator.com/news"
    And I wait for page to load
    Then the page should be loaded successfully
    And I should be able to extract HTML from the page
    And the HTML should contain the page title

  Scenario: Extract and convert HTML to Markdown
    Given I have HTML content from a webpage
    When I convert the HTML to markdown
    Then the markdown should contain post titles
    And the markdown should be non-empty

  Scenario: Convert HTML with headings and paragraphs
    Given I have HTML with headings and paragraphs
    When I convert the HTML to markdown
    Then the markdown should contain headings
    And the markdown should contain paragraphs
    And the markdown should be non-empty

  Scenario: Convert HTML with code blocks
    Given I have HTML with code blocks
    When I convert the HTML to markdown
    Then the markdown should contain code
    And the markdown should be non-empty

  Scenario: Convert HTML with links
    Given I have HTML with links
    When I convert the HTML to markdown
    Then the markdown should contain markdown links
    And the markdown should be non-empty

