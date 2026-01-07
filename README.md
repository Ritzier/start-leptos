# Leptos Axum Starter Template

Leptos template with Axum SSR, optional testing, and automated workflows

## Quick Start

Prerequisites

```bash
# Install Rust WASM target
rustup target add wasm32-unknown-unknown

# Install tools
cargo install cargo-leptos cargo-generate cargo-make
```

Create Project

```bash
# Method 1: cargo-generate (interactive)
cargo generate --git https://github.com/ritzier/start-leptos-workspace

# Method 2: cargo-leptos
cargo leptos new --git https://github.com/ritzier/start-leptos-workspace my-app
```

### Interacitve Prompts

````
? What is the project name? my-leptos-app
? Websocket? no
? Style? default
? Makefile? yes
? Makefile: (Choose with space, confirm with Enter)
[x] Cucumber
[ ] Playwright```

### Commands

```bash
# Development (hot reload)
cargo leptos watch

# Build production
cargo leptos build --release

# Run tests
cargo make chrome        # Cucumber Chrome
cargo make firefox       # Cucumber Firefox
cargo make playwright    # Playwright E2E

# Full test suite
cargo make both
````

## Structure

```text
my-leptos-app/
├── Cargo.toml          # Workspace config
├── Makefile.toml       # Task runner (optional)
├── uno.config.ts       # UnoCSS config (if selected)
├── package.json        # UnoCSS deps (if selected)
├── app/                # Shared app logic
│   └── src/
│       ├── pages/      # Lazy-loaded route pages
│       └── structs/    # WebSocket structs (if enabled
├── frontend/           # WASM library
├── server/             # Axum SSR server
├── style/              # SCSS styles
├── public/
│ └── uno.css           # Generated UnoCSS (if selected)
├── makefile/           # Task definitions (optional)
│ ├── leptos.toml
│ ├── cucumber.toml     # If Cucumber selected
│ └── playwright.toml   # If Playwright selected
└── tests/
    ├── cucumber_test/  # BDD tests (if selected)
    └── playwright/     # E2E tests (if selected)
```

## Features

### Lazy Loading (Default)

This template used **lazy loading with code-splitting** by default. Application is automatically split into smaller
`WASM` chunks that load on-demand.

### Websocket (Optional)

Enable real-time bidirectional communication with optional `Websocket` support

#### When Enabled

The template includes:

- `WebsocketManager`: Connection lifecycle management with `StoredValue` and `RwSignal` for efficient cloning
- `rkyv` **serialization**: Zero-copy binary encoding
- `Request`/`Response` **enums**: Type-safe message handling with `Archive`, `Deserialize`, `Serialize` traits

Usage Example:

```rust
// Connect to WebSocket
let manager = WebSocketManager::new(Uuid::new_v4());
manager.connect();

// Send messages
manager.send(Request::CustomMessage { data: "hello" })?;

// Disconnect
manager.disconnect();
```

The manager automatically handles connection state, message sending, and stream processing in a spawned task

### Tracing (Optional)

Enable structured logging with `tracing` and `tracing-subscriber` for better observability

#### When Enabled

- **Trace module**: Pre-configured `tracing-subscriber` setup with environment-based filtering
- **Automatic log levels**: `debug` in development, `info` in production for both `server` and `app` crates
- **Environment override**: Use `RUST_LOG` environment variable to customize filter directives
- **WebSocket integration**: Replaces `leptos::logging` with `tracing` macros when both features are enabled

## Styling Options

- **Default**: Uses Leptos built-in CSS bundling (`/pkg/{{project-name}}.css`)

- **UnoCSS**: Atomic CSS engine with:
  - Auto pattern scanning from `src/**/*.rs` (Project) or `app/**/*.rs` (Workspace)
  - Output to `public/uno.css`
  - `npm run watch` for development HMR
  - `npm run build` for production minification

## Testing Frameworks

### Cucumber (BDD)

- WebDriver-based browser automation with `Fantoccini`
- Supports Chrome (`chromedriver`) and Firefox (`geckodriver`)
- Feature files in `tests/cucumber_test/features/`
- Run: `cargo make chrome` or `cargo make both`

### Playwright

- Modern E2E testing with Node.js runtime
- Cross-browser support (Chromium/Firefox/WebKit)
- TypeScript test files in `tests/playwright/`
- Run: `cargo make playwright`

## Template Features

- **Workspace architecture**: Modular `app/frontend/server` separation
- **Lazy loading by default**: Automatically be `code-split` into a separate `WASM` chunk that loads on-demand
- **Optional WebSocket**: Real-time communication with `rkyv` encode
- **Conditional test setup**: Only includes selected test frameworks
- **Auto-cleanup**: Template removes unused files after generation
- **Hot reload**: Leptos watch mode with live CSS injection
