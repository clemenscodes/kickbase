use crate::router;
use std::{net::SocketAddr, sync::LazyLock};
use tokio::net::TcpListener;
use tracing::{debug, error, info, warn};

static PORT: LazyLock<u16> = LazyLock::new(|| {
  let port: u16 = 8000;
  std::env::var("PORT")
    .map(|port_str| {
      port_str.parse::<u16>().unwrap_or_else(|_| {
        warn!("PORT must be a valid u16, got: {}", port_str);
        port
      })
    })
    .unwrap_or(port)
});

fn get_address() -> SocketAddr {
  #[cfg(debug_assertions)]
  {
    SocketAddr::from(([127, 0, 0, 1], *PORT))
  }
  #[cfg(not(debug_assertions))]
  {
    SocketAddr::from(([0, 0, 0, 0], *PORT))
  }
}

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
