pub mod auth;
pub mod league;
pub mod user;

use axum::http::HeaderMap;
use reqwest::{cookie::Jar, Client, ClientBuilder, Method, StatusCode, Url};
use serde::Serialize;
use serde_json::{json, Value};
use std::sync::{Arc, LazyLock};
use thiserror::Error;
use tokio::sync::RwLock;
use tracing::{debug, warn};

const API: &str = "https://api.kickbase.com";

pub static KICKBASE: LazyLock<RwLock<HttpClient>> = LazyLock::new(|| {
  let client = HttpClient::new(API).unwrap();
  RwLock::new(client)
});

#[derive(Debug)]
pub struct HttpClient {
  client: Client,
  base_url: Url,
}

#[derive(Debug)]
pub struct HttpResponse {
  pub value: Value,
  pub status: StatusCode,
}

impl HttpClient {
  pub fn new(base_url: &str) -> Result<Self, HttpClientError> {
    let cookies = Arc::new(Jar::default());

    let client = ClientBuilder::new().cookie_provider(cookies).build()?;

    let base_url = Url::parse(base_url)?;

    Ok(Self { client, base_url })
  }

  pub async fn get(
    &self,
    method: Method,
    endpoint: &str,
    headers: Option<HeaderMap>,
  ) -> Result<HttpResponse, HttpClientError> {
    self.req(method, endpoint, Some(&json!({})), headers).await
  }

  pub async fn req<T: Serialize>(
    &self,
    method: Method,
    endpoint: &str,
    payload: Option<&T>,
    headers: Option<HeaderMap>,
  ) -> Result<HttpResponse, HttpClientError> {
    let url = self.base_url.join(endpoint)?;

    let mut request = self.client.request(method, url);

    if let Some(headers) = headers {
      request = request.headers(headers)
    }

    if let Some(payload) = payload {
      request = request.json(payload);
    }

    debug!("{request:#?}");

    let response = request.send().await.unwrap();

    let status = response.status();

    let value = response
      .json::<Value>()
      .await
      .map_err(|err| {
        warn!("Failed to parse JSON: {err}");
        err
      })
      .unwrap_or_else(|_| json!({}));

    let response = HttpResponse { value, status };

    debug!("{response:#?}");

    Ok(response)
  }
}

#[derive(Error, Debug)]
pub enum HttpClientError {
  #[error("HTTP client error: {0}")]
  ReqwestError(#[from] reqwest::Error),

  #[error("URL parsing error: {0}")]
  UrlParseError(#[from] url::ParseError),
}
