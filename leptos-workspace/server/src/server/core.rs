use tokio_util::sync::CancellationToken;

use crate::{Error, TaskSupervisor};

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

        let mut supervisor = TaskSupervisor::<ServerError>::new();

        supervisor.spawn(axum_server.serve());

        // hook (e.g. cucumber read signal)
        before_select().await;

        tokio::select! {
            _ = shutdown.cancelled() => {
                tracing::info!("shutting down");
            }

            Some(result) = supervisor.join_next() => {
                shutdown.cancel();
                TaskSupervisor::handle_result(result);
            }
        }

        supervisor.drain().await;

        Ok(())
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
