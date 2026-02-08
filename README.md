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
cargo generate ritzier/start-leptos
```

### Interacitve Prompts

- **Websocket?** (default: false) - Enable real-time **Websocket** communication with `rkyv`
- **Tracing?** (default: false) - Add structed logging with `tracing`
- **Style?**: Choices: `default`, `unocss` (default: `default`)
- **Docker?** (default: false) - Include **Docker** setup with multi-stage builds
- **Cucumber?** (default: false) - Add BDD end-to-end testing
  - **Benchmark?** (default: false) - Add performance benchmarking

### Commands

```bash
# Development (hot reload)
cargo leptos watch

# Build production
cargo leptos build --release
```

## Docker Deployment

When `Docker` is enabled during setup

### Build and Run

```bash
# Build image
docker compose build

# Run container
docker compose up -d

# View logs
docker compose logs -f
```

### Configuration

The `Docker` setup uses:

- **Multi-stage build**: optimized builder and runtime stages
- **Debian bookworm-slim**: Better network-performance for Websocket apps
- **Layer caching**: Seperate dependency and source code layers for faster rebuilds
- **Non-root user**: Enhanced security with dedicated `appuser`
- **Health checks**: Automatic service monitoring
- **Port mapping**: Exposes port `3000` by default

Modify `docker-compose.yml` to customize:

```bash
services:
  leptos-app:
    ports:
      - "8000:3000"  # Map to different host port
    environment:
      - RUST_LOG=debug  # Adjust log level
```

## Structure

```text
my-leptos-app/
├── Cargo.toml              # Workspace config
├── Dockerfile              # Multi-stage Docker build (if `Docker` enabled)
├── docker-compose.yml      # Container orchestration (if `Docker` enabled)
├── uno.config.ts           # UnoCSS config (if `Unocss` selected)
├── package.json            # Node deps (if `UnoCSS` selected)
├── app/                    # Shared app logic
│   └── src/
│       ├── pages/          # Lazy-loaded route pages
│       └── structs/        # WebSocket structs (if `Websocket` enabled)
├── frontend/               # WASM library
├── server/                 # Axum SSR server
├── style/                  # SCSS styles
├── public/
│   └── uno.css             # Generated UnoCSS (if `Unocss` selected)
├── e2e-tests/              # Cucumber BDD tests (if Cucumber enabled)
│   ├── features/           # Gherkin feature files
│   └── src/
│       ├── app_world/      # Test world and step definitions
│       └── utils/          # WebDriver helpers
└── benchmark/              # Performance benchmarking (if Benchmark enabled)
    └── src/
        ├── benchmarks/     # Benchmark implementations
        └── cli.rs          # CLI argument parsing
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

## Testing && Benchmarking

### Cucumber (BDD)

When enabled, provides behavior-driven development testing with:

- **WebDriver automation**: Browser testing with `Fantoccini`
- **Cross-browser support**: Chrome (`chromedriver`) and Firefox (`geckodriver`)
- **Gherkin syntax**: Human-readable test scenario in `e2e-tests/feature/`
- **Console log validation**: Verify console output
- **Test helpers**: Pre-configured `AppWorld` with comman step definitions

```bash
# Run all feature files
cargo run --bin cucumber

# Set WebDriver (default: chromedriver)
WEBDRIVER=geckodriver cargo run --bin cucumber
```

### Benchmark (Performance Testing)

**Note**: Only available when `Cucumber` is enabled (shares test infrastructure)

Provides performance benchmarking with statistical analysis:

- **CLI interface**: Specify iteration count via `command-line` argument

- **Flexible metrics**: Dynamic benchmark naming with `HashMap-based` storage

- **Statistical analysis**: Calculates avg, min, max, median, and standard deviation

- **Colorized output**: Easy-to-read terminal output with `owo-colors`

- **Feature-aware**: Different benchmarks for `WebSocket` vs `default mode`

#### Benchmarks:

##### WebSocket Mode:

- Connect handshake timing

- Disconnect closure timing

##### Default Mode:

- Button click and state update timing

##### Running Benchmarks

```bash
# Run 20 iterations
cargo run --bin benchmark -- 20
```

## Template Features

- **Workspace architecture**: Modular `app/frontend/server` separation

- **Lazy loading by default**: Automatically code-split into separate WASM chunks that load on-demand

- **Optional WebSocket**: Real-time communication with `rkyv` encoding

- **Optional testing**: BDD tests with `Cucumber` + optional performance benchmarking

- **Conditional setup**: Only includes selected features

- **Hot reload**: **Leptos** watch mode with live CSS injection
