#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> Result<(), server::Error> {
    use server::*;
    use tokio::signal::unix::{SignalKind, signal};

    {% if tracing == true -%}
    Trace::setup();

    {% endif -%}
    #[cfg(debug_assertions)]
    Env::setup().await;

    let mut sigint = signal(SignalKind::interrupt())?;
    let mut sigterm = signal(SignalKind::terminate())?;

    tokio::select! {
        result = Server::setup() => {
            {%- if tracing == true %}
            tracing::error!("Server: {result:#?}");
            {%- else %}
            leptos::logging::error!("Server: {result:#?}");
            {%- endif %}
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
