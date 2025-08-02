use crate::{AppWorld, Result};
use cucumber::{given, then, when};
use fantoccini::Locator;

#[given("I am on the homepage")]
pub async fn go_to_homepage(world: &mut AppWorld) -> Result<()> {
    let addr = &world.leptos_site_addr;
    world
        .client
        .as_mut()
        .expect("Client not initialized")
        .goto(addr)
        .await?;
    Ok(())
}

#[then(regex = r#"I see an h1 with text "(.*)""#)]
pub async fn check_h1_text(world: &mut AppWorld, expected_text: String) -> Result<()> {
    let h1_text = world
        .client
        .as_mut()
        .expect("Client not initialized")
        .find(Locator::Css("h1"))
        .await?
        .text()
        .await?;
    assert_eq!(h1_text, expected_text);
    Ok(())
}

#[when(regex = r#"I click the button labeled "(.*)""#)]
pub async fn click_button_with_label(world: &mut AppWorld, label: String) -> Result<()> {
    let button = world
        .client
        .as_mut()
        .expect("Client not initialized")
        .find(Locator::Css("button"))
        .await?;
    let button_text = button.text().await?;

    assert_eq!(button_text, label);

    button.click().await?;

    Ok(())
}

#[then(regex = r#"the button label changes to "(.*)""#)]
pub async fn check_button_label(world: &mut AppWorld, expected_label: String) -> Result<()> {
    // Sometimes after clicking, you might want to wait for update
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;

    let button_text = world
        .client
        .as_mut()
        .expect("Client not initialized")
        .find(Locator::Css("button"))
        .await?
        .text()
        .await?;

    assert_eq!(button_text, expected_label);
    Ok(())
}
