use reqwest::Method;
use serde_json::Value;

use crate::{HttpClient, HttpClientError, HttpResponse};

#[derive(Debug, Clone)]
pub struct Player {
  pub id: String,
  pub first_name: String,
  pub last_name: String,
  pub average_points: u16,
  pub market_value: f32,
  pub market_value_trend: u8,
  pub number: u8,
  pub position: u8,
  pub image: String,
  pub team_id: String,
  pub team_name: String,
  pub team_symbol: String,
  pub total_points: u16,
}

impl From<Value> for Player {
  fn from(value: Value) -> Self {
    let id = value
      .get("id")
      .unwrap()
      .as_str()
      .unwrap_or_default()
      .to_string();

    let first_name = value
      .get("firstName")
      .unwrap()
      .as_str()
      .unwrap_or_default()
      .to_string();

    let last_name = value
      .get("lastName")
      .unwrap()
      .as_str()
      .unwrap_or_default()
      .to_string();

    let average_points = value
      .get("averagePoints")
      .unwrap()
      .as_u64()
      .unwrap_or_default() as u16;

    let market_value = value
      .get("marketValue")
      .unwrap()
      .as_f64()
      .unwrap_or_default() as f32;

    let market_value_trend = value
      .get("marketValueTrend")
      .unwrap()
      .as_u64()
      .unwrap_or_default() as u8;

    let number =
      value.get("number").unwrap().as_u64().unwrap_or_default() as u8;

    let position =
      value.get("position").unwrap().as_u64().unwrap_or_default() as u8;

    let image = value
      .get("profileBig")
      .unwrap()
      .as_str()
      .unwrap_or_default()
      .to_string();

    let team_id = value
      .get("teamId")
      .unwrap()
      .as_str()
      .unwrap_or_default()
      .to_string();

    let team_name = value
      .get("teamName")
      .unwrap()
      .as_str()
      .unwrap_or_default()
      .to_string();

    let team_symbol = value
      .get("teamSymbol")
      .unwrap()
      .as_str()
      .unwrap_or_default()
      .to_string();

    let total_points = value
      .get("totalPoints")
      .unwrap()
      .as_u64()
      .unwrap_or_default() as u16;

    Player {
      id,
      first_name,
      last_name,
      average_points,
      market_value,
      market_value_trend,
      number,
      position,
      image,
      team_id,
      team_name,
      team_symbol,
      total_points,
    }
  }
}

impl HttpClient {
  pub async fn get_player_info(
    &self,
    league_id: &str,
    player_id: &str,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let url = format!("/leagues/{}/players/{}", league_id, player_id);
    let response: HttpResponse<Value> = self.get(Method::GET, &url).await?;
    Ok(response)
  }
}

#[cfg(test)]
mod tests {
  use crate::{
    player::get_player_info::Player,
    tests::{get_test_client, TEST_LEAGUE_ID, TEST_PLAYER_ID},
  };

  #[ignore]
  #[tokio::test]
  async fn test_get_player_info() {
    let client = get_test_client();
    let result = client
      .get_player_info(TEST_LEAGUE_ID, TEST_PLAYER_ID)
      .await
      .unwrap()
      .value;
    let player: Player = result.into();
    println!("{player:#?}");
  }
}
