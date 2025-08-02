use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicU16, Ordering};

use cucumber::World;
use fantoccini::wd::Capabilities;
use fantoccini::{Client, ClientBuilder};
use serde_json::json;

use crate::{Result, env::Dotenv};

static PORT: AtomicU16 = AtomicU16::new(3311);

#[derive(Debug, World)]
#[world(init = Self::new)]
pub struct AppWorld {
    pub client: Option<Client>,
    pub leptos_site_addr: String,
    child: Child,
}

impl AppWorld {
    async fn new() -> Result<Self> {
        let Dotenv {
            webdriver,
            leptos_site_addr,
        } = Dotenv::new()?;

        let (client, child) = match webdriver.as_ref() {
            "geckodriver" => build_geckodriver().await?,
            "chromedriver" => build_chromedriver().await?,
            unknown_webdriver => {
                return Err(anyhow::Error::msg(format!(
                    "Unknown webdriver: {unknown_webdriver}"
                )));
            }
        };

        let leptos_site_addr = format!("http://{leptos_site_addr}");

        Ok(Self {
            client: Some(client),
            leptos_site_addr,
            child,
        })
    }
}

async fn build_chromedriver() -> Result<(Client, Child)> {
    let port = PORT.load(Ordering::Relaxed);

    let child = Command::new("chromedriver")
        .arg(format!("--port={port}"))
        .stdout(Stdio::null()) // silence output
        .stderr(Stdio::null())
        .spawn()?;

    let cap: Capabilities = serde_json::from_str(
        r#"{"browserName":"chrome","goog:chromeOptions":{"args":["--headless"]}}"#,
    )?;

    let client = ClientBuilder::native()
        .capabilities(cap)
        .connect(&format!("http://localhost:{port}"))
        .await?;

    Ok((client, child))
}

async fn build_geckodriver() -> Result<(Client, Child)> {
    let port = PORT.fetch_add(1, Ordering::SeqCst);

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

impl Drop for AppWorld {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}
