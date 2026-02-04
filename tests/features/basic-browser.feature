Feature: Basic Browser Operations
  As an agentic coding agent
  I want to perform basic browser operations
  So that I can test the browser setup

  Scenario: Create a WebDriver browser instance
    Given the WebDriver backend is available
    When I attempt to create a browser
    Then the browser type should be WebDriver

  Scenario: Create a Chromiumoxide browser instance
    Given the Chromiumoxide backend is available
    When I attempt to create a Chromiumoxide browser
    Then the browser type should be Chromiumoxide
