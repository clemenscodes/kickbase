mod constants;
mod html;
mod http;
mod router;
mod routes;
mod server;
mod templates;
mod trace;

#[tokio::main]
async fn main() {
  trace::setup_tracing();
  if let Err(err) = server::start_server().await {
    tracing::error!("Server encountered an error: {}", err);
  }
}
