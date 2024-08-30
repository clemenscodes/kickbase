mod routes;

use axum::Router;
use routes::{assets, home, login};

pub fn create_router() -> Router {
  Router::new()
    .merge(home::router())
    .merge(login::router())
    .merge(assets::router())
}
