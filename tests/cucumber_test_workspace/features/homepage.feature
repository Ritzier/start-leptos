@homepage
Feature: Homepage UI

  Scenario: Verify the homepage heading and button functionality
    Given I am on the homepage
    Then I see an h1 with text "Welcome to Leptos!"
    When I click the button labeled "Click Me: 0"
    Then the button label changes to "Click Me: 1"
