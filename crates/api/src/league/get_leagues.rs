use reqwest::Method;
use serde_json::Value;

use crate::{league::League, HttpClient, HttpClientError};

impl HttpClient {
  pub async fn get_leagues(&self) -> Result<Vec<League>, HttpClientError> {
    let response = self.get::<Value>(Method::GET, "/leagues").await?;

    let leagues = response
      .value
      .get("leagues")
      .unwrap()
      .as_array()
      .unwrap()
      .iter()
      .map(League::from)
      .collect();

    Ok(leagues)
  }
}

#[cfg(test)]
mod tests {
  use crate::tests::get_test_client;

  #[ignore]
  #[tokio::test]
  async fn test_get_leagues() {
    let client = get_test_client();
    let result = client.get_leagues().await.unwrap();
    println!("{result:#?}");
  }
}
