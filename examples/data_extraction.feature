Feature: Data Extraction and Scraping

  Scenario: Extract all links from page
    Given a browser is available
    When I navigate to "https://example.com"
    And I wait for page to load
    When I extract all links from page
    Then I should see at least 1 links

  Scenario: Extract structured content
    Given a browser is available
    When I navigate to "https://example.com"
    And I wait for page to load
    When I extract all h1 headings
    And I extract all h2 headings
    And I extract all images from page
    Then I should see at least 1 "img[src]" elements

  Scenario: Hacker News title extraction
    Given a browser is available
    When I navigate to Hacker News
    And I wait for page to load
    And I extract post titles from page
    Then I should see at least 10
    And each title has meaningful text

  Scenario: Extract product information
    Given a browser is available
    When I navigate to "https://example-shop.com/products"
    And I wait for page to load
    When I extract all links from page
    And I scroll to bottom
    And I wait 2 seconds
    When I extract all links from page
    Then I should see at least 5 links

  Scenario: Multi-site data extraction
    Given a browser is available
    When I navigate to "https://example.com"
    And I wait for page to load
    When I extract page HTML
    And I take a screenshot "screenshot.png"
    When I go to "https://example.org"
    And I wait for page to load
    And I take a screenshot "screenshot2.png"
    Then I should see ".content"
