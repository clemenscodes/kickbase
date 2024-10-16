use super::{HttpClient, HttpClientError};
use crate::HttpResponse;
use reqwest::Method;

impl HttpClient {
  pub async fn get_lineup_extended(
    &self,
    league_id: &str,
  ) -> Result<HttpResponse, HttpClientError> {
    let url = format!("/leagues/{}/lineupex", league_id);
    let response = self.get(Method::GET, &url, None).await?;
    Ok(response)
  }
}

#[cfg(test)]
mod tests {
  use crate::tests::{get_test_client, TEST_LEAGUE_ID};

  #[tokio::test]
  async fn test_get_lineup() {
    let client = get_test_client();
    let result = client.get_lineup_extended(TEST_LEAGUE_ID).await;
    assert!(result.is_ok());
  }
}
