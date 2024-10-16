pub mod achievements;
pub mod competition;
pub mod gift;
pub mod league;
pub mod lineup;
pub mod live;
pub mod market;
pub mod player;
pub mod user;

use reqwest::{cookie::Jar, Client, ClientBuilder, Method, StatusCode, Url};
use serde::Serialize;
use serde_json::{json, Value};
use std::{
  collections::HashMap,
  sync::{Arc, LazyLock},
};
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
pub struct HttpResponse<T: From<Value>> {
  pub value: T,
  pub status: StatusCode,
}

impl HttpClient {
  pub fn new(base_url: &str) -> Result<Self, HttpClientError> {
    let base_url = Url::parse(base_url)?;
    let cookies = Arc::new(Jar::default());
    let client = ClientBuilder::new().cookie_provider(cookies).build()?;

    Ok(Self { client, base_url })
  }

  pub async fn get<T: From<Value>>(
    &self,
    method: Method,
    endpoint: &str,
  ) -> Result<HttpResponse<T>, HttpClientError> {
    self.req(method, endpoint, &json!({})).await
  }

  pub async fn req<T: Serialize, K: From<Value>>(
    &self,
    method: Method,
    endpoint: &str,
    payload: &T,
  ) -> Result<HttpResponse<K>, HttpClientError> {
    let url = self.base_url.join(endpoint).map_err(|err| {
      warn!("Failed to construct URL: {err}");
      HttpClientError::UrlParse(err)
    })?;

    let mut request = self.client.request(method.clone(), url.clone());

    request = request.json(payload);

    let response = request.send().await.map_err(|err| {
      warn!("Request failed: {err}");
      HttpClientError::Reqwest(err)
    })?;

    debug!("{response:#?}");

    let status = response.status();

    if status == StatusCode::FORBIDDEN {
      dotenvy::dotenv().unwrap();

      let email = std::env::var("KICKBASE_EMAIL").map_err(|_| {
        HttpClientError::MissingEnvVar("KICKBASE_EMAIL".to_string())
      })?;
      let password = std::env::var("KICKBASE_PASSWORD").map_err(|_| {
        HttpClientError::MissingEnvVar("KICKBASE_PASSWORD".to_string())
      })?;

      let mut login_payload = HashMap::new();
      login_payload.insert("email", email);
      login_payload.insert("password", password);

      let login_url = self.base_url.join("/user/login").map_err(|err| {
        warn!("Failed to construct URL: {err}");
        HttpClientError::UrlParse(err)
      })?;

      let login_request = self
        .client
        .request(Method::POST, login_url)
        .json(&login_payload);

      let response = login_request.send().await.map_err(|err| {
        warn!("Request failed: {err}");
        HttpClientError::Reqwest(err)
      })?;

      let status = response.status();

      if status == StatusCode::FORBIDDEN {
        return Err(HttpClientError::Forbidden);
      }

      let mut request = self.client.request(method, url);

      request = request.json(payload);

      let response = request.send().await.map_err(|err| {
        warn!("Request failed: {err}");
        HttpClientError::Reqwest(err)
      })?;

      let value = response
        .json::<Value>()
        .await
        .map_err(|err| {
          warn!("Failed to parse JSON: {err}");
          err
        })
        .unwrap_or_else(|_| json!({}));

      debug!("{value:#?}");

      let value: K = value.into();

      let response = HttpResponse { value, status };

      return Ok(response);
    }

    let value = response
      .json::<Value>()
      .await
      .map_err(|err| {
        warn!("Failed to parse JSON: {err}");
        err
      })
      .unwrap_or_else(|_| json!({}));

    debug!("{value:#?}");

    let value: K = value.into();

    let response = HttpResponse { value, status };

    Ok(response)
  }
}

#[derive(Error, Debug)]
pub enum HttpClientError {
  #[error("HTTP client error: {0}")]
  Reqwest(#[from] reqwest::Error),

  #[error("URL parsing error: {0}")]
  UrlParse(#[from] url::ParseError),

  #[error("Forbidden error: Access is denied (403)")]
  Forbidden,

  #[error("Missing environment variable: {0}")]
  MissingEnvVar(String),

  #[error("Unexpected error")]
  Unexpected,
}

#[cfg(test)]
mod tests {
  use super::*;
  use httpmock::MockServer;
  use reqwest::Method;
  use reqwest::StatusCode;
  use serde_json::json;

  pub const TEST_USER_ID: &str = "3408447";
  pub const TEST_LEAGUE_ID: &str = "6195342";
  pub const TEST_PLAYER_ID: &str = "7508";

  pub fn get_test_client() -> HttpClient {
    HttpClient::new(API).unwrap()
  }

  #[test]
  fn test_httpclient_new_valid_url() {
    let client = HttpClient::new("http://localhost");
    assert!(client.is_ok());
  }

  #[test]
  fn test_httpclient_new_invalid_url() {
    let client = HttpClient::new("invalid-url");
    assert!(client.is_err());
  }

  #[tokio::test]
  async fn test_httpclient_get_success() {
    let server = MockServer::start();

    let mock = server.mock(|when, then| {
      when.method(httpmock::Method::GET).path("/test");
      then
        .status(200)
        .header("content-type", "application/json")
        .json_body(json!({"message": "success"}));
    });

    let client = HttpClient::new(&server.url("")).unwrap();
    let result = client.get::<Value>(Method::GET, "/test").await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.status, StatusCode::OK);
    assert_eq!(response.value, json!({"message": "success"}));

    mock.assert();
  }

  #[tokio::test]
  async fn test_httpclient_get_error() {
    let server = MockServer::start();

    let mock = server.mock(|when, then| {
      when.method(httpmock::Method::GET).path("/not-found");
      then
        .status(404)
        .header("content-type", "application/json")
        .json_body(json!({"error": "Not Found"}));
    });

    let client = HttpClient::new(&server.url("")).unwrap();
    let result = client.get::<Value>(Method::GET, "/not-found").await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.status, StatusCode::NOT_FOUND);
    assert_eq!(response.value, json!({"error": "Not Found"}));

    mock.assert();
  }

  #[tokio::test]
  async fn test_httpclient_req_post_success() {
    let server = MockServer::start();

    let mock = server.mock(|when, then| {
      when
        .method(httpmock::Method::POST)
        .path("/submit")
        .json_body(json!({"name": "test"}));
      then
        .status(201)
        .header("content-type", "application/json")
        .json_body(json!({"message": "created"}));
    });

    let client = HttpClient::new(&server.url("")).unwrap();
    let payload = json!({"name": "test"});
    let result = client
      .req::<_, Value>(Method::POST, "/submit", &payload)
      .await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.status, StatusCode::CREATED);
    assert_eq!(response.value, json!({"message": "created"}));

    mock.assert();
  }
}
