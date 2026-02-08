//! Cucumber step definitions for browser interactions.
//!
//! Provides reusable Gherkin steps for common testing scenarios.

use std::time::Duration;

use anyhow::Result;
use cucumber::{given, then, when};
use fantoccini::Locator;

use super::AppWorld;
use super::console_log::ConsoleLog;

/// Step: Given Goto /path
///
/// Navigates to a specific path on the server.
///
/// # Example
/// ```gherkin
/// Given Goto /
/// Given Goto /dashboard
/// ```
#[given(regex = r"^Goto (.+)$")]
pub async fn goto_dynamic_path(world: &mut AppWorld, path: String) -> Result<()> {
    world.goto_path(&path).await?;

    Ok(())
}

/// Step: Then I see an "element" with text "content"
///
/// Verifies that a specific element exists with exact text content.
///
/// # Example
/// ```gherkin
/// Then I see an "h1" with text "Welcome to Leptos!"
/// Then I see a "p" with text "Loading..."
/// ```
#[then(regex = r#"^I see an? "([^"]+)" with text "([^"]+)"$"#)]
async fn i_see_element_with_text(
    world: &mut AppWorld,
    element_type: String,
    text: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let h1_text = world
        .find(Locator::Css(&element_type))
        .await?
        .text()
        .await?;

    assert_eq!(h1_text, text, "Element text doesn't match expected value");

    Ok(())
}

/// Step: Then I see a button with "text"
///
/// Verifies that a button exists with specific text.
///
/// # Example
/// ```gherkin
/// Then I see a button with "Connect"
/// Then I see a button with "Click Me: 0"
/// ```
#[then(regex = r#"I see a button with "(.*)""#)]
pub async fn check_button_with_text(world: &mut AppWorld, expected_text: String) -> Result<()> {
    let button_text = world.find(Locator::Css("button")).await?.text().await?;

    assert_eq!(button_text, expected_text);

    Ok(())
}

/// Step: When I click the button labeled "text"
///
/// Finds a button with specific text and clicks it.
///
/// # Example
/// ```gherkin
/// When I click the button labeled "Connect"
/// When I click the button labeled "Submit"
/// ```
#[when(regex = r#"I click the button labeled "(.*)""#)]
pub async fn click_button_with_label(world: &mut AppWorld, label: String) -> Result<()> {
    let button = world.find(Locator::Css("button")).await?;
    let button_text = button.text().await?;

    // Verify button has expected label before clicking
    assert_eq!(button_text, label);

    button.click().await?;

    Ok(())
}

/// Step: Then the button label changes to "text"
///
/// Verifies that the button text has changed to a new value.
///
/// # Example
/// ```gherkin
/// Then the button label changes to "Disconnect"
/// Then the button label changes to "Click Me: 1"
/// ```
#[then(regex = r#"the button label changes to "(.*)""#)]
pub async fn check_button_label(world: &mut AppWorld, expected_label: String) -> Result<()> {
    let button_text = world.find(Locator::Css("button")).await?.text().await?;

    assert_eq!(button_text, expected_label);
    Ok(())
}

/// Step: Then I should see the following console logs:
///
/// Validates that specific console logs appear in order.
///
/// # Table Format
/// ```gherkin
/// Then I should see the following console logs:
///   | message             | level |
///   | "User logged in"    | log   |
///   | "Connection failed" | error |
/// ```
///
/// # Notes
/// - Waits up to 1 second for logs to appear
/// - Clears logs after validation for next step
#[then("I should see the following console logs:")]
pub async fn check_console_logs_table(
    world: &mut AppWorld,
    step: &cucumber::gherkin::Step,
) -> Result<()> {
    let table = step
        .table
        .as_ref()
        .ok_or_else(|| anyhow::Error::msg("Expected data table"))?;
    let expected_console_log = ConsoleLog::from_table(table)?;

    // Wait for logs to appear (1 second timeout)
    let logs = world
        .wait_for_console_logs(&expected_console_log, Duration::from_secs(1))
        .await?;

    assert_eq!(expected_console_log, logs);

    // Clear logs for next step
    world.clear_console_logs().await?;

    Ok(())
}
