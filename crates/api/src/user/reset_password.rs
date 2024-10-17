use crate::{HttpClient, HttpClientError, HttpResponse};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResetPasswordPayload {
  pub password: String,
  pub token: String,
}

impl HttpClient {
  pub async fn reset_password(
    &self,
    payload: ResetPasswordPayload,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let response = self
      .req(Method::POST, "/user/resetpassword", &payload)
      .await?;

    Ok(response)
  }
}

#[cfg(test)]
mod tests {
  #[tokio::test]
  async fn test_reset_password() {}
}
