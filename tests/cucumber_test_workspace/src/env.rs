use crate::Result;
use std::env;

const WEBDRIVER: &str = "WEBDRIVER";
const LEPTOS_SITE_ADDR: &str = "LEPTOS_SITE_ADDR";

pub struct Dotenv {
    pub webdriver: String,
    pub leptos_site_addr: String,
}

impl Dotenv {
    pub fn new() -> Result<Self> {
        let webdriver = env::var(WEBDRIVER)?;
        let leptos_site_addr = env::var(LEPTOS_SITE_ADDR)?;

        Ok(Self {
            webdriver,
            leptos_site_addr,
        })
    }
}
