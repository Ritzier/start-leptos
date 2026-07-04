use std::net::SocketAddr;

use app::{App, shell};
use axum::Router;
use leptos::prelude::*;
use leptos_axum::{LeptosRoutes, generate_route_list};
use tokio::net::TcpListener;
use tokio_util::sync::CancellationToken;

use super::errors::ServerError;

pub struct AxumServer {
    listener: TcpListener,
    app: Router,
    addr: SocketAddr,
    shutdown: CancellationToken,
}

impl AxumServer {
    pub async fn new(shutdown: CancellationToken) -> Result<Self, ServerError> {
        let conf = get_configuration(None)?;
        let addr = conf.leptos_options.site_addr;
        let leptos_options = conf.leptos_options;

        // build `router`
        let app = Self::build_router(leptos_options)?;

        let listener = tokio::net::TcpListener::bind(&addr)
            .await
            .map_err(|e| ServerError::AdressUsed { addr, source: e })?;

        Ok(Self {
            listener,
            app,
            addr,
            shutdown,
        })
    }
    {%- if cucumber == true %}

    #[cfg(feature = "cucumber")]
    pub async fn cucumber_new(
        addr: SocketAddr,
        cargo_toml_path: Option<&str>,
        shutdown: CancellationToken,
    ) -> Result<Self, ServerError> {
        let conf = get_configuration(cargo_toml_path)?;
        let leptos_options = conf.leptos_options;

        let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(1)
            .ok_or_else(|| {
                ServerError::Io(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Failed to find parent directory of `CARGO_MANIFEST_DIR`",
                ))
            })?;

        // Create symlink for WASM file compatibility
        let pkg = manifest_dir.join("target").join("site").join("pkg");
        let wasm_file = pkg.join(format!("{}.wasm", leptos_options.output_name));
        let bg_wasm_file = pkg.join(format!("{}_bg.wasm", leptos_options.output_name));

        // Create symlink only if _bg.wasm doesn't exist
        if !bg_wasm_file.exists() {
            tokio::fs::symlink(&wasm_file, &bg_wasm_file).await?;
        }

        // build `router`
        let app = Self::build_router(leptos_options)?;

        let listener = tokio::net::TcpListener::bind(&addr)
            .await
            .map_err(|e| ServerError::AdressUsed { addr, source: e })?;

        Ok(Self {
            listener,
            app,
            addr,
            shutdown,
        })
    }
    {%- endif %}

    fn build_router(leptos_options: LeptosOptions) -> Result<Router, ServerError> {
        let routes = generate_route_list(App);

        let router = Router::new()
            .leptos_routes(&leptos_options, routes, {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            })
            .fallback(leptos_axum::file_and_error_handler(shell))
            .with_state(leptos_options);

        Ok(router)
    }

    pub async fn serve(self) -> Result<(), ServerError> {
        let Self {
            listener,
            app,
            addr,
            shutdown,
        } = self;

        axum::serve(listener, app.into_make_service())
            .with_graceful_shutdown(async move {
                shutdown.cancelled().await;
                tracing::info!("Axum shutting down");
            })
            .await
            .map_err(|e| ServerError::AdressUsed { addr, source: e })
    }
}
