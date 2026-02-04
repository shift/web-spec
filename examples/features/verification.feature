Feature: Element Verification and Assertions
  As a QA engineer, I want to verify page elements
  So that I can ensure the application works correctly

  Scenario: Verify element visibility
    Given I navigate to "https://example.com"
    Then the element "header" should be visible
    And the element "footer" should exist

  Scenario: Verify text content
    Given I navigate to "https://example.com"
    Then the page should contain "Welcome"
    And the element "title" should contain "Example"

  Scenario: Verify element states
    Given I navigate to "https://example.com/form"
    Then the element "submit" should be enabled
    And the element "disabled-field" should be disabled

  Scenario: Verify element does not exist
    Given I navigate to "https://example.com"
    Then the element "hidden-feature" should not exist
    And the page should not contain "error message"
