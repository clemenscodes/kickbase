use askama::Template;
use axum::{
  http::StatusCode,
  response::{Html, IntoResponse, Response},
  routing::get,
  Router,
};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing::{debug, info};
use tracing_subscriber::{
  fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter,
};

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

fn router() -> Router {
  let assets_path = std::env::current_dir().unwrap();
  Router::new().route("/", get(home)).nest_service(
    "/assets",
    ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
  )
}

fn port() -> u16 {
  std::env::var("PORT")
    .map(|port_str| port_str.parse::<u16>().expect("PORT must be a valid u16"))
    .unwrap_or(8000_u16)
}

fn addr() -> SocketAddr {
  SocketAddr::from(([0, 0, 0, 0], port()))
}

async fn listener() -> TcpListener {
  TcpListener::bind(&addr()).await.unwrap()
}

fn tracing() {
  #[cfg(debug_assertions)]
  {
    let env_filter = EnvFilter::try_from_default_env()
      .unwrap_or_else(|_| "warn,info,kickbase=debug".into());
    tracing_subscriber::registry()
      .with(env_filter)
      .with(fmt::layer())
      .init();
  }

  debug!("debug logging initialized");

  #[cfg(not(debug_assertions))]
  {
    let env_filter =
      EnvFilter::try_from_default_env().unwrap_or_else(|_| "warn".into());
    tracing_subscriber::registry()
      .with(env_filter)
      .with(fmt::layer())
      .init();
  }

  info!("info logging initialized");
}

async fn server() {
  tracing();
  axum::serve(listener().await, router().into_make_service())
    .await
    .unwrap();
}

#[tokio::main]
async fn main() {
  server().await;
}
