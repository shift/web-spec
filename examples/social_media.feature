Feature: Social Media Interaction

  Scenario: Login and post content
    Given a browser is available
    When I navigate to "https://example-social.com/login"
    And I wait for page to load
    When I type "user@example.com" into "#username"
    And I type "password123" into "#password"
    And I click on "button.login"
    And I wait 3 seconds
    Then I should see ".dashboard"
    And I should not see ".error-message"

  Scenario: Browse feed and like posts
    Given a browser is available
    When I navigate to "https://example-social.com/feed"
    And I wait for page to load
    And I scroll to bottom
    And I wait 2 seconds
    When I click on ".post:first-child button.like"
    And I wait 1 second
    Then the "button.like" attribute of ".post:first-child button.like" should be "liked"

  Scenario: Comment on post
    Given a browser is available
    When I navigate to "https://example-social.com/feed"
    And I wait for page to load
    When I scroll to ".post:first-child"
    And I click on "button.comment"
    And I wait 1 second
    And I type "Great post!" into "textarea.comment-input"
    And I click on "button.submit-comment"
    And I wait 2 seconds
    Then I should see ".comment:contains('Great post!')"

  Scenario: Search and follow user
    Given a browser is available
    When I navigate to "https://example-social.com/search"
    And I wait for page to load
    When I type "johndoe" into "input.search"
    And I click on "button.search-submit"
    And I wait 2 seconds
    When I click on "a.user-profile:first-child"
    And I wait 2 seconds
    And I click on "button.follow"
    And I wait 1 second
    Then the "aria-label" attribute of "button.follow" should be "Following"

  Scenario: Upload image post
    Given a browser is available
    When I navigate to "https://example-social.com/create-post"
    And I wait for page to load
    When I type "Check out my photo!" into "textarea.caption"
    And I click on "input.file-upload"
    And I wait 1 second
    And I click on "button.publish"
    And I wait 3 seconds
    Then I should see "post created successfully"
