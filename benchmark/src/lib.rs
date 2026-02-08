//! Benchmark library for performance testing Leptos applications.
//!
//! This library provides CLI tools and benchmark runners that measure
//! WebSocket connection performance or UI interaction timings depending
//! on the application's feature flags.
//!
//! # Architecture
//! - `cli`: Command-line argument parsing
//! - `benchmarks`: Core benchmark logic and results tracking

mod cli;
pub use cli::Cli;

mod benchmarks;
pub use benchmarks::Benchmarks;
