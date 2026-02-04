Feature: Advanced Website Navigation

  Scenario: Navigate and interact with forms
    Given a browser is available
    When I navigate to "https://example.com"
    And I wait for page to load
    And I wait for element "input[name=search]" to be visible
    And I type "Hello World" into "input[name=search]"
    And I click on "button[type=submit]"
    And I wait 2 seconds
    Then I should see "search results"

  Scenario: Extract content from page
    Given a browser is available
    When I navigate to "https://example.com"
    And I wait for page to load
    When I extract all links from page
    And I extract all images from page
    Then I should see at least 1 links

  Scenario: Scrolling and interaction
    Given a browser is available
    When I navigate to "https://example.com"
    And I wait for page to load
    When I scroll to bottom
    And I wait 1 seconds
    When I scroll to top
    And I wait for element "header" to be visible
    Then "header" should be visible

  Scenario: Multiple waits and element checks
    Given a browser is available
    When I navigate to "https://example.com"
    And I wait for page to load
    When I wait 3 seconds
    And I click on "button#action"
    And I wait for element ".result" to appear
    Then I should see ".result"
    And the text of ".result" should be "Success"

  Scenario: Hover and double click interactions
    Given a browser is available
    When I navigate to "https://example.com"
    And I wait for page to load
    When I hover over "button.menu"
    And I wait 1 seconds
    When I double click on "button.item"
    Then I should see "modal dialog"
