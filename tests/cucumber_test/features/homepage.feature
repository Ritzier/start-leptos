{% if websocket == "yes" -%}
{%- raw %}@{% endraw %}homepage
@homepage
Feature: Homepage UI

  Scenario: Verify the homepage heading and button functionality
    Given Goto /
    Then I see a button with "Connect"
    When I click the button labeled "Connect"
    Then I should see the following console logs:
        | Received: FrontendResponse::HandshakeResponse | log |

    Then the button label changes to "Disconnect"
    Then I see a button with "Disconnect"

    When I click the button labeled "Disconnect"
    Then the button label changes to "Connect"
    Then I should see the following console logs:
        | WebSocket Closed: code: 1005, reason: | error |
        | Websocket closed: error reaching server to call server function: WebSocket Closed: code: 1005, reason: | log |
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
    And I should see the following console logs:
        | Update num: 1 | log |

    When I click the button labeled "Click Me: 1"
    Then the button label changes to "Click Me: 2"
    And I should see the following console logs:
        | Update num: 2 | log |
{% endif -%}

