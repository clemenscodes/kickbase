use super::constants::KICKBASE;
use crate::http::HttpClient;
use std::sync::LazyLock;
use tokio::sync::RwLock;

pub static HTTP: LazyLock<RwLock<HttpClient>> = LazyLock::new(|| {
  let client = HttpClient::new(KICKBASE).unwrap();
  RwLock::new(client)
});
