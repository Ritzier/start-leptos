use cucumber_test::{LeptosServer, Trace, cucumber_test};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    color_eyre::install()?;

    Trace::setup();

    LeptosServer::serve_and_wait(5).await?;

    cucumber_test("tests/cucumber_test/features").await?;

    Ok(())
}
