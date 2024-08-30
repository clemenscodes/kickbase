mod routes;

use routes::{
  home::home,
  login::{get::get_login, post::post_login},
};
use axum::{routing::get, Router};
use tower_http::services::ServeDir;

pub fn create_router(assets: &str) -> Router {
  Router::new()
    .route("/", get(home))
    .route("/login", get(get_login).post(post_login))
    .nest_service("/assets", ServeDir::new(assets))
}
