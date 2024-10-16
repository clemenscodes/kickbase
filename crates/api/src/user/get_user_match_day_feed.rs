use reqwest::Method;
use serde_json::Value;

use crate::{HttpClient, HttpClientError, HttpResponse};

impl HttpClient {
  pub async fn get_user_match_day_feed(
    &self,
    league_id: &str,
    user_id: &str,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let url = format!("/leagues/{}/users/{}/feed", league_id, user_id);
    let response = self.get::<Value>(Method::GET, &url).await?;
    Ok(response)
  }
}

#[cfg(test)]
mod tests {
  use crate::tests::{get_test_client, TEST_LEAGUE_ID, TEST_USER_ID};

  #[ignore]
  #[tokio::test]
  async fn test_get_user_match_day_feed() {
    let client = get_test_client();
    let result = client
      .get_user_match_day_feed(TEST_USER_ID, TEST_LEAGUE_ID)
      .await
      .unwrap()
      .value;
    println!("{result:#?}");
  }
}
