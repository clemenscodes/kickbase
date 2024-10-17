use super::{HttpClient, HttpClientError};
use crate::HttpResponse;
use reqwest::Method;
use serde_json::Value;

impl HttpClient {
  pub async fn get_lineup_extended(
    &self,
    league_id: &str,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let url = format!("/leagues/{}/lineupex", league_id);
    let response = self.get(Method::GET, &url).await?;
    Ok(response)
  }
}

#[cfg(test)]
mod tests {
  use crate::tests::{get_test_client, TEST_LEAGUE_ID};

  #[ignore]
  #[tokio::test]
  async fn test_get_lineup_extended() {
    let client = get_test_client();
    let result = client
      .get_lineup_extended(TEST_LEAGUE_ID)
      .await
      .unwrap()
      .value;
    println!("{result:#?}");
  }
}
