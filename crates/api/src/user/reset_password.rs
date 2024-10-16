use crate::{HttpClient, HttpClientError, HttpResponse};
use reqwest::Method;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct ResetPasswordPayload {
  pub password: String,
  pub token: String,
}

impl HttpClient {
  pub async fn reset_password(
    &self,
    payload: ResetPasswordPayload,
  ) -> Result<HttpResponse, HttpClientError> {
    let mut map = HashMap::new();
    map.insert("password", payload.password);
    map.insert("token", payload.token);

    let response = self
      .req(Method::POST, "/user/resetpassword", Some(&map), None)
      .await?;

    Ok(response)
  }
}

#[cfg(test)]
mod tests {
  #[tokio::test]
  async fn test_reset_password() {}
}
