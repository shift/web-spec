Feature: Extract Hacker News Post Titles
  As an agentic coding agent
  I want to extract post titles from Hacker News
  So that I can analyze web content

  Scenario: Extract top 10 posts using WebDriver
    Given I am using a WebDriver browser backend
    When I navigate to "https://news.ycombinator.com/news"
    And I wait for the page to load
    And I extract the HTML from the page
    Then I should be able to extract post titles from the HTML
    And I should find at least 10 post titles
    And each post title should contain text

  Scenario: Extract top 10 posts using Chromiumoxide
    Given I am using a Chromiumoxide browser backend
    When I navigate to "https://news.ycombinator.com/news"
    And I wait for the page to load
    And I extract the HTML from the page
    Then I should be able to extract post titles from the HTML
    And I should find at least 10 post titles
    And each post title should contain text

  Scenario: Verify Hacker News page structure
    Given I have loaded the Hacker News homepage
    Then the page should contain a title element
    And the page should contain a stories list
    And each story should have a title link
    And each story should have metadata

  Scenario: Display extracted post titles
    Given I have extracted post titles from Hacker News
    When I format the titles for display
    Then the format should show the title number
    And the format should show the title text
    And the format should be readable

  Scenario: Compare WebDriver and Chromiumoxide results
    Given I extract titles using WebDriver
    And I extract titles using Chromiumoxide
    Then both backends should return the same number of titles
    And both backends should return similar title content
