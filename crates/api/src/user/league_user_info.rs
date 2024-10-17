use crate::{HttpClient, HttpClientError, HttpResponse};
use serde_json::Value;

impl HttpClient {
  pub async fn league_user_info(
    &self,
    league_id: &str,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let url = format!("/leagues/{league_id}/me");
    let response = self.get(&url).await?;
    Ok(response)
  }
}

#[cfg(test)]
mod tests {
  use crate::tests::{get_test_client, TEST_LEAGUE_ID};

  #[ignore]
  #[tokio::test]
  async fn test_league_user_info() {
    let client = get_test_client();
    let result = client.league_user_info(TEST_LEAGUE_ID).await.unwrap().value;
    println!("{result:#?}");
  }
}
