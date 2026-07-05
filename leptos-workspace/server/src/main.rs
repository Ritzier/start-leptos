#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> Result<(), color_eyre::Report> {
    use server::*;
    use tokio::signal::unix::{SignalKind, signal};

    // Initialize enhanced error reporting with backtraces and diagnostics
    color_eyre::install()?;

    // Setup structured logging/tracing subsystem (subscriber, filters, etc.)
    Trace::setup();

    // Load development environment variables when running in debug mode
    #[cfg(debug_assertions)]
    Env::setup().await;

    // Root shutdown coordinator; propagates cancellation to child components
    let shutdown_manager = ShutdownManager::new();

    // Construct the applicant server with a scoped shutdown token
    let server = Server::new(shutdown_manager.child()).await?;

    // Task supervisor manages all long-running async tasks spawned by the runtime
    let mut supervisor = TaskSupervisor::<Error>::new();

    // Spawn the main Axum server future
    supervisor.spawn(server.serve());

    // Register Unix signal listeners for graceful shutdown handling
    let mut sigint = signal(SignalKind::interrupt()).map_err(color_eyre::Report::from)?;
    let mut sigterm = signal(SignalKind::terminate()).map_err(color_eyre::Report::from)?;

    // Main event loop: wait for either shutdown signal or task termination
    tokio::select! {
        // Ctrl+C (SIGINT) triggered shutdown
        _ = sigint.recv() => {
            tracing::info!("Received SIGTINT");
            shutdown_manager.shutdown();
        }

        // SIGTERM (system/service shutdown)
        _ = sigterm.recv() => {
            tracing::info!("Received SIGTERM");
            shutdown_manager.shutdown();
        }

        // A supervised task finished (success, error, or panic)
        Some(result) = supervisor.join_next() => {
            shutdown_manager.shutdown();
            TaskSupervisor::handle_result(result);
        }
    }

    // Drain remaining tasks to ensure clean shutdown and log all outcomes
    supervisor.drain().await;

    Ok(())
}

#[cfg(not(feature = "ssr"))]
fn main() {}
