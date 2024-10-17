pub mod achievements;
pub mod chat;
pub mod competition;
pub mod gift;
pub mod league;
pub mod lineup;
pub mod live;
pub mod market;
pub mod player;
pub mod user;

use reqwest::{cookie::Jar, Client, ClientBuilder, Method, StatusCode, Url};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{json, Value};
use std::sync::{Arc, LazyLock};
use thiserror::Error;
use tokio::sync::RwLock;
use tracing::warn;
use user::login::LoginPayload;

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
pub struct SerializableStatusCode(StatusCode);

impl SerializableStatusCode {
  pub fn as_status(&self) -> StatusCode {
    self.0
  }
}

impl Serialize for SerializableStatusCode {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    serializer.serialize_u16(self.as_status().as_u16())
  }
}

impl<'de> Deserialize<'de> for SerializableStatusCode {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let status_code = u16::deserialize(deserializer)?;

    StatusCode::from_u16(status_code)
      .map(SerializableStatusCode)
      .map_err(serde::de::Error::custom)
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpResponse<T: From<serde_json::Value>> {
  pub value: T,
  pub status: SerializableStatusCode,
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
    endpoint: &str,
  ) -> Result<HttpResponse<T>, HttpClientError> {
    self.req(Method::GET, endpoint, &json!({})).await
  }

  pub async fn post<T: From<Value>>(
    &self,
    endpoint: &str,
  ) -> Result<HttpResponse<T>, HttpClientError> {
    self.req(Method::POST, endpoint, &json!({})).await
  }

  pub async fn delete<T: From<Value>>(
    &self,
    endpoint: &str,
  ) -> Result<HttpResponse<T>, HttpClientError> {
    self.req(Method::DELETE, endpoint, &json!({})).await
  }

  pub async fn req<T: Serialize + ?Sized, K: From<Value>>(
    &self,
    method: Method,
    endpoint: &str,
    payload: &T,
  ) -> Result<HttpResponse<K>, HttpClientError> {
    let url = self.construct_url(endpoint)?;

    let mut response = self
      .send_request(method.clone(), url.clone(), payload)
      .await?;

    let status = SerializableStatusCode(response.status());

    if status.as_status() == StatusCode::FORBIDDEN {
      self.env_login().await?;
      response = self.send_request(method, url, payload).await?;
    }

    self.process_response(response).await
  }

  fn construct_url(
    &self,
    endpoint: &str,
  ) -> Result<reqwest::Url, HttpClientError> {
    self.base_url.join(endpoint).map_err(|err| {
      warn!("Failed to construct URL: {err}");
      HttpClientError::UrlParse(err)
    })
  }

  async fn send_request<T: Serialize + ?Sized>(
    &self,
    method: Method,
    url: reqwest::Url,
    payload: &T,
  ) -> Result<reqwest::Response, HttpClientError> {
    let request = self.client.request(method, url).json(payload);

    request.send().await.map_err(|err| {
      warn!("Request failed: {err}");
      HttpClientError::Reqwest(err)
    })
  }

  async fn env_login(&self) -> Result<(), HttpClientError> {
    dotenvy::dotenv().unwrap();

    let login_url = self.construct_url("/user/login")?;
    let email = self.get_env_var("KICKBASE_EMAIL")?;
    let password = self.get_env_var("KICKBASE_PASSWORD")?;

    let login_payload = LoginPayload { email, password };

    let response = self
      .send_request(Method::POST, login_url, &login_payload)
      .await?;

    let status = SerializableStatusCode(response.status());

    if status.as_status() == StatusCode::FORBIDDEN {
      return Err(HttpClientError::Forbidden);
    }

    Ok(())
  }

  fn get_env_var(&self, key: &str) -> Result<String, HttpClientError> {
    std::env::var(key)
      .map_err(|_| HttpClientError::MissingEnvVar(key.to_string()))
  }

  async fn process_response<K: From<Value>>(
    &self,
    response: reqwest::Response,
  ) -> Result<HttpResponse<K>, HttpClientError> {
    let status = SerializableStatusCode(response.status());
    let value = self.get_response_value(response).await;

    Ok(HttpResponse { value, status })
  }

  async fn get_response_value<K: From<Value>>(
    &self,
    response: reqwest::Response,
  ) -> K {
    response
      .json::<Value>()
      .await
      .map_err(|err| {
        warn!("Failed to parse JSON: {err}");
        err
      })
      .unwrap_or_default()
      .into()
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
  use reqwest::{Method, StatusCode};
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
    let result = client.get::<Value>("/test").await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.status.as_status(), StatusCode::OK);
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
    let result = client.get::<Value>("/not-found").await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.status.as_status(), StatusCode::NOT_FOUND);
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
    assert_eq!(response.status.as_status(), StatusCode::CREATED);
    assert_eq!(response.value, json!({"message": "created"}));

    mock.assert();
  }
}
