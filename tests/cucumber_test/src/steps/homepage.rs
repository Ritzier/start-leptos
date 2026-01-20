{% if websocket == "yes" -%}
use crate::{AppWorld, Result};
use cucumber::{given, then, when};
use fantoccini::Locator;

#[given("I am on the homepage")]
pub async fn go_to_homepage(world: &mut AppWorld) -> Result<()> {
    let addr = &world.leptos_site_addr;
    let client = world.client.as_mut().expect("Client not initialized");

    client.goto(addr).await?;

    // Enable browser logging
    client
        .execute(
            r#"
        window.__consoleLogs = [];
        ['log', 'info', 'warn', 'error'].forEach(level => {
            const original = console[level];
            console[level] = function(...args) {
                window.__consoleLogs.push({
                    level: level,
                    message: args.join(' '),
                    timestamp: Date.now()
                });
                original.apply(console, args);
            };
        });
        "#,
            vec![],
        )
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
    // Wait for UI update
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

#[then(regex = r#"I see a button with "(.*)""#)]
pub async fn check_button_with_text(world: &mut AppWorld, expected_text: String) -> Result<()> {
    let button_text = world
        .client
        .as_mut()
        .expect("Client not initialized")
        .find(Locator::Css("button"))
        .await?
        .text()
        .await?;
    assert_eq!(button_text, expected_text);

    Ok(())
}

#[then(regex = r#"I should see console log containing "(.*)""#)]
pub async fn check_console_log(world: &mut AppWorld, expected_message: String) -> Result<()> {
    // Wait a bit for console logs to be captured
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    let client = world.client.as_mut().expect("Client not initialized");

    // Retrieve console logs from the browser
    let logs = client
        .execute("return window.__consoleLogs || [];", vec![])
        .await?;

    let logs_array = logs
        .as_array()
        .ok_or_else(|| anyhow::Error::msg("Failed to get console logs as array"))?;

    // Check if any log contains the expected message
    let found = logs_array.iter().any(|log| {
        if let Some(message) = log.get("message").and_then(|m| m.as_str()) {
            message.contains(&expected_message)
        } else {
            false
        }
    });

    assert!(
        found,
        "Expected console log containing '{}' not found. Logs: {:?}",
        expected_message, logs_array
    );

    Ok(())
}
{% else -%}
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
{% endif -%}
