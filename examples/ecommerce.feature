Feature: E-commerce Shopping Flow

  Scenario: Browse products and add to cart
    Given a browser is available
    When I navigate to "https://example-shop.com/products"
    And I wait for page to load
    And I should see ".product-grid"
    Then I should see at least 1 ".product-card" elements
    When I hover over ".product-card:first-child"
    And I wait 1 second
    And I click on ".product-card:first-child button.add-to-cart"
    And I wait 2 seconds
    Then I should see ".cart-count"
    And text of ".cart-count" should be "1"

  Scenario: Search for products
    Given a browser is available
    When I navigate to "https://example-shop.com"
    And I wait for page to load
    When I clear "input.search"
    And I type "laptop" into "input.search"
    And I click on "button.search"
    And I wait for page to load
    Then I should see at least 3 ".product-card" elements
    And I should see "laptop" in page

  Scenario: Filter and sort products
    Given a browser is available
    When I navigate to "https://example-shop.com/products"
    And I wait for page to load
    When I select "price-low" from "select.sort"
    And I wait 1 second
    And I click on "button.filter-brand"
    And I wait for page to load
    Then I should see at least 1 ".product-card" elements

  Scenario: Multi-page product browsing
    Given a browser is available
    When I navigate to "https://example-shop.com/products"
    And I wait for page to load
    When I scroll to bottom
    And I wait 2 seconds
    When I click on "button.load-more"
    And I wait 3 seconds
    And I scroll to ".products-section"
    Then I should see ".product-card"

  Scenario: Checkout process
    Given a browser is available
    When I navigate to "https://example-shop.com/cart"
    And I wait for page to load
    And I should see ".cart-items"
    When I click on "button.checkout"
    And I wait 2 seconds
    And I fill in "#email" with "test@example.com"
    And I fill in "#address" with "123 Main St"
    And I fill in "#city" with "Anytown"
    And I click on "button.place-order"
    And I wait 3 seconds
    Then I should see "order confirmed"
