use reqwest::Method;

use super::Player;
use crate::{HttpClient, HttpClientError, HttpResponse};

impl HttpClient {
  pub async fn get_player_info(
    &self,
    league_id: &str,
    player_id: &str,
  ) -> Result<HttpResponse<Player>, HttpClientError> {
    let url = format!("/leagues/{}/players/{}", league_id, player_id);
    let response = self.get(Method::GET, &url).await?;
    Ok(response)
  }
}

#[cfg(test)]
mod tests {
  use crate::tests::{get_test_client, TEST_LEAGUE_ID, TEST_PLAYER_ID};

  #[ignore]
  #[tokio::test]
  async fn test_get_player_info() {
    let client = get_test_client();
    let player = client
      .get_player_info(TEST_LEAGUE_ID, TEST_PLAYER_ID)
      .await
      .unwrap()
      .value;
    println!("{player:#?}");
  }
}
