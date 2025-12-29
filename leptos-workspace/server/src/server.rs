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

        axum::serve(listener, app.into_make_service())
            .await
            .map_err(|e| Error::AdressUsed { addr, source: e })?;

        Ok(())
    }
}
