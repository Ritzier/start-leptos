#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> Result<(), color_eyre::Report> {
    use server::*;
    use tokio::signal::unix::{SignalKind, signal};

    // Install color-eyre panic and error reporting
    color_eyre::install()?;

    // Initialize structured logging/tracing
    Trace::setup();

    // Load development environment variables when running in debug mode
    #[cfg(debug_assertions)]
    Env::setup().await;

    let shutdown_manager = ShutdownManager::new();

    // Build and configure the application server
    let server = Server::new(shutdown_manager.child()).await?;

    let mut supervisor = TaskSupervisor::<Error>::new();
    supervisor.spawn(server.serve());

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

        Some(result) = supervisor.join_next() => {
            shutdown_manager.shutdown();
            TaskSupervisor::handle_result(result);
        }
    }

    supervisor.drain().await;

    Ok(())
}

#[cfg(not(feature = "ssr"))]
fn main() {}
