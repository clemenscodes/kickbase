use std::net::SocketAddr;
use askama::Template;
use axum::{
  http::StatusCode,
  response::{Html, IntoResponse, Response},
  routing::get,
  Router,
};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

const KICKBASE_API_ENDPOINT: &str = "https://api.kickbase.com";

#[derive(Template)]
#[template(path = "pages/home.html")]
struct Home<'a> {
  api: &'a str,
}

async fn home() -> impl IntoResponse {
  let template = Home {
    api: KICKBASE_API_ENDPOINT,
  };
  HtmlTemplate(template)
}

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
  T: Template,
{
  fn into_response(self) -> Response {
    match self.0.render() {
      Ok(html) => Html(html).into_response(),
      Err(err) => {
        let code = StatusCode::INTERNAL_SERVER_ERROR;
        let message = format!("Failed to render template. Error: {}", err);
        (code, message).into_response()
      }
    }
  }
}

pub(crate) fn router() -> Router {
  let assets_path = std::env::current_dir().unwrap();
  let router = Router::new().route("/", get(home)).nest_service(
    "/assets",
    ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
  );
  router
}

pub(crate) fn port() -> u16 {
  let port = std::env::var("PORT")
    .map(|port_str| port_str.parse::<u16>().expect("PORT must be a valid u16"))
    .unwrap_or(8000_u16);
  port
}

pub(crate) fn addr() -> SocketAddr {
  let addr = SocketAddr::from(([0, 0, 0, 0], port()));
  addr
}

pub(crate) async fn listener() -> TcpListener {
  let listener = TcpListener::bind(&addr()).await.unwrap();
  listener
}

pub(crate) fn tracing() {
  let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
    .unwrap_or_else(|_| "with_axum_htmx_askama=debug".into());
  tracing_subscriber::registry()
    .with(env_filter)
    .with(tracing_subscriber::fmt::layer())
    .init();
}

pub(crate) async fn server() {
  tracing();
  axum::serve(listener().await, router().into_make_service())
    .await
    .unwrap();
}

#[tokio::main]
async fn main() {
  server().await;
}
