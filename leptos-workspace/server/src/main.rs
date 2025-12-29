#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> Result<(), server::Error> {
    use server::*;

    #[cfg(debug_assertions)]
    Env::setup().await;

    Server::setup().await
}

#[cfg(not(feature = "ssr"))]
fn main() {}
