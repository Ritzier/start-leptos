#[tokio::main]
async fn main() {
    use {{project-name | snake_case}}::ssr::*;

    Trace::setup();
    Server::setup().await;
}
