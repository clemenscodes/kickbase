use crate::{HttpClient, HttpClientError, HttpResponse};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginPayload {
  pub email: String,
  pub password: String,
}

impl HttpClient {
  pub async fn login(
    &self,
    payload: LoginPayload,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let response = self.req(Method::POST, "/user/login", &payload).await?;

    Ok(response)
  }
}

#[cfg(test)]
mod tests {
  #[tokio::test]
  async fn test_login() {}
}
