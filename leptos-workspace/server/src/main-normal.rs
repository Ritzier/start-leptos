mod server;

#[tokio::main]
async fn main() {
    server::setup().await;
}
