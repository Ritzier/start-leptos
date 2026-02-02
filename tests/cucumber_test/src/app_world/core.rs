use std::net::SocketAddr;

use anyhow::{Context, Result};
use cucumber::World;
use fantoccini::Locator;
use fantoccini::elements::Element;

use crate::{Webdriver, get_server_addr};

#[derive(Debug, World)]
#[world(init = Self::new)]
pub struct AppWorld {
    pub webdriver: Webdriver,
    addr: SocketAddr,
}

impl AppWorld {
    pub async fn new() -> Result<Self> {
        let webdriver = Webdriver::new().await?;
        let addr = get_server_addr();

        Ok(Self { webdriver, addr })
    }

    pub async fn goto_path(&mut self, path: &str) -> anyhow::Result<()> {
        let path = path.strip_prefix('/').unwrap_or(path);
        let target_url = format!("http://{}/{}", self.addr, path);

        self.webdriver
            .client
            .goto(&target_url)
            .await
            .context(format!("Failed to navigate to {}", target_url))?;

        Ok(())
    }

    pub async fn find<'a>(&mut self, locator: Locator<'a>) -> Result<Element> {
        self.webdriver
            .client
            .find(locator)
            .await
            .map_err(Into::into)
    }
}
