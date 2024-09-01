use axum::Router;
use tower_http::services::ServeDir;
use tracing::{error, warn};

pub fn router() -> Router {
  let path = std::env::var("KICKBASE_ASSETS").map_or_else(
        |env_err| {
            warn!(
                "KICKBASE_ASSETS environment variable is not set or could not be read: {}. Falling back to current directory.",
                env_err
            );
            std::env::current_dir()
                .map(|path| path.join("assets"))
                .unwrap_or_else(|err| {
                    error!("Failed to get current directory: {}", err);
                    std::process::exit(1);
                })
        },
        std::path::PathBuf::from
    );

  let assets = path.to_str().unwrap_or_else(|| {
    error!("Failed to convert assets path to string.");
    std::process::exit(1);
  });

  Router::new().nest_service("/assets", ServeDir::new(assets))
}
