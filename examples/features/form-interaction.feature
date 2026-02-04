Feature: Form Interaction and Input Handling
  As a user, I want to fill out and submit forms
  So that I can interact with web applications

  Scenario: Fill in text fields
    Given I navigate to "https://example.com/login"
    When I type "john@example.com" into "username"
    And I type "password123" into "password"
    Then the element "username" should exist

  Scenario: Select options from dropdown
    Given I navigate to "https://example.com/filter"
    When I select "Newest" from "sort"
    Then the element "sort" should be selected

  Scenario: Check and uncheck checkboxes
    Given I navigate to "https://example.com/preferences"
    When I check "newsletter"
    And I check "updates"
    Then the element "newsletter" should be checked

  Scenario: Select radio buttons
    Given I navigate to "https://example.com/options"
    When I select the "email" radio button
    Then the element "email" should be checked

  Scenario: Clear input fields
    Given I navigate to "https://example.com/search"
    When I type "initial text" into "search-field"
    And I clear "search-field"
    Then the element "search-field" should exist
