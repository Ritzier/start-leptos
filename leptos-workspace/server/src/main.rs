#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> Result<(), color_eyre::Report> {
    use server::*;
    use tokio::signal::unix::{SignalKind, signal};
    use tokio::task::JoinSet;

    // Install color-eyre panic and error reporting
    color_eyre::install()?;

    {% if tracing == true -%}
    // Initialize structured logging/tracing
    Trace::setup();

    {% endif -%}
    // Load development environment variables when running in debug mode
    #[cfg(debug_assertions)]
    Env::setup().await;

    let shutdown_manager = ShutdownManager::new();

    // Build and configure the application server
    let server = Server::new(shutdown_manager.child()).await?;

    let mut tasks = JoinSet::new();

    tasks.spawn(server.serve());

    // Listen for common `Unix` shutdown signals
    let mut sigint = signal(SignalKind::interrupt()).map_err(color_eyre::Report::from)?;
    let mut sigterm = signal(SignalKind::terminate()).map_err(color_eyre::Report::from)?;

    // Run the server until either:
    // - the server exits with an error, or
    // - the process receives a shutdown signal
    tokio::select! {
        _ = sigint.recv() => {
            tracing::info!("Received SIGTINT");
            shutdown_manager.shutdown();
        }

        _ = sigterm.recv() => {
            tracing::info!("Received SIGTERM");
            shutdown_manager.shutdown();
        }

        Some(res) = tasks.join_next() => {
            shutdown_manager.shutdown();
            handle_result(res);
        }
    }

    while let Some(res) = tasks.join_next().await {
        handle_result(res);
    }

    Ok(())
}

#[cfg(feature = "ssr")]
fn handle_result(result: Result<Result<(), server::Error>, tokio::task::JoinError>) {
    match result {
        Ok(Ok(())) => tracing::info!("Task exited"),
        Ok(Err(err)) => tracing::error!("{err:#}"),
        Err(join_err) => tracing::error!("Task panicked: {join_err:#}"),
    }
}

#[cfg(not(feature = "ssr"))]
fn main() {}
