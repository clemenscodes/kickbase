use super::{HttpClient, HttpClientError};
use crate::HttpResponse;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessagePayload {
  pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomTokenPayload {
  pub token: String,
}

impl HttpClient {
  pub async fn exchange_custom_token(
    &self,
    payload: CustomTokenPayload,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let response = self
      .req(Method::POST, "/chat/exchangetoken", &payload)
      .await?;
    Ok(response)
  }

  pub async fn post_message(
    &self,
    league_id: &str,
    payload: ChatMessagePayload,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let url = format!("/leagues/{league_id}/chat");
    let response = self.req(Method::POST, &url, &payload).await?;
    Ok(response)
  }

  pub async fn get_messages(
    &self,
    league_id: &str,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let url = format!("/leagues/{league_id}/chat/messages");
    let response = self.get(&url).await?;
    Ok(response)
  }
}
