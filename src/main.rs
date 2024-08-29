mod constants;
mod html;
mod router;
mod templates;

use router::create_router;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::{debug, error, info, warn};
use tracing_subscriber::{
  fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter,
};

fn get_port() -> u16 {
  let port: u16 = 8000;
  std::env::var("PORT")
    .map(|port_str| {
      port_str.parse::<u16>().unwrap_or_else(|_| {
        warn!("PORT must be a valid u16, got: {}", port_str);
        port
      })
    })
    .unwrap_or(port)
}

fn get_address() -> SocketAddr {
  #[cfg(debug_assertions)]
  {
    SocketAddr::from(([127, 0, 0, 1], get_port()))
  }
  #[cfg(not(debug_assertions))]
  {
    SocketAddr::from(([0, 0, 0, 0], get_port()))
  }
}

async fn create_listener() -> Result<TcpListener, std::io::Error> {
  let addr = get_address();
  debug!("Attempting to bind to address: http://{}", addr);
  TcpListener::bind(&addr).await
}

fn setup_tracing() {
  #[cfg(debug_assertions)]
  {
    let env_filter =
      EnvFilter::try_from_default_env().unwrap_or_else(|_| "debug".into());

    tracing_subscriber::registry()
      .with(env_filter)
      .with(fmt::layer())
      .init();
  }

  #[cfg(not(debug_assertions))]
  {
    let env_filter =
      EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into());

    tracing_subscriber::registry()
      .with(env_filter)
      .with(fmt::layer())
      .init();
  }
}

async fn start_server() -> Result<(), Box<dyn std::error::Error>> {
  let listener = match create_listener().await {
    Ok(listener) => listener,
    Err(err) => {
      error!("Failed to bind to address: {}", err);
      return Err(Box::new(err));
    }
  };

  let router = create_router();

  let addr = listener.local_addr()?;

  info!("Server running on http://{}", addr);

  axum::serve(listener, router.into_make_service()).await?;

  Ok(())
}

#[tokio::main]
async fn main() {
  setup_tracing();
  if let Err(err) = start_server().await {
    error!("Server encountered an error: {}", err);
  }
}
