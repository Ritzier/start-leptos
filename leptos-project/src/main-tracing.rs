#[tokio::main]
async fn main() {
    use {{project-name | snake_case}}::ssr::*;

    #[cfg(debug_assertions)]
    {
        Env::setup().await;
    }

    Trace::setup();
    Server::setup().await;
}

fn main() {}
