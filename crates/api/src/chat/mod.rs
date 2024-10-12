use super::{HttpClient, HttpClientError};
use crate::HttpResponse;
use reqwest::Method;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct ChatMessagePayload {
  pub message: String,
}

impl HttpClient {
  pub async fn exchange_custom_token(
    &self,
    custom_token: &str,
  ) -> Result<HttpResponse, HttpClientError> {
    let mut map = HashMap::new();
    map.insert("token", custom_token.to_string());

    let url = "/chat/exchangetoken".to_string();
    let response = self.req(Method::POST, &url, Some(&map), None).await?;
    Ok(response)
  }

  pub async fn post_message(
    &self,
    league_id: &str,
    payload: ChatMessagePayload,
  ) -> Result<HttpResponse, HttpClientError> {
    let mut map = HashMap::new();
    map.insert("message", payload.message);

    let url = format!("/leagues/{}/chat", league_id);
    let response = self.req(Method::POST, &url, Some(&map), None).await?;
    Ok(response)
  }

  pub async fn get_messages(
    &self,
    league_id: &str,
  ) -> Result<HttpResponse, HttpClientError> {
    let url = format!("/leagues/{}/chat/messages", league_id);
    let response = self.get(Method::GET, &url, None).await?;
    Ok(response)
  }
}
