<picture>
    <source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_Solid_White.svg" media="(prefers-color-scheme: dark)">
    <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo">
</picture>

# Leptos Axum Starter Template

A comprehensive template for building modern web applications with the [Leptos](https://github.com/leptos-rs/leptos) web
framework and [Axum](https://github.com/tokio-rs/axum). This template includes optional testing frameworks and
development tools

## Feature

- **Multiple Template Types**: Choose between `CSR`, `Project`, or `Workspace` configurations

- **Testing Integration**: Optional `Cucumber` and `Playwright` test suites

- **Tracing Support**: Optional structured logging with tracing

- **Tailwind CSS v4**: Modern `CSS` framework integration

- **WebDriver Testing**: Multi-browser support with `Chrome` and `Firefox`

## Prerequisites

```sh
# Install Rust WebAssembly target
rustup target add wasm32-unknown-unknown

# Install Leptos CLI
cargo install cargo-leptos

# Install cargo-generate (if using cargo generate method)
cargo install cargo-generate

# Install cargo-make for task automation
cargo install cargo-make
```

<details>
<summary>cargo-binstall</summary>

```bash
# Install Rust WebAssembly target
rustup target add wasm32-unknown-unknown

cargo binstall cargo-leptos cargo-generate cargo-make
```

</details>

## Creating Project

**Method 1**: Using `cargo-generate`

```bash
cargo generate ritzier/start-leptos-workspace
```

**Method 2**: Using `cargo-leptos`

```bash
cargo leptos new --git https://github.com/ritzier/start-leptos-workspace/
```

During setup, you'll be prompted to choose:

- **Template type**: CSR, Project, or Workspace

- **Tracing**: Enable structured logging

- **Cucumber testing**: Browser-based integration tests

- **Playwright testing**: End-to-end testing framework
