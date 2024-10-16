use crate::{HttpClient, HttpClientError, HttpResponse};
use reqwest::Method;

impl HttpClient {
  pub async fn get_player_stats(
    &self,
    league_id: &str,
    player_id: &str,
  ) -> Result<HttpResponse, HttpClientError> {
    let url = format!("/leagues/{}/players/{}/stats", league_id, player_id);
    let response = self.get(Method::GET, &url, None).await?;
    Ok(response)
  }
}
