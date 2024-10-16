use crate::{HttpClient, HttpClientError, HttpResponse};
use reqwest::Method;

impl HttpClient {
  pub async fn league_user_info(
    &self,
    league_id: &str,
  ) -> Result<HttpResponse, HttpClientError> {
    let url = format!("/leagues/{}/me", league_id);
    let response = self.get(Method::GET, &url, None).await?;
    Ok(response)
  }
}

#[cfg(test)]
mod tests {
  use crate::tests::{get_test_client, TEST_LEAGUE_ID};

  #[tokio::test]
  async fn test_league_user_info() {
    let client = get_test_client();
    let result = client.league_user_info(TEST_LEAGUE_ID).await;
    assert!(result.is_ok());
  }
}
