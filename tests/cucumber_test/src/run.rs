use std::ffi::OsStr;
use std::path::Path;

use color_eyre::eyre::Result;
use cucumber::World;
use tokio::fs;

use crate::AppWorld;

pub async fn cucumber_test<P: AsRef<Path>>(path: P) -> Result<()> {
    let mut dir = fs::read_dir(path).await?;

    while let Some(entry) = dir.next_entry().await? {
        let path = entry.path();
        if path.extension() == Some(OsStr::new("feature")) {
            AppWorld::cucumber()
                .fail_on_skipped()
                .run_and_exit(path)
                .await;
        }
    }

    Ok(())
}
