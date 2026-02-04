use app::*;
use axum::Router;
use leptos::prelude::*;
use leptos_axum::{LeptosRoutes, generate_route_list};

use crate::Error;

pub struct Server;

impl Server {
    pub async fn setup() -> Result<(), Error> {
        let conf = get_configuration(None)?;
        let addr = conf.leptos_options.site_addr;
        let leptos_options = conf.leptos_options;
        let routes = generate_route_list(App);

        let app = Router::new()
            .leptos_routes(&leptos_options, routes, {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            })
            .fallback(leptos_axum::file_and_error_handler(shell))
            .with_state(leptos_options);

        let listener = tokio::net::TcpListener::bind(&addr)
            .await
            .map_err(|e| Error::AdressUsed { addr, source: e })?;

        {% if tracing == true %}tracing::info!("Listening on: {addr:#?}");{% else %}println!("Listening on: {addr:#?}");{% endif %}

        axum::serve(listener, app.into_make_service())
            .await
            .map_err(|e| Error::AdressUsed { addr, source: e })?;

        Ok(())
    }
    {%- if cucumber == true %}

    #[cfg(feature = "cucumber")]
    pub async fn cucumber_setup(
        addr: std::net::SocketAddr,
        cargo_toml_path: Option<&str>,
        sender: tokio::sync::oneshot::Sender<()>,
    ) -> Result<(), Error> {
        let conf = get_configuration(cargo_toml_path)?;

        let leptos_options = conf.leptos_options;
        let routes = generate_route_list(App);

        let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(1)
            .ok_or_else(|| {
                Error::Io(std::io::Error::new(
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

        let app = Router::new()
            .leptos_routes(&leptos_options, routes, {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            })
            .fallback(leptos_axum::file_and_error_handler(shell))
            .with_state(leptos_options);

        let listener = tokio::net::TcpListener::bind(&addr)
            .await
            .map_err(|e| Error::AdressUsed { addr, source: e })?;

        tracing::info!("Listening: {addr:?}");

        // Signal that server is ready
        let _ = sender.send(());

        axum::serve(listener, app.into_make_service())
            .await
            .map_err(|e| Error::AdressUsed { addr, source: e })?;

        Ok(())
    }
    {%- endif %}
}
