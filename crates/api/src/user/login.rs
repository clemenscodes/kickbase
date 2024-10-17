use crate::{HttpClient, HttpClientError, HttpResponse};
use reqwest::Method;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct LoginPayload {
  pub email: String,
  pub password: String,
}

impl HttpClient {
  pub async fn login(
    &self,
    payload: LoginPayload,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let mut map = HashMap::new();
    map.insert("email", payload.email);
    map.insert("password", payload.password);

    let response = self.req(Method::POST, "/user/login", &map).await?;

    Ok(response)
  }
}

#[cfg(test)]
mod tests {
  #[tokio::test]
  async fn test_login() {}
}
