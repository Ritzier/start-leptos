use anyhow::Result;
use insta::assert_json_snapshot;

use crate::{CargoGenerate, Style};

// tokio::fs::write("1.json", serde_json::to_string_pretty(&files).unwrap()).await?;

#[tokio::test]
async fn a() -> Result<()> {
    let generate_result = CargoGenerate::default().build().await?;
    let files = generate_result.to_snapshot().await?;
    let files_json = serde_json::to_string_pretty(&files)?;
    assert_json_snapshot!("default_template", files_json);

    generate_result.check_clippy().await?;

    Ok(())
}

#[tokio::test]
async fn b() -> Result<()> {
    let cargo_generate = CargoGenerate {
        style: Style::Unocss,
        ..Default::default()
    };
    let generate_result = cargo_generate.build().await?;
    let files = generate_result.to_snapshot().await?;
    let files_json = serde_json::to_string_pretty(&files)?;

    assert_json_snapshot!("unocss_template", files_json);

    generate_result.check_clippy().await?;

    Ok(())
}
