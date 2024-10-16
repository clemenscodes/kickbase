use super::{formation::Formation, HttpClient, HttpClientError};
use crate::{player::Player, HttpResponse};
use reqwest::Method;

pub const PLAYERS: usize = 11;

#[derive(Debug, Clone, Copy)]
pub struct Lineup {
  pub formation: Formation,
  pub players: [Player; PLAYERS],
}

impl HttpClient {
  pub async fn get_lineup(
    &self,
    league_id: &str,
  ) -> Result<HttpResponse, HttpClientError> {
    let url = format!("/leagues/{}/lineup", league_id);
    let response = self.get(Method::GET, &url, None).await?;
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
