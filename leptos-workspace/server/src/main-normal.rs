mod debug;
mod server;

use debug::Env;
use server::Server;

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    {
        Env::setup().await;
    }

    Server::setup().await;
}
