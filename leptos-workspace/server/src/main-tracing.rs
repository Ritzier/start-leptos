#[cfg(debug_assertions)]
mod debug;
#[cfg(debug_assertions)]
use debug::Env;

mod server;
mod trace;

use server::Server;
use trace::Trace;

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    Env::setup().await;

    Trace::setup();

    Server::setup().await;
}
