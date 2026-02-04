#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> Result<(), color_eyre::Report> {
    use server::*;
    use tokio::signal::unix::{SignalKind, signal};

    color_eyre::install()?;

    {% if tracing == true -%}
    Trace::setup();

    {% endif -%}
    #[cfg(debug_assertions)]
    Env::setup().await;

    let mut sigint = signal(SignalKind::interrupt()).map_err(color_eyre::Report::from)?;
    let mut sigterm = signal(SignalKind::terminate()).map_err(color_eyre::Report::from)?;

    tokio::select! {
        result = Server::setup() => {
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
