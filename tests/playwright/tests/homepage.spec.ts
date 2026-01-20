{% if websocket == "yes" -%}
import { test, expect } from "@playwright/test";

test.describe("Homepage UI", () => {
  test("should verify WebSocket connection flow", async ({ page }) => {
    // Set up console log listener before navigation
    const logs: string[] = [];
    page.on("console", (msg) => logs.push(msg.text()));

    // Go to homepage
    await page.goto("/");

    // Assert button label is `Connect`
    const connect_button = page.locator("button");
    await expect(connect_button).toHaveText("Connect");

    // Click the button
    await connect_button.click();

    // Wait for the specific console message
    await page.waitForEvent("console", (msg) =>
      msg.text().includes("Received: Response::HandshakeResponse"),
    );

    // Assert the specific console log exists
    const handshakeLog = logs.find((log) =>
      log.includes("Received: Response::HandshakeResponse"),
    );
    expect(handshakeLog).toBeTruthy();

    // Assert button label is `Disconnect`
    const disconnect_button = page.locator("button");
    await expect(disconnect_button).toHaveText("Disconnect");
  });
});
{% else -%}
import { test, expect } from "@playwright/test";

test.describe("Homepage UI", () => {
  test("should display correct heading and increment button label", async ({
    page,
  }) => {
    // Go to homepage
    await page.goto("/");

    // Assert h1 text
    const h1 = page.locator("h1");
    await expect(h1).toHaveText("Welcome to Leptos!");

    // Assert button label is "Click Me: 0"
    const button = page.locator("button");
    await expect(button).toHaveText("Click Me: 0");

    // Click the button
    await button.click();

    // Button label changes to "Click Me: 1"
    await expect(button).toHaveText("Click Me: 1");
  });
});
{% endif -%}

