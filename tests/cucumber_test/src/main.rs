use cucumber_test::{LeptosServer, Trace, cucumber_test};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    color_eyre::install()?;

    Trace::setup();

    LeptosServer::build().await?;
    // Currently would hanging the process, cause the Leptos server still have some issue
    // LeptosServer::serve().await?;
    LeptosServer::serve_from_command().await?;

    cucumber_test("tests/cucumber_test/features").await?;

    Ok(())
}
