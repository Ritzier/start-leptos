use std::net::SocketAddr;

use anyhow::{Context, Result};
use cucumber::World;
use fantoccini::Locator;
use fantoccini::elements::Element;
use serde_json::Value;

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

        // Enable logging with sessionStorage (persists across page updates)
        self.execute(
            r#"
            if (!window.__consoleLoggerInstalled__) {
                window.__consoleLoggerInstalled__ = true;
                
                // Initialize storage
                if (!sessionStorage.getItem('__consoleLogs__')) {
                    sessionStorage.setItem('__consoleLogs__', JSON.stringify([]));
                }
                
                ['log', 'info', 'warn', 'error', 'debug'].forEach(method => {
                    const original = console[method];
                    console[method] = function(...args) {
                        const logs = JSON.parse(sessionStorage.getItem('__consoleLogs__') || '[]');
                        logs.push({
                            level: method,
                            message: args.map(arg => {
                                try {
                                    return typeof arg === 'object' ? JSON.stringify(arg) : String(arg);
                                } catch(e) {
                                    return String(arg);
                                }
                            }),
                            timestamp: Date.now()
                        });
                        sessionStorage.setItem('__consoleLogs__', JSON.stringify(logs));
                        original.apply(console, args);
                    };
                });
            }
            "#,
            vec![],
        )
        .await?;

        Ok(())
    }

    pub async fn execute(&mut self, script: &str, args: Vec<Value>) -> Result<Value> {
        self.webdriver
            .client
            .execute(script, args)
            .await
            .map_err(Into::into)
    }

    pub async fn find<'a>(&mut self, locator: Locator<'a>) -> Result<Element> {
        self.webdriver
            .client
            .find(locator)
            .await
            .map_err(Into::into)
    }
}
