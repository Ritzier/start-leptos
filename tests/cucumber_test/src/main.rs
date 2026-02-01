use cucumber_test::{LeptosServer, Trace};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    color_eyre::install()?;

    Trace::setup();

    LeptosServer::build().await?;
    LeptosServer::serve().await?;

    Ok(())
}
