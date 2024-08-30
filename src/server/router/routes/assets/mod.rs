use axum::Router;
use tower_http::services::ServeDir;
use tracing::error;

pub fn router() -> Router {
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

  Router::new().nest_service("/assets", ServeDir::new(assets))
}
