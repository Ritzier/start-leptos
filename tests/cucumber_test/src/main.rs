type Result<T> = std::result::Result<T, anyhow::Error>;

mod app_world;
mod env;
mod steps;
use std::ffi::OsStr;
use std::fs;

use app_world::AppWorld;
use cucumber::World;

#[tokio::main]
async fn main() -> Result<()> {
    for entry in fs::read_dir("tests/cucumber_test/features")? {
        let path = entry?.path();
        if path.extension() == Some(OsStr::new("feature")) {
            AppWorld::cucumber()
                .fail_on_skipped()
                .run_and_exit(path)
                .await;
        }
    }

    Ok(())
}
