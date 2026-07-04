use tokio::task::{JoinError, JoinSet};
use tokio_util::sync::CancellationToken;

use crate::Error;

use super::axum_server::AxumServer;
use super::errors::ServerError;

pub struct Server {
    axum_server: AxumServer,
    shutdown: CancellationToken,
}
{%- if cucumber == true %}

#[cfg(feature = "cucumber")]
pub struct CucumberServer {
    server: Server,
    sender: tokio::sync::oneshot::Sender<()>,
}
{%- endif %}

impl Server {
    pub async fn new(shutdown: CancellationToken) -> Result<Self, ServerError> {
        let axum_server = AxumServer::new(shutdown.clone()).await?;

        Ok(Self {
            axum_server,
            shutdown,
        })
    }
    {%- if cucumber == true %}

    #[cfg(feature = "cucumber")]
    pub async fn cucumber_new(
        addr: std::net::SocketAddr,
        cargo_toml_path: Option<&str>,
        sender: tokio::sync::oneshot::Sender<()>,
        shutdown: CancellationToken,
    ) -> Result<CucumberServer, ServerError> {
        let axum_server = AxumServer::cucumber_new(addr, cargo_toml_path, shutdown.clone()).await?;

        let server = Server {
            axum_server,
            shutdown,
        };

        Ok(CucumberServer { server, sender })
    }
    {%- endif %}

    pub async fn serve(self) -> Result<(), Error> {
        self.run_server(|| async {}).await.map_err(Into::into)
    }

    async fn run_server<F, Fut>(self, before_select: F) -> Result<(), ServerError>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = ()>,
    {
        let Self {
            axum_server,
            shutdown,
        } = self;

        let mut tasks = JoinSet::new();

        tasks.spawn(axum_server.serve());

        // hook (e.g. cucumber read signal)
        before_select().await;

        tokio::select! {
            _ = shutdown.cancelled() => {
                tracing::info!("shutting down");
            }

            Some(res) = tasks.join_next() => {
                Self::handle_result(res);
            }
        }

        while let Some(res) = tasks.join_next().await {
            Self::handle_result(res);
        }

        Ok(())
    }

    fn handle_result(result: Result<Result<(), ServerError>, JoinError>) {
        match result {
            Ok(Ok(())) => tracing::info!("Task exited"),
            Ok(Err(err)) => tracing::error!("{err:#}"),
            Err(join_err) => tracing::error!("Task panicked: {join_err:#}"),
        }
    }
}
{%- if cucumber == true %}

#[cfg(feature = "cucumber")]
impl CucumberServer {
    pub async fn serve(self) -> Result<(), ServerError> {
        let Self { server, sender } = self;

        server
            .run_server(|| async move {
                let _ = sender.send(());
            })
            .await
    }
}
{%- endif %}
