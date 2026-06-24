use super::axum_server::AxumServer;
use super::errors::ServerError;

pub struct Server {
    axum_server: AxumServer,
}
{%- if cucumber == true %}

#[cfg(feature = "cucumber")]
pub struct CucumberServer {
    server: Server,
    sender: tokio::sync::oneshot::Sender<()>,
}
{%- endif %}

impl Server {
    pub async fn new() -> Result<Self, ServerError> {
        let axum_server = AxumServer::new().await?;

        Ok(Self { axum_server })
    }
    {%- if cucumber == true %}

    #[cfg(feature = "cucumber")]
    pub async fn cucumber_new(
        addr: std::net::SocketAddr,
        cargo_toml_path: Option<&str>,
        sender: tokio::sync::oneshot::Sender<()>,
    ) -> Result<CucumberServer, ServerError> {
        let axum_server = AxumServer::cucumber_new(addr, cargo_toml_path).await?;

        let server = Server { axum_server };

        Ok(CucumberServer { server, sender })
    }
    {%- endif %}

    pub async fn serve(self) -> Result<(), ServerError> {
        let Self { axum_server } = self;

        // Axum task
        let axum_handle = tokio::spawn(async move { axum_server.serve().await });

        tokio::select! {
            res = axum_handle => res??,
        }

        Ok(())
    }
}
{%- if cucumber == true %}

#[cfg(feature = "cucumber")]
impl CucumberServer {
    pub async fn serve(self) -> Result<(), ServerError> {
        let Self { server, sender } = self;

        let Server { axum_server } = server;

        // Axum task
        let axum_handle = tokio::spawn(async move { axum_server.serve().await });

        // ---- READY SIGNAL ----
        let _ = sender.send(());

        tokio::select! {
            res = axum_handle => {
                res??
            }
        }

        Ok(())
    }
}
{%- endif %}
