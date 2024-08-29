use crate::templates::*;
use axum::{routing::get, Router};
use tower_http::services::ServeDir;
use tracing::error;

pub fn create_router() -> Router {
  let assets_path = match std::env::current_dir() {
    Ok(path) => path.join("assets"),
    Err(err) => {
      error!("Failed to get current directory: {}", err);
      std::process::exit(1);
    }
  };

  if let Some(assets_str) = assets_path.to_str() {
    Router::new()
      .route("/", get(home))
      .route("/login", get(get_login).post(post_login))
      .nest_service("/assets", ServeDir::new(assets_str))
  } else {
    error!("Failed to convert assets path to string.");
    std::process::exit(1);
  }
}
