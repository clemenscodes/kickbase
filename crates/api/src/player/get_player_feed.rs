use serde_json::Value;

use crate::{HttpClient, HttpClientError, HttpResponse};

impl HttpClient {
  pub async fn get_player_feed(
    &self,
    league_id: &str,
    player_id: &str,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let url = format!("/leagues/{league_id}/players/{player_id}/feed");
    let response = self.get(&url).await?;
    Ok(response)
  }
}
