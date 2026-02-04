Feature: Extract Top 10 Hacker News Posts

  Scenario: Extract top 10 post titles from Hacker News
    Given a browser is available
    When I navigate to Hacker News
    And the page loads
    And I extract post titles from the page
    Then I should see at least 10 post titles
    And each title has meaningful text
