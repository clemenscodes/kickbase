use reqwest::Method;
use serde_json::Value;

use crate::{league::League, HttpClient, HttpClientError};

impl From<&Value> for League {
  fn from(value: &Value) -> Self {
    let name = value
      .get("name")
      .unwrap()
      .as_str()
      .unwrap_or_default()
      .to_string();

    let id = value
      .get("id")
      .unwrap()
      .as_str()
      .unwrap_or_default()
      .to_string();

    let creator = value
      .get("creator")
      .unwrap()
      .as_str()
      .unwrap_or_default()
      .to_string();

    let creation = value
      .get("creation")
      .unwrap()
      .as_str()
      .unwrap_or_default()
      .to_string();

    let image = value
      .get("ci")
      .map(|v| v.as_str().unwrap_or_default().to_string())
      .unwrap_or_default();

    League {
      id,
      name,
      creator,
      creation,
      image,
    }
  }
}

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
