use serde_json::Value;

use crate::{HttpClient, HttpClientError, HttpResponse};

impl HttpClient {
  pub async fn get_players_for_match_day(
    &self,
    league_id: &str,
    user_id: &str,
    match_day: u64,
  ) -> Result<HttpResponse<Value>, HttpClientError> {
    let url = format!(
      "/leagues/{league_id}/users/{user_id}/players?matchDay={match_day}"
    );
    let response = self.get(&url).await?;
    Ok(response)
  }
}
