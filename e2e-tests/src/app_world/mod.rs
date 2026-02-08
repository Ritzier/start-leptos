//! Cucumber World and step definitions.
//!
//! Contains the test context (`AppWorld`) and reusable step definitions
//! for browser automation.

mod action;
mod console_log;
mod core;

pub use console_log::ConsoleLog;
pub use core::AppWorld;
