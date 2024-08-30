mod address;
mod constants;
mod html;
mod port;
mod router;
mod templates;

use crate::http::HttpClient;
use address::get_address;
use constants::KICKBASE_API_ENDPOINT;
use std::sync::{Arc, LazyLock};
use tokio::net::TcpListener;
use tracing::{debug, error, info};

pub static KICKBASE_HTTP_CLIENT: LazyLock<Arc<HttpClient>> =
  LazyLock::new(|| {
    let client = HttpClient::new(KICKBASE_API_ENDPOINT)
      .expect("Failed to create HttpClient");
    Arc::new(client)
  });

pub async fn start_server() -> Result<(), Box<dyn std::error::Error>> {
  let addr = get_address();
  debug!("Attempting to bind to address: http://{}", addr);
  let listener = match TcpListener::bind(&addr).await {
    Ok(listener) => listener,
    Err(err) => {
      error!("Failed to bind to address: {}", err);
      return Err(Box::new(err));
    }
  };

  let addr = listener.local_addr()?;

  let path = std::env::current_dir()
    .map(|path| path.join("assets"))
    .unwrap_or_else(|err| {
      error!("Failed to get current directory: {}", err);
      std::process::exit(1);
    });

  let assets = path.to_str().unwrap_or_else(|| {
    error!("Failed to convert assets path to string.");
    std::process::exit(1);
  });

  let router = router::create_router(assets);

  info!("Server running on http://{}", addr);

  axum::serve(listener, router.into_make_service()).await?;

  Ok(())
}
