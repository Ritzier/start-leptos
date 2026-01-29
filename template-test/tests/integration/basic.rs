use anyhow::Result;

use crate::CargoGenerate;

#[tokio::test]
async fn a() -> Result<()> {
    let _temp = CargoGenerate::default().build().await?;

    Ok(())
}
