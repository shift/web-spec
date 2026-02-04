Feature: Waiting and Timing Operations
  As a user, I want to wait for elements to appear
  So that I can handle dynamic content loading

  Scenario: Wait for element to appear
    Given I navigate to "https://example.com"
    When I click on "load-data"
    And I wait for "data-results" to appear
    Then the element "data-results" should exist

  Scenario: Wait for element to become visible
    Given I navigate to "https://example.com"
    When I click on "show-content"
    And I wait for "lazy-content" to be visible
    Then the element "lazy-content" should be visible

  Scenario: Wait for element to become hidden
    Given I navigate to "https://example.com"
    When I click on "close-modal"
    And I wait for "modal" to disappear
    Then the element "modal" should not be visible

  Scenario: Wait for text to appear
    Given I navigate to "https://example.com/live-updates"
    When I wait for text "Success" to appear
    Then the page should contain "Success"

  Scenario: Wait for element text
    Given I navigate to "https://example.com/countdown"
    When I wait for element "countdown" to contain "00:00"
    Then the element "countdown" should contain "00:00"
