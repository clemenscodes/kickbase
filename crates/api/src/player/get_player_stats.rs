use crate::{HttpClient, HttpClientError, HttpResponse};
use serde_json::Value;

impl HttpClient {
  pub async fn get_player_stats(
    &self,
    league_id: &str,
    player_id: &str,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let url = format!("/leagues/{league_id}/players/{player_id}/stats");
    let response = self.get(&url).await?;
    Ok(response)
  }
}
