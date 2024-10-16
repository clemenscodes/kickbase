use crate::{HttpClient, HttpClientError, HttpResponse};
use reqwest::Method;

impl HttpClient {
  pub async fn refresh_chat_token(
    &self,
  ) -> Result<HttpResponse, HttpClientError> {
    let response = self
      .get(Method::POST, "/user/refreshchattoken", None)
      .await?;
    Ok(response)
  }
}

#[cfg(test)]
mod tests {
  #[tokio::test]
  async fn test_refresh_chat_token() {}
}
