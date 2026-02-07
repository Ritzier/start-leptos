use std::time::Duration;

use anyhow::Result;
use cucumber::{given, then, when};
use fantoccini::Locator;

use super::AppWorld;
use super::console_log::ConsoleLog;

#[given(regex = r"^Goto (.+)$")]
pub async fn goto_dynamic_path(world: &mut AppWorld, path: String) -> Result<()> {
    world.goto_path(&path).await?;

    Ok(())
}

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

    assert_eq!(h1_text, text, "Wtf");

    Ok(())
}

#[then(regex = r#"I see a button with "(.*)""#)]
pub async fn check_button_with_text(world: &mut AppWorld, expected_text: String) -> Result<()> {
    let button_text = world.find(Locator::Css("button")).await?.text().await?;

    assert_eq!(button_text, expected_text);

    Ok(())
}

#[when(regex = r#"I click the button labeled "(.*)""#)]
pub async fn click_button_with_label(world: &mut AppWorld, label: String) -> Result<()> {
    let button = world.find(Locator::Css("button")).await?;
    let button_text = button.text().await?;

    assert_eq!(button_text, label);

    button.click().await?;

    Ok(())
}

#[then(regex = r#"the button label changes to "(.*)""#)]
pub async fn check_button_label(world: &mut AppWorld, expected_label: String) -> Result<()> {
    let button_text = world.find(Locator::Css("button")).await?.text().await?;

    assert_eq!(button_text, expected_label);
    Ok(())
}

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

    let logs = world
        .wait_for_console_logs(&expected_console_log, Duration::from_secs(1))
        .await?;

    assert_eq!(expected_console_log, logs);

    // Clear Webdriver log
    world.clear_console_logs().await?;

    Ok(())
}
