import { defineConfig, devices } from "@playwright/test";

const LEPTOS_SITE_ADDR = process.env.LEPTOS_SITE_ADDR;

if (!LEPTOS_SITE_ADDR || LEPTOS_SITE_ADDR.trim() === "") {
  throw new Error(
    "Environment variable LEPTOS_SITE_ADDR is not set or empty. Please set it before running tests.",
  );
}

const baseURL = LEPTOS_SITE_ADDR.startsWith("http")
  ? LEPTOS_SITE_ADDR
  : `http://${LEPTOS_SITE_ADDR}`;

export default defineConfig({
  testDir: "./tests",
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 2,
  workers: process.env.CI ? 1 : undefined,
  reporter: "list",
  use: {
    baseURL,
    trace: "on-first-retry",
  },

  projects: [
    {
      name: "chromium",
      use: { ...devices["Desktop Chrome"] },
    },

    {
      name: "firefox",
      use: { ...devices["Desktop Firefox"] },
    },

    // {
    //   name: "webkit",
    //   use: { ...devices["Desktop Safari"] },
    // },

    /* Test against mobile viewports. */
    {
      name: "Mobile Chrome",
      use: { ...devices["Pixel 5"] },
    },
    // {
    //   name: 'Mobile Safari',
    //   use: { ...devices['iPhone 12'] },
    // },

    /* Test against branded browsers. */
    // {
    //   name: 'Microsoft Edge',
    //   use: { ...devices['Desktop Edge'], channel: 'msedge' },
    // },
    // {
    //   name: 'Google Chrome',
    //   use: { ..devices['Desktop Chrome'], channel: 'chrome' },
    // },
  ],

  /* Run your local dev server before starting the tests */
  // webServer: {
  //   command: "cd ../ && cargo leptos serve",
  //   url: "http://127.0.0.1:3000",
  //   reuseExistingServer: false, //!process.env.CI,
  // },
});
