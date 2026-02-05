use std::env;
use std::process::Stdio;

use anyhow::{Result, anyhow};
use fantoccini::wd::Capabilities;
use fantoccini::{Client, ClientBuilder};
use serde_json::json;
use tokio::process::{Child, Command};

use crate::PortFinder;

#[derive(Debug)]
pub struct Webdriver {
    pub client: Client,
    #[expect(dead_code)]
    child: Child,
}

impl Webdriver {
    pub async fn new() -> Result<Self> {
        let (client, child) = match env::var("WEBDRIVER") {
            Err(_) => build_chromedriver().await?,
            Ok(webdriver_env) => match webdriver_env.to_lowercase().as_str() {
                "chromedriver" | "chrome" => build_chromedriver().await?,
                "geckodriver" | "gecko" => build_geckodriver().await?,
                invalid => return Err(anyhow!("Invalid WEBDRIVER value: `{invalid}`")),
            },
        };

        Ok(Self { client, child })
    }
}

async fn build_chromedriver() -> Result<(Client, Child)> {
    let port = PortFinder::get_available_port()
        .await
        .map_err(|e| anyhow!("{e}"))?;

    let child = Command::new("chromedriver")
        .arg(format!("--port={port}"))
        .stdout(Stdio::null()) // silence output
        .stderr(Stdio::null())
        .spawn()?;

    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let cap: Capabilities = serde_json::from_str(
        r#"{"goog:loggingPrefs":{"browser":"ALL","performance":"ALL"},"browserName":"chrome","goog:chromeOptions":{"args":["--headless"]}}"#,
    )?;

    let client = ClientBuilder::native()
        .capabilities(cap)
        .connect(&format!("http://localhost:{port}"))
        .await?;

    Ok((client, child))
}

async fn build_geckodriver() -> Result<(Client, Child)> {
    let port = PortFinder::get_available_port()
        .await
        .map_err(|e| anyhow!("{e}"))?;

    let child = Command::new("geckodriver")
        .arg(format!("--port={port}"))
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;

    let mut caps = serde_json::Map::new();
    caps.insert(
        "moz:firefoxOptions".to_string(),
        json!({ "args": ["--headless", "-headless"] }),
    );

    let webdriver_url = format!("http://localhost:{port}");
    let client = ClientBuilder::native()
        .capabilities(caps)
        .connect(&webdriver_url)
        .await?;

    Ok((client, child))
}
