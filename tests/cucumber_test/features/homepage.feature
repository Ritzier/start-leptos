{% if websocket == "yes" -%}
{%- raw %}@{% endraw %}homepage
Feature: Homepage UI

  Scenario: Verify the homepage heading and button functionality
    Given Goto /
    Then I see a button with "Connect"
    When I click the button labeled "Connect"
    Then the button label changes to "Disconnect"
    Then I see a button with "Disconnect"
    When I click the button labeled "Disconnect"
    Then the button label changes to "Connect"
{% else -%}
{%- raw %}@{% endraw %}homepage
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
{% endif -%}

