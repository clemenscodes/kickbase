pub mod http;

use http::HttpClient;
use std::sync::LazyLock;
use tokio::sync::RwLock;

const API: &str = "https://api.kickbase.com";

pub static KICKBASE: LazyLock<RwLock<HttpClient>> = LazyLock::new(|| {
  let client = HttpClient::new(API).unwrap();
  RwLock::new(client)
});
