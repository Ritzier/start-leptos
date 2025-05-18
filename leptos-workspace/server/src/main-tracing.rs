mod server;
mod trace;

use server::Server;
use trace::Trace;

#[tokio::main]
async fn main() {
    Trace::setup();

    Server::setup().await;
}
