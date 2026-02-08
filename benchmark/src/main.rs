use benchmark::{Benchmarks, Cli};
use clap::Parser;
use color_eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let Cli { iteration } = Cli::parse();

    let benchmark = Benchmarks::new(iteration).await?;
    let results = benchmark.start().await?;
    results.print_summary();

    Ok(())
}
