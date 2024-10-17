use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{HttpClient, HttpClientError, HttpResponse};

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
}
