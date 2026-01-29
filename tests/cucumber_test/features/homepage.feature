{% if websocket == "yes" -%}
{%- raw %}@{% endraw %}homepage
Feature: Homepage UI

  Scenario: Verify the homepage heading and button functionality
    Given I am on the homepage
    Then I see a button with "Connect"
    When I click the button labeled "Connect"
    Then I should see console log containing "Received: FrontendResponse::HandshakeResponse"
    Then the button label changes to "Disconnect"
    Then I see a button with "Disconnect"
    When I click the button labeled "Disconnect"
    Then the button label changes to "Connect"
{% else -%}
{%- raw %}@{% endraw %}homepage
Feature: Homepage UI

  Scenario: Verify the homepage heading and button functionality
    Given I am on the homepage
    Then I see an h1 with text "Welcome to Leptos!"
    When I click the button labeled "Click Me: 0"
    Then the button label changes to "Click Me: 1"
{% endif -%}

