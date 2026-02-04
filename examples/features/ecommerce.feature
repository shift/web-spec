Feature: E-commerce Shopping Flow
  As a customer, I want to browse and purchase products
  So that I can complete an online shopping transaction

  Scenario: Browse product catalog
    Given I navigate to "https://shop.example.com"
    Then the page should contain "Product Catalog"
    And the element "product-list" should be visible

  Scenario: Search for a product
    Given I navigate to "https://shop.example.com"
    When I type "laptop" into "search-box"
    And I press the Enter key
    Then the page should contain "laptop"

  Scenario: Add items to cart
    Given I navigate to "https://shop.example.com/products"
    When I click on "product-1"
    And I click the "add-to-cart" button
    Then the page should contain "Added to cart"

  Scenario: View shopping cart
    Given I navigate to "https://shop.example.com/cart"
    Then the element "cart-items" should be visible
    And the page should contain "Total"

  Scenario: Proceed to checkout
    Given I navigate to "https://shop.example.com/cart"
    When I click the "checkout" button
    Then the title should contain "Checkout"

  Scenario: Complete purchase
    Given I navigate to "https://shop.example.com/checkout"
    When I type "customer@example.com" into "email"
    And I type "123 Main St" into "address"
    And I click the "place-order" button
    Then the page should contain "Order confirmed"
