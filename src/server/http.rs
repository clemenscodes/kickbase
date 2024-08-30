use super::constants::KICKBASE;
use crate::http::HttpClient;
use std::sync::{Arc, LazyLock};

pub static HTTP: LazyLock<Arc<HttpClient>> = LazyLock::new(|| {
  let client = HttpClient::new(KICKBASE).unwrap();
  Arc::new(client)
});
