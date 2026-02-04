Feature: Click and Interaction Patterns
  As a user, I want to interact with page elements
  So that I can perform actions on web applications

  Scenario: Basic click operations
    Given I navigate to "https://example.com"
    When I click on "learn-more"
    Then the element "details" should be visible

  Scenario: Click button and link elements
    Given I navigate to "https://example.com"
    When I click the "submit" button
    Then the element "confirmation" should exist

  Scenario: Double-click operations
    Given I navigate to "https://example.com/editor"
    When I double click on "editable-content"
    Then the element "editable-content" should be visible

  Scenario: Right-click context menu
    Given I navigate to "https://example.com"
    When I right click on "menu-item"
    Then the element "context-menu" should be visible

  Scenario: Hover over elements
    Given I navigate to "https://example.com"
    When I hover over "help-tooltip"
    Then the element "help-tooltip" should be visible

  Scenario: Drag and drop
    Given I navigate to "https://example.com/drag-drop"
    When I drag "draggable-item" to "drop-zone"
    Then the element "drop-zone" should contain "draggable-item"
