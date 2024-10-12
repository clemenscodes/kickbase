use axum::Router;
use tower_http::services::ServeDir;
use tracing::{debug, error};

pub fn router() -> Router {
  let path = std::env::var("WEBSERVER_ASSETS").map_or_else(
    |_| {
      let fallback = std::env::current_dir()
        .map(|path| path.join("..").join("server").join("assets"))
        .unwrap_or_else(|err| {
          error!("Failed to get current directory: {}", err);
          std::process::exit(1);
        });
      debug!("WEBSERVER_ASSETS environment variable is not set. Using {fallback:#?} as the fallback for assets.");
      fallback
    },
    |path_str| {
      debug!("WEBSERVER_ASSETS environment variable found, using path: {}", path_str);
      std::path::PathBuf::from(path_str)
    },
  );

  let assets = path.to_str().unwrap_or_else(|| {
    error!("Failed to convert assets path to string.");
    std::process::exit(1);
  });

  Router::new().nest_service("/assets", ServeDir::new(assets))
}
