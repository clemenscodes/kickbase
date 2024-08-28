//! Run with
//!
//! ```not_rust
//! cargo run
//! ```

use axum::{routing::get, Router};

const KICKBASE_API_ENDPOINT: &str = "https://api.kickbase.com";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = Router::new().route("/", get(root));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    KICKBASE_API_ENDPOINT
}
