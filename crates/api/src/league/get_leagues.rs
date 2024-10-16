use crate::{league::League, HttpClient, HttpClientError};
use reqwest::Method;

impl HttpClient {
  pub async fn get_leagues(&self) -> Result<Vec<League>, HttpClientError> {
    let response = self.get(Method::GET, "/leagues", None).await.unwrap();

    let leagues: Vec<League> = response
      .value
      .get("leagues")
      .unwrap()
      .as_array()
      .unwrap()
      .iter()
      .map(|league| {
        let name = league.get("name").unwrap().to_string().replace("\"", "");
        let id = league.get("id").unwrap().to_string().replace("\"", "");
        let creator =
          league.get("creator").unwrap().to_string().replace("\"", "");
        let creation = league
          .get("creation")
          .unwrap()
          .to_string()
          .replace("\"", "");
        let image = league
          .get("ci")
          .map(|value| value.to_string().replace("\"", ""))
          .unwrap_or_default();

        League {
          id,
          name,
          creator,
          creation,
          image,
        }
      })
      .collect();

    Ok(leagues)
  }
}

#[cfg(test)]
mod tests {
  use crate::tests::get_test_client;

  #[tokio::test]
  async fn test_get_leagues() {
    let client = get_test_client();
    let result = client.get_leagues().await;
    assert!(result.is_ok())
  }
}
