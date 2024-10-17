use super::{formation::Formation, HttpClient, HttpClientError};
use crate::{league::League, player::Player, HttpResponse};
use reqwest::Method;
use serde_json::Value;

#[derive(Debug)]
pub struct ExtendedLineup {
  pub league: League,
  pub formation: Formation,
  pub players: Vec<Player>,
}

impl From<&Value> for ExtendedLineup {
  fn from(value: &Value) -> Self {
    let formation: Formation = value.into();
    let league: League = value.into();
    let players = value
      .get("players")
      .unwrap()
      .as_array()
      .unwrap()
      .iter()
      .map(Player::from)
      .collect();

    Self {
      formation,
      league,
      players,
    }
  }
}

impl From<Value> for ExtendedLineup {
  fn from(value: Value) -> Self {
    Self::from(&value)
  }
}

impl HttpClient {
  pub async fn get_lineup_extended(
    &self,
    league_id: &str,
  ) -> Result<HttpResponse<ExtendedLineup>, HttpClientError> {
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
