Feature: Scrolling and Page Navigation
  As a user, I want to scroll and navigate within pages
  So that I can access content at different positions

  Scenario: Scroll to a specific element
    Given I navigate to "https://example.com"
    When I scroll to "footer"
    Then the element "footer" should be visible

  Scenario: Scroll to percentage 50%
    Given I navigate to "https://example.com"
    When I scroll to position 50%
    Then the element "content" should be visible

  Scenario: Scroll to top percentage
    Given I navigate to "https://example.com"
    When I scroll to position 0%
    Then the element "header" should be visible

  Scenario: Scroll to bottom percentage
    Given I navigate to "https://example.com"
    When I scroll to position 100%
    Then the element "footer" should be visible

  Scenario: Scroll to multiple positions
    Given I navigate to "https://example.com"
    When I scroll to position 25%
    And I scroll to position 75%
    Then the element "content" should be visible
