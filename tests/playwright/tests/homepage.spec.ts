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
