use crate::{HttpClient, HttpClientError, HttpResponse};
use reqwest::Method;
use serde_json::Value;

impl HttpClient {
  pub async fn refresh_chat_token(
    &self,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let response = self
      .get::<Value>(Method::POST, "/user/refreshchattoken")
      .await?;
    Ok(response)
  }
}

#[cfg(test)]
mod tests {
  #[tokio::test]
  async fn test_refresh_chat_token() {}
}
