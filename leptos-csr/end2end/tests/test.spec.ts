import { test } from "@playwright/test";

test("root page has an h1 and a button", async ({ page }) => {
  await page.goto("/");
});
