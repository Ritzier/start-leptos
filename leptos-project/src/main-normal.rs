#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use {{project-name | snake_case}}::ssr::*;

    Server::setup().await;
}
