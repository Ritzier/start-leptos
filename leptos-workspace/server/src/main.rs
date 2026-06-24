#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> Result<(), color_eyre::Report> {
    use server::*;
    use tokio::signal::unix::{SignalKind, signal};

    // Install color-eyre panic and error reporting
    color_eyre::install()?;

    {% if tracing == true -%}
    // Initialize structured logging/tracing
    Trace::setup();

    {% endif -%}
    // Load development environment variables when running in debug mode
    #[cfg(debug_assertions)]
    Env::setup().await;

    // Build and configure the application server
    let server = Server::new().await?;

    // Listen for common `Unix` shutdown signals
    let mut sigint = signal(SignalKind::interrupt()).map_err(color_eyre::Report::from)?;
    let mut sigterm = signal(SignalKind::terminate()).map_err(color_eyre::Report::from)?;

    // Run the server until either:
    // - the server exits with an error, or
    // - the process receives a shutdown signal
    tokio::select! {
        result = server.serve() => {
             if let Err(err) = result {
                Err::<(), _>(color_eyre::Report::from(err))?;
            }
        }

        _ = sigint.recv() => {
            {%- if tracing == true %}
            tracing::info!("Received SIGTINT");
            {%- else %}
            leptos::logging::log!("Received SIGTINT");
            {%- endif %}
        }

        _ = sigterm.recv() => {
            {%- if tracing == true %}
            tracing::info!("Received SIGTERM");
            {%- else %}
            leptos::logging::log!("Received SIGTERM");
            {%- endif %}
        }
    }

    Ok(())
}

#[cfg(not(feature = "ssr"))]
fn main() {}
