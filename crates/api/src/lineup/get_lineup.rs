use serde_json::Value;

use super::{formation::Formation, HttpClient, HttpClientError};
use crate::{player::Player, HttpResponse};

pub const PLAYERS: usize = 11;

#[derive(Debug, Clone)]
pub struct Lineup {
  pub formation: Formation,
  pub players: [Player; PLAYERS],
}

impl HttpClient {
  pub async fn get_lineup(
    &self,
    league_id: &str,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let url = format!("/leagues/{league_id}/lineup");
    let response = self.get(&url).await?;
    Ok(response)
  }
}

#[cfg(test)]
mod tests {
  use crate::tests::{get_test_client, TEST_LEAGUE_ID};

  #[ignore]
  #[tokio::test]
  async fn test_get_lineup() {
    let client = get_test_client();
    let result = client.get_lineup(TEST_LEAGUE_ID).await.unwrap().value;
    println!("{result:#?}");
  }
}
