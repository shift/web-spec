Feature: Navigation and URL Handling
  As a user, I want to navigate between different pages
  So that I can access various parts of the application

  Scenario: Navigate to a website
    Given I navigate to "https://example.com"
    Then the title should be "Example Domain"
    And the URL should be "https://example.com/"

  Scenario: Verify current URL contains domain
    Given I navigate to "https://example.com"
    Then the URL should contain "example.com"
    And the path should be "/"

  Scenario: Verify page title contains text
    Given I navigate to "https://example.com"
    Then the title should contain "Example"
