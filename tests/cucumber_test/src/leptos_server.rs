use std::net::SocketAddr;
use std::path::Path;
use std::process::Stdio;
use std::time::Duration;

use color_eyre::{Result, eyre};
use server::Server;
use tokio::process::Command;

use crate::{PortFinder, set_server_addr};

pub struct LeptosServer;

impl LeptosServer {
    pub async fn build() -> Result<()> {
        let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(2)
            .unwrap();

        println!("Compiling");
        let output = Command::new("cargo")
            .arg("leptos")
            .arg("build")
            .arg("--split")
            .current_dir(manifest_dir)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .output()
            .await?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);

            return Err(eyre::eyre!(
                "`cargo leptos build` failed\n\
            Exit code: {:?}\n\
            Stderr: {}\n\
            Stdout: {}",
                output.status.code(),
                stderr.trim(),
                stdout.trim()
            ));
        }

        Ok(())
    }

    pub async fn serve() -> Result<()> {
        let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(2)
            .unwrap();
        let cargo_toml_path = manifest_dir.join("Cargo.toml");

        // Server addr
        let port = PortFinder::get_available_port()
            .await
            .map_err(|e| eyre::eyre!("{e}"))?;
        let addr = std::net::SocketAddr::new(
            std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)),
            port,
        );

        Server::cucumber_setup(addr, Some(cargo_toml_path.to_str().unwrap())).await?;
        set_server_addr(addr);

        Ok(())
    }

    pub async fn serve_from_command() -> Result<()> {
        let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(2)
            .unwrap();

        // Server addr
        let port = PortFinder::get_available_port()
            .await
            .map_err(|e| eyre::eyre!("{e}"))?;
        let addr = std::net::SocketAddr::new(
            std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)),
            port,
        );

        Command::new("cargo")
            .arg("leptos")
            .arg("serve")
            .arg("--split")
            .env("LEPTOS_SITE_ADDR", addr.to_string())
            .current_dir(manifest_dir)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;

        wait_for_server_ready(addr, Duration::from_secs(5)).await?;
        set_server_addr(addr);

        Ok(())
    }
}

async fn wait_for_server_ready(addr: SocketAddr, timeout: Duration) -> Result<()> {
    let start = tokio::time::Instant::now();

    while start.elapsed() < timeout {
        if tokio::net::TcpStream::connect(addr).await.is_ok() {
            return Ok(());
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    Err(eyre::eyre!("Server did not become ready within timeout"))
}
