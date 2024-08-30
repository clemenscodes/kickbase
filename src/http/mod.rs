use reqwest::{Client, Response, Url};
use serde::Serialize;
use std::sync::Arc;
use thiserror::Error;
use tracing::debug;

#[derive(Debug)]
pub struct HttpClient {
  client: Arc<Client>,
  base_url: Url,
}

impl HttpClient {
  pub fn new(base_url: &str) -> Result<Self, HttpClientError> {
    let client = Client::new();
    let base_url = Url::parse(base_url)?;
    Ok(Self {
      client: Arc::new(client),
      base_url,
    })
  }

  // pub async fn get(&self, endpoint: &str) -> Result<Response, HttpClientError> {
  //   let url = self.base_url.join(endpoint)?;
  //   debug!("GET: {url:#?}");
  //   let response = self.client.get(url).send().await?;
  //   debug!("{response:#?}");
  //   Ok(response)
  // }

  pub async fn post<T: Serialize>(
    &self,
    endpoint: &str,
    payload: &T,
  ) -> Result<Response, HttpClientError> {
    let url = self.base_url.join(endpoint)?;
    debug!("POST: {url:#?}");
    let response = self.client.post(url).json(payload).send().await?;
    debug!("{response:#?}");
    Ok(response)
  }

  // pub async fn put<T: Serialize>(
  //   &self,
  //   endpoint: &str,
  //   payload: &T,
  // ) -> Result<Response, HttpClientError> {
  //   let url = self.base_url.join(endpoint)?;
  //   debug!("PUT: {url:#?}");
  //   let response = self.client.put(url).json(payload).send().await?;
  //   debug!("{response:#?}");
  //   Ok(response)
  // }
}

#[derive(Error, Debug)]
pub enum HttpClientError {
  #[error("HTTP client error: {0}")]
  ReqwestError(#[from] reqwest::Error),

  #[error("URL parsing error: {0}")]
  UrlParseError(#[from] url::ParseError),
}
