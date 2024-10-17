use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{HttpClient, HttpClientError, HttpResponse};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessagePayload {
  pub message: String,
}

impl HttpClient {
  pub async fn post_message(
    &self,
    league_id: &str,
    payload: ChatMessagePayload,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let url = format!("/leagues/{league_id}/chat");
    let response = self.req(Method::POST, &url, &payload).await?;
    Ok(response)
  }
}
