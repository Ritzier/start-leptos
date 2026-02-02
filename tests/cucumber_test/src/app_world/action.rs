use std::time::Duration;

use anyhow::Result;
use cucumber::{given, then, when};
use fantoccini::Locator;

use super::AppWorld;

#[given(regex = r"^Goto (.+)$")]
pub async fn goto_dynamic_path(world: &mut AppWorld, path: String) -> Result<()> {
    world.goto_path(&path).await?;

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
    tokio::time::sleep(Duration::from_secs(1)).await;

    let button_text = world.find(Locator::Css("button")).await?.text().await?;

    assert_eq!(button_text, expected_label);
    Ok(())
}
