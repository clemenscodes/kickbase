use super::{HttpClient, HttpClientError};
use crate::HttpResponse;
use reqwest::Method;

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
  use crate::tests::TEST_LEAGUE_ID;
  use crate::KICKBASE;

  #[tokio::test]
  async fn test_get_lineup() {
    let result = KICKBASE.read().await.get_lineup(TEST_LEAGUE_ID).await;
    assert!(result.is_ok());

    let lineup = result.unwrap();
    dbg!(&lineup);
  }
}
