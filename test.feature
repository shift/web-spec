Feature: Test Navigation
  Scenario: Basic Navigation
    Given a browser is available
    When I navigate to "https://example.com"
    Then I should see "Example Domain"
