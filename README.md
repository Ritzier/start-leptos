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
├── Dockerfile              # Multi-stage Docker build (if Docker enabled)
├── docker-compose.yml      # Container orchestration (if Docker enabled)
├── uno.config.ts           # UnoCSS config (if UnoCSS selected)
├── package.json            # Node deps (if UnoCSS selected)
├── app/                    # Shared app logic
│   └── src/
│       ├── pages/          # Lazy-loaded route pages
│       │   └── home/
│       │       ├── page.rs
│       │       └── ws/     # WebSocket implementation (if WebSocket enabled)
│       └── ws_core/        # Generic WebSocket traits (if WebSocket enabled)
│           ├── client.rs   # Client-side trait & manager
│           └── server.rs   # Server-side trait & backend
├── frontend/               # WASM library
├── server/                 # Axum SSR server
├── style/                  # SCSS styles
├── public/
│   └── uno.css             # Generated UnoCSS (if UnoCSS selected)
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

The template includes a generic, reusable WebSocket system:

#### Core Traits

- `WebSocketClient` trait: Define client-side message handling
- `WebSocketMessage` trait: Define server-side message processing
- `GenericWebSocketManager<T>`: Type-safe connection manager
- `GenericWebsocketBackend<T>`: Generic server-side handler

#### Built-in Implementation

- `rkyv` serialization: Zero-copy binary encoding

- Type-safe `Request/Response` enums with `Archive`, `Deserialize`, `Serialize` traits

- Automatic connection lifecycle management with reactive signals

#### Basic Usage (Home Page)

```rust
use uuid::Uuid;

// Connect to WebSocket
let manager = WebSocketManager::new(Uuid::new_v4());
manager.connect();

// WebSocket automatically handles handshake
// UI reactively updates via is_connected signal

// Disconnect
manager.disconnect();
```

#### Creating Custom WebSocket Endpoints

```rust
// 1. Define your message types
#[derive(Debug, Clone, Archive, Deserialize, Serialize)]
pub enum MyRequest {
    Subscribe { topic: String },
    Unsubscribe,
}

#[derive(Debug, Clone, Archive, Deserialize, Serialize)]
pub enum MyResponse {
    Data { payload: String },
    Error { message: String },
}

// 2. Implement WebSocketClient trait
use crate::ws_core::client::{WebSocketClient, GenericWebSocketManager};

#[derive(Clone)]
pub struct MyWebSocketClient {
    topic: StoredValue<String>,
}

impl WebSocketClient for MyWebSocketClient {
    type Request = MyRequest;
    type Response = MyResponse;

    fn create_handshake_request(&self) -> Self::Request {
        MyRequest::Subscribe {
            topic: self.topic.get_value(),
        }
    }

    fn create_disconnect_request(&self) -> Self::Request {
        MyRequest::Unsubscribe
    }

    fn handle_response(response: Self::Response, is_connected: RwSignal<bool>) {
        match response {
            MyResponse::Data { payload } => {
                is_connected.set(true);
                leptos::logging::log!("Received: {payload}");
            }
            MyResponse::Error { message } => {
                leptos::logging::error!("Error: {message}");
            }
        }
    }

    async fn get_stream(
        rx: UnboundedReceiver<Result<Self::Request, ServerFnError>>,
    ) -> Result<BoxedStream<Self::Response, ServerFnError>, ServerFnError> {
        my_websocket(rx.into()).await
    }
}

// 3. Implement server-side handler
#[cfg(feature = "ssr")]
use crate::ws_core::server::WebSocketMessage;

pub struct MyWebSocketHandler;

impl WebSocketMessage for MyWebSocketHandler {
    type Request = MyRequest;
    type Response = MyResponse;

    fn handle_request(
        request: Self::Request,
        tx: &UnboundedSender<Result<Self::Response, ServerFnError>>,
    ) -> bool {
        match request {
            MyRequest::Subscribe { topic } => {
                tracing::info!("Subscribed to: {topic}");
                tx.unbounded_send(Ok(MyResponse::Data {
                    payload: format!("Welcome to {topic}"),
                })).ok();
                true // Keep connection alive
            }
            MyRequest::Unsubscribe => {
                tracing::info!("Unsubscribed");
                false // Close connection
            }
        }
    }
}

// 4. Create server function
#[server(protocol = Websocket<RkyvEncoding, RkyvEncoding>)]
pub async fn my_websocket(
    input: BoxedStream<MyRequest, ServerFnError>,
) -> Result<BoxedStream<MyResponse, ServerFnError>, ServerFnError> {
    use crate::ws_core::server::GenericWebsocketBackend;

    let (tx, rx) = mpsc::unbounded();
    let backend = GenericWebsocketBackend::<MyWebSocketHandler>::new(input, tx);

    tokio::spawn(async move {
        backend.serve().await;
    });

    Ok(rx.into())
}

// 5. Use in your component
pub type MyWebSocketManager = GenericWebSocketManager<MyWebSocketClient>;

let manager = MyWebSocketManager::new(topic);
manager.connect();
```

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

## WebSocket File Structure

```text
app/src/
├── ws_core/                  # Generic WebSocket abstractions
│   ├── mod.rs
│   ├── client.rs             # WebSocketClient trait + GenericWebSocketManager
│   └── server.rs             # WebSocketMessage trait + GenericWebsocketBackend
└── pages/
    └── home/
        ├── mod.rs
        ├── page.rs           # Page component
        └── ws/               # Page-specific WebSocket implementation
            ├── mod.rs
            ├── messages.rs   # Request/Response enums
            ├── client.rs     # Implements WebSocketClient
            ├── connection.rs # Server function
            └── handler.rs    # Implements WebSocketMessage (SSR only)
```
