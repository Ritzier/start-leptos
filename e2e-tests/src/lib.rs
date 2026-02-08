//! End-to-end testing library for Leptos applications.
//!
//! This library provides infrastructure for browser-based testing using:
//! - **Cucumber**: BDD testing with Gherkin syntax
//! - **WebDriver**: Browser automation (Chrome/Firefox)
//! - **Console log validation**: Verify JavaScript console output
//!
//! # Architecture
//! - `app_world`: Test world and step definitions
//! - `leptos_server`: Server lifecycle management
//! - `utils`: WebDriver setup and port management
//! - `trace`: Logging configuration
//!
//! # Example
//! ```rust
//! use e2e_tests::{LeptosServer, AppWorld};
//!
//! // Start server
//! LeptosServer::serve_and_wait(5).await?;
//!
//! // Run tests
//! let mut world = AppWorld::new().await?;
//! world.goto_path("/").await?;
//! ```

mod app_world;
pub use app_world::{AppWorld, ConsoleLog};

mod leptos_server;
pub use leptos_server::LeptosServer;

mod trace;
pub use trace::Trace;

mod utils;
use utils::{PortFinder, Webdriver, get_server_addr, set_server_addr};

mod run;
pub use run::cucumber_test;
