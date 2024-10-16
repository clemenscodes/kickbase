use reqwest::Method;

use crate::{HttpClient, HttpClientError, HttpResponse};

impl HttpClient {
  pub async fn get_player_feed(
    &self,
    league_id: &str,
    player_id: &str,
  ) -> Result<HttpResponse, HttpClientError> {
    let url = format!("/leagues/{}/players/{}/feed", league_id, player_id);
    let response = self.get(Method::GET, &url, None).await?;
    Ok(response)
  }
}
