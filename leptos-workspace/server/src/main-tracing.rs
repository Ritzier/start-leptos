mod server;
mod trace;

#[tokio::main]
async fn main() {
    trace::setup();

    server::setup().await;
}
