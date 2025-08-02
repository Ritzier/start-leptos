mod debug;
mod server;
mod trace;

use debug::Env;
use server::Server;
use trace::Trace;

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    {
        Env::setup().await;
    }
    Trace::setup();

    Server::setup().await;
}
