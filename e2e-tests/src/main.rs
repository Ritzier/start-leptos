use e2e_tests::{LeptosServer, Trace, cucumber_test};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    color_eyre::install()?;

    Trace::setup();

    LeptosServer::serve_and_wait(5).await?;

    cucumber_test("e2e-tests/features").await?;

    Ok(())
}
