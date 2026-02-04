@homepage
Feature: Homepage UI

  Scenario: Verify the homepage heading and button functionality
    Given Goto /
    Then I see an "h1" with text "Welcome to Leptos!"
    Then I see a button with "Click Me: 0"
    When I click the button labeled "Click Me: 0"
    Then the button label changes to "Click Me: 1"
    When I click the button labeled "Click Me: 1"
    Then the button label changes to "Click Me: 2"
